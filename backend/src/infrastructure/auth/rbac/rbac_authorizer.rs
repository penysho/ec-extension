use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter};

use crate::{
    domain::error::error::DomainError,
    infrastructure::{
        db::{
            model::{
                prelude::{Permission, RoleResoucePermission, UserRole},
                role_resouce_permission, user_role,
            },
            transaction_manager_interface::TransactionManager,
        },
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
    usecase::{
        authorizer::authorizer_interface::{Action, Authorizer, Resource, ResourceType},
        user::UserInterface,
    },
};

use super::schema::DetailAction;

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
        user: Arc<dyn UserInterface>,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), DomainError> {
        let role_query = UserRole::find().filter(user_role::Column::UserId.eq(user.id()));
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
                user.id(),
                e
            );
            InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
        })?;

        if roles.is_empty() {
            log::error!("User has no role. user_id: {}", user.id(),);
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
                user.id(),
                e
            );
            InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
        })?;

        if !role_resource_permission.iter().any(|permission| {
            let ok = match ResourceType::try_from(permission.0.resource_id) {
                Ok(permission_resource) => permission_resource == *resource.resource_type(),
                Err(_) => false,
            } && match permission.1.clone().unwrap().action.parse::<DetailAction>() {
                Ok(allowed_detail_action) => {
                    if allowed_detail_action.is_own_action()
                        && resource.owner_user_id().is_some()
                        && resource.owner_user_id().as_ref().unwrap() != user.id()
                    {
                        // If the action is an own action and the owner is different, it is not allowed.
                        return false;
                    }
                    let allowed_actions = allowed_detail_action.to_actions();

                    allowed_actions.contains(action)
                }
                Err(_) => false,
            };

            ok
        }) {
            log::error!(
                "User is not authorized. user_id: {}, resource: {}, action: {}",
                user.id(),
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
            auth::{idp_user::IdpUser, rbac::schema::DetailAction},
            config::config::DatabaseConfig,
            db::{
                model::{permission, role, role_resouce_permission, user, user_role},
                sea_orm::sea_orm_manager::{SeaOrmConnectionProvider, SeaOrmTransactionManager},
                transaction_manager_interface::TransactionManager,
            },
        },
        usecase::{
            authorizer::authorizer_interface::{Action, Authorizer, Resource, ResourceType},
            user::UserInterface,
        },
    };

    use super::RbacAuthorizer;

    const ADMIN_ROLE_ID: i32 = 1;

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

    /// Insert an admin user into the database.
    async fn insert_admin_user(
        transaction: &DatabaseTransaction,
        user: Arc<dyn UserInterface>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let user_id = user.id();

        let admin_user = user::ActiveModel {
            id: Set(user_id.to_string()),
            name: Set("name".to_string()),
        };
        admin_user.insert(transaction).await?;

        let admin_user_role = user_role::ActiveModel {
            id: Set(rng.gen_range(1000..10000)),
            user_id: Set(user_id.to_string()),
            role_id: Set(ADMIN_ROLE_ID),
        };
        admin_user_role.insert(transaction).await?;

        Ok(())
    }

    /// Insert a custom user into the database.
    /// The user has a custom role and permission.
    async fn insert_custom_user(
        transaction: &DatabaseTransaction,
        user: Arc<dyn UserInterface>,
        resource: &Resource,
        detail_action: &DetailAction,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let user_id = user.id();

        let custom_user = user::ActiveModel {
            id: Set(user_id.to_string()),
            name: Set("name".to_string()),
        };
        custom_user.insert(transaction).await?;

        let custom_role_id = rng.gen_range(1000..10000);
        let custom_role = role::ActiveModel {
            id: Set(custom_role_id),
            name: Set("custom".to_string()),
        };
        custom_role.insert(transaction).await?;

        let custom_user_role = user_role::ActiveModel {
            id: Set(rng.gen_range(1000..10000)),
            user_id: Set(user_id.to_string()),
            role_id: Set(custom_role_id),
        };
        custom_user_role.insert(transaction).await?;

        let permission_id = rng.gen_range(1000..10000);
        let custom_permission = permission::ActiveModel {
            id: Set(permission_id),
            action: Set(detail_action.to_string()),
        };
        custom_permission.insert(transaction).await?;

        let custom_role_resource_permission = role_resouce_permission::ActiveModel {
            id: Set(rng.gen_range(1000..10000)),
            role_id: Set(custom_role_id),
            resource_id: Set(resource.resource_type().clone() as i32),
            permission_id: Set(permission_id),
        };
        custom_role_resource_permission.insert(transaction).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_authorize_with_admin_user_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user = Arc::new(IdpUser {
            id: Alphanumeric.sample_string(&mut rng, 10),
            email: "example@example.com".to_string(),
        }) as Arc<dyn UserInterface>;
        let resource = Resource::new(ResourceType::Product, None);
        let action = Action::Read;

        insert_admin_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            user.clone(),
        )
        .await
        .expect("Failed to insert test data");

        let result = authorizer.authorize(user, &resource, &action).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_non_admin_user_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user = Arc::new(IdpUser {
            id: Alphanumeric.sample_string(&mut rng, 10),
            email: "example@example.com".to_string(),
        }) as Arc<dyn UserInterface>;
        let resource = Resource::new(ResourceType::Product, None);
        let action = Action::Read;

        insert_custom_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            user.clone(),
            &resource,
            &DetailAction::AllRead,
        )
        .await
        .expect("Failed to insert test data");

        let result = authorizer.authorize(user, &resource, &action).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_owner_user_id_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user = Arc::new(IdpUser {
            id: Alphanumeric.sample_string(&mut rng, 10),
            email: "example@example.com".to_string(),
        }) as Arc<dyn UserInterface>;
        let resource = Resource::new(ResourceType::Product, Some(user.id().to_string()));
        let action = Action::Delete;

        insert_custom_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            user.clone(),
            &resource,
            &DetailAction::OwnDelete,
        )
        .await
        .expect("Failed to insert test data");

        let result = authorizer.authorize(user, &resource, &action).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_user_not_found() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user = Arc::new(IdpUser {
            id: Alphanumeric.sample_string(&mut rng, 10),
            email: "example@example.com".to_string(),
        }) as Arc<dyn UserInterface>;
        let resource = Resource::new(ResourceType::Product, None);
        let action = Action::Read;

        let result = authorizer.authorize(user, &resource, &action).await;

        assert!(result.is_err());
        if let Err(DomainError::SystemError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::SystemError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_authorize_with_no_permission() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user = Arc::new(IdpUser {
            id: Alphanumeric.sample_string(&mut rng, 10),
            email: "example@example.com".to_string(),
        }) as Arc<dyn UserInterface>;
        let resource = Resource::new(ResourceType::Product, None);
        let action = Action::Delete;

        insert_custom_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            user.clone(),
            &resource,
            &DetailAction::OwnRead, // This action is not allowed.
        )
        .await
        .expect("Failed to insert test data");

        let result = authorizer.authorize(user, &resource, &action).await;

        assert!(result.is_err());
        if let Err(DomainError::AuthorizationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::AuthorizationError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_authorize_with_different_owner_user_id() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let mut rng = rand::thread_rng();
        let user = Arc::new(IdpUser {
            id: Alphanumeric.sample_string(&mut rng, 10),
            email: "example@example.com".to_string(),
        }) as Arc<dyn UserInterface>;
        let resource = Resource::new(ResourceType::Product, Some("another_user_id".to_string())); // Different owner user ID
        let action = Action::Delete;

        insert_custom_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            user.clone(),
            &resource,
            &DetailAction::OwnDelete,
        )
        .await
        .expect("Failed to insert test data");

        let result = authorizer.authorize(user, &resource, &action).await;

        assert!(result.is_err());
        if let Err(DomainError::AuthorizationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::AuthorizationError, but got something else");
        }
    }
}
