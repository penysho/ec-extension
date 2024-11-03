use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError,
    product::product::{Id, Product},
};

#[async_trait]
pub trait ProductQueryService: Send + Sync {
    async fn search_related_products(&self, id: &Id) -> Result<Vec<Product>, DomainError>;
}
