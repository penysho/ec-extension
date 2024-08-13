use async_trait::async_trait;

use crate::{
    entity::{error::error::DomainError, product::product::Product},
    usecase::{
        interactor::product_interactor_interface::ProductInteractorInterface,
        repository::product_repository_interface::ProductRepository,
    },
};

/// Product Interactor.
pub struct ProductInteractorImpl {
    product_repository: Box<dyn ProductRepository>,
}

impl ProductInteractorImpl {
    pub fn new(product_repository: Box<dyn ProductRepository>) -> Self {
        Self {
            product_repository: product_repository,
        }
    }
}

#[async_trait]
impl ProductInteractorInterface for ProductInteractorImpl {
    /// Obtain a list of products
    async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        self.product_repository.get_products().await
    }
}
