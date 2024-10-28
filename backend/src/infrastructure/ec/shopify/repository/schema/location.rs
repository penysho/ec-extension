use serde::Deserialize;

use crate::{
    domain::{error::error::DomainError, location::location::Location},
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::{address::AddressNode, common::Edges};

impl LocationNode {
    pub fn to_domain(self) -> Result<Location, DomainError> {
        let id = ShopifyGQLQueryHelper::remove_gid_prefix(&self.id);

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

#[derive(Debug, Deserialize)]
pub struct LocationsData {
    pub locations: Edges<LocationNode>,
}

#[derive(Debug, Deserialize)]
pub struct LocationNode {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub fulfills_online_orders: bool,
    pub address: AddressNode,
    pub suggested_addresses: Vec<AddressNode>,
}
