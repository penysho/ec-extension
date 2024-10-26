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
            self.custom,
            self.variant
                .map(|v| ShopifyGQLQueryHelper::remove_gid_prefix(&v.id)),
            self.quantity as u32,
            self.applied_discount.map(|d| d.to_domain()).transpose()?,
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
            self.value,
            value_type,
            Some(self.amount_set.shop_money.to_domain()?),
        )
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineItemNode {
    pub id: String,
    pub custom: bool,
    pub variant: Option<VariantIdNode>,
    pub quantity: i32,
    pub applied_discount: Option<DiscountNode>,
    pub discounted_total_set: MoneyBagNode,
    pub original_total_set: MoneyBagNode,
}

#[derive(Debug, Deserialize)]
pub struct VariantIdNode {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscountNode {
    pub title: Option<String>,
    pub description: String,
    pub value: f32,
    pub value_type: String,
    pub amount_set: MoneyBagNode,
}
