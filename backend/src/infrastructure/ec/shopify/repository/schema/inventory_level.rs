use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        error::error::DomainError,
        inventory_level::{
            inventory_change::inventory_change::InventoryChange,
            inventory_level::InventoryLevel,
            quantity::quantity::{InventoryType, Quantity},
        },
    },
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::{common::UserErrors, location::LocationNode};

impl InventoryLevelSchema {
    pub fn to_domain(self) -> Result<InventoryLevel, DomainError> {
        let quantities: Result<Vec<Quantity>, DomainError> = self
            .quantities
            .into_iter()
            .map(|quantity| quantity.to_domain())
            .collect();

        InventoryLevel::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.inventory_item_id),
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.location_id),
            quantities?,
        )
    }

    pub fn to_domains(
        schemas: Vec<InventoryLevelSchema>,
    ) -> Result<Vec<InventoryLevel>, DomainError> {
        schemas
            .into_iter()
            .map(|schema| schema.to_domain())
            .collect()
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

        Quantity::new(self.quantity, inventory_type)
    }
}

#[derive(Debug, Deserialize)]
pub struct InventoryLevelSchema {
    pub id: String,
    pub inventory_item_id: String,
    pub location_id: String,
    pub quantities: Vec<QuantitySchema>,
}

#[derive(Debug, Deserialize)]
pub struct QuantitySchema {
    pub quantity: i32,
    pub inventory_type: String,
}

impl From<InventoryLevelNode> for InventoryLevelSchema {
    fn from(node: InventoryLevelNode) -> Self {
        InventoryLevelSchema {
            id: node.id,
            inventory_item_id: node.item.id,
            location_id: node.location.id,
            quantities: node.quantities.into_iter().map(|q| q.into()).collect(),
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
pub struct InventoryLevelNode {
    pub id: String,
    pub item: InventoryItemIdNode,
    pub location: LocationNode,
    pub quantities: Vec<QuantityNode>,
}

#[derive(Debug, Deserialize)]
pub struct QuantityNode {
    pub quantity: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemIdNode {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct InventoryChangeInput {
    pub delta: i32,
    #[serde(rename = "inventoryItemId")]
    pub inventory_item_id: String,
    #[serde(rename = "ledgerDocumentUri")]
    pub ledger_document_uri: Option<String>,
    #[serde(rename = "locationId")]
    pub location_id: String,
}

#[derive(Debug, Serialize)]
pub struct InventoryAdjustQuantitiesInput {
    pub changes: Vec<InventoryChangeInput>,
    pub name: String,
    pub reason: String,
}

impl From<InventoryType> for String {
    fn from(inventory_type: InventoryType) -> Self {
        match inventory_type {
            InventoryType::Available => "available".to_string(),
            InventoryType::Incoming => "incoming".to_string(),
            InventoryType::Committed => "committed".to_string(),
            InventoryType::Damaged => "damaged".to_string(),
            InventoryType::SafetyStock => "safety_stock".to_string(),
            InventoryType::Reserved => "reserved".to_string(),
        }
    }
}

impl From<InventoryChange> for InventoryAdjustQuantitiesInput {
    fn from(domain: InventoryChange) -> Self {
        InventoryAdjustQuantitiesInput {
            changes: domain
                .changes()
                .into_iter()
                .map(|change| InventoryChangeInput {
                    delta: *change.delta(),
                    inventory_item_id: ShopifyGQLQueryHelper::add_inventory_item_gid_prefix(
                        change.inventory_item_id(),
                    ),
                    ledger_document_uri: change
                        .ledger_document_uri()
                        .as_ref()
                        .and_then(|l| Some(l.value().to_string())),
                    location_id: ShopifyGQLQueryHelper::add_location_gid_prefix(
                        change.location_id(),
                    ),
                })
                .collect(),
            name: domain.name().to_owned().into(),
            reason: domain.reason().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct InventoryAdjustQuantitiesData {
    #[serde(rename = "inventoryAdjustQuantities")]
    pub inventory_adjust_quantities: UserErrors,
}
