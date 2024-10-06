use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError, inventory_item::inventory_item::Id as InventoryItemId,
    location::location::Id as LocationId,
};

use super::ledger_document_uri::ledger_document_uri::LedgerDocumentUri;

/// Represents a specific change in the inventory system.
///
/// The `Change` struct captures the details of an individual inventory adjustment, including the quantity change,
/// the associated inventory item, and its location. It also includes an optional ledger document URI to track the source of the change.
///
/// # Fields
/// - `delta` - The amount by which the inventory quantity has changed (positive or negative).
/// - `inventory_item_id` - The ID of the inventory item that this change applies to.
/// - `ledger_document_uri` - An optional URI pointing to a ledger document associated with this change, used for tracking purposes.
/// - `location_id` - The location where the inventory change has taken place.
///
/// # Example
/// A positive `delta` might indicate an increase in stock at a certain location, while a negative `delta` could represent an item being removed or used.
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
        inventory_item_id: impl Into<InventoryItemId>,
        ledger_document_uri: Option<LedgerDocumentUri>,
        location_id: impl Into<LocationId>,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            delta,
            inventory_item_id: inventory_item_id.into(),
            ledger_document_uri,
            location_id: location_id.into(),
        })
    }
}
