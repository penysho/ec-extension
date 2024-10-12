use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError, money::money_bag::MoneyBag,
    product::variant::variant::Id as VariantId,
};

use super::discount::discount::Discount;

pub type Id = String;

/// Represents a line item in an order.
///
/// A line item can either be a custom product or a variant of a product.
/// If it's a custom product, the `variant_id` field will be `None`.
/// If it's a variant, the `is_custom` field will be `false`.
///
/// # Fields
/// - `id` - The unique identifier for the line item.
/// - `is_custom` - Whether the line item is a custom product or not.
/// - `variant_id` - The identifier of the variant of the product (if applicable).
/// - `quantity` - The quantity of the line item.
/// - `discount` - The discount applied to the line item (if any).
/// - `discounted_amount_set` - The discounted amount for each currency.
/// - `original_total_amount_set` - The original total amount for each currency.
#[derive(Debug, Getters)]
pub struct LineItem {
    id: Id,
    is_custom: bool,
    variant_id: Option<VariantId>,
    quantity: u32,
    discount: Option<Discount>,
    discounted_amount_set: Option<MoneyBag>,
    original_total_amount_set: Option<MoneyBag>,
}

impl LineItem {
    pub fn new(
        id: Id,
        is_custom: bool,
        variant_id: Option<impl Into<VariantId>>,
        quantity: u32,
        discount: Option<Discount>,
        discounted_amount_set: Option<MoneyBag>,
        original_total_amount_set: Option<MoneyBag>,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id,
            is_custom,
            variant_id: variant_id.map(|id| id.into()),
            quantity,
            discount,
            discounted_amount_set,
            original_total_amount_set,
        })
    }
}
