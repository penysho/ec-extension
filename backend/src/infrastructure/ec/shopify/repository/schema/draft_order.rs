use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{address::AddressNode, line_item::LineItemNode, money::MoneyBagNode};

#[derive(Debug, Deserialize)]
pub struct DraftOrderNode {
    pub id: String,
    pub name: String,
    pub status: String,

    #[serde(rename = "lineItems")]
    pub line_items: Vec<LineItemNode>,

    #[serde(rename = "subtotalPriceSet")]
    pub subtotal_price_set: MoneyBagNode,
    #[serde(rename = "taxesIncluded")]
    pub taxes_included: bool,
    #[serde(rename = "taxExempt")]
    pub tax_exempt: bool,
    #[serde(rename = "totalTaxSet")]
    pub total_tax_set: MoneyBagNode,
    #[serde(rename = "totalDiscountsSet")]
    pub total_discounts_set: MoneyBagNode,
    #[serde(rename = "totalShippingPriceSet")]
    pub total_shipping_price_set: MoneyBagNode,
    #[serde(rename = "totalPriceSet")]
    pub total_price_set: MoneyBagNode,

    pub customer: Option<CustomerIdNode>,
    #[serde(rename = "billingAddress")]
    pub billing_address: AddressNode,
    #[serde(rename = "shippingAddress")]
    pub shipping_address: AddressNode,
    pub note: Option<String>,

    pub order_id: Option<OrderIdNode>,

    #[serde(rename = "completedAt")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub update_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerIdNode {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderIdNode {
    pub id: String,
}
