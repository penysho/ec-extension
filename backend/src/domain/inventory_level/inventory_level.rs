use crate::domain::{
    error::error::DomainError, inventory_item::inventory_item::Id as InventoryItemId,
    location::location::Id as LocationId,
};
use derive_getters::Getters;

use super::{
    inventory_change::{
        change::{change::Change, ledger_document_uri::ledger_document_uri::LedgerDocumentUri},
        inventory_change::{InventoryChange, InventoryChangeReason},
    },
    quantity::quantity::{InventoryType, Quantity},
};

pub type Id = String;

/// Represents the inventory level of a specific item at a particular location.
///
/// The `InventoryLevel` struct holds information about the inventory of a specific
/// item in a given location. It includes the item's identifier, location, and the
/// quantities available at that location.
///
/// # Fields
/// - `id` - The unique identifier for this inventory level record.
/// - `inventory_item_id` - The identifier of the inventory item associated with this record.
/// - `location_id` - The identifier of the location where this inventory level applies.
/// - `quantities` - A vector representing the quantities available for this item at the location, including various types (e.g., available, reserved).
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

    pub fn create_inventory_change(
        &self,
        name: &InventoryType,
        reason: &InventoryChangeReason,
        delta: i32,
        ledger_document_uri: &Option<LedgerDocumentUri>,
    ) -> Result<InventoryChange, DomainError> {
        let change = Change::new(
            delta,
            self.inventory_item_id(),
            ledger_document_uri.to_owned(),
            self.location_id(),
        )?;

        InventoryChange::new(name.to_owned(), reason.to_owned(), vec![change])
    }

    pub fn update_quantity_by_delta(
        &mut self,
        name: &InventoryType,
        delta: i32,
    ) -> Result<(), DomainError> {
        let index = self
            .quantities
            .iter()
            .position(|q| q.inventory_type() == name);

        match index {
            Some(index) => {
                let _ = self.quantities[index].apply_delta(delta)?;
                Ok(())
            }
            None => {
                let quantity = Quantity::new(delta, name.to_owned())?;
                self.quantities.push(quantity);
                Ok(())
            }
        }
    }
}
