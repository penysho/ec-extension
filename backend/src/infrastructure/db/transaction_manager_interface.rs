use crate::domain::error::error::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionManager {
    type Transaction;

    async fn begin(&self) -> Result<Self::Transaction, DomainError>;

    async fn commit(&self, transaction: Self::Transaction) -> Result<(), DomainError>;

    async fn rollback(&self, transaction: Self::Transaction) -> Result<(), DomainError>;
}
