use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::{
    address::address::Address,
    customer::customer::Id as CustomerId,
    error::error::DomainError,
    line_item::line_item::LineItem,
    money::money_bag::{CurrencyCode, MoneyBag},
    order::order::Id as OrderId,
};

pub type Id = String;

/// Represents the status of a draft order.
///
/// # Variants
/// - `Open` - The draft order is open and can be edited.
/// - `Completed` - The draft order has been completed and cannot be edited anymore.
/// - `Canceled` - The draft order has been canceled and cannot be edited anymore.
#[derive(Debug, Clone, PartialEq)]
pub enum DraftOrderStatus {
    Open,
    Completed,
    Canceled,
}

/// Representing Draft Orders.
///
/// Unlike regular orders, they serve as Admin Orders, which are created from the application or admin screen.
///
/// # Fields
/// - `id` - A unique identifier for the draft order.
/// - `name` - The name of the draft order.
/// - `status` - The current status of the draft order.
/// - `customer_id` - An optional identifier for the associated customer.
/// - `billing_address` - An optional billing address for the draft order.
/// - `shipping_address` - An optional shipping address for the draft order.
/// - `note` - An optional note or memo related to the order.
/// - `line_items` - The list of products or services associated with the order.
/// - `reserve_inventory_until` - The date and time after which the reserved inventory will be automatically restocked if the order remains incomplete.
/// - `subtotal_price_set` - The subtotal price of all line items and applied discounts, excluding shipping and taxes.
/// - `taxes_included` - A flag indicating whether taxes are included in the item prices.
/// - `tax_exempt` - A flag indicating whether the order is exempt from taxes.
/// - `total_tax_set` - The total tax amount for the draft order.
/// - `total_discounts_set` - The total amount of discounts applied to the order.
/// - `total_shipping_price_set` - The total cost of shipping for the order.
/// - `total_price_set` - The final total price of the order, including shipping, discounts, and taxes.
/// - `presentment_currency_code` - Currency code used for the order. May differ from the store's default currency code.
/// - `order_id` - An optional identifier for the associated order, if the draft was converted to a finalized order.
/// - `completed_at` - An optional timestamp indicating when the order was completed.
/// - `created_at` - The timestamp when the draft order was initially created.
/// - `update_at` - The timestamp when the draft order was last updated.
#[derive(Debug, Getters)]
pub struct DraftOrder {
    id: Id,
    name: String,
    status: DraftOrderStatus,

    customer_id: Option<CustomerId>,
    billing_address: Option<Address>,
    shipping_address: Option<Address>,
    note: Option<String>,

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
    /// Currency code used for the order.
    /// May differ from the store's default currency code.
    presentment_currency_code: CurrencyCode,

    order_id: Option<OrderId>,

    completed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl DraftOrder {
    /// Constructor to be used from the repository.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        status: DraftOrderStatus,
        customer_id: Option<CustomerId>,
        billing_address: Option<Address>,
        shipping_address: Option<Address>,
        note: Option<String>,
        line_items: Vec<LineItem>,
        reserve_inventory_until: Option<DateTime<Utc>>,
        subtotal_price_set: MoneyBag,
        taxes_included: bool,
        tax_exempt: bool,
        total_tax_set: MoneyBag,
        total_discounts_set: MoneyBag,
        total_shipping_price_set: MoneyBag,
        total_price_set: MoneyBag,
        presentment_currency_code: CurrencyCode,
        order_id: Option<OrderId>,
        completed_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let instance = Self {
            id: id.into(),
            name: name.into(),
            status,
            customer_id,
            billing_address,
            shipping_address,
            note,
            line_items,
            reserve_inventory_until,
            subtotal_price_set,
            taxes_included,
            tax_exempt,
            total_tax_set,
            total_discounts_set,
            total_shipping_price_set,
            total_price_set,
            presentment_currency_code,
            order_id,
            completed_at,
            created_at,
            updated_at,
        };

        instance.validate()?;
        Ok(instance)
    }

