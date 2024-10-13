use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{draft_order::draft_order::DraftOrder, error::error::DomainError},
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::{address::AddressNode, common::Edges, line_item::LineItemNode, money::MoneyBagNode};

impl DraftOrderNode {
    pub fn to_domain(self) -> Result<DraftOrder, DomainError> {
        let status = match self.status.as_str() {
            "OPEN" => Ok(crate::domain::draft_order::draft_order::DraftOrderStatus::Open),
            "COMPLETED" => Ok(crate::domain::draft_order::draft_order::DraftOrderStatus::Completed),
            "CANCELED" => Ok(crate::domain::draft_order::draft_order::DraftOrderStatus::Canceled),
            _ => Err(DomainError::ConversionError),
        }?;

        DraftOrder::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            self.name,
            status,
            self.line_items
                .edges
                .into_iter()
                .map(|node| node.node.to_domain())
                .collect::<Result<Vec<_>, _>>()?,
            self.reserve_inventory_until,
            self.subtotal_price_set.to_domain()?,
            self.taxes_included,
            self.tax_exempt,
            self.total_tax_set.to_domain()?,
            self.total_discounts_set.to_domain()?,
            self.total_shipping_price_set.to_domain()?,
            self.total_price_set.to_domain()?,
            self.customer
                .map(|c| ShopifyGQLQueryHelper::remove_gid_prefix(&c.id)),
            self.billing_address.to_domain()?,
            self.shipping_address.to_domain()?,
            self.note,
            self.order
                .map(|o| ShopifyGQLQueryHelper::remove_gid_prefix(&o.id)),
            self.completed_at,
            self.created_at,
            self.update_at,
        )
    }

    pub fn to_domains(schemas: Vec<Self>) -> Result<Vec<DraftOrder>, DomainError> {
        schemas
            .into_iter()
            .map(|schema| schema.to_domain())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct DraftOrdersData {
    #[serde(rename = "draftOrders")]
    pub draft_orders: Edges<DraftOrderNode>,
}

#[derive(Debug, Deserialize)]
pub struct DraftOrderNode {
    pub id: String,
    pub name: String,
    pub status: String,

    #[serde(rename = "lineItems")]
    pub line_items: Edges<LineItemNode>,
    #[serde(rename = "reserveInventoryUntil")]
    pub reserve_inventory_until: Option<DateTime<Utc>>,

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

    pub order: Option<OrderIdNode>,

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
