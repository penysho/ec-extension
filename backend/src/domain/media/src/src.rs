use derive_getters::Getters;

use crate::domain::error::error::DomainError;

#[derive(Debug, Getters, Clone)]
pub struct Src {
    value: String,
}

impl Src {
    pub fn new(value: impl Into<String>) -> Result<Src, DomainError> {
        Ok(Src {
            value: value.into(),
        })
    }
}
