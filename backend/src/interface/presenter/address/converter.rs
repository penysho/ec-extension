use crate::domain::address::address::Address;

use super::schema::AddressSchema;

impl From<&Address> for AddressSchema {
    fn from(address: &Address) -> Self {
        Self {
            id: address.id().to_string(),
            address1: address.address1().to_owned(),
            address2: address.address2().to_owned(),
            city: address.city().to_owned(),
            coordinates_validated: *address.coordinates_validated(),
            country: address.country().to_owned(),
            first_name: address.first_name().to_owned(),
            last_name: address.last_name().to_owned(),
            province: address.province().to_owned(),
            zip: address.zip().to_owned(),
            phone: address.phone().to_owned(),
        }
    }
}
