use serde::Deserialize;

use crate::domain::{
    error::error::DomainError,
    money::{
        money::money::Money,
        money_bag::{CurrencyCode, MoneyBag},
    },
};

impl MoneyBagNode {
    pub fn to_domain(self) -> Result<MoneyBag, DomainError> {
        self.shop_money.to_domain()
    }
}

impl MoneyNode {
    pub fn to_domain(self) -> Result<MoneyBag, DomainError> {
        let amount = Money::new(self.amount.parse::<f64>().unwrap_or(0.0))?;
        let currency_code = self.currency_code.to_domain()?;
        MoneyBag::new(currency_code, amount)
    }
}

#[derive(Debug, Deserialize)]
pub struct MoneyBagNode {
    #[serde(rename = "shopMoney")]
    pub shop_money: MoneyNode,
}

#[derive(Debug, Deserialize)]
pub struct MoneyNode {
    pub amount: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: CurrencyCodeNode,
}

#[derive(Debug, Deserialize)]
pub struct CurrencyCodeNode(String);

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
