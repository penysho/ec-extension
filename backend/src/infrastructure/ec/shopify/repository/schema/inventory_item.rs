use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{error::error::DomainError, inventory_item::inventory_item::InventoryItem},
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::{common::Edges, inventory_level::InventoryLevelNode};

impl InventoryItemNode {
    pub fn to_domain(self) -> Result<InventoryItem, DomainError> {
        InventoryItem::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.variant.id),
            self.requires_shipping,
            self.tracked,
            self.created_at,
            self.updated_at,
        )
    }

    pub fn to_domains(schemas: Vec<Self>) -> Result<Vec<InventoryItem>, DomainError> {
        schemas
            .into_iter()
            .map(|schema| schema.to_domain())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantsDataForInventory {
    pub product_variants: Edges<VariantNodeForInventory>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VariantNodeForInventory {
    pub inventory_item: InventoryItemNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemsData {
    pub inventory_items: Edges<InventoryItemNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemNode {
    pub id: String,
    pub variant: VariantIdNode,
    // Used when obtaining a single InventoryLevel.
    pub inventory_level: Option<InventoryLevelNode>,
    // Used when acquiring multiple InventoryLevels.
    #[serde(default)]
    pub inventory_levels: Edges<InventoryLevelNode>,
    pub requires_shipping: bool,
    pub tracked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct VariantIdNode {
    pub id: String,
}
