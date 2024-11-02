use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

use actix_http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::domain::error::error::DomainError;
use crate::interface::presenter::address::schema::AddressSchema;
use crate::{define_error_response, interface::presenter::common::exception::ErrorResponseBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationSchema {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) is_active: bool,
    pub(super) fulfills_online_orders: bool,
    pub(super) address: AddressSchema,
    pub(super) suggested_addresses: Vec<AddressSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLocationsResponse {
    pub locations: Vec<LocationSchema>,
}

define_error_response!(GetLocationsErrorResponse, "Locations");
