use crate::entity::error::error::DomainError;
use crate::entity::product::product::Product;
use async_trait::async_trait;
use mockall::automock;

/// Interactor interface for products.

#[automock]
#[async_trait]
pub trait ProductInteractor {
    async fn get_product(&self, id: &str) -> Result<Option<Product>, DomainError>;
    async fn get_products(&self) -> Result<Vec<Product>, DomainError>;
}
