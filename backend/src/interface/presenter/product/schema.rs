use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::interface::presenter::common::exception::GenericResponseError;

#[derive(Debug, Serialize, Deserialize)]
pub(super) enum ProductStatusEnum {
    Active,
    Inactive,
    Draft,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) enum MediaStatusEnum {
    Active,
    Inactive,
    InPreparation,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct ProductSchema {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) description: String,
    pub(super) status: ProductStatusEnum,
    pub(super) category_id: Option<String>,
    pub(super) media: Vec<MediaSchema>,
    pub(super) variants: Vec<VariantSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct VariantSchema {
    pub(super) id: String,
    pub(super) price: u32,
    pub(super) sku: Option<String>,
    pub(super) barcode: Option<String>,
    pub(super) inventory_quantity: Option<u32>,
    pub(super) list_order: u8,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct MediaSchema {
    pub(super) id: String,
    pub(super) status: MediaStatusEnum,
    pub(super) alt: Option<String>,
    pub(super) src: Option<String>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct GetProductResponse {
    pub(super) product: ProductSchema,
}

#[derive(Debug, Display, Error)]
pub(super) enum GetProductResponseError {
    #[display(fmt = "Product not found.")]
    NotFound,
    #[display(fmt = "Bad request.")]
    BadRequest,
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl GenericResponseError for GetProductResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            GetProductResponseError::NotFound => StatusCode::NOT_FOUND,
            GetProductResponseError::BadRequest => StatusCode::BAD_REQUEST,
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

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct GetProductsResponse {
    pub products: Vec<ProductSchema>,
}

#[derive(Debug, Display, Error)]
pub(super) enum GetProductsResponseError {
    #[display(fmt = "Product not found.")]
    NotFound,
    #[display(fmt = "Bad request.")]
    BadRequest,
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl GenericResponseError for GetProductsResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            GetProductsResponseError::NotFound => StatusCode::NOT_FOUND,
            GetProductsResponseError::BadRequest => StatusCode::BAD_REQUEST,
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
