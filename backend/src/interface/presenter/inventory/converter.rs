use crate::domain::{
    inventory_item::inventory_item::InventoryItem,
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

impl From<InventoryLevel> for InventoryLevelSchema {
    fn from(inventory_level: InventoryLevel) -> Self {
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

impl InventorySchema {
    pub(super) fn to_schema(
        inventory_item: InventoryItem,
        inventory_level: Vec<InventoryLevel>,
    ) -> Self {
        InventorySchema {
            id: inventory_item.id().to_string(),
            variant_id: inventory_item.variant_id().to_string(),
            inventory_levels: inventory_level.into_iter().map(|l| l.into()).collect(),
            requires_shipping: *inventory_item.requires_shipping(),
            tracked: *inventory_item.tracked(),
            created_at: inventory_item.created_at().to_owned(),
            updated_at: inventory_item.updated_at().to_owned(),
        }
    }
}
