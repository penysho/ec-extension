use derive_getters::Getters;

use crate::domain::error::error::DomainError;

#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Sku {
    value: String,
}

impl Sku {
    pub fn new(value: impl Into<String>) -> Result<Sku, DomainError> {
        Ok(Sku {
            value: value.into(),
        })
    }
}
