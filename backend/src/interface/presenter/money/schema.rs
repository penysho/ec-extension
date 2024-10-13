use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyBagSchema {
    pub(super) currency_code: CurrencyCodeSchema,
    pub(super) amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CurrencyCodeSchema {
    USD,
    EUR,
    GBP,
    JPY,
}
