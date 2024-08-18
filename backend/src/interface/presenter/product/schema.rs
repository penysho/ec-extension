use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::{
    entity::product::product::Product,
    interface::presenter::common::exception::GenericResponseError,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductSchema {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) price: u32,
    pub(super) description: String,
}

impl From<Product> for ProductSchema {
    fn from(domain: Product) -> Self {
        ProductSchema {
            id: domain.id().to_string(),
            name: domain.name().to_string(),
            price: *(domain.price()),
            description: domain.description().to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProductResponse {
    pub(super) product: ProductSchema,
}

#[derive(Debug, Display, Error)]
pub enum GetProductResponseError {
    #[display(fmt = "Product not found.")]
    ProductNotFound,
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl GenericResponseError for GetProductResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            GetProductResponseError::ProductNotFound => StatusCode::NOT_FOUND,
            GetProductResponseError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl ResponseError for GetProductResponseError {
    fn error_response(&self) -> HttpResponse {
        <Self as GenericResponseError>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as GenericResponseError>::status_code(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProductsResponse {
    pub products: Vec<ProductSchema>,
}

#[derive(Debug, Display, Error)]
pub enum GetProductsResponseError {
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl GenericResponseError for GetProductsResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            GetProductsResponseError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl ResponseError for GetProductsResponseError {
    fn error_response(&self) -> HttpResponse {
        <Self as GenericResponseError>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as GenericResponseError>::status_code(self)
    }
}
