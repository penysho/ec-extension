use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MoneyBagSchema {
    pub(super) currency_code: String,
    pub(super) amount: f64,
}
