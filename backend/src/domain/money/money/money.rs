use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// Money value object.
///
/// # Example
///
/// ```
/// use backend::domain::money::money::money::Money;
///
/// let money = Money::new(100.0).unwrap();
/// assert_eq!(money.value(), &100.0);
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Money {
    value: f64,
}

impl Money {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if value < 0.0 {
            log::error!("Money value cannot be negative: {}", value);
            return Err(DomainError::ValidationError);
        }

        Ok(Self { value })
    }

    pub fn zero() -> Self {
        Self { value: 0.0 }
    }
}
