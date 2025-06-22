use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use async_trait::async_trait;
use sea_orm::{
    ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, FromQueryResult,
    QueryFilter, Statement,
};

use crate::{
    domain::{
        authorized_resource::authorized_resource::{
            AuthorizedResource, ResourceAction, ResourceType,
        },
        error::error::DomainError,
        user::user::{Role, UserAction, UserInterface},
    },
    infrastructure::{
        db::{
            model::{
                prelude::{Permission, RoleResourcePermission},
                role_resource_permission,
            },
            transaction_manager_interface::TransactionManager,
        },
        error::{InfrastructureError, InfrastructureErrorMapper},
    },
    log_error,
    usecase::auth::authorizer_interface::Authorizer,
};

/// Result structure for combined role query
#[derive(Debug, FromQueryResult)]
struct RoleIdResult {
    role_id: i32,
}

/// Authorization by RBAC.
#[derive(Clone)]
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

    /// Get all role ids for a user (both direct roles and group roles) in a single query.
    /// This method combines user_role and user_group_role queries using UNION for efficiency.
    async fn get_all_user_role_ids(&self, user_id: &str) -> Result<Vec<i32>, DomainError> {
        // Build the raw SQL query using UNION to combine direct user roles and group roles
        let sql = r#"
            SELECT role_id FROM user_role WHERE user_id = $1
            UNION
            SELECT ugr.role_id
            FROM user_group_role ugr
            INNER JOIN user_user_group uug ON ugr.user_group_id = uug.user_group_id
            WHERE uug.user_id = $1
        "#;

        let stmt = Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            sql,
            vec![user_id.into()],
        );

        let role_results = if self.transaction_manager.is_transaction_started().await {
            RoleIdResult::find_by_statement(stmt)
                .all(
                    self.transaction_manager
                        .get_transaction()
                        .await?
                        .as_ref()
                        .ok_or(DomainError::SystemError)?,
                )
                .await
        } else {
            RoleIdResult::find_by_statement(stmt)
                .all(self.transaction_manager.get_connection().await?.as_ref())
                .await
        }
        .map_err(|e| {
            log_error!("Failed to get user roles.", "user_id" => user_id, "error" => e);
            InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
        })?;

        Ok(role_results.into_iter().map(|r| r.role_id).collect())
    }

    /// Get the permissions of the user.
    async fn get_user_permissions(
        &self,
        role_ids: Vec<i32>,
    ) -> Result<HashMap<ResourceType, HashSet<UserAction>>, DomainError> {
        let permission_query = RoleResourcePermission::find()
            .find_also_related(Permission)
            .filter(role_resource_permission::Column::RoleId.is_in(role_ids));
        let role_resource_permissions =
            if self.transaction_manager.is_transaction_started().await {
                permission_query
                    .all(
                        self.transaction_manager
                            .get_transaction()
                            .await?
                            .as_ref()
                            .ok_or(DomainError::SystemError)?,
                    )
                    .await
            } else {
                permission_query
                    .all(self.transaction_manager.get_connection().await?.as_ref())
                    .await
            }
            .map_err(|e| {
                log_error!("Failed to get role resource permissions.", "error" => e);
                InfrastructureErrorMapper::to_domain(InfrastructureError::DatabaseError(e))
            })?;

        let mut permission_map = HashMap::new();
        for (role_resource_permission, permission) in role_resource_permissions {
            let resource_type = ResourceType::try_from(role_resource_permission.resource_id)?;
            let user_action = permission.clone().unwrap().action.parse::<UserAction>()?;

            permission_map
                .entry(resource_type)
                .or_insert(HashSet::new())
                .insert(user_action);
        }

        Ok(permission_map)
    }

    /// Get the authorization of the user.
    pub async fn get_user_authorization(
        &self,
        user_id: &str,
    ) -> Result<(Vec<Role>, HashMap<ResourceType, HashSet<UserAction>>), DomainError> {
        let role_ids = self.get_all_user_role_ids(user_id).await?;

        // Convert to HashSet to remove duplicates (UNION should already handle this, but being safe)
        let unique_role_ids: HashSet<i32> = role_ids.into_iter().collect();

        let roles = unique_role_ids
            .iter()
            .map(|role_id| Role::try_from(*role_id))
            .collect::<Result<Vec<Role>, DomainError>>()?;

        let permissions = self
            .get_user_permissions(unique_role_ids.into_iter().collect())
            .await?;

        Ok((roles, permissions))
    }

    /// Get the authorization of the not login user.
    pub async fn get_not_login_user_authorization(
        &self,
    ) -> Result<(Vec<Role>, HashMap<ResourceType, HashSet<UserAction>>), DomainError> {
        let permissions = self
            .get_user_permissions(vec![Role::NotLogin as i32])
            .await?;

        Ok((vec![Role::NotLogin], permissions))
    }
}

