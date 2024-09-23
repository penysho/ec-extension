use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        media::media::{Media, MediaStatus},
        product::{
            product::{Product, ProductStatus},
            variant::variant::Variant,
        },
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
    pub(super) description: String,
    pub(super) status: ProductStatusEnum,
    pub(super) category_id: Option<String>,
    pub(super) media: Vec<MediaSchema>,
    pub(super) variants: Vec<VariantSchema>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VariantSchema {
    pub(super) id: String,
    pub(super) price: u32,
    pub(super) sku: Option<String>,
    pub(super) barcode: Option<String>,
    pub(super) inventory_quantity: Option<u32>,
    pub(super) list_order: u8,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
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
    pub fn to_schema(product: Product, media: Vec<Media>) -> Self {
        ProductSchema {
            id: product.id().to_string(),
            name: product.name().to_string(),
            description: product.description().to_string(),
            status: match product.status() {
                ProductStatus::Active => ProductStatusEnum::Active,
                ProductStatus::Inactive => ProductStatusEnum::Inactive,
                ProductStatus::Draft => ProductStatusEnum::Draft,
            },
            category_id: product.category_id().to_owned(),
            media: media
                .into_iter()
                .map(|media| MediaSchema::from(media))
                .collect(),
            variants: product
                .variants()
                .iter()
                .map(|variant| VariantSchema::from(variant))
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

impl From<&Variant> for VariantSchema {
    fn from(variant: &Variant) -> Self {
        VariantSchema {
            id: variant.id().to_string(),
            price: *(variant.price()),
            sku: variant.sku().as_ref().map(|sku| sku.value().to_owned()),
            barcode: variant
                .barcode()
                .as_ref()
                .map(|barcode| barcode.value().to_owned()),
            inventory_quantity: variant.inventory_quantity().to_owned(),
            list_order: variant.list_order().to_owned(),
            created_at: variant.created_at().to_owned(),
            updated_at: variant.updated_at().to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProductResponse {
    pub product: ProductSchema,
}

#[derive(Debug, Display, Error)]
pub enum GetProductResponseError {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct GetProductsResponse {
    pub products: Vec<ProductSchema>,
}

#[derive(Debug, Display, Error)]
pub enum GetProductsResponseError {
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
