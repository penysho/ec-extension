use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError, inventory_level::quantity::quantity::InventoryType,
};

use super::change::change::Change;

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
            InventoryType::Available => {}
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
}
