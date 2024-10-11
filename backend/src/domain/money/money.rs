use derive_getters::Getters;

use crate::domain::error::error::DomainError;

/// Currency code enum.
#[derive(Debug, Clone, PartialEq)]
pub enum CurrencyCode {
    USD,
    EUR,
    GBP,
    JPY,
}

/// Money value object.
///
/// # Examples
///
/// ```
/// use backend::domain::money::money::{Money,CurrencyCode};
///
/// let money = Money::new(CurrencyCode::USD, 10.0).unwrap();
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Money {
    currency_code: CurrencyCode,
    amount: f64,
}

impl Money {
    pub fn new(currency_code: CurrencyCode, amount: f64) -> Result<Self, DomainError> {
        if amount < 0.0 {
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            currency_code,
            amount,
        })
    }
}
