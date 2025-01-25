use crate::domain::error::error::DomainError;
use async_trait::async_trait;
use tokio::sync::MutexGuard;

/// Transaction manager interface.
/// Manage transactions used by the application and provide for each use case.
///
/// # Parameters
/// - `T`: Transaction type.
/// - `C`: Connection type.
#[async_trait]
pub trait TransactionManager<T, C>: Send + Sync {
    /// Start a transaction.
    async fn begin(&self) -> Result<(), DomainError>;

    /// Returns whether or not a Transaction has been initiated.
    async fn is_transaction_started(&self) -> bool;

    /// Get current transaction.
    async fn get_transaction(&self) -> Result<MutexGuard<'_, Option<T>>, DomainError>;

    /// Get connection.
    async fn get_connection(&self) -> Result<C, DomainError>;

    /// Commit transaction.
    async fn commit(&self) -> Result<(), DomainError>;

    /// Roll back a transaction.
    async fn rollback(&self) -> Result<(), DomainError>;
}
