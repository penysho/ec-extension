use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::amount::amount::Amount;

/// Currency code enum.
#[derive(Debug, Clone, PartialEq)]
pub enum CurrencyCode {
    USD,
    EUR,
    GBP,
    JPY,
}

impl Default for CurrencyCode {
    fn default() -> Self {
        CurrencyCode::JPY
    }
}

/// Money value object.
///
/// # Examples
///
/// ```
/// use backend::domain::money::money::Money;
/// use backend::domain::money::money::CurrencyCode;
/// use backend::domain::money::amount::amount::Amount;
///
/// let amount = Amount::new(100.0).unwrap();
/// let money_bag = Money::new(CurrencyCode::USD, amount).unwrap();
/// assert_eq!(money_bag.amount().value(), &100.0);
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Money {
    currency_code: CurrencyCode,
    amount: Amount,
}

impl Money {
    pub fn new(currency_code: CurrencyCode, amount: Amount) -> Result<Self, DomainError> {
        Ok(Self {
            currency_code,
            amount,
        })
    }

    pub fn zero() -> Self {
        Self {
            currency_code: CurrencyCode::default(),
            amount: Amount::zero(),
        }
    }
}
