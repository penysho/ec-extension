use serde::Serialize;

use crate::domain::address::address::Address;

impl From<Address> for AddressInput {
    fn from(address: Address) -> Self {
        AddressInput {
            address1: address.address1().to_owned(),
            address2: address.address2().to_owned(),
            city: address.city().to_owned(),
            country: address.country().to_owned(),
            first_name: address.first_name().to_owned(),
            last_name: address.last_name().to_owned(),
            province: address.province().to_owned(),
            zip: address.zip().to_owned(),
            phone: address.phone().to_owned(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressInput {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub province: Option<String>,
    pub zip: Option<String>,
    pub phone: Option<String>,
}
