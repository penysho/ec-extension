use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseTransaction};

use crate::{
    domain::error::error::DomainError,
    infrastructure::db::transaction_manager_interface::TransactionManager,
    usecase::authorizer::authorizer_interface::{Action, Authorizer, Resource},
};

/// Authorization by RBAC.
pub struct RbacAuthorizer {
    transaction_manager: Arc<dyn TransactionManager<DatabaseTransaction>>,
}

impl RbacAuthorizer {
    pub fn new(transaction_manager: Arc<dyn TransactionManager<DatabaseTransaction>>) -> Self {
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
        Ok(())
    }
}
