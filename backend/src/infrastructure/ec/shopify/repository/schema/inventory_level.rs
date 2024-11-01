use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        inventory_level::{
            inventory_level::InventoryLevel,
            quantity::quantity::{InventoryType, Quantity},
        },
    },
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

impl InventoryLevelNode {
    pub fn to_domain(self) -> Result<InventoryLevel, DomainError> {
        let quantities: Result<Vec<Quantity>, DomainError> = self
            .quantities
            .into_iter()
            .map(|quantity| quantity.to_domain())
            .collect();

        InventoryLevel::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.item.id),
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.location.id),
            quantities?,
        )
    }

    pub fn to_domains(schemas: Vec<Self>) -> Result<Vec<InventoryLevel>, DomainError> {
        schemas
            .into_iter()
            .map(|schema| schema.to_domain())
            .collect()
    }
}

impl QuantityNode {
    pub fn to_domain(self) -> Result<Quantity, DomainError> {
        let inventory_type = match self.name.as_str() {
            "available" => InventoryType::Available,
            "incoming" => InventoryType::Incoming,
            "committed" => InventoryType::Committed,
            "damaged" => InventoryType::Damaged,
            "safety_stock" => InventoryType::SafetyStock,
            "reserved" => InventoryType::Reserved,
            _ => return Err(DomainError::ConversionError),
        };

        Quantity::new(self.quantity, inventory_type)
    }
}

#[derive(Debug, Deserialize)]
pub struct InventoryLevelNode {
    pub id: String,
    pub item: InventoryItemIdNode,
    pub location: LocationIdNode,
    pub quantities: Vec<QuantityNode>,
}

#[derive(Debug, Deserialize)]
pub struct QuantityNode {
    pub quantity: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LocationIdNode {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryItemIdNode {
    pub id: String,
}
