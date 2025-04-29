use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        media::media::Media,
        product::product::{Id as ProductId, Product},
    },
    usecase::{
        interactor::product_interactor_interface::ProductInteractor,
        query_service::{
            dto::product::ProductDTO,
            product_query_service_interface::{ProductQueryService, RelatedProductFilter},
        },
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
    product_query_service: Box<dyn ProductQueryService>,
}

impl ProductInteractorImpl {
    pub fn new(
        product_repository: Box<dyn ProductRepository>,
        media_repository: Box<dyn MediaRepository>,
        product_query_service: Box<dyn ProductQueryService>,
    ) -> Self {
        Self {
            product_repository: product_repository,
            media_repository: media_repository,
            product_query_service: product_query_service,
        }
    }
}

#[async_trait]
impl ProductInteractor for ProductInteractorImpl {
    async fn get_product_with_media(
        &self,
        id: &ProductId,
    ) -> Result<(Product, Vec<Media>), DomainError> {
        tracing::info!("get_product_with_media: {}", id);
        let product_result = self.product_repository.find_product_by_id(id).await;
        let media_result = self.media_repository.find_media_by_product_id(id).await;

        match (product_result, media_result) {
            (Ok(product), Ok(media)) => Ok((product, media)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }

    async fn get_products_with_media(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<(Vec<Product>, Vec<Media>), DomainError> {
        let products_result = self.product_repository.find_products(limit, offset).await;
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
            .find_media_by_product_ids(product_ids)
            .await;

        match (products_result, media_result) {
            (Ok(products), Ok(media)) => Ok((products, media)),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    }

    async fn get_related_products(&self, id: &ProductId) -> Result<Vec<ProductDTO>, DomainError> {
        let product = self.product_repository.find_product_by_id(id).await?;
        let category_id = product.category_id().clone();

        match category_id {
            Some(category_id) => {
                self.product_query_service
                    .search_related_products(&RelatedProductFilter {
                        id: id.clone(),
                        category_id,
                    })
                    .await
            }
            None => Ok(vec![]),
        }
    }
}
