use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        media::media::{Media, MediaStatus},
        product::product::{Product, ProductStatus},
    },
    interface::presenter::common::exception::GenericResponseError,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProductStatusEnum {
    Active,
    Inactive,
    Draft,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MediaStatusEnum {
    Active,
    Inactive,
    InPreparation,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductSchema {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) price: u32,
    pub(super) description: String,
    pub(super) status: ProductStatusEnum,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
    pub(super) category_id: Option<String>,
    pub(super) media: Vec<MediaSchema>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaSchema {
    pub(super) id: String,
    pub(super) status: MediaStatusEnum,
    pub(super) alt: Option<String>,
    pub(super) src: Option<String>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

impl ProductSchema {
    pub fn to_response(product: Product, media: Vec<Media>) -> Self {
        ProductSchema {
            id: product.id().to_string(),
            name: product.name().to_string(),
            price: *(product.variants()[0].price()),
            description: product.description().to_string(),
            status: match product.status() {
                ProductStatus::Active => ProductStatusEnum::Active,
                ProductStatus::Inactive => ProductStatusEnum::Inactive,
                ProductStatus::Draft => ProductStatusEnum::Draft,
            },
            created_at: product.variants()[0].created_at().to_owned(),
            updated_at: product.variants()[0].updated_at().to_owned(),
            category_id: product.category_id().to_owned(),
            media: media
                .into_iter()
                .map(|media| MediaSchema::from(media))
                .collect(),
        }
    }
}

impl From<Media> for MediaSchema {
    fn from(media: Media) -> Self {
        MediaSchema {
            id: media.id().to_string(),
            status: match media.status() {
                MediaStatus::Active => MediaStatusEnum::Active,
                MediaStatus::Inactive => MediaStatusEnum::Inactive,
                MediaStatus::InPreparation => MediaStatusEnum::InPreparation,
            },
            alt: media.alt().to_owned(),
            src: media.published_src().as_ref().map(|s| s.value().to_owned()),
            created_at: media.created_at().to_owned(),
            updated_at: media.updated_at().to_owned(),
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
