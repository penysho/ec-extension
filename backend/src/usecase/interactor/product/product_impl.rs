use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        media::media::Media,
        product::product::{Id, Product},
    },
    usecase::{
        interactor::product_interactor_interface::ProductInteractor,
        repository::{
            media_repository_interface::MediaRepository,
            product_repository_interface::ProductRepository,
        },
    },
};

/// Product Interactor.
pub struct ProductInteractorImpl {
    product_repository: Box<dyn ProductRepository>,
    media_repository: Box<dyn MediaRepository>,
}

impl ProductInteractorImpl {
    pub fn new(
        product_repository: Box<dyn ProductRepository>,
        media_repository: Box<dyn MediaRepository>,
    ) -> Self {
        Self {
            product_repository: product_repository,
            media_repository: media_repository,
        }
    }
}

#[async_trait]
impl ProductInteractor for ProductInteractorImpl {
    /// Obtain detailed product information.
    async fn get_product(&self, id: &str) -> Result<Product, DomainError> {
        self.product_repository.get_product(id).await
    }
    /// Obtain a list of products.
    async fn get_products(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Product>, DomainError> {
        let products = self.product_repository.get_products(limit, offset).await;
        // let product_ids: Vec<Id> = products?.iter().map(|p| p.id().clone()).collect();

        products
    }
}
