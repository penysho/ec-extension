use crate::domain::inventory::{
    inventory::Inventory,
    inventory_level::{
        inventory_level::InventoryLevel,
        quantity::quantity::{InventoryType, Quantity},
    },
};

use super::schema::{InventoryLevelSchema, InventorySchema, InventoryTypeEnum, QuantitySchema};

impl From<&InventoryType> for InventoryTypeEnum {
    fn from(inventory_type: &InventoryType) -> Self {
        match inventory_type {
            InventoryType::Available => InventoryTypeEnum::Available,
            InventoryType::Committed => InventoryTypeEnum::Committed,
            InventoryType::Incoming => InventoryTypeEnum::Incoming,
            InventoryType::Reserved => InventoryTypeEnum::Reserved,
            InventoryType::SafetyStock => InventoryTypeEnum::SafetyStock,
            InventoryType::Damaged => InventoryTypeEnum::Damaged,
        }
    }
}

impl From<&Quantity> for QuantitySchema {
    fn from(quantity: &Quantity) -> Self {
        QuantitySchema {
            inventory_type: quantity.inventory_type().into(),
            quantity: *quantity.quantity(),
        }
    }
}

impl From<&InventoryLevel> for InventoryLevelSchema {
    fn from(inventory_level: &InventoryLevel) -> Self {
        InventoryLevelSchema {
            id: inventory_level.id().to_string(),
            location_id: inventory_level.location_id().to_string(),
            quantities: inventory_level
                .quantities()
                .iter()
                .map(|q| q.into())
                .collect(),
        }
    }
}

impl From<Inventory> for InventorySchema {
    fn from(inventory: Inventory) -> Self {
        InventorySchema {
            id: inventory.id().to_string(),
            variant_id: inventory.variant_id().to_string(),
            inventory_level: inventory.inventory_level().as_ref().map(|i| i.into()),
            requires_shipping: *inventory.requires_shipping(),
            tracked: *inventory.tracked(),
            created_at: inventory.created_at().to_owned(),
            updated_at: inventory.updated_at().to_owned(),
        }
    }
}
