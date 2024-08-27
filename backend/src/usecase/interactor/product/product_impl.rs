use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, product::product::Product},
    usecase::{
        interactor::product_interactor_interface::ProductInteractor,
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
impl ProductInteractor for ProductInteractorImpl {
    /// Obtain detailed product information.
    async fn get_product(&self, id: &str) -> Result<Option<Product>, DomainError> {
        self.product_repository.get_product(id).await
    }
    /// Obtain a list of products.
    async fn get_products(&self) -> Result<Vec<Product>, DomainError> {
        self.product_repository.get_products().await
    }
}
