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
    infrastructure::ec::shopify::{
        gql_helper::ShopifyGQLHelper,
        schema::{Edges, Metafield},
    },
};

use super::{address::AddressNode, media::ImageNode};

impl CustomerNode {
    pub fn to_domain(self) -> Result<Customer, DomainError> {
        let id = ShopifyGQLHelper::remove_gid_prefix(&self.id);
        let status = match self.state.as_str() {
            "ENABLED" => Ok(CustomerStatus::Active),
            "DISABLED" => Ok(CustomerStatus::Inactive),
            _ => Err(DomainError::ConversionError),
        }?;
        let image = match self.image.id.clone() {
            Some(id) => Some(
                self.image
                    .to_domain(Some(AssociatedId::Customer(id.clone())))?,
            ),
            None => None,
        };

        Customer::new(
            id.clone(),
            self.user_id.value,
            self.addresses
                .into_iter()
                .map(|address| address.to_domain())
                .collect::<Result<Vec<_>, _>>()?,
            self.default_address
                .map(|address| address.to_domain())
                .transpose()?,
            self.display_name,
            self.email.map(|email| Email::new(email)).transpose()?,
            self.first_name,
            self.last_name,
            image,
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
#[serde(rename_all = "camelCase")]
pub struct CustomerNode {
    pub id: String,
    #[serde(rename = "metafield")]
    pub user_id: Metafield<String>,
    pub addresses: Vec<AddressNode>,
    pub default_address: Option<AddressNode>,
    pub display_name: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub image: ImageNode,
    pub phone: Option<String>,
    pub note: Option<String>,
    pub state: String,
    pub verified_email: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
