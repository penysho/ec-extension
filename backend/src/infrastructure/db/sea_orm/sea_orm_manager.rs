use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use std::sync::Arc;

use crate::{
    domain::error::error::DomainError,
    infrastructure::db::transaction_manager_interface::TransactionManager,
};

pub struct SeaOrmTransactionManager {
    conn: Arc<DatabaseConnection>,
}

impl SeaOrmTransactionManager {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

pub struct SeaOrmTransaction {
    pub(crate) transaction: DatabaseTransaction,
}

#[async_trait]
impl TransactionManager for SeaOrmTransactionManager {
    type Transaction = SeaOrmTransaction;

    async fn begin(&self) -> Result<Self::Transaction, DomainError> {
        let transaction = self
            .conn
            .begin()
            .await
            .map_err(|_| DomainError::SystemError)?;
        Ok(SeaOrmTransaction { transaction })
    }

    async fn commit(&self, transaction: Self::Transaction) -> Result<(), DomainError> {
        transaction
            .transaction
            .commit()
            .await
            .map_err(|_| DomainError::SystemError)
    }

    async fn rollback(&self, transaction: Self::Transaction) -> Result<(), DomainError> {
        transaction
            .transaction
            .rollback()
            .await
            .map_err(|_| DomainError::SystemError)
    }
}
