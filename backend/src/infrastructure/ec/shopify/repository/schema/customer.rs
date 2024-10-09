use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{
        customer::customer::{Customer, CustomerStatus},
        email::email::Email,
        error::error::DomainError,
        media::associated_id::associated_id::AssociatedId,
        phone::phone::Phone,
    },
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::{address::AddressNode, common::Edges, media::MediaNode};

impl CustomerNode {
    pub fn to_domain(self) -> Result<Customer, DomainError> {
        let id = ShopifyGQLQueryHelper::remove_gid_prefix(&self.id);
        let status = match self.status.as_str() {
            "ENABLED" => CustomerStatus::Active,
            "DISABLED" => CustomerStatus::Inactive,
            _ => CustomerStatus::Inactive,
        };

        Customer::new(
            id.clone(),
            self.addresses
                .into_iter()
                .map(|address| address.to_domain())
                .collect::<Result<Vec<_>, _>>()?,
            self.can_delete,
            self.default_address
                .map(|address| ShopifyGQLQueryHelper::remove_gid_prefix(&address.id)),
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

    pub fn to_domains(schemas: Vec<Self>) -> Result<Vec<Customer>, DomainError> {
        schemas
            .into_iter()
            .map(|schema| schema.to_domain())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct CustomersData {
    pub customers: Edges<CustomerNode>,
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
