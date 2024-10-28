use derive_getters::Getters;

use crate::domain::{address::address::Address, error::error::DomainError};

pub type Id = String;

/// Represent locations where inventory is controlled.
///
/// # Fields
/// - `id` - The unique identifier for the location.
/// - `name` - The name of the location.
/// - `is_active` - Indicates whether the location is currently active.
/// - `fulfills_online_orders` - Indicates whether the location fulfills online orders.
/// - `address` - The primary address of the location.
/// - `suggested_addresses` - A list of suggested alternative addresses for the location.
#[derive(Debug, Getters)]
pub struct Location {
    id: Id,
    name: String,
    is_active: bool,
    fulfills_online_orders: bool,
    address: Address,
    suggested_addresses: Vec<Address>,
}

impl Location {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        is_active: bool,
        fulfills_online_orders: bool,
        address: Address,
        suggested_addresses: Vec<Address>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let name = name.into();
        if name.is_empty() {
            log::error!("Name cannot be empty");
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            id,
            name,
            is_active,
            fulfills_online_orders,
            address,
            suggested_addresses,
        })
    }
}
