use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::{
    domain::{
        address::address::Address,
        authorized_resource::authorized_resource::{AuthorizedResource, ResourceType},
        email::email::Email,
        error::error::DomainError,
        media::media_content::image::image::Image,
        phone::phone::Phone,
        user::user::Id as UserId,
    },
    log_error,
};

pub type Id = String;

/// Represents the status of a customer.
///
/// # Variants
/// - `Active` - The customer is active and can perform certain actions.
/// - `Inactive` - The customer is inactive and cannot perform certain actions.
#[derive(Debug, Clone, PartialEq)]
pub enum CustomerStatus {
    Active,
    Inactive,
}

/// Representing Customers.
///
/// A `Customer` contains various details such as addresses, contact information,
/// and status. It provides core customer-related attributes and ensures that
/// required fields such as `id` and `display_name` are not empty.
///
/// # Fields
/// - `id` - A unique identifier for the customer.
/// - `user_id` - A unique identifier for for authentication and authorization.
/// - `addresses` - A list of addresses associated with the customer.
/// - `default_address` - The default address for the customer, if applicable.
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
    user_id: UserId,
    addresses: Vec<Address>,
    default_address: Option<Address>,
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
    /// Constructor to be used from the repository.
    pub fn new(
        id: impl Into<Id>,
        user_id: impl Into<String>,
        addresses: Vec<Address>,
        default_address: Option<Address>,
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
            log_error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let display_name = display_name.into();
        if display_name.is_empty() {
            log_error!("Display name cannot be empty");
            return Err(DomainError::ValidationError);
        }
        if let Some(default_address) = &default_address {
            if !addresses
                .iter()
                .any(|a| a.clone() == default_address.clone())
            {
                log_error!("Default address ID is invalid");
                return Err(DomainError::ValidationError);
            }
        }

        Ok(Self {
            id,
            user_id: user_id.into(),
            addresses,
            default_address,
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

impl AuthorizedResource for Customer {
    fn resource_type(&self) -> ResourceType {
        ResourceType::Customer
    }

    fn owner_user_id(&self) -> Option<UserId> {
        Some(self.user_id().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

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
        let customer = Customer::new(
            "123",
            "user123",
            vec![mock_address()],
            Some(mock_address()),
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
        assert_eq!(customer.user_id(), "user123");
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
    }

    #[test]
    fn test_new_error_empty_id() {
        let customer = Customer::new(
            "",
            "user123",
            vec![mock_address()],
            Some(mock_address()),
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
    fn test_new_error_empty_display_name() {
        let customer = Customer::new(
            "123",
            "user123",
            vec![mock_address()],
            Some(mock_address()),
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
    fn test_new_error_invalid_default_address() {
        let customer = Customer::new(
            "123",
            "user123",
            vec![mock_address()],
            Some(
                Address::new(
                    None::<String>, // Different value from mock_address()
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
                .expect("Failed to create mock address"),
            ),
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
