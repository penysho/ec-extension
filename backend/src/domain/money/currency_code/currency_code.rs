use derive_getters::Getters;

use crate::domain::error::error::DomainError;

#[derive(Debug, Getters, Clone, PartialEq)]
pub struct CurrencyCode {
    value: String,
}

impl CurrencyCode {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        Ok(Self {
            value: value.into(),
        })
    }
}
