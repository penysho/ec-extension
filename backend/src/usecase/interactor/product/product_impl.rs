use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        media::media::Media,
        product::product::{Id as ProductId, Product},
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
    async fn get_product_with_media(
        &self,
        id: &ProductId,
    ) -> Result<(Product, Vec<Media>), DomainError> {
        let product_result = self.product_repository.get_product(id).await;
        let media_result = self.media_repository.get_media_by_product_id(id).await;

        match (product_result, media_result) {
            (Ok(product), Ok(media)) => Ok((product, media)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
    /// Obtain a list of products.
    async fn get_products(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<(Vec<Product>, Vec<Media>), DomainError> {
        let products_result = self.product_repository.get_products(limit, offset).await;
        if let Err(e) = products_result {
            return Err(e);
        }

        let product_ids: Vec<&ProductId> = products_result
            .as_ref()
            .unwrap()
            .iter()
            .map(|product| product.id())
            .collect();

        let media_result = self
            .media_repository
            .get_media_by_product_ids(product_ids)
            .await;

        match (products_result, media_result) {
            (Ok(products), Ok(media)) => Ok((products, media)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }
}
