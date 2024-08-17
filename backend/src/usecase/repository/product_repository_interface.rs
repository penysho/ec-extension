use async_trait::async_trait;

use crate::entity::{error::error::DomainError, product::product::Product};

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn get_product(&self, id: &str) -> Result<Option<Product>, DomainError>;
    async fn get_products(&self) -> Result<Vec<Product>, DomainError>;
}
