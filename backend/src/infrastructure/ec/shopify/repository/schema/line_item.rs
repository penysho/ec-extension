use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        line_item::{
            discount::discount::{Discount, DiscountValueType},
            line_item::LineItem,
        },
    },
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::money::MoneyBagNode;

impl LineItemNode {
    pub fn to_domain(self) -> Result<LineItem, DomainError> {
        LineItem::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            self.is_custom,
            self.variant
                .map(|v| ShopifyGQLQueryHelper::remove_gid_prefix(&v.id)),
            self.quantity as u32,
            self.discount.map(|d| d.to_domain()).transpose()?,
            self.discounted_total_set.to_domain()?,
            self.original_total_set.to_domain()?,
        )
    }
}

impl DiscountNode {
    pub fn to_domain(self) -> Result<Discount, DomainError> {
        let value_type = match self.value_type.as_str() {
            "FIXED_AMOUNT" => Ok(DiscountValueType::Fixed),
            "PERCENTAGE" => Ok(DiscountValueType::Percentage),
            _ => Err(DomainError::ConversionError),
        }?;

        Discount::new(
            self.title,
            Some(self.description),
            self.value.parse::<f32>().unwrap_or(0.0),
            value_type,
            self.amount_set.shop_money.to_domain()?,
        )
    }
}

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