    fn validate(&self) -> Result<(), DomainError> {
        if self.id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        if self.name.is_empty() {
            log::error!("Name cannot be empty");
            return Err(DomainError::ValidationError);
        }
        Ok(())
    }

    /// Create an entity in its initial state.
    pub fn create(
        customer_id: Option<CustomerId>,
        billing_address: Option<Address>,
        shipping_address: Option<Address>,
        note: Option<impl Into<String>>,
        line_items: Vec<LineItem>,
        reserve_inventory_until: Option<DateTime<Utc>>,
        tax_exempt: Option<bool>,
        presentment_currency_code: Option<CurrencyCode>,
    ) -> Result<Self, DomainError> {
        let now = Utc::now();
        let tax_exempt = tax_exempt.unwrap_or(false);
        let presentment_currency_code = match presentment_currency_code {
            Some(presentment_currency_code) => presentment_currency_code,
            None => CurrencyCode::default(),
        };

        Ok(Self {
            id: String::new(),
            name: String::new(),
            status: DraftOrderStatus::Open,
            customer_id,
            billing_address,
            shipping_address,
            note: note.map(|n| n.into()),
            line_items,
            reserve_inventory_until,
            subtotal_price_set: MoneyBag::zero(),
            taxes_included: false,
            tax_exempt,
            total_tax_set: MoneyBag::zero(),
            total_discounts_set: MoneyBag::zero(),
            total_shipping_price_set: MoneyBag::zero(),
            total_price_set: MoneyBag::zero(),
            presentment_currency_code,
            order_id: None,
            completed_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn complete(&mut self) -> Result<(), DomainError> {
        if self.status == DraftOrderStatus::Completed {
            log::error!("Draft order is already completed");
            return Err(DomainError::ValidationError);
        }
        self.status = DraftOrderStatus::Completed;

        let default_date = DateTime::<Utc>::default();
        self.completed_at = Some(default_date);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        address::address::Address,
        line_item::discount::discount::{Discount, DiscountValueType},
        money::{amount::amount::Amount, money_bag::CurrencyCode},
    };

    use super::*;
    use chrono::Utc;

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

    fn mock_money_bag() -> MoneyBag {
        let amount = Amount::new(100.0).unwrap();
        MoneyBag::new(CurrencyCode::USD, amount).expect("Failed to create mock money bag")
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
                .expect("Failed to create mock line item")
            })
            .collect()
    }

    fn mock_address() -> Option<Address> {
        Some(
            Address::new(
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
            .expect("Failed to create mock address"),
        )
    }

    /// Helper to create a valid `DraftOrder` for testing.
    fn mock_draft_order() -> DraftOrder {
        DraftOrder::new(
            "0",
            "Test Order",
            DraftOrderStatus::Open,
            None,
            mock_address(),
            mock_address(),
            None,
            mock_line_items(2),
            None,
            mock_money_bag(),
            true,
            false,
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            CurrencyCode::default(),
            None,
            None,
            Utc::now(),
            Utc::now(),
        )
        .expect("Failed to create mock draft order")
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
            None,
            mock_address(),
            mock_address(),
            None,
            mock_line_items(1),
            None,
            mock_money_bag(),
            true,
            false,
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            CurrencyCode::default(),
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
            None,
            mock_address(),
            mock_address(),
            None,
            mock_line_items(1),
            None,
            mock_money_bag(),
            true,
            false,
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            mock_money_bag(),
            CurrencyCode::default(),
            None,
            None,
            Utc::now(),
            Utc::now(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create() {
        let draft_order = DraftOrder::create(
            None,
            mock_address(),
            mock_address(),
            Some("note"),
            mock_line_items(2),
            None,
            Some(false),
            None,
        )
        .expect("Failed to create draft order");

        assert_eq!(draft_order.id(), "");
        assert_eq!(draft_order.name(), "");
    }

    #[test]
    fn test_complete() {
        let mut draft_order = mock_draft_order();
        draft_order
            .complete()
            .expect("Failed to complete draft order");

        assert_eq!(draft_order.status(), &DraftOrderStatus::Completed);
    }
}
