use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        authorized_resource::authorized_resource::{
            AuthorizedResource, Resource, ResourceAction, ResourceType,
        },
        error::error::DomainError,
        media::media::Media,
        product::product::{Id as ProductId, Product},
        user::user::UserInterface,
    },
    usecase::{
        auth::authorizer_interface::Authorizer,
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
    authorizer: Arc<dyn Authorizer>,
}

impl ProductInteractorImpl {
    pub fn new(
        product_repository: Box<dyn ProductRepository>,
        media_repository: Box<dyn MediaRepository>,
        product_query_service: Box<dyn ProductQueryService>,
        authorizer: Arc<dyn Authorizer>,
    ) -> Self {
        Self {
            product_repository: product_repository,
            media_repository: media_repository,
            product_query_service: product_query_service,
            authorizer: authorizer,
        }
    }
}

#[async_trait]
impl ProductInteractor for ProductInteractorImpl {
    async fn get_product_with_media(
        &self,
        user: Arc<dyn UserInterface>,
        id: &ProductId,
    ) -> Result<(Product, Vec<Media>), DomainError> {
        self.authorizer
            .authorize(
                user.clone(),
                vec![&Resource::new(ResourceType::Product, Some(id.clone()))],
                &ResourceAction::Read,
            )
            .await?;

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
        user: Arc<dyn UserInterface>,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<(Vec<Product>, Vec<Media>), DomainError> {
        let products_result = self.product_repository.find_products(limit, offset).await;
        if let Err(e) = products_result {
            return Err(e);
        }

        self.authorizer
            .authorize(
                user.clone(),
                products_result
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|product| product as &dyn AuthorizedResource)
                    .collect(),
                &ResourceAction::Read,
            )
            .await?;

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

    async fn get_related_products(
        &self,
        user: Arc<dyn UserInterface>,
        id: &ProductId,
    ) -> Result<Vec<ProductDTO>, DomainError> {
        self.authorizer
            .authorize(
                user.clone(),
                vec![&Resource::new(ResourceType::Product, Some(id.clone()))],
                &ResourceAction::Read,
            )
            .await?;

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
