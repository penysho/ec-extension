use derive_getters::Getters;

use crate::domain::error::error::DomainError;

#[derive(Debug, Getters, Clone, PartialEq)]
pub struct LedgerDocumentUri {
    value: String,
}

impl LedgerDocumentUri {
    pub fn new(value: impl Into<String>) -> Result<LedgerDocumentUri, DomainError> {
        Ok(LedgerDocumentUri {
            value: value.into(),
        })
    }
}
