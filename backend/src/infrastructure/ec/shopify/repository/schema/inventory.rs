use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::infrastructure::ec::shopify::repository::schema::product::VariantSchema;

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
    pub nventory_type: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemNode {
    pub id: String,
    pub variant: VariantSchema,
    #[serde(rename = "inventoryLevel")]
    pub inventory_level: Option<InventoryLevelSchema>,
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
