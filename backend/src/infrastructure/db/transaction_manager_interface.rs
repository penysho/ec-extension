use crate::domain::error::error::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionManager {
    type Transaction: Send + Sync;

    /// Get allocated transaction.
    async fn get_transaction(&mut self) -> Result<&mut Self::Transaction, DomainError>;

    /// Commit transaction.
    async fn commit(&mut self) -> Result<(), DomainError>;

    /// Roll back a transaction.
    async fn rollback(&mut self) -> Result<(), DomainError>;
}
