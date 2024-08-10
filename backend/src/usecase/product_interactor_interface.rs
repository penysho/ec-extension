use crate::entity::error::error::DomainError;
use crate::entity::product::product::Product;
use async_trait::async_trait;

#[async_trait]
pub trait ProductInteractorInterface: Send + Sync {
    async fn get_products(&self) -> Result<Vec<Product>, DomainError>;
}
