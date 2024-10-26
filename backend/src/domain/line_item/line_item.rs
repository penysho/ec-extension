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
/// - `discounted_total_set` - The total price with discounts applied.
/// - `original_total_set` - The total price excluding discounts, equal to the original unit price multiplied by quantity.
#[derive(Debug, Getters)]
pub struct LineItem {
    id: Id,
    is_custom: bool,
    variant_id: Option<VariantId>,
    quantity: u32,
    discount: Option<Discount>,
    discounted_total_set: MoneyBag,
    original_total_set: MoneyBag,
}

impl LineItem {
    /// Constructor to be used from the repository.
    pub fn new(
        id: Id,
        is_custom: bool,
        variant_id: Option<impl Into<VariantId>>,
        quantity: u32,
        discount: Option<Discount>,
        discounted_total_set: MoneyBag,
        original_total_set: MoneyBag,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id,
            is_custom,
            variant_id: variant_id.map(|id| id.into()),
            quantity,
            discount,
            discounted_total_set,
            original_total_set,
        })
    }

    /// Create an entity in its initial state.
    pub fn create(
        is_custom: bool,
        variant_id: Option<impl Into<VariantId>>,
        quantity: u32,
        discount: Option<Discount>,
    ) -> Result<Self, DomainError> {
        Ok(Self {
            id: String::new(),
            is_custom,
            variant_id: variant_id.map(|id| id.into()),
            quantity,
            discount,
            discounted_total_set: MoneyBag::zero(),
            original_total_set: MoneyBag::zero(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        line_item::discount::discount::DiscountValueType,
        money::{money::money::Money, money_bag::CurrencyCode},
    };

    use super::*;

    fn mock_money_bag() -> MoneyBag {
        let money = Money::new(100.0).unwrap();
        MoneyBag::new(CurrencyCode::USD, money).expect("Failed to create mock money bag")
    }

    fn mock_discount() -> Discount {
        Discount::new(
            Some("Test Discount".to_string()),
            Some("Test description".to_string()),
            10.0,
            DiscountValueType::Percentage,
            Some(mock_money_bag()),
        )
        .expect("Failed to create mock discount")
    }

    #[test]
    fn test_new() {
        let line_item = LineItem::new(
            "valid_id".into(),
            false,
            Some("variant_id"),
            5,
            Some(mock_discount()),
            mock_money_bag(),
            mock_money_bag(),
        )
        .expect("Failed to create mock line item");

        assert_eq!(line_item.id().to_string(), "valid_id");
        assert_eq!(line_item.quantity(), &5);
        assert_eq!(line_item.is_custom(), &false);
        assert!(line_item.variant_id().is_some());
        assert!(line_item.discount().is_some());
    }

    #[test]
    fn test_new_without_variant() {
        let line_item = LineItem::new(
            "valid_id".into(),
            true,
            None::<String>,
            1,
            None,
            mock_money_bag(),
            mock_money_bag(),
        )
        .expect("Failed to create mock line item");

        assert_eq!(line_item.variant_id(), &None);
        assert_eq!(line_item.discount(), &None);
    }
}
