use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError, inventory_level::quantity::quantity::InventoryType,
};
use crate::log_error;

use super::change::change::Change;

/// Reason for changing inventory.
///
/// # Variants
/// * correction - Used to correct an inventory error or as a general adjustment reason.
/// * cycle_count_available	 - Used to specify an adjusted inventory count due to a discrepancy between the actual inventory quantity and previously recorded inventory quantity.
/// * damaged	 - Used to remove units from inventory count due to damage.
/// * movement_created	 - Used to specify that an inventory transfer or a purchase order has been created.
/// * movement_updated	 - Used to specify that an inventory transfer or a purchase order has been updated.
/// * movement_received	 - Used to specify that an inventory transfer or a purchase order has been received.
/// * movement_canceled	 - Used to specify that an inventory transfer or a purchase order has been canceled.
/// * other	 - Used to specify an alternate reason for the inventory adjustment.
/// * promotion	 - Used to remove units from inventory count due to a promotion or donation.
/// * quality_control	 - Used to specify that on-hand units that aren't sellable because they're currently in inspection for quality purposes.
/// * received	 - Used to specify inventory that the merchant received.
/// * reservation_created	 - Used to reserve, or temporarily set aside unavailable units.
/// * reservation_deleted	 - Used to remove the number of unavailable units that have been reserved.
/// * reservation_updated	 - Used to update the number of unavailable units that have been reserved.
/// * restock	 - Used to add a returned unit back to available inventory so the unit can be resold.
/// * safety_stock	 - Used to specify that on-hand units are being set aside to help guard against overselling.
/// * shrinkage	 - Used when actual inventory levels are less than recorded due to theft or loss.
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryChangeReason {
    Correction,
    CycleCountAvailable,
    Damaged,
    MovementCreated,
    MovementUpdated,
    MovementReceived,
    MovementCanceled,
    Other,
    Promotion,
    QualityControl,
    Received,
    ReservationCreated,
    ReservationDeleted,
    ReservationUpdated,
}

/// Represents a change made to the inventory.
///
/// The `InventoryChange` struct tracks changes applied to the inventory, such as movements, corrections, or other adjustments.
/// It captures the type of inventory affected, the reason for the change, and a list of specific changes that have been made.
///
/// # Fields
/// - `name` - The type of inventory that the change applies to (e.g., Available, Committed).
/// - `reason` - The reason for the inventory change, represented by the `InventoryChangeReason` enum (e.g., MovementCreated, Damaged).
/// - `changes` - A vector of `Change` structs, each representing a specific alteration to the inventory, such as adjustments to the quantity or location.
///
/// # Example
/// An example of an inventory change could be receiving new stock, where the `name` is `InventoryType::Incoming`,
/// the `reason` is `InventoryChangeReason::Received`, and the `changes` list contains the specific adjustments.
#[derive(Debug, Getters)]
pub struct InventoryChange {
    name: InventoryType,
    reason: InventoryChangeReason,
    changes: Vec<Change>,
}

impl InventoryChange {
    pub fn new(
        name: InventoryType,
        reason: InventoryChangeReason,
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
                log_error!("Available inventory cannot have ledger document URIs");
                return Err(DomainError::ValidationError);
            }
            InventoryType::Available => {}
            _ if changes
                .iter()
                .filter(|c| c.ledger_document_uri().is_none())
                .count()
                > 0 =>
            {
                log_error!("Inventory change must have ledger document URIs");
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

#[cfg(test)]
mod tests {
    use crate::domain::inventory_level::inventory_change::change::ledger_document_uri::ledger_document_uri::LedgerDocumentUri;

    use super::*;

    fn mock_change(delta: i32, ledger_document_uri: Option<LedgerDocumentUri>) -> Change {
        Change::new(
            delta,
            "inventory_item_123",
            ledger_document_uri,
            "location_456",
        )
        .unwrap()
    }

    #[test]
    fn test_new_with_available_and_invalid_ledger_document_uri() {
        let change_with_ledger_uri = mock_change(
            10,
            Some(LedgerDocumentUri::new("https://example.com/document").unwrap()),
        );

        let result = InventoryChange::new(
            InventoryType::Available,
            InventoryChangeReason::Correction,
            vec![change_with_ledger_uri],
        );

        assert!(result.is_err());
        if let Err(DomainError::ValidationError) = result {
            assert!(true);
        } else {
            assert!(false, "Expected DomainError::ValidationError");
        }
    }

    #[test]
    fn test_new_with_available_and_no_ledger_document_uri() {
        let change_without_ledger_uri = mock_change(10, None);

        let result = InventoryChange::new(
            InventoryType::Available,
            InventoryChangeReason::Correction,
            vec![change_without_ledger_uri],
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_new_with_non_available_and_missing_ledger_document_uri() {
        let change_without_ledger_uri = mock_change(10, None);

        let result = InventoryChange::new(
            InventoryType::Committed,
            InventoryChangeReason::Correction,
            vec![change_without_ledger_uri],
        );

        assert!(result.is_err());
        if let Err(DomainError::ValidationError) = result {
            assert!(true);
        } else {
            assert!(false, "Expected DomainError::ValidationError");
        }
    }

    #[test]
    fn test_new_with_non_available_and_valid_ledger_document_uri() {
        let change_with_ledger_uri = mock_change(
            10,
            Some(LedgerDocumentUri::new("https://example.com/document").unwrap()),
        );

        let result = InventoryChange::new(
            InventoryType::Committed,
            InventoryChangeReason::Correction,
            vec![change_with_ledger_uri],
        );

        assert!(result.is_ok());
    }
}
