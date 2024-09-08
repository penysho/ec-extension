use async_trait::async_trait;

use crate::domain::{error::error::DomainError, product::product::Product};

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn get_product(&self, id: &str) -> Result<Product, DomainError>;
    async fn get_products(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Product>, DomainError>;
}
