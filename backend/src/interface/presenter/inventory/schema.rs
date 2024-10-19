use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

use actix_http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::interface::presenter::common::exception::ErrorResponseBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct InventorySchema {
    pub(super) id: String,
    pub(super) variant_id: String,
    pub(super) inventory_levels: Vec<InventoryLevelSchema>,
    pub(super) requires_shipping: bool,
    pub(super) tracked: bool,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryLevelSchema {
    pub(super) id: String,
    pub(super) location_id: String,
    pub(super) quantities: Vec<QuantitySchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantitySchema {
    pub(super) quantity: i32,
    pub(super) inventory_type: InventoryTypeEnum,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InventoryTypeEnum {
    Available,
    Committed,
    Incoming,
    Reserved,
    SafetyStock,
    Damaged,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetInventoriesResponse {
    pub inventories: Vec<InventorySchema>,
}

#[derive(Debug, Display, Error)]
pub enum GetInventoriesErrorResponse {
    #[display(fmt = "Inventory not found.")]
    NotFound,
    #[display(fmt = "Bad request.")]
    BadRequest,
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl ErrorResponseBuilder for GetInventoriesErrorResponse {
    fn status_code(&self) -> StatusCode {
        match *self {
            GetInventoriesErrorResponse::NotFound => StatusCode::NOT_FOUND,
            GetInventoriesErrorResponse::BadRequest => StatusCode::BAD_REQUEST,
            GetInventoriesErrorResponse::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl ResponseError for GetInventoriesErrorResponse {
    fn error_response(&self) -> HttpResponse {
        <Self as ErrorResponseBuilder>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as ErrorResponseBuilder>::status_code(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PutInventoryResponse {
    pub inventory_level: InventoryLevelSchema,
}

#[derive(Debug, Display, Error)]
pub enum PutInventoryErrorResponse {
    #[display(fmt = "Inventory level not found.")]
    NotFound,
    #[display(fmt = "Bad request.")]
    BadRequest,
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl ErrorResponseBuilder for PutInventoryErrorResponse {
    fn status_code(&self) -> StatusCode {
        match *self {
            PutInventoryErrorResponse::NotFound => StatusCode::NOT_FOUND,
            PutInventoryErrorResponse::BadRequest => StatusCode::BAD_REQUEST,
            PutInventoryErrorResponse::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl ResponseError for PutInventoryErrorResponse {
    fn error_response(&self) -> HttpResponse {
        <Self as ErrorResponseBuilder>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as ErrorResponseBuilder>::status_code(self)
    }
}
