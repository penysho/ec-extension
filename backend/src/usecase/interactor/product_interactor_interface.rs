use crate::domain::error::error::DomainError;
use crate::domain::product::product::{Id as ProductId, Product};
use async_trait::async_trait;
use mockall::automock;

/// Interactor interface for products.

#[automock]
#[async_trait]
pub trait ProductInteractor {
    async fn get_product(&self, id: &ProductId) -> Result<Product, DomainError>;
    async fn get_products(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Product>, DomainError>;
}
