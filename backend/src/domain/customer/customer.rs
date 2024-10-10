use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::{
    address::address::{Address, Id as AddressId},
    email::email::Email,
    error::error::DomainError,
    media::media_content::image::image::Image,
    phone::phone::Phone,
};

pub type Id = String;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomerStatus {
    Active,
    Inactive,
}

/// Representing Customers on an E-Commerce Site.
///
/// A `Customer` contains various details such as addresses, contact information,
/// and status. It provides core customer-related attributes and ensures that
/// required fields such as `id` and `display_name` are not empty.
///
/// # Fields
/// - `id` - A unique identifier for the customer.
/// - `addresses` - A list of addresses associated with the customer.
/// - `can_delete` - A flag indicating whether the customer can be deleted.
/// - `default_address_id` - The default address id for the customer, if applicable.
/// - `display_name` - The name displayed for the customer.
/// - `email` - The customer's email address (optional).
/// - `first_name` - The customer's first name (optional).
/// - `last_name` - The customer's last name (optional).
/// - `image` - The customer's profile image (optional).
/// - `phone` - The customer's phone number.
/// - `note` - Additional notes about the customer (optional).
/// - `status` - The current status of the customer (e.g., `Active`, `Inactive`).
/// - `verified_email` - A flag indicating whether the customer's email is verified.
/// - `created_at` - The date and time the customer record was created.
/// - `updated_at` - The date and time the customer record was last updated.
#[derive(Debug, Getters)]
pub struct Customer {
    id: Id,
    addresses: Vec<Address>,
    can_delete: bool,
    default_address_id: Option<AddressId>,
    display_name: String,
    email: Option<Email>,
    first_name: Option<String>,
    last_name: Option<String>,
    image: Option<Image>,
    phone: Option<Phone>,
    note: Option<String>,
    status: CustomerStatus,
    verified_email: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Customer {
    pub fn new(
        id: impl Into<String>,
        addresses: Vec<Address>,
        can_delete: bool,
        default_address_id: Option<impl Into<AddressId>>,
        display_name: impl Into<String>,
        email: Option<Email>,
        first_name: Option<impl Into<String>>,
        last_name: Option<impl Into<String>>,
        image: Option<Image>,
        phone: Option<Phone>,
        note: Option<impl Into<String>>,
        status: CustomerStatus,
        verified_email: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let display_name = display_name.into();
        if display_name.is_empty() {
            log::error!("Display name cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let default_address_id = default_address_id.map(|id| id.into());
        if let Some(default_address_id) = &default_address_id {
            if !addresses
                .iter()
                .any(|a| a.id().clone() == default_address_id.clone())
            {
                log::error!("Default address ID is invalid");
                return Err(DomainError::ValidationError);
            }
        }

        Ok(Self {
            id,
            addresses,
            can_delete,
            default_address_id,
            display_name,
            email,
            first_name: first_name.map(|f| f.into()),
            last_name: last_name.map(|l| l.into()),
            image,
            phone,
            note: note.map(|n| n.into()),
            status,
            verified_email,
            created_at,
            updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn mock_address(id: &str) -> Address {
        Address::new(
            id,
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
        .unwrap()
    }

    #[test]
    fn test_create_customer_success() {
        let address = mock_address("1");
        let addresses = vec![address];

        let customer = Customer::new(
            "123",
            addresses,
            true,
            Some("1"),
            "John Doe",
            Some(Email::new("john@example.com").unwrap()),
            Some("John"),
            Some("Doe"),
            None,
            Some(Phone::new("+1234567890").unwrap()),
            Some("Note"),
            CustomerStatus::Active,
            true,
            Utc::now(),
            Utc::now(),
        );

        assert!(customer.is_ok());

        let customer = customer.unwrap();
        assert_eq!(customer.id(), "123");
        assert_eq!(customer.display_name(), "John Doe");
        assert_eq!(
            customer.email().as_ref().unwrap().value(),
            "john@example.com"
        );
        assert_eq!(customer.first_name().as_ref().unwrap(), "John");
        assert_eq!(customer.last_name().as_ref().unwrap(), "Doe");
        assert_eq!(customer.phone().as_ref().unwrap().value(), "+1234567890");
        assert_eq!(customer.note().as_ref().unwrap(), "Note");
        assert!(customer.verified_email());
        assert_eq!(customer.addresses().len(), 1);
        assert_eq!(customer.default_address_id().as_ref().unwrap(), "1",);
    }

    #[test]
    fn test_create_customer_error_empty_id() {
        let address = mock_address("1");
        let addresses = vec![address];

        let customer = Customer::new(
            "",
            addresses,
            true,
            Some("1"),
            "John Doe",
            Some(Email::new("john@example.com").unwrap()),
            Some("John"),
            Some("Doe"),
            None,
            Some(Phone::new("+1234567890").unwrap()),
            Some("Note"),
            CustomerStatus::Active,
            true,
            Utc::now(),
            Utc::now(),
        );

        assert!(customer.is_err());
        assert_eq!(customer.unwrap_err(), DomainError::ValidationError);
    }

    #[test]
    fn test_create_customer_error_empty_display_name() {
        let address = mock_address("1");
        let addresses = vec![address];

        let customer = Customer::new(
            "123",
            addresses,
            true,
            Some("1"),
            "",
            Some(Email::new("john@example.com").unwrap()),
            Some("John"),
            Some("Doe"),
            None,
            Some(Phone::new("+1234567890").unwrap()),
            Some("Note"),
            CustomerStatus::Active,
            true,
            Utc::now(),
            Utc::now(),
        );

        assert!(customer.is_err());
        assert_eq!(customer.unwrap_err(), DomainError::ValidationError);
    }

    #[test]
    fn test_create_customer_error_invalid_default_address() {
        let address = mock_address("1");
        let addresses = vec![address];

        let customer = Customer::new(
            "123",
            addresses,
            true,
            Some("invalid"),
            "John Doe",
            Some(Email::new("john@example.com").unwrap()),
            Some("John"),
            Some("Doe"),
            None,
            Some(Phone::new("+1234567890").unwrap()),
            Some("Note"),
            CustomerStatus::Active,
            true,
            Utc::now(),
            Utc::now(),
        );

        assert!(customer.is_err());
        assert_eq!(customer.unwrap_err(), DomainError::ValidationError);
    }
}
