use crate::domain::money::money_bag::{CurrencyCode, MoneyBag};

use super::schema::{CurrencyCodeSchema, MoneyBagSchema};

impl From<MoneyBag> for MoneyBagSchema {
    fn from(money_bag: MoneyBag) -> Self {
        Self {
            amount: *money_bag.amount().value(),
            currency_code: money_bag.currency_code().to_owned().into(),
        }
    }
}

impl From<CurrencyCode> for CurrencyCodeSchema {
    fn from(currency_code: CurrencyCode) -> Self {
        match currency_code {
            CurrencyCode::USD => Self::USD,
            CurrencyCode::EUR => Self::EUR,
            CurrencyCode::GBP => Self::GBP,
            CurrencyCode::JPY => Self::JPY,
        }
    }
}
