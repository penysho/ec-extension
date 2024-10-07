use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::domain::{
    customer::customer::{Customer, CustomerStatus},
    email::email::Email,
    error::error::DomainError,
    media::{associated_id::associated_id::AssociatedId, media::Media},
    phone::phone::Phone,
};

use super::{
    address::{AddressNode, AddressSchema},
    common::Edges,
    media::{MediaNode, MediaSchema},
};

impl CustomerSchema {
    pub fn to_domain(self) -> Result<Customer, DomainError> {
        let id = self.id;
        let status = match self.status.as_str() {
            "active" => CustomerStatus::Active,
            "inactive" => CustomerStatus::Inactive,
            _ => CustomerStatus::Inactive,
        };

        Customer::new(
            id.clone(),
            self.addresses
                .into_iter()
                .map(|address| address.to_domain())
                .collect::<Result<Vec<_>, _>>()?,
            self.can_delete,
            self.default_address.map(|address| address.id),
            self.display_name,
            self.email.map(|email| Email::new(email)).transpose()?,
            self.first_name,
            self.last_name,
            self.image
                .map(|image| image.to_domain(Some(AssociatedId::Customer(id.clone()))))
                .transpose()?,
            self.phone.map(|phone| Phone::new(phone)).transpose()?,
            self.note,
            status,
            self.verified_email,
            self.created_at,
            self.updated_at,
        )
    }
}

impl From<CustomerNode> for CustomerSchema {
    fn from(node: CustomerNode) -> Self {
        Self {
            id: node.id,
            addresses: node.addresses.into_iter().map(Into::into).collect(),
            can_delete: node.can_delete,
            default_address: node.default_address.map(Into::into),
            display_name: node.display_name,
            email: node.email,
            first_name: node.first_name,
            last_name: node.last_name,
            image: node.image.map(Into::into),
            phone: node.phone,
            note: node.note,
            status: node.status,
            verified_email: node.verified_email,
            created_at: node.created_at,
            updated_at: node.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CustomerSchema {
    pub id: String,
    pub addresses: Vec<AddressSchema>,
    pub can_delete: bool,
    pub default_address: Option<AddressSchema>,
    pub display_name: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image: Option<MediaSchema>,
    pub phone: Option<String>,
    pub note: Option<String>,
    pub status: String,
    pub verified_email: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CustomerNode {
    pub id: String,
    pub addresses: Vec<AddressNode>,
    pub can_delete: bool,
    #[serde(rename = "defaultAddress")]
    pub default_address: Option<AddressNode>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub email: Option<String>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub image: Option<MediaNode>,
    pub phone: Option<String>,
    pub note: Option<String>,
    pub status: String,
    #[serde(rename = "verifiedEmail")]
    pub verified_email: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct LocationsData {
    pub customers: Edges<CustomerNode>,
}
