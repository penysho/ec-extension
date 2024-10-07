use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::{
    address::address::{Address, Id as AddressId},
    email::email::Email,
    error::error::DomainError,
    media::media::Media,
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
    addresses: Vec<Address>,
    can_delete: bool,
    default_address_id: Option<AddressId>,
    display_name: String,
    email: Option<Email>,
    first_name: Option<String>,
    last_name: Option<String>,
    image: Option<Media>,
    phone: Phone,
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
        default_address_id: Option<AddressId>,
        display_name: impl Into<String>,
        email: Option<Email>,
        first_name: Option<impl Into<String>>,
        last_name: Option<impl Into<String>>,
        image: Option<Media>,
        phone: Phone,
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
