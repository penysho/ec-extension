use crate::{
    entity::{error::error::DomainError, product::product::Product},
    usecase::product_interactor_interface::ProductInteractorInterface,
};

pub struct ProductInteractorImpl;

impl ProductInteractorImpl {
    pub fn new() -> Self {
        Self
    }
}

impl ProductInteractorInterface for ProductInteractorImpl {
    async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        let dummy_product = Product::new(
            "1".to_string(),
            "Product 1".to_string(),
            100,
            "This is a dummy product.".to_string(),
        );
        Ok(vec![dummy_product])
    }
}
