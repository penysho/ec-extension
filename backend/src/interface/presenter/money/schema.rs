use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneySchema {
    pub currency_code: CustomerStatusEnum,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CustomerStatusEnum {
    USD,
    EUR,
    GBP,
    JPY,
}
