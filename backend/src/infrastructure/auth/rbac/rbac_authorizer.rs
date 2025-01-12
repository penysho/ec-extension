use async_trait::async_trait;
use sea_orm::ConnectionTrait;

use crate::{
    domain::error::error::DomainError,
    infrastructure::db::{
        sea_orm::sea_orm_manager::SeaOrmTransactionManager,
        transaction_manager_interface::TransactionManager,
    },
    usecase::authorizer::authorizer_interface::{Action, Authorizer, Resource},
};

/// Authorization by RBAC.
pub struct RbacAuthorizer {
    transaction_manager: SeaOrmTransactionManager,
}

impl RbacAuthorizer {
    pub fn new(transaction_manager: SeaOrmTransactionManager) -> Self {
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
        let sql = r#"
            INSERT INTO test (id, name) VALUES (4, 'test10')
        "#;

        self.transaction_manager
            .get_transaction()
            .await
            .map_err(|e| {
                log::error!("Failed to get transaction: {}", e);
                DomainError::SystemError
            })?
            .as_ref()
            .unwrap()
            .execute_unprepared(sql)
            .await
            .map_err(|e| {
                log::error!("Failed to execute SQL: {}", e);
                DomainError::SystemError
            })?;

        Ok(())
    }
}
