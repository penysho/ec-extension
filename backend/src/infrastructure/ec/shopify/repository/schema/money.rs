use serde::Deserialize;

use crate::domain::{
    error::error::DomainError,
    money::{
        amount::amount::Amount,
        money::{CurrencyCode, Money},
    },
};

impl MoneyBagNode {
    pub fn to_domain(self) -> Result<Money, DomainError> {
        self.shop_money.to_domain()
    }
}

impl MoneyNode {
    pub fn to_domain(self) -> Result<Money, DomainError> {
        let amount = Amount::new(self.amount.parse::<f64>().unwrap_or(0.0))?;
        let currency_code = self.currency_code.to_domain()?;
        Money::new(currency_code, amount)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyBagNode {
    pub shop_money: MoneyNode,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyNode {
    pub amount: String,
    pub currency_code: CurrencyCodeNode,
}

#[derive(Debug, Deserialize)]
pub struct CurrencyCodeNode(pub String);

impl CurrencyCodeNode {
    pub fn to_domain(self) -> Result<CurrencyCode, DomainError> {
        match self.0.as_str() {
            "USD" => Ok(CurrencyCode::USD),
            "EUR" => Ok(CurrencyCode::EUR),
            "GBP" => Ok(CurrencyCode::GBP),
            "JPY" => Ok(CurrencyCode::JPY),
            _ => Err(DomainError::ConversionError),
        }
    }
}
