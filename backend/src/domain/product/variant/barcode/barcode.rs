use derive_getters::Getters;

use crate::domain::error::error::DomainError;

#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Barcode {
    value: String,
}

impl Barcode {
    pub fn new(value: impl Into<String>) -> Result<Barcode, DomainError> {
        Ok(Barcode {
            value: value.into(),
        })
    }
}
