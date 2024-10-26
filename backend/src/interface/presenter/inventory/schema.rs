use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

use actix_http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::error::error::DomainError;
use crate::{define_error_response, interface::presenter::common::exception::ErrorResponseBuilder};

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

define_error_response!(GetInventoriesErrorResponse, "Inventories");

#[derive(Debug, Serialize, Deserialize)]
pub struct PutInventoryResponse {
    pub inventory_level: InventoryLevelSchema,
}

define_error_response!(PutInventoryErrorResponse, "Inventory");
