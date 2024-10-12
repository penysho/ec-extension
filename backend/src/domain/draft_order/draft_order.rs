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

    customer: Option<CustomerId>,
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
        customer: Option<CustomerId>,
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
            customer,
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
