use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        product::product::{Id, Product},
    },
    infrastructure::ec::ec_client_interface::ECClient,
    usecase::query_service::product_query_service_interface::ProductQueryService,
};

/// Query service for products for Shopify.
pub struct ProductQueryServiceImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> ProductQueryServiceImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> ProductQueryService for ProductQueryServiceImpl<C> {
    async fn search_related_products(&self, id: &Id) -> Result<Vec<Product>, DomainError> {
        todo!()
    }
}
