use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    entity::{error::error::DomainError, product::product::Product},
    interface::presenter::{
        product_presenter_interface::ProductPresenter,
        schema::product::{
            GetProductResponse, GetProductResponseError, GetProductsResponse,
            GetProductsResponseError, ProductSchema,
        },
    },
};

/// Generate a response schema for the product
pub struct ProductPresenterImpl;
impl ProductPresenterImpl {
    pub fn new() -> Self {
        ProductPresenterImpl
    }
}

#[async_trait]
impl ProductPresenter for ProductPresenterImpl {
    type GetProductResponse = Json<GetProductResponse>;
    type GetProductResponseError = GetProductResponseError;
    /// Generate a response with detailed product information.
    async fn present_get_product(
        &self,
        result: Result<Option<Product>, DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductResponseError> {
        match result {
            Ok(Some(product)) => Ok(web::Json(GetProductResponse {
                product: ProductSchema::from(product),
            })),
            Ok(None) => Err(GetProductResponseError::ProductNotFound),
            Err(_) => Err(GetProductResponseError::ServiceUnavailable),
        }
    }

    type GetProductsResponse = Json<GetProductsResponse>;
    type GetProductsResponseError = GetProductsResponseError;
    /// Generate a response for the product list.
    async fn present_get_products(
        &self,
        result: Result<Vec<Product>, DomainError>,
    ) -> Result<Self::GetProductsResponse, Self::GetProductsResponseError> {
        match result {
            Ok(products) => {
                let product_schemas: Vec<ProductSchema> = products
                    .into_iter()
                    .map(|product| ProductSchema::from(product))
                    .collect();

                Ok(web::Json(GetProductsResponse {
                    products: product_schemas,
                }))
            }
            Err(_) => Err(GetProductsResponseError::ServiceUnavailable),
        }
    }
}
