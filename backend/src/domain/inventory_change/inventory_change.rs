use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError,
    inventory_item::inventory_item::Id as InventoryItemId,
    inventory_level::quantity::quantity::{InventoryType, Quantity},
    location::location::Id as LocationId,
};

use super::change::{change::Change, ledger_document_uri::ledger_document_uri::LedgerDocumentUri};

#[derive(Debug, Getters)]
pub struct InventoryChange {
    name: InventoryType,
    reason: String,
    changes: Vec<Change>,
}

impl InventoryChange {
    pub fn new(
        name: InventoryType,
        reason: String,
        changes: Vec<Change>,
    ) -> Result<Self, DomainError> {
        match name {
            InventoryType::Available
                if changes
                    .iter()
                    .filter(|c| c.ledger_document_uri().is_some())
                    .count()
                    > 0 =>
            {
                log::error!("Available inventory cannot have ledger document URIs");
                return Err(DomainError::ValidationError);
            }
            _ if changes
                .iter()
                .filter(|c| c.ledger_document_uri().is_none())
                .count()
                > 0 =>
            {
                log::error!("Inventory change must have ledger document URIs");
                return Err(DomainError::ValidationError);
            }
            _ => {}
        }

        Ok(Self {
            name,
            reason,
            changes,
        })
    }

    /// Creates a new InventoryChange instance for a single change.
    pub fn create(
        name: InventoryType,
        reason: String,
        ledger_document_uri: Option<LedgerDocumentUri>,
        current_quantity: Quantity,
        updated_quantity: Quantity,
        inventory_item_id: InventoryItemId,
        location_id: LocationId,
    ) -> Result<Self, DomainError> {
        let delta = (updated_quantity.quantity().clone() as i32)
            - (current_quantity.quantity().clone() as i32);
        let change = Change::new(delta, inventory_item_id, ledger_document_uri, location_id)?;

        Self::new(name, reason, vec![change])
    }
}
