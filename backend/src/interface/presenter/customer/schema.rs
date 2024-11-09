use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

use actix_http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::error::error::DomainError;
use crate::{
    define_error_response,
    interface::presenter::{
        address::schema::AddressSchema, common::exception::ErrorResponseBuilder,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerSchema {
    pub(super) id: String,
    pub(super) addresses: Vec<AddressSchema>,
    pub(super) default_address: Option<AddressSchema>,
    pub(super) display_name: String,
    pub(super) email: Option<String>,
    pub(super) first_name: Option<String>,
    pub(super) last_name: Option<String>,
    // TODO
    // image: Option<Image>,
    pub(super) phone: Option<String>,
    pub(super) note: Option<String>,
    pub(super) status: CustomerStatusSchema,
    pub(super) verified_email: bool,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CustomerStatusSchema {
    Active,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCustomersResponse {
    pub customers: Vec<CustomerSchema>,
}

define_error_response!(GetCustomersErrorResponse, "Customer");
