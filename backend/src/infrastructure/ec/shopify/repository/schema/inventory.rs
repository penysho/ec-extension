use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        inventory::{
            inventory::Inventory,
            inventory_level::{
                inventory_level::InventoryLevel,
                quantity::quantity::{InventoryType, Quantity},
            },
        },
    },
    infrastructure::ec::shopify::{
        query_helper::ShopifyGQLQueryHelper, repository::schema::product::VariantSchema,
    },
};

use super::{common::Edges, location::LocationNode, product::VariantNode};

impl InventoryItemSchema {
    pub fn to_domain(self) -> Result<Inventory, DomainError> {
        let inventory_level = self
            .inventory_level
            .map(|level| level.to_domain())
            .transpose()?;

        Inventory::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            self.variant.id,
            inventory_level,
            self.requires_shipping,
            self.tracked,
            self.created_at,
            self.updated_at,
        )
    }

    pub fn to_domains(schemas: Vec<InventoryItemSchema>) -> Result<Vec<Inventory>, DomainError> {
        schemas
            .into_iter()
            .map(|schema| schema.to_domain())
            .collect()
    }
}

impl InventoryLevelSchema {
    pub fn to_domain(self) -> Result<InventoryLevel, DomainError> {
        let quantities: Result<Vec<Quantity>, DomainError> = self
            .quantities
            .into_iter()
            .map(|quantity| quantity.to_domain())
            .collect();

        InventoryLevel::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            self.location_id,
            quantities?,
        )
    }
}

impl QuantitySchema {
    pub fn to_domain(self) -> Result<Quantity, DomainError> {
        let inventory_type = match self.inventory_type.as_str() {
            "available" => InventoryType::Available,
            "incoming" => InventoryType::Incoming,
            "committed" => InventoryType::Committed,
            "damaged" => InventoryType::Damaged,
            "safety_stock" => InventoryType::SafetyStock,
            "reserved" => InventoryType::Reserved,
            _ => return Err(DomainError::ValidationError),
        };

        Quantity::new(self.quantity as u32, inventory_type)
    }
}

impl From<InventoryItemNode> for InventoryItemSchema {
    fn from(node: InventoryItemNode) -> Self {
        InventoryItemSchema {
            id: node.id,
            variant: node.variant.into(),
            inventory_level: node.inventory_level.map(|level| level.into()),
            requires_shipping: node.requires_shipping,
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
            quantity: node.quantity,
            inventory_type: node.name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemSchema {
    pub id: String,
    pub variant: VariantSchema,
    pub inventory_level: Option<InventoryLevelSchema>,
    pub requires_shipping: bool,
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
    pub quantity: i32,
    pub inventory_type: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemNode {
    pub id: String,
    pub variant: VariantNode,
    #[serde(rename = "inventoryLevel")]
    pub inventory_level: Option<InventoryLevelNode>,
    #[serde(rename = "requiresShipping")]
    pub requires_shipping: bool,
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
pub struct VariantNodeForInventory {
    #[serde(rename = "inventoryItem")]
    pub inventory_item: InventoryItemNode,
}

#[derive(Debug, Deserialize)]
pub struct VariantsDataForInventory {
    #[serde(rename = "productVariants")]
    pub product_variants: Edges<VariantNodeForInventory>,
}
