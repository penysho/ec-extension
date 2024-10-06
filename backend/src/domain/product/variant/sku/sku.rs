use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// SKU value object.
///
/// # Examples
///
/// ```
/// use backend::domain::product::variant::sku::sku::Sku;
///
/// let sku = Sku::new("SKU123").unwrap();
/// assert_eq!(sku.value(), "SKU123");
/// ```
///
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
