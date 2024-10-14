use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// Email value object.
///
/// # Examples
///
/// ```
/// use backend::domain::email::email::Email;
///
/// let email = Email::new("test@example.com".to_string()).unwrap();
/// assert_eq!(email.value(), "test@example.com");
/// ```
///
/// ```
/// use backend::domain::email::email::Email;
///
/// let email = Email::new("invalid_email".to_string());
/// assert!(email.is_err());
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.contains('@') {
            Ok(Self { value })
        } else {
            log::error!("Invalid email address: {}", value);
            Err(DomainError::ValidationError)
        }
    }
}
