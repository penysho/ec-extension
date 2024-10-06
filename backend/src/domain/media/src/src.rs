use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// Media source value object.
///
/// # Examples
///
/// ```
/// use backend::domain::media::src::src::Src;
///
/// let src = Src::new("https://example.com").unwrap();
/// assert_eq!(src.value(), "https://example.com");
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
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
