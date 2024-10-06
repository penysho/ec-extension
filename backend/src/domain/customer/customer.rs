use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::{
    address::address::Address, email::email::Email, error::error::DomainError, media::media::Media,
    phone::phone::Phone,
};

pub type Id = String;

#[derive(Debug, Clone, PartialEq)]
pub enum CustomerStatus {
    Active,
    Inactive,
}

#[derive(Debug, Getters)]
pub struct Customer {
    id: Id,
    addresses: Vec<Address>,
    can_delete: bool,
    default_address: Option<Address>,
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
        default_address: Option<Address>,
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

        Ok(Self {
            id,
            addresses,
            can_delete,
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
