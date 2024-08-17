use crate::entity::error::error::DomainError;
use crate::entity::product::product::Product;
use async_trait::async_trait;

/// Interactor interface for products.
#[async_trait]
pub trait ProductInteractorInterface {
    async fn get_product(&self, id: &str) -> Result<Option<Product>, DomainError>;
    async fn get_products(&self) -> Result<Vec<Product>, DomainError>;
}
