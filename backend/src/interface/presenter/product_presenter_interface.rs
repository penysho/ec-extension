use async_trait::async_trait;

use crate::domain::{error::error::DomainError, media::media::Media, product::product::Product};

/// Interface to generate response schema for products.
#[async_trait]
pub trait ProductPresenter {
    type GetProductResponse;
    type GetProductErrorResponse;
    /// Generate a response with detailed product information.
    async fn present_get_product(
        &self,
        result: Result<(Product, Vec<Media>), DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductErrorResponse>;

    type GetProductsResponse;
    type GetProductsErrorResponse;
    /// Generate a response for the product list.
    async fn present_get_products(
        &self,
        result: Result<(Vec<Product>, Vec<Media>), DomainError>,
    ) -> Result<Self::GetProductsResponse, Self::GetProductsErrorResponse>;
}
