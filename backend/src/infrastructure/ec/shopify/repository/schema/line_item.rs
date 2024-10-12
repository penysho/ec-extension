use serde::Deserialize;

use super::money::MoneyBagNode;

#[derive(Debug, Deserialize)]
pub struct LineItemNode {
    pub id: String,
    #[serde(rename = "isCustom")]
    pub is_custom: bool,
    pub variant: Option<VariantIdNode>,
    pub quantity: i32,
    pub discount: Option<DiscountNode>,
    #[serde(rename = "discountedTotalSet")]
    pub discounted_total_set: MoneyBagNode,
    #[serde(rename = "originalTotalSet")]
    pub original_total_set: MoneyBagNode,
}

#[derive(Debug, Deserialize)]
pub struct VariantIdNode {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscountNode {
    pub title: Option<String>,
    pub description: String,
    pub value: String,
    #[serde(rename = "valueType")]
    pub value_type: String,
    #[serde(rename = "amountSet")]
    pub amount_set: MoneyBagNode,
}
