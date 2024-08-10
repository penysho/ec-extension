use crate::entity::{error::error::DomainError, product::product::Product};

pub trait ProductPresenter {
    type GetProductResponse;
    type GetProductResponseError;
    type GetProductsResponse;
    type GetProductsResponseError;

    async fn present_get_product(
        &self,
        result: Result<Option<Product>, DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductResponseError>;
    async fn present_get_products(
        &self,
        result: Result<Vec<Product>, DomainError>,
    ) -> Result<Self::GetProductsResponse, Self::GetProductsResponseError>;
}
