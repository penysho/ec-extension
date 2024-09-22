use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::infrastructure::ec::shopify::repository::schema::product::VariantSchema;

use super::product::VariantNode;

impl From<InventoryItemNode> for InventoryItemSchema {
    fn from(node: InventoryItemNode) -> Self {
        InventoryItemSchema {
            id: node.id,
            variant: node.variant.into(),
            inventory_level: node.inventory_level.map(|level| level.into()),
            tracked: node.tracked,
            created_at: node.created_at,
            updated_at: node.updated_at,
        }
    }
}

impl From<InventoryLevelNode> for InventoryLevelSchema {
    fn from(node: InventoryLevelNode) -> Self {
        InventoryLevelSchema {
            id: node.id,
            location_id: node.location.id,
            quantities: vec![node.quantities.into()],
        }
    }
}

impl From<QuantityNode> for QuantitySchema {
    fn from(node: QuantityNode) -> Self {
        QuantitySchema {
            quantity: node.quantity as u32,
            inventory_type: node.name,
        }
    }
}

impl From<LocationNode> for LocationSchema {
    fn from(node: LocationNode) -> Self {
        LocationSchema { id: node.id }
    }
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemSchema {
    pub id: String,
    pub variant: VariantSchema,
    pub inventory_level: Option<InventoryLevelSchema>,
    pub tracked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryLevelSchema {
    pub id: String,
    pub location_id: String,
    pub quantities: Vec<QuantitySchema>,
}

#[derive(Debug, Deserialize)]
pub struct QuantitySchema {
    pub quantity: u32,
    pub inventory_type: String,
}

#[derive(Debug, Deserialize)]
pub struct LocationSchema {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemNode {
    pub id: String,
    pub variant: VariantNode,
    #[serde(rename = "inventoryLevel")]
    pub inventory_level: Option<InventoryLevelNode>,
    pub tracked: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct InventoryLevelNode {
    pub id: String,
    pub location: LocationNode,
    pub quantities: QuantityNode,
}

#[derive(Debug, Deserialize)]
pub struct QuantityNode {
    pub quantity: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LocationNode {
    pub id: String,
}
