use async_trait::async_trait;

use crate::{
    entity::{error::error::DomainError, product::product::Product},
    usecase::repository::product_repository_interface::ProductRepository,
};

pub struct ProductRepositoryImpl;

impl ProductRepositoryImpl {
    pub fn new() -> Self {
        ProductRepositoryImpl
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        let dummy_product = Product::new(
            "1".to_string(),
            "Product 1".to_string(),
            100,
            "This is a dummy product from shopify.".to_string(),
        );
        Ok(vec![dummy_product])
    }
}
