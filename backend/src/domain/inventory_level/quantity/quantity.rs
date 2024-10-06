use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// Represents different types of inventory in the system.
///
/// The `InventoryType` enum defines various categories of inventory that an item can be classified into.
/// It helps in distinguishing the stock based on its state, such as available, committed, or damaged.
///
/// # Variants
/// - `Available`: Represents the inventory that is currently available for sale or use.
/// - `Committed` - Represents inventory that has been committed for an order or use but not yet fulfilled.
/// - `Incoming` - Represents inventory that is expected to arrive in the future.
/// - `Reserved` - Represents inventory that has been reserved for future use.
/// - `SafetyStock` - Represents the inventory kept aside as safety stock for emergencies.
/// - `Damaged` - Represents inventory that has been damaged and is not available for sale.
#[derive(Debug, Clone, PartialEq)]
pub enum InventoryType {
    Available,
    Committed,
    Incoming,
    Reserved,
    SafetyStock,
    Damaged,
}

/// Represents the quantity of a specific inventory type.
///
/// The `Quantity` struct holds the actual quantity of an item along with the type of inventory it belongs to.
/// This helps in tracking how much of each type of inventory (e.g., available, committed) exists for a given item.
///
/// # Fields
/// - `quantity` - The numerical value representing the amount of this inventory.
/// - `inventory_type` - The type of inventory (e.g., Available, Committed, Damaged).
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Quantity {
    quantity: i32,
    inventory_type: InventoryType,
}

impl Quantity {
    pub fn new(quantity: i32, inventory_type: InventoryType) -> Result<Self, DomainError> {
        Ok(Self {
            quantity,
            inventory_type,
        })
    }

    pub fn apply_delta(&mut self, delta: i32) -> Result<(), DomainError> {
        self.quantity = self.quantity + delta;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_new() {
        let quantity = Quantity::new(10, InventoryType::Available).unwrap();
        assert_eq!(quantity.quantity(), &10);
        assert_eq!(quantity.inventory_type(), &InventoryType::Available);
    }

    #[test]
    fn test_quantity_apply_delta_positive() {
        let mut quantity = Quantity::new(10, InventoryType::Available).unwrap();
        quantity.apply_delta(5).unwrap();
        assert_eq!(quantity.quantity(), &15);
    }

    #[test]
    fn test_quantity_apply_delta_negative() {
        let mut quantity = Quantity::new(10, InventoryType::Available).unwrap();
        quantity.apply_delta(-3).unwrap();
        assert_eq!(quantity.quantity(), &7);
    }

    #[test]
    fn test_quantity_apply_delta_zero() {
        let mut quantity = Quantity::new(10, InventoryType::Available).unwrap();
        quantity.apply_delta(0).unwrap();
        assert_eq!(quantity.quantity(), &10);
    }
}
