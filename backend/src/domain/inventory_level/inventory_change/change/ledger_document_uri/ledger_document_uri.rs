use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// A freeform URI that represents what changed the inventory quantities.
///
/// # Examples
///
/// ```
/// use backend::domain::inventory_level::inventory_change::change::ledger_document_uri::ledger_document_uri::LedgerDocumentUri;
///
/// let uri = LedgerDocumentUri::new("https://example.com/file.pdf").unwrap();
/// assert_eq!(uri.value(), "https://example.com/file.pdf");
/// ```
///
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
