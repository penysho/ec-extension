use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneySchema {
    pub currency_code: CurrencyCodeSchema,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CurrencyCodeSchema {
    USD,
    EUR,
    GBP,
    JPY,
}
