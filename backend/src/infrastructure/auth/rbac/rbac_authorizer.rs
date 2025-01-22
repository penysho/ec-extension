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
                "User is not authorized. user_id: {}, resource: {:?}, action: {:?}",
                user_id,
                resource,
                action
            );
            return Err(DomainError::SystemError);
        }

        Ok(())
    }
}
