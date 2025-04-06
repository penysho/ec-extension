use derive_getters::Getters;
use regex::Regex;

use crate::{
    domain::error::error::DomainError,
    log_error
};

/// Phone value object.
///
/// # Examples
///
/// ```
/// use backend::domain::phone::phone::Phone;
///
/// let phone = Phone::new("1234567890").unwrap();
/// assert_eq!(phone.value(), "1234567890");
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Phone {
    value: String,
}

impl Phone {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value: String = value.into();

        // Regular expression for international phone numbers (up to 15 digits)
        let international_regex = Regex::new(r"^\+?\d{1, 15}$").unwrap();
        // Regular expression for phone numbers in Japan (10 or 11 digits)
        let domestic_regex = Regex::new(r"^\d{10, 11}$").unwrap();

        if international_regex.is_match(&value) || domestic_regex.is_match(&value) {
            return Ok(Self { value });
        }

        log_error!("Invalid phone number: {}", value);
        Err(DomainError::ValidationError)
    }
}
