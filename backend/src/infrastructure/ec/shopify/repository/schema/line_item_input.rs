use serde::Serialize;

use crate::{
    domain::line_item::{
        discount::discount::{Discount, DiscountValueType},
        line_item::LineItem,
    },
    infrastructure::ec::shopify::query_helper::ShopifyGQLQueryHelper,
};

use super::money_input::MoneyInput;

impl From<&LineItem> for LineItemInput {
    fn from(line_item: &LineItem) -> Self {
        LineItemInput {
            variant_id: line_item
                .variant_id()
                .as_ref()
                .map(|id| ShopifyGQLQueryHelper::add_product_variant_gid_prefix(&id)),
            quantity: *line_item.quantity() as i32,
            applied_discount: line_item.discount().as_ref().map(|d| d.to_owned().into()),
        }
    }
}

impl From<Discount> for DiscountInput {
    fn from(discount: Discount) -> Self {
        Self {
            title: discount.title().to_owned(),
            description: discount.description().to_owned(),
            value: *discount.value(),
            value_type: discount.value_type().to_owned().into(),
            amount_with_currency: discount.amount_set().to_owned().into(),
        }
    }
}

impl From<DiscountValueType> for String {
    fn from(value_type: DiscountValueType) -> Self {
        match value_type {
            DiscountValueType::Fixed => "FIXED_AMOUNT".to_string(),
            DiscountValueType::Percentage => "PERCENTAGE".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LineItemInput {
    #[serde(rename = "variantId")]
    pub variant_id: Option<String>,
    pub quantity: i32,
    #[serde(rename = "appliedDiscount")]
    pub applied_discount: Option<DiscountInput>,
}

#[derive(Debug, Serialize)]
pub struct DiscountInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub value: f32,
    #[serde(rename = "valueType")]
    pub value_type: String,
    #[serde(rename = "amountWithCurrency")]
    pub amount_with_currency: MoneyInput,
}
