use serde::Deserialize;

use crate::{
    domain::{address::address::Address, error::error::DomainError, location::location::Location},
    infrastructure::ec::shopify::{gql_helper::ShopifyGQLHelper, schema::Edges},
};

impl LocationNode {
    pub fn to_domain(self) -> Result<Location, DomainError> {
        let id = ShopifyGQLHelper::remove_gid_prefix(&self.id);

        Location::new(
            id,
            self.name,
            self.is_active,
            self.fulfills_online_orders,
            self.address.to_domain()?,
            self.suggested_addresses
                .into_iter()
                .map(|address| address.to_domain())
                .collect::<Result<Vec<_>, _>>()?,
        )
    }

    pub fn to_domains(schemas: Vec<Self>) -> Result<Vec<Location>, DomainError> {
        schemas
            .into_iter()
            .map(|schema| schema.to_domain())
            .collect::<Result<Vec<_>, _>>()
    }
}

impl LocationAddressNode {
    pub fn to_domain(self) -> Result<Address, DomainError> {
        Address::new(
            self.address1,
            self.address2,
            self.city,
            true,
            self.country,
            None::<String>,
            None::<String>,
            self.province,
            self.zip,
            None::<String>,
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct LocationsData {
    pub locations: Edges<LocationNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationNode {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub fulfills_online_orders: bool,
    pub address: LocationAddressNode,
    pub suggested_addresses: Vec<LocationAddressNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationAddressNode {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub zip: Option<String>,
}
