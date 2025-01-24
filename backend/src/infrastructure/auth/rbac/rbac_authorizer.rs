use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter};

use crate::{
    domain::error::error::DomainError,
    infrastructure::{
        db::{
            model::{
                prelude::Permission, prelude::RoleResoucePermission, prelude::UserRole,
                role_resouce_permission, user_role,
            },
            transaction_manager_interface::TransactionManager,
        },
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
    usecase::authorizer::authorizer_interface::{Action, Authorizer, Resource},
};

/// Authorization by RBAC.
pub struct RbacAuthorizer {
    transaction_manager: Arc<dyn TransactionManager<DatabaseTransaction, Arc<DatabaseConnection>>>,
}

impl RbacAuthorizer {
    /// Create a new instance.
    pub fn new(
        transaction_manager: Arc<
            dyn TransactionManager<DatabaseTransaction, Arc<DatabaseConnection>>,
        >,
    ) -> Self {
        Self {
            transaction_manager,
        }
    }
}

#[async_trait]
impl Authorizer for RbacAuthorizer {
    async fn authorize(
        &self,
        user_id: &str,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), DomainError> {
        let role_query = UserRole::find().filter(user_role::Column::UserId.eq(user_id));
        let roles = if self.transaction_manager.is_transaction_started().await {
            role_query
                .all(
                    self.transaction_manager
                        .get_transaction()
                        .await?
                        .as_ref()
                        .unwrap(),
                )
                .await
        } else {
            role_query
                .all(self.transaction_manager.get_connection().await?.as_ref())
                .await
        }
        .map_err(|e| {
            log::error!(
                "Failed to get user roles. user_id: {}, error: {:?}",
                user_id,
                e
            );
            InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
        })?;

        if roles.is_empty() {
            log::error!("User has no role. user_id: {}", user_id);
            return Err(DomainError::SystemError);
        }
        let role_ids: Vec<i32> = roles.iter().map(|role| role.role_id).collect();

        let permission_query = RoleResoucePermission::find()
            .find_also_related(Permission)
            .filter(role_resouce_permission::Column::RoleId.is_in(role_ids));
        let role_resource_permission = if self.transaction_manager.is_transaction_started().await {
            permission_query
                .all(
                    self.transaction_manager
                        .get_transaction()
                        .await?
                        .as_ref()
                        .unwrap(),
                )
                .await
        } else {
            permission_query
                .all(self.transaction_manager.get_connection().await?.as_ref())
                .await
        }
        .map_err(|e| {
            log::error!(
                "Failed to get role resource permissions. user_id: {}, error: {:?}",
                user_id,
                e
            );
            InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
        })?;

        if !role_resource_permission.iter().any(|permission| {
            let allow_resource = match Resource::try_from(permission.0.resource_id) {
                Ok(permission_resource) => permission_resource == *resource,
                Err(_) => false,
            };

            let allow_action = match permission.1.clone().unwrap().action.parse::<Action>() {
                Ok(permission_action) => {
                    permission_action == *action || permission_action == Action::All
                }
                Err(_) => false,
            };

            return allow_resource && allow_action;
        }) {
            log::error!(
                "User is not authorized. user_id: {}, resource: {}, action: {}",
                user_id,
                resource,
                action
            );
            return Err(DomainError::AuthorizationError);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{env, sync::Arc};

    use rand::{
        distributions::{Alphanumeric, DistString},
        Rng,
    };
    use sea_orm::{ActiveModelTrait, DatabaseTransaction, Set};

    use crate::{
        domain::error::error::DomainError,
        infrastructure::{
            config::config::DatabaseConfig,
            db::{
                model::{user, user_role},
                sea_orm::sea_orm_manager::{SeaOrmConnectionProvider, SeaOrmTransactionManager},
                transaction_manager_interface::TransactionManager,
            },
        },
        usecase::authorizer::authorizer_interface::{Action, Authorizer, Resource},
    };

    use super::RbacAuthorizer;

    const ADMIN_ROLE_ID: i32 = 1;
    const OPERATOR_ROLE_ID: i32 = 2;

    async fn transaction_manager() -> SeaOrmTransactionManager {
        env::set_var(
            "DATABASE_URL",
            "postgres://postgres:postgres@backend-db/postgres",
        );
        let connection_provider = SeaOrmConnectionProvider::new(
            DatabaseConfig::new().expect("Failed to get database config"),
        )
        .await
        .expect("Failed to get connection provider");

        let transaction_manager =
            SeaOrmTransactionManager::new(Arc::clone(&connection_provider.get_connection()))
                .await
                .expect("Failed to get transaction manager");

        transaction_manager
            .begin()
            .await
            .expect("Failed to begin transaction");

        transaction_manager
    }

    // Assume that non-user data such as roles and permissions are registered in the DB as master.
    async fn insert_authorization_data(
        transaction: &DatabaseTransaction,
        user_id: &str,
        role_id: i32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();

        let user = user::ActiveModel {
            id: Set(user_id.to_string()),
            name: Set("name".to_string()),
        };
        user.insert(transaction).await?;

        let user_role_id: i32 = rng.gen_range(1000..10000);
        let user_role = user_role::ActiveModel {
            id: Set(user_role_id),
            user_id: Set(user_id.to_string()),
            role_id: Set(role_id),
        };
        user_role.insert(transaction).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_authorize_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user_id = Alphanumeric.sample_string(&mut rng, 10);
        let resource = Resource::Product;
        let action = Action::Read;

        insert_authorization_data(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &user_id,
            ADMIN_ROLE_ID,
        )
        .await
        .expect("Failed to insert authorization data");

        let result = authorizer.authorize(&user_id, &resource, &action).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_user_not_found() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user_id = Alphanumeric.sample_string(&mut rng, 10);
        let resource = Resource::Product;
        let action = Action::Read;

        let result = authorizer.authorize(&user_id, &resource, &action).await;

        assert!(result.is_err());
        if let Err(DomainError::SystemError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::SystemError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_authorize_with_no_role() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user_id = Alphanumeric.sample_string(&mut rng, 10);
        let resource = Resource::Product;
        let action = Action::All;

        insert_authorization_data(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &user_id,
            OPERATOR_ROLE_ID,
        )
        .await
        .expect("Failed to insert authorization data");

        let result = authorizer.authorize(&user_id, &resource, &action).await;

        assert!(result.is_err());
        if let Err(DomainError::AuthorizationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::AuthorizationError, but got something else");
        }
    }
}
