use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{customer::customer::Id as CustomerId, draft_order::draft_order::DraftOrder};

use super::{
    address_input::AddressInput, common::UserError, draft_order::DraftOrderNode,
    line_item_input::LineItemInput,
};

impl From<DraftOrder> for DraftOrderInput {
    fn from(draft_order: DraftOrder) -> Self {
        Self {
            purchasing_entity: draft_order.customer_id().to_owned().map(|p| p.into()),
            billing_address: draft_order.billing_address().to_owned().map(|a| a.into()),
            shipping_address: draft_order.shipping_address().to_owned().map(|a| a.into()),
            note: draft_order.note().to_owned(),
            line_items: draft_order.line_items().iter().map(|l| l.into()).collect(),
            reserve_inventory_until: draft_order.reserve_inventory_until().to_owned(),
            tax_exempt: Some(*draft_order.tax_exempt()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DraftOrderInput {
    #[serde(rename = "purchasingEntity")]
    pub purchasing_entity: Option<PurchasingEntityInput>,
    #[serde(rename = "billingAddress")]
    pub billing_address: Option<AddressInput>,
    #[serde(rename = "shippingAddress")]
    pub shipping_address: Option<AddressInput>,
    pub note: Option<String>,

    #[serde(rename = "lineItems")]
    pub line_items: Vec<LineItemInput>,
    #[serde(rename = "reserveInventoryUntil")]
    pub reserve_inventory_until: Option<DateTime<Utc>>,

    #[serde(rename = "taxExempt")]
    pub tax_exempt: Option<bool>,
}

impl From<CustomerId> for PurchasingEntityInput {
    fn from(customer_id: CustomerId) -> Self {
        Self {
            customer_id: Some(customer_id),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PurchasingEntityInput {
    #[serde(rename = "customerId")]
    pub customer_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DraftOrderCreateData {
    #[serde(rename = "draftOrderCreate")]
    pub draft_order_create: DraftOrderCreate,
}

#[derive(Debug, Deserialize)]
pub struct DraftOrderCreate {
    #[serde(rename = "draftOrder")]
    pub draft_order: Option<DraftOrderNode>,
    pub user_errors: Vec<UserError>,
}
