use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{draft_order::draft_order::DraftOrder, error::error::DomainError},
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::{
    address::AddressNode,
    common::Edges,
    line_item::LineItemNode,
    money::{CurrencyCodeNode, MoneyBagNode},
};

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
            self.customer
                .map(|c| ShopifyGQLQueryHelper::remove_gid_prefix(&c.id)),
            self.billing_address.map(|a| a.to_domain()).transpose()?,
            self.shipping_address.map(|a| a.to_domain()).transpose()?,
            self.note2,
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
            self.presentment_currency_code.to_domain()?,
            self.order
                .map(|o| ShopifyGQLQueryHelper::remove_gid_prefix(&o.id)),
            self.completed_at,
            self.created_at,
            self.updated_at,
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
#[serde(rename_all = "camelCase")]
pub struct DraftOrderData {
    pub draft_order: DraftOrderNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrdersData {
    pub draft_orders: Edges<DraftOrderNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderNode {
    pub id: String,
    pub name: String,
    pub status: String,

    pub customer: Option<CustomerIdNode>,
    pub billing_address: Option<AddressNode>,
    pub shipping_address: Option<AddressNode>,
    pub note2: Option<String>,

    pub line_items: Edges<LineItemNode>,
    pub reserve_inventory_until: Option<DateTime<Utc>>,

    pub subtotal_price_set: MoneyBagNode,
    pub taxes_included: bool,
    pub tax_exempt: bool,
    pub total_tax_set: MoneyBagNode,
    pub total_discounts_set: MoneyBagNode,
    pub total_shipping_price_set: MoneyBagNode,
    pub total_price_set: MoneyBagNode,
    pub presentment_currency_code: CurrencyCodeNode,

    pub order: Option<OrderIdNode>,

    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerIdNode {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct OrderIdNode {
    pub id: String,
}
