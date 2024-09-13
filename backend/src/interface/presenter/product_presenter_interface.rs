use async_trait::async_trait;

use crate::domain::{error::error::DomainError, media::media::Media, product::product::Product};

/// Interface to generate response schema for products.
#[async_trait]
pub trait ProductPresenter {
    type GetProductResponse;
    type GetProductResponseError;
    async fn present_get_product(
        &self,
        product_result: Result<Product, DomainError>,
        media_result: Result<Vec<Media>, DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductResponseError>;

    type GetProductsResponse;
    type GetProductsResponseError;
    async fn present_get_products(
        &self,
        result: Result<Vec<Product>, DomainError>,
    ) -> Result<Self::GetProductsResponse, Self::GetProductsResponseError>;
}
