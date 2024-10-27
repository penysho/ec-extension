use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

use actix_http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::error::error::DomainError;
use crate::{
    define_error_response,
    interface::presenter::{
        address::schema::AddressSchema,
        common::exception::ErrorResponseBuilder,
        line_item::schema::LineItemSchema,
        money::schema::{CurrencyCodeSchema, MoneySchema},
    },
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
    pub(super) subtotal_price_set: MoneySchema,
    pub(super) taxes_included: bool,
    pub(super) tax_exempt: bool,
    pub(super) total_tax_set: MoneySchema,
    pub(super) total_discounts_set: MoneySchema,
    pub(super) total_shipping_price_set: MoneySchema,
    pub(super) total_price_set: MoneySchema,
    pub(super) presentment_currency_code: CurrencyCodeSchema,
    pub(super) order_id: Option<String>,
    pub(super) completed_at: Option<DateTime<Utc>>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
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

define_error_response!(GetDraftOrdersErrorResponse, "DraftOrder");

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDraftOrderResponse {
    pub draft_order: DraftOrderSchema,
}

define_error_response!(PostDraftOrderErrorResponse, "DraftOrder");

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteDraftOrderResponse {
    pub draft_order: DraftOrderSchema,
}

define_error_response!(CompleteDraftOrderErrorResponse, "DraftOrder");
