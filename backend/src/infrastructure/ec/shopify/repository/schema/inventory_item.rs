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
pub struct VariantsDataForInventory {
    #[serde(rename = "productVariants")]
    pub product_variants: Edges<VariantNodeForInventory>,
}

#[derive(Debug, Deserialize)]
pub struct VariantNodeForInventory {
    #[serde(rename = "inventoryItem")]
    pub inventory_item: InventoryItemNode,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemsData {
    #[serde(rename = "inventoryItems")]
    pub inventory_items: Edges<InventoryItemNode>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemNode {
    pub id: String,
    pub variant: VariantIdNode,
    // Used when obtaining a single InventoryLevel.
    #[serde(rename = "inventoryLevel")]
    pub inventory_level: Option<InventoryLevelNode>,
    // Used when acquiring multiple InventoryLevels.
    #[serde(default)]
    #[serde(rename = "inventoryLevels")]
    pub inventory_levels: Edges<InventoryLevelNode>,
    #[serde(rename = "requiresShipping")]
    pub requires_shipping: bool,
    pub tracked: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct VariantIdNode {
    pub id: String,
}
