use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::{
    address::address::Address, customer::customer::Id as CustomerId, error::error::DomainError,
    line_item::line_item::LineItem, money::money_bag::MoneyBag, order::order::Id as OrderId,
};

pub type Id = String;

#[derive(Debug, Clone, PartialEq)]
pub enum DraftOrderStatus {
    Open,
    Completed,
    Canceled,
}

#[derive(Debug, Getters)]
pub struct DraftOrder {
    id: Id,
    name: String,
    status: DraftOrderStatus,

    /// The list of the line items in the draft order.
    line_items: Vec<LineItem>,
    /// The time after which inventory will automatically be restocked.
    reserve_inventory_until: Option<DateTime<Utc>>,

    /// The subtotal, of the line items and their discounts, excluding shipping charges, shipping discounts, and taxes.
    subtotal_price_set: MoneyBag,
    /// Whether the line item prices include taxes.
    taxes_included: bool,
    /// Whether the draft order is tax exempt.
    tax_exempt: bool,
    /// The total tax.
    total_tax_set: MoneyBag,
    /// Total discounts.
    total_discounts_set: MoneyBag,
    /// The total shipping price.
    total_shipping_price_set: MoneyBag,
    /// The total price, includes taxes, shipping charges, and discounts.
    total_price_set: MoneyBag,

    customer_id: Option<CustomerId>,
    billing_address: Address,
    shipping_address: Address,
    note: Option<String>,

    order_id: Option<OrderId>,

    completed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    update_at: DateTime<Utc>,
}

impl DraftOrder {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        status: DraftOrderStatus,
        line_items: Vec<LineItem>,
        reserve_inventory_until: Option<DateTime<Utc>>,
        subtotal_price_set: MoneyBag,
        taxes_included: bool,
        tax_exempt: bool,
        total_tax_set: MoneyBag,
        total_discounts_set: MoneyBag,
        total_shipping_price_set: MoneyBag,
        total_price_set: MoneyBag,
        customer_id: Option<CustomerId>,
        billing_address: Address,
        shipping_address: Address,
        note: Option<String>,
        order_id: Option<OrderId>,
        completed_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        update_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        let name = name.into();
        if name.is_empty() {
            log::error!("Name cannot be empty");
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            id,
            name,
            status,
            line_items,
            reserve_inventory_until,
            subtotal_price_set,
            taxes_included,
            tax_exempt,
            total_tax_set,
            total_discounts_set,
            total_shipping_price_set,
            total_price_set,
            customer_id,
            billing_address,
            shipping_address,
            note,
            order_id,
            completed_at,
            created_at,
            update_at,
        })
    }

    pub fn complete(&mut self) {
        self.status = DraftOrderStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    pub fn cancel(&mut self) {
        self.status = DraftOrderStatus::Canceled;
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        address::address::Address,
        line_item::discount::discount::{Discount, DiscountValueType},
        money::{money::money::Money, money_bag::CurrencyCode},
    };

    use super::*;
    use chrono::Utc;

    fn mock_discount() -> Discount {
        Discount::new(
            Some("Test Discount".to_string()),
            Some("Test description".to_string()),
            10.0,
            DiscountValueType::Percentage,
            mock_money_bag(),
        )
        .expect("Failed to create mock discount")
    }

    fn mock_money_bag() -> MoneyBag {
        let money = Money::new(100.0).unwrap();
        MoneyBag::new(CurrencyCode::USD, money).expect("Failed to create mock money bag")
    }

    fn mock_line_items(count: usize) -> Vec<LineItem> {
        (0..count)
            .map(|i| {
                LineItem::new(
                    format!("{i}"),
                    false,
                    Some("variant_id"),
                    5,
                    Some(mock_discount()),
                    mock_money_bag(),
                    mock_money_bag(),
                )
                .expect("Failed to create valid line item")
            })
            .collect()
    }

    fn mock_address() -> Address {
        fn count(x: &str) -> usize {
            x.len()
        }
        Address::new(
            "123",
            Some("123 Main St"),
            None::<String>,
            Some("City"),
            true,
            Some("Country"),
            Some("John"),
            Some("Doe"),
            Some("Province"),
            Some("12345"),
            Some("+1234567890"),
        )
        .expect("Failed to create mock address")
    }

    /// Helper to create a valid `DraftOrder` for testing.
    fn mock_draft_order() -> DraftOrder {
        DraftOrder::new(
            "valid_id",
            "Test Order",
            DraftOrderStatus::Open,
            mock_line_items(2),
            None,
            mock_money_bag(),
            true,
            false,
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            None,
            mock_address(),
            mock_address(),
            None,
            None,
            None,
            Utc::now(),
            Utc::now(),
        )
        .expect("Failed to create valid draft order")
    }

    #[test]
    fn test_new() {
        let draft_order = mock_draft_order();
        assert_eq!(draft_order.name(), "Test Order");
        assert_eq!(draft_order.status(), &DraftOrderStatus::Open);
    }

    #[test]
    fn test_new_with_empty_id_should_fail() {
        let result = DraftOrder::new(
            "",
            "Test Order",
            DraftOrderStatus::Open,
            mock_line_items(1),
            None,
            mock_money_bag(),
            true,
            false,
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            None,
            mock_address(),
            mock_address(),
            None,
            None,
            None,
            Utc::now(),
            Utc::now(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_empty_name_should_fail() {
        let result = DraftOrder::new(
            "valid_id",
            "",
            DraftOrderStatus::Open,
            mock_line_items(1),
            None,
            mock_money_bag(),
            true,
            false,
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            None,
            mock_address(),
            mock_address(),
            None,
            None,
            None,
            Utc::now(),
            Utc::now(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_complete() {
        let mut draft_order = mock_draft_order();
        assert_eq!(draft_order.status(), &DraftOrderStatus::Open);

        draft_order.complete();
        assert_eq!(draft_order.status(), &DraftOrderStatus::Completed);
        assert!(draft_order.completed_at().is_some());
    }

    #[test]
    fn test_cancel() {
        let mut draft_order = mock_draft_order();
        assert_eq!(draft_order.status(), &DraftOrderStatus::Open);

        draft_order.cancel();
        assert_eq!(draft_order.status(), &DraftOrderStatus::Canceled);
    }
}
