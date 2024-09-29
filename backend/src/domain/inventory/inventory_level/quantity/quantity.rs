use derive_getters::Getters;

use crate::domain::error::error::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub enum InventoryType {
    Available,
    Committed,
    Incoming,
    Reserved,
    SafetyStock,
    Damaged,
}

#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Quantity {
    quantity: u32,
    inventory_type: InventoryType,
}

impl Quantity {
    pub fn new(quantity: u32, inventory_type: InventoryType) -> Result<Self, DomainError> {
        Ok(Self {
            quantity,
            inventory_type,
        })
    }
}
