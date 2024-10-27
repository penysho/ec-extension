use serde::Serialize;

use crate::domain::money::money::{CurrencyCode, Money};

impl From<Money> for MoneyInput {
    fn from(money_bag: Money) -> Self {
        MoneyInput {
            amount: money_bag.amount().value().to_string(),
            currency_code: money_bag.currency_code().to_owned().into(),
        }
    }
}

impl From<CurrencyCode> for String {
    fn from(currency_code: CurrencyCode) -> Self {
        match currency_code {
            CurrencyCode::USD => "USD".to_string(),
            CurrencyCode::EUR => "EUR".to_string(),
            CurrencyCode::GBP => "GBP".to_string(),
            CurrencyCode::JPY => "JPY".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyInput {
    pub amount: String,
    pub currency_code: String,
}
