use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError, inventory_level::quantity::quantity::InventoryType,
};

use super::change::change::Change;

/// Reason for changing inventory.
///
/// * correction	Used to correct an inventory error or as a general adjustment reason.
/// * cycle_count_available	Used to specify an adjusted inventory count due to a discrepancy between the actual inventory quantity and previously recorded inventory quantity.
/// * damaged	Used to remove units from inventory count due to damage.
/// * movement_created	Used to specify that an inventory transfer or a purchase order has been created.
/// * movement_updated	Used to specify that an inventory transfer or a purchase order has been updated.
/// * movement_received	Used to specify that an inventory transfer or a purchase order has been received.
/// * movement_canceled	Used to specify that an inventory transfer or a purchase order has been canceled.
/// * other	Used to specify an alternate reason for the inventory adjustment.
/// * promotion	Used to remove units from inventory count due to a promotion or donation.
/// * quality_control	Used to specify that on-hand units that aren't sellable because they're currently in inspection for quality purposes.
/// * received	Used to specify inventory that the merchant received.
/// * reservation_created	Used to reserve, or temporarily set aside unavailable units.
/// * reservation_deleted	Used to remove the number of unavailable units that have been reserved.
/// * reservation_updated	Used to update the number of unavailable units that have been reserved.
/// * restock	Used to add a returned unit back to available inventory so the unit can be resold.
/// * safety_stock	Used to specify that on-hand units are being set aside to help guard against overselling.
/// * shrinkage	Used when actual inventory levels are less than recorded due to theft or loss.
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
