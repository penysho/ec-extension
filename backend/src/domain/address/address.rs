use derive_getters::Getters;

use crate::domain::error::error::DomainError;

pub type Id = String;

/// Represent the address that each entity has.
///
/// # Fields
/// * `id` - The unique identifier of the address.
/// * `address1` - The first line of the address.
/// * `address2` - The second line of the address.
/// * `city` - The city of the address.
/// * `coordinates_validated` - Whether the coordinates of the address are validated.
/// * `country` - The country of the address.
/// * `first_name` - The first name of the address.
/// * `last_name` - The last name of the address.
/// * `province` - The province of the address.
/// * `zip` - The zip code of the address.
/// * `phone` - The phone number of the address.
#[derive(Debug, Getters)]
pub struct Address {
    id: Id,
    address1: Option<String>,
    address2: Option<String>,
    city: Option<String>,
    coordinates_validated: bool,
    country: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    province: Option<String>,
    zip: Option<String>,
    phone: Option<String>,
}

impl Address {
    pub fn new(
        id: impl Into<String>,
        address1: impl Into<Option<String>>,
        address2: impl Into<Option<String>>,
        city: impl Into<Option<String>>,
        coordinates_validated: bool,
        country: impl Into<Option<String>>,
        first_name: impl Into<Option<String>>,
        last_name: impl Into<Option<String>>,
        province: impl Into<Option<String>>,
        zip: impl Into<Option<String>>,
        phone: impl Into<Option<String>>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            id: id.into(),
            address1: address1.into(),
            address2: address2.into(),
            city: city.into(),
            coordinates_validated,
            country: country.into(),
            first_name: first_name.into(),
            last_name: last_name.into(),
            province: province.into(),
            zip: zip.into(),
            phone: phone.into(),
        })
    }
}
