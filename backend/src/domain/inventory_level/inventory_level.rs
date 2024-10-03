use crate::domain::{
    error::error::DomainError, inventory_item::inventory_item::Id as InventoryItemId,
    location::location::Id as LocationId,
};
use derive_getters::Getters;

use super::quantity::quantity::Quantity;

pub type Id = String;

#[derive(Debug, Getters)]
pub struct InventoryLevel {
    id: Id,
    inventory_item_id: InventoryItemId,
    location_id: LocationId,
    quantities: Vec<Quantity>,
}

impl InventoryLevel {
    pub fn new(
        id: impl Into<String>,
        inventory_item_id: impl Into<InventoryItemId>,
        location_id: impl Into<LocationId>,
        quantities: Vec<Quantity>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            id,
            inventory_item_id: inventory_item_id.into(),
            location_id: location_id.into(),
            quantities,
        })
    }
}
