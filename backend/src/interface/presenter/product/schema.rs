use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::domain::error::error::DomainError;
use crate::interface::presenter::media::schema::MediaSchema;
use crate::usecase::query_service::dto::product::ProductDTO;
use crate::{define_error_response, interface::presenter::common::exception::ErrorResponseBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSchema {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) description: String,
    pub(super) status: ProductStatusEnum,
    pub(super) category_id: Option<String>,
    pub(super) media: Vec<MediaSchema>,
    pub(super) variants: Vec<VariantSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProductStatusEnum {
    Active,
    Inactive,
    Draft,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariantSchema {
    pub(super) id: String,
    pub(super) name: Option<String>,
    pub(super) sku: Option<String>,
    pub(super) barcode: Option<String>,
    pub(super) available_for_sale: bool,
    pub(super) list_order: u8,

    pub(super) inventory_item_id: String,
    pub(super) inventory_policy: InventoryPolicyEnum,
    pub(super) inventory_quantity: Option<u32>,

    pub(super) price: f64,
    pub(super) taxable: bool,
    pub(super) tax_code: Option<String>,

    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InventoryPolicyEnum {
    Deny,
    Continue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProductResponse {
    pub(super) product: ProductSchema,
}

define_error_response!(GetProductErrorResponse, "Product");

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProductsResponse {
    pub products: Vec<ProductSchema>,
}

define_error_response!(GetProductsErrorResponse, "Products");

#[derive(Debug, Serialize, Deserialize)]
pub struct GetRelatedProductsResponse {
    pub products: Vec<ProductDTO>,
}

define_error_response!(GetRelatedProductsErrorResponse, "Products");
