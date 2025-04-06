use derive_getters::Getters;

use crate::{domain::error::error::DomainError, log_error};

/// Amount value object.
///
/// # Example
///
/// ```
/// use backend::domain::money::amount::amount::Amount;
///
/// let amount = Amount::new(100.0).unwrap();
/// assert_eq!(amount.value(), &100.0);
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Amount {
    value: f64,
}

impl Amount {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value < 0.0 {
            log_error!("Money value cannot be negative."; "value" => value);
            return Err(DomainError::ValidationError);
        }

        Ok(Self { value })
    }

    pub fn zero() -> Self {
        Self { value: 0.0 }
    }
}
