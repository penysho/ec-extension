use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction};

use crate::{
    domain::error::error::DomainError,
    infrastructure::db::transaction_manager_interface::TransactionManager,
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
        let sql = r#"
            INSERT INTO test (id, name) VALUES (7, 'test10')
        "#;

        if self.transaction_manager.is_transaction_started().await {
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
        } else {
            let conn = self
                .transaction_manager
                .get_connection()
                .await
                .map_err(|e| {
                    log::error!("Failed to get connection: {}", e);
                    DomainError::SystemError
                })?;

            conn.execute_unprepared(sql).await.map_err(|e| {
                log::error!("Failed to execute SQL: {}", e);
                DomainError::SystemError
            })?;
        }

        Ok(())
    }
}
