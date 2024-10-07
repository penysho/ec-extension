use serde::Deserialize;

use crate::{
    domain::{address::address::Address, error::error::DomainError},
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

impl AddressSchema {
    pub fn to_domain(self) -> Result<Address, DomainError> {
        Address::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            self.address1,
            self.address2,
            self.city,
            self.coordinates_validated,
            self.country,
            self.first_name,
            self.last_name,
            self.state,
            self.zip,
            self.phone,
        )
    }
}

impl From<AddressNode> for AddressSchema {
    fn from(node: AddressNode) -> Self {
        AddressSchema {
            id: node.id,
            address1: node.address1,
            address2: node.address2,
            city: node.city,
            coordinates_validated: node.coordinates_validated,
            country: node.country,
            first_name: node.first_name,
            last_name: node.last_name,
            state: node.state,
            zip: node.zip,
            phone: node.phone,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AddressSchema {
    pub id: String,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub coordinates_validated: bool,
    pub country: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddressNode {
    pub id: String,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "coordinatesValidated")]
    pub coordinates_validated: bool,
    pub country: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
}
