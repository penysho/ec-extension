use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

use actix_http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::interface::presenter::{
    address::schema::AddressSchema,
    common::exception::GenericResponseError,
    line_item::schema::LineItemSchema,
    money::schema::{CurrencyCodeSchema, MoneyBagSchema},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftOrderSchema {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) status: DraftOrderStatusSchema,
    pub(super) customer_id: Option<String>,
    pub(super) billing_address: Option<AddressSchema>,
    pub(super) shipping_address: Option<AddressSchema>,
    pub(super) note: Option<String>,
    pub(super) line_items: Vec<LineItemSchema>,
    pub(super) reserve_inventory_until: Option<DateTime<Utc>>,
    pub(super) subtotal_price_set: MoneyBagSchema,
    pub(super) taxes_included: bool,
    pub(super) tax_exempt: bool,
    pub(super) total_tax_set: MoneyBagSchema,
    pub(super) total_discounts_set: MoneyBagSchema,
    pub(super) total_shipping_price_set: MoneyBagSchema,
    pub(super) total_price_set: MoneyBagSchema,
    pub(super) presentment_currency_code: CurrencyCodeSchema,
    pub(super) order_id: Option<String>,
    pub(super) completed_at: Option<DateTime<Utc>>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) update_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DraftOrderStatusSchema {
    Open,
    Completed,
    Canceled,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDraftOrdersResponse {
    pub draft_orders: Vec<DraftOrderSchema>,
}

#[derive(Debug, Display, Error)]
pub enum GetDraftOrdersErrorResponse {
    #[display(fmt = "Draft order not found.")]
    NotFound,
    #[display(fmt = "Bad request.")]
    BadRequest,
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl GenericResponseError for GetDraftOrdersErrorResponse {
    fn status_code(&self) -> StatusCode {
        match *self {
            GetDraftOrdersErrorResponse::NotFound => StatusCode::NOT_FOUND,
            GetDraftOrdersErrorResponse::BadRequest => StatusCode::BAD_REQUEST,
            GetDraftOrdersErrorResponse::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl ResponseError for GetDraftOrdersErrorResponse {
    fn error_response(&self) -> HttpResponse {
        <Self as GenericResponseError>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as GenericResponseError>::status_code(self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDraftOrderResponse {
    pub draft_order: DraftOrderSchema,
}

#[derive(Debug, Display, Error)]
pub enum PostDraftOrderErrorResponse {
    #[display(fmt = "Bad request.")]
    BadRequest,
    #[display(fmt = "Service unavailable. Give it some time and try again.")]
    ServiceUnavailable,
}

impl GenericResponseError for PostDraftOrderErrorResponse {
    fn status_code(&self) -> StatusCode {
        match *self {
            PostDraftOrderErrorResponse::BadRequest => StatusCode::BAD_REQUEST,
            PostDraftOrderErrorResponse::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }
}

impl ResponseError for PostDraftOrderErrorResponse {
    fn error_response(&self) -> HttpResponse {
        <Self as GenericResponseError>::error_response(self)
    }

    fn status_code(&self) -> StatusCode {
        <Self as GenericResponseError>::status_code(self)
    }
}
