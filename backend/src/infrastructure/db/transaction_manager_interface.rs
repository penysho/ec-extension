use crate::domain::error::error::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionManager {
    type Transaction;

    /// Initiating a transaction.
    async fn begin(&self) -> Result<Self::Transaction, DomainError>;

    /// Commit transaction.
    async fn commit(&self, transaction: Self::Transaction) -> Result<(), DomainError>;

    /// Roll back a transaction.
    async fn rollback(&self, transaction: Self::Transaction) -> Result<(), DomainError>;
}
