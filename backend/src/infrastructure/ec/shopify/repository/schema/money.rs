use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MoneyBagNode {
    pub shop_money: MoneyNode,
}

#[derive(Debug, Deserialize)]
pub struct MoneyNode {
    pub amount: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
}
