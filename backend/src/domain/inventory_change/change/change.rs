use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError, inventory_item::inventory_item::Id as InventoryItemId,
    location::location::Id as LocationId,
};

use super::ledger_document_uri::ledger_document_uri::LedgerDocumentUri;

#[derive(Debug, Getters)]
pub struct Change {
    delta: i32,
    inventory_item_id: InventoryItemId,
    ledger_document_uri: Option<LedgerDocumentUri>,
    location_id: LocationId,
}

impl Change {
    pub fn new(
        delta: i32,
        inventory_item_id: InventoryItemId,
        ledger_document_uri: Option<LedgerDocumentUri>,
        location_id: LocationId,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            delta,
            inventory_item_id,
            ledger_document_uri,
            location_id,
        })
    }
}
