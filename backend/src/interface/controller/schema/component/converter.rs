use crate::domain::{address::address::Address, error::error::DomainError};

use super::component::AddressSchema;

impl AddressSchema {
    pub fn to_domain(self) -> Result<Address, DomainError> {
        Address::new(
            self.address1.to_owned(),
            self.address2.to_owned(),
            self.city.to_owned(),
            false,
            self.country.to_owned(),
            self.first_name.to_owned(),
            self.last_name.to_owned(),
            self.province.to_owned(),
            self.zip.to_owned(),
            self.phone.to_owned(),
        )
    }
}
