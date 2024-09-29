use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

use actix_http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::interface::presenter::common::exception::GenericResponseError;

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
    pub(super) quantity: u32,
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
pub enum GetInventoriesResponseError {
    #[display(fmt = "Inventory not found.")]
    NotFound,
    #[display(fmt = "Bad request.")]
    BadRequest,
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl GenericResponseError for GetInventoriesResponseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            GetInventoriesResponseError::NotFound => StatusCode::NOT_FOUND,
            GetInventoriesResponseError::BadRequest => StatusCode::BAD_REQUEST,
            GetInventoriesResponseError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl ResponseError for GetInventoriesResponseError {
    fn error_response(&self) -> HttpResponse {
        <Self as GenericResponseError>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as GenericResponseError>::status_code(self)
    }
}
