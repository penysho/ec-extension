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

    line_items: Vec<LineItem>,
    line_items_subtotal_price_set: MoneyBag,

    subtotal_price_set: MoneyBag,
    total_price_set: MoneyBag,
    taxes_included: bool,
    tax_exempt: bool,
    total_tax_set: MoneyBag,
    total_discounts_set: MoneyBag,

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
        line_items: Vec<LineItem>,
        subtotal_price_set: MoneyBag,
        line_items_subtotal_price_set: MoneyBag,
        taxes_included: bool,
        tax_exempt: bool,
        total_tax_set: MoneyBag,
        total_discounts_set: MoneyBag,
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
            status: DraftOrderStatus::Open,
            line_items,
            line_items_subtotal_price_set,
            subtotal_price_set,
            total_price_set,
            total_tax_set,
            total_discounts_set,
            taxes_included,
            tax_exempt,
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
