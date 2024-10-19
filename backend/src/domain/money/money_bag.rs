use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::money::money::Money;

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

/// MoneyBag value object.
///
/// # Examples
///
/// ```
/// use backend::domain::money::money_bag::MoneyBag;
/// use backend::domain::money::money_bag::CurrencyCode;
/// use backend::domain::money::money::money::Money;
///
/// let money = Money::new(100.0).unwrap();
/// let money_bag = MoneyBag::new(CurrencyCode::USD, money).unwrap();
/// assert_eq!(money_bag.amount().value(), &100.0);
/// ```
///
#[derive(Debug, Getters, Clone, PartialEq)]
pub struct MoneyBag {
    currency_code: CurrencyCode,
    amount: Money,
}

impl MoneyBag {
    pub fn new(currency_code: CurrencyCode, amount: Money) -> Result<Self, DomainError> {
        Ok(Self {
            currency_code,
            amount,
        })
    }

    pub fn zero() -> Self {
        Self {
            currency_code: CurrencyCode::default(),
            amount: Money::zero(),
        }
    }
}
