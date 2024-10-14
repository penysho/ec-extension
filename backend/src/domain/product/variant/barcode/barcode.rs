use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// Barcode value object.
///
/// # Examples
///
/// ```
/// use backend::domain::product::variant::barcode::barcode::Barcode;
///
/// let barcode = Barcode::new("1234567890").unwrap();
/// assert_eq!(barcode.value(), "1234567890");
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Barcode {
    value: String,
}

impl Barcode {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        Ok(Self {
            value: value.into(),
        })
    }
}
