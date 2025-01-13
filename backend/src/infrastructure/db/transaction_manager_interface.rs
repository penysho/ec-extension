use crate::domain::error::error::DomainError;
use async_trait::async_trait;
use tokio::sync::MutexGuard;

#[async_trait]
pub trait TransactionManager<T>: Send + Sync {
    /// Start a transaction.
    async fn begin(&self) -> Result<(), DomainError>;

    /// Get current transaction.
    async fn get_transaction(&self) -> Result<MutexGuard<'_, Option<T>>, DomainError>;

    /// Commit transaction.
    async fn commit(&self) -> Result<(), DomainError>;

    /// Roll back a transaction.
    async fn rollback(&self) -> Result<(), DomainError>;
}
