use derive_getters::Getters;

use crate::domain::{address::address::Address, error::error::DomainError};
use crate::log_error;

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
        id: impl Into<Id>,
        name: impl Into<String>,
        is_active: bool,
        fulfills_online_orders: bool,
        address: Address,
        suggested_addresses: Vec<Address>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log_error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let name = name.into();
        if name.is_empty() {
            log_error!("Name cannot be empty");
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

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_address() -> Address {
        Address::new(
            Some("123 Main St"),
            None::<String>,
            Some("City"),
            true,
            Some("Country"),
            Some("John"),
            Some("Doe"),
            Some("Province"),
            Some("12345"),
            Some("+1234567890"),
        )
        .expect("Failed to create mock address")
    }

    #[test]
    fn test_new_success() {
        let location = Location::new(
            "loc_1",
            "Main Warehouse",
            true,
            false,
            mock_address(),
            vec![mock_address()],
        );

        assert!(location.is_ok());
        let location = location.unwrap();
        assert_eq!(location.id(), "loc_1");
        assert_eq!(location.name(), "Main Warehouse");
        assert!(location.is_active());
        assert!(!location.fulfills_online_orders());
    }

    #[test]
    fn test_new_with_empty_id_should_fail() {
        let result = Location::new("", "Main Warehouse", true, false, mock_address(), vec![]);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::ValidationError);
    }

    #[test]
    fn test_new_with_empty_name_should_fail() {
        let result = Location::new("loc_1", "", true, false, mock_address(), vec![]);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainError::ValidationError);
    }
}
