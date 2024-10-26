use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// Represent the address that each entity has.
///
/// # Fields
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
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Address {
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
        address1: Option<impl Into<String>>,
        address2: Option<impl Into<String>>,
        city: Option<impl Into<String>>,
        coordinates_validated: bool,
        country: Option<impl Into<String>>,
        first_name: Option<impl Into<String>>,
        last_name: Option<impl Into<String>>,
        province: Option<impl Into<String>>,
        zip: Option<impl Into<String>>,
        phone: Option<impl Into<String>>,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            address1: address1.map(|a| a.into()),
            address2: address2.map(|a| a.into()),
            city: city.map(|a| a.into()),
            coordinates_validated,
            country: country.map(|a| a.into()),
            first_name: first_name.map(|a| a.into()),
            last_name: last_name.map(|a| a.into()),
            province: province.map(|a| a.into()),
            zip: zip.map(|a| a.into()),
            phone: phone.map(|a| a.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_success() {
        let address = Address::new(
            Some("123 Main St"), // address1
            None::<String>,      // address2
            Some("City"),        // city
            true,                // coordinates_validated
            Some("Country"),     // country
            Some("John"),        // first_name
            Some("Doe"),         // last_name
            Some("Province"),    // province
            Some("12345"),       // zip
            Some("+1234567890"), // phone
        );

        assert!(address.is_ok());

        let address = address.unwrap();
        assert_eq!(address.address1().clone().unwrap(), "123 Main St");
        assert_eq!(address.city().clone().unwrap(), "City");
        assert_eq!(*address.coordinates_validated(), true);
        assert_eq!(address.country().clone().unwrap(), "Country");
        assert_eq!(address.first_name().clone().unwrap(), "John");
        assert_eq!(address.last_name().clone().unwrap(), "Doe");
        assert_eq!(address.province().clone().unwrap(), "Province");
        assert_eq!(address.zip().clone().unwrap(), "12345");
        assert_eq!(address.phone().clone().unwrap(), "+1234567890");
    }
}