#[async_trait]
impl Authorizer for RbacAuthorizer {
    async fn authorize<'a>(
        &self,
        user: Arc<dyn UserInterface>,
        resources: Vec<&'a dyn AuthorizedResource>,
        action: &ResourceAction,
    ) -> Result<(), DomainError> {
        Ok(for resource in resources {
            if !user.permissions().into_iter().any(|permission| {
                let is_resource_type_match = permission.0 == resource.resource_type();
                let is_action_match = permission.1.into_iter().any(|user_action| {
                    if user_action.is_own_action()
                        && resource.owner_user_id().is_some()
                        && resource.owner_user_id().as_ref().unwrap() != user.id()
                    {
                        // If the action is an own action and the owner is different, it is not allowed.
                        return false;
                    }

                    user_action.to_resource_actions().contains(action)
                });

                is_resource_type_match && is_action_match
            }) {
                log_error!(
                    "User is not authorized.",
                    "user_id" => user.id(),
                    "resource" => resource.resource_type(),
                    "owner_user_id" => resource.owner_user_id().unwrap_or_else(|| "".to_string()),
                    "action" => action
                );
                return Err(DomainError::AuthorizationError);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rand::{
        distributions::{Alphanumeric, DistString},
        Rng,
    };
    use sea_orm::{ActiveModelTrait, DatabaseTransaction, Set};

    use crate::{
        domain::{
            authorized_resource::authorized_resource::{
                AuthorizedResource, ResourceAction, ResourceType,
            },
            error::error::DomainError,
            user::user::{Id as UserId, Role, UserInterface},
        },
        infrastructure::{
            auth::{idp_user::IdpUser, rbac::rbac_authorizer::RbacAuthorizer},
            config::config::{DatabaseConfig, Env},
            db::{
                model::{user, user_role},
                sea_orm::sea_orm_manager::{SeaOrmConnectionProvider, SeaOrmTransactionManager},
                transaction_manager_interface::TransactionManager,
            },
            secret::secrets_manager::SecretsManagerClient,
        },
        usecase::auth::authorizer_interface::Authorizer,
    };

    async fn transaction_manager() -> SeaOrmTransactionManager {
        let aws_sdk_config = aws_config::load_from_env().await;
        let secrets_client = SecretsManagerClient::new(&aws_sdk_config).await.unwrap();

        let connection_provider = SeaOrmConnectionProvider::new(
            DatabaseConfig::new(&secrets_client, &Env::Local)
                .await
                .expect("Failed to get database config"),
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

    /// Insert an user into the database.
    async fn insert_user(
        transaction: &DatabaseTransaction,
        role: &Role,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();
        let user_id = Alphanumeric.sample_string(&mut rng, 10);

        let user = user::ActiveModel {
            id: Set(user_id.to_string()),
            name: Set("name".to_string()),
        };
        user.insert(transaction).await?;

        let user_role = user_role::ActiveModel {
            id: Set(rng.gen_range(1000..10000)),
            user_id: Set(user_id.to_string()),
            role_id: Set(role.clone() as i32),
        };
        user_role.insert(transaction).await?;

        Ok(user_id)
    }

    /// Insert a user into the database via group membership.
    async fn insert_user_via_group(
        transaction: &DatabaseTransaction,
        role: &Role,
    ) -> Result<String, Box<dyn std::error::Error>> {
        use crate::infrastructure::db::model::{user_group, user_group_role, user_user_group};

        let mut rng = rand::thread_rng();
        let user_id = Alphanumeric.sample_string(&mut rng, 10);
        let group_id = rng.gen_range(1000..10000);

        // Create user
        let user = user::ActiveModel {
            id: Set(user_id.to_string()),
            name: Set("group_user".to_string()),
        };
        user.insert(transaction).await?;

        // Create user group
        let user_group = user_group::ActiveModel {
            id: Set(group_id),
            name: Set(format!("test_group_{}", group_id)),
        };
        user_group.insert(transaction).await?;

        // Assign role to group
        let user_group_role = user_group_role::ActiveModel {
            id: Set(rng.gen_range(1000..10000)),
            user_group_id: Set(group_id),
            role_id: Set(role.clone() as i32),
        };
        user_group_role.insert(transaction).await?;

        // Add user to group
        let user_user_group = user_user_group::ActiveModel {
            id: Set(rng.gen_range(1000..10000)),
            user_id: Set(user_id.to_string()),
            user_group_id: Set(group_id),
        };
        user_user_group.insert(transaction).await?;

        Ok(user_id)
    }

    async fn user_interface(authorizer: &RbacAuthorizer, user_id: &str) -> Arc<dyn UserInterface> {
        let (roles, permissions) = authorizer
            .get_user_authorization(user_id)
            .await
            .expect("Failed to get user authorization");

        Arc::new(IdpUser::new(
            user_id.to_string(),
            "example@example.com".to_string(),
            roles,
            permissions,
        ))
    }

    struct MockProduct {
        owner_user_id: Option<UserId>,
    }
    impl AuthorizedResource for MockProduct {
        fn resource_type(&self) -> ResourceType {
            ResourceType::Product
        }

        fn owner_user_id(&self) -> Option<UserId> {
            self.owner_user_id.clone()
        }
    }

    struct MockOrder {
        owner_user_id: Option<UserId>,
    }
    impl AuthorizedResource for MockOrder {
        fn resource_type(&self) -> ResourceType {
            ResourceType::Order
        }

        fn owner_user_id(&self) -> Option<UserId> {
            self.owner_user_id.clone()
        }
    }

    #[tokio::test]
    async fn test_authorize_with_admin_user_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Admin,
        )
        .await
        .expect("Failed to insert test data");

        let resource = vec![&MockProduct {
            owner_user_id: None,
        } as &dyn AuthorizedResource];
        let action = ResourceAction::Read;

        let result = authorizer
            .authorize(
                user_interface(&authorizer, &user_id).await,
                resource,
                &action,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_admin_user_with_multiple_resources_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Admin,
        )
        .await
        .expect("Failed to insert test data");

        let binding = MockProduct {
            owner_user_id: Some(user_id.to_string()),
        };
        let resource = vec![
            &MockProduct {
                owner_user_id: None,
            } as &dyn AuthorizedResource,
            &binding as &dyn AuthorizedResource,
        ];
        let action = ResourceAction::Read;

        let result = authorizer
            .authorize(
                user_interface(&authorizer, &user_id).await,
                resource,
                &action,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_non_admin_user_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Operator,
        )
        .await
        .expect("Failed to insert test data");

        let user = user_interface(&authorizer, &user_id).await;
        let resource = vec![&MockProduct {
            owner_user_id: None,
        } as &dyn AuthorizedResource];
        let action = ResourceAction::Read;

        let result = authorizer.authorize(user, resource, &action).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_owner_user_id_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Operator,
        )
        .await
        .expect("Failed to insert test data");

        let user = user_interface(&authorizer, &user_id).await;
        let binding = MockProduct {
            owner_user_id: Some(user_id.to_string()),
        };
        let resource = vec![&binding as &dyn AuthorizedResource];
        let action = ResourceAction::Delete;

        let result = authorizer.authorize(user, resource, &action).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_no_permission() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Customer,
        )
        .await
        .expect("Failed to insert test data");

        let resource = vec![&MockProduct {
            owner_user_id: None,
        } as &dyn AuthorizedResource];
        let action = ResourceAction::Delete; // The action is not allowed.

        let result = authorizer
            .authorize(
                user_interface(&authorizer, &user_id).await,
                resource,
                &action,
            )
            .await;

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

        let user_id = insert_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Customer,
        )
        .await
        .expect("Failed to insert test data");

        let binding = MockOrder {
            owner_user_id: Some("another_user_id".to_string()), // The owner is different.
        };
        let resource = vec![&binding as &dyn AuthorizedResource];
        let action = ResourceAction::Delete;

        let result = authorizer
            .authorize(
                user_interface(&authorizer, &user_id).await,
                resource,
                &action,
            )
            .await;

        assert!(result.is_err());
        if let Err(DomainError::AuthorizationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::AuthorizationError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_authorize_with_multiple_resources_failed() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Customer,
        )
        .await
        .expect("Failed to insert test data");

        let binding1 = MockProduct {
            owner_user_id: Some(user_id.to_string()),
        };
        let binding2 = MockOrder {
            owner_user_id: Some("another_user_id".to_string()), // The owner is different.
        };
        let resource = vec![
            &binding1 as &dyn AuthorizedResource,
            &binding2 as &dyn AuthorizedResource,
        ];
        let action = ResourceAction::Delete;

        let result = authorizer
            .authorize(
                user_interface(&authorizer, &user_id).await,
                resource,
                &action,
            )
            .await;

        assert!(result.is_err());
        if let Err(DomainError::AuthorizationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::AuthorizationError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_authorize_with_group_admin_user_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user_via_group(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Admin,
        )
        .await
        .expect("Failed to insert test data");

        let resource = vec![&MockProduct {
            owner_user_id: None,
        } as &dyn AuthorizedResource];
        let action = ResourceAction::Read;

        let result = authorizer
            .authorize(
                user_interface(&authorizer, &user_id).await,
                resource,
                &action,
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_group_operator_user_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user_via_group(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Operator,
        )
        .await
        .expect("Failed to insert test data");

        let user = user_interface(&authorizer, &user_id).await;
        let resource = vec![&MockProduct {
            owner_user_id: None,
        } as &dyn AuthorizedResource];
        let action = ResourceAction::Read;

        let result = authorizer.authorize(user, resource, &action).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_group_owner_user_id_success() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user_via_group(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Operator,
        )
        .await
        .expect("Failed to insert test data");

        let user = user_interface(&authorizer, &user_id).await;
        let binding = MockProduct {
            owner_user_id: Some(user_id.to_string()),
        };
        let resource = vec![&binding as &dyn AuthorizedResource];
        let action = ResourceAction::Delete;

        let result = authorizer.authorize(user, resource, &action).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authorize_with_group_customer_no_permission() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user_via_group(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Customer,
        )
        .await
        .expect("Failed to insert test data");

        let resource = vec![&MockProduct {
            owner_user_id: None,
        } as &dyn AuthorizedResource];
        let action = ResourceAction::Delete; // The action is not allowed.

        let result = authorizer
            .authorize(
                user_interface(&authorizer, &user_id).await,
                resource,
                &action,
            )
            .await;

        assert!(result.is_err());
        if let Err(DomainError::AuthorizationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::AuthorizationError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_authorize_with_group_different_owner_user_id() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user_via_group(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Customer,
        )
        .await
        .expect("Failed to insert test data");

        let binding = MockOrder {
            owner_user_id: Some("another_user_id".to_string()), // The owner is different.
        };
        let resource = vec![&binding as &dyn AuthorizedResource];
        let action = ResourceAction::Delete;

        let result = authorizer
            .authorize(
                user_interface(&authorizer, &user_id).await,
                resource,
                &action,
            )
            .await;

        assert!(result.is_err());
        if let Err(DomainError::AuthorizationError) = result {
            // Test passed
        } else {
            panic!("Expected DomainError::AuthorizationError, but got something else");
        }
    }

    #[tokio::test]
    async fn test_get_user_authorization_with_group_role() {
        let transaction_manager = transaction_manager().await;
        let authorizer = RbacAuthorizer::new(Arc::new(transaction_manager.clone()));

        let user_id = insert_user_via_group(
            transaction_manager
                .clone()
                .get_transaction()
                .await
                .unwrap()
                .as_ref()
                .unwrap(),
            &Role::Admin,
        )
        .await
        .expect("Failed to insert test data");

        // Test that get_user_authorization correctly retrieves group roles
        let (roles, _permissions) = authorizer
            .get_user_authorization(&user_id)
            .await
            .expect("Failed to get user authorization");

        // Should contain the Admin role from group membership
        assert!(roles.contains(&Role::Admin));
        assert_eq!(roles.len(), 1);
    }
}
