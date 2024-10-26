use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{customer::customer::Id as CustomerId, draft_order::draft_order::DraftOrder},
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

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
#[serde(rename_all = "camelCase")]
pub struct DraftOrderInput {
    pub purchasing_entity: Option<PurchasingEntityInput>,
    pub billing_address: Option<AddressInput>,
    pub shipping_address: Option<AddressInput>,
    pub note: Option<String>,

    pub line_items: Vec<LineItemInput>,
    pub reserve_inventory_until: Option<DateTime<Utc>>,

    pub tax_exempt: Option<bool>,
}

impl From<CustomerId> for PurchasingEntityInput {
    fn from(customer_id: CustomerId) -> Self {
        Self {
            customer_id: Some(ShopifyGQLQueryHelper::add_customer_gid_prefix(&customer_id)),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchasingEntityInput {
    pub customer_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderCreateData {
    pub draft_order_create: DraftOrderCreate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderCreate {
    pub draft_order: Option<DraftOrderNode>,
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderUpdateData {
    pub draft_order_update: DraftOrderUpdate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderUpdate {
    pub draft_order: Option<DraftOrderNode>,
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderCompleteData {
    pub draft_order_complete: DraftOrderComplete,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderComplete {
    pub draft_order: Option<DraftOrderNode>,
    pub user_errors: Vec<UserError>,
}
