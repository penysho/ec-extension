use derive_getters::Getters;

use super::currency_code::currency_code::CurrencyCode;

#[derive(Debug, Getters, Clone, PartialEq)]
pub struct Money {
    currency_code: CurrencyCode,
    amount: f64,
}
