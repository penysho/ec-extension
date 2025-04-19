use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::automock;
use std::sync::Arc;

use crate::domain::user::user::UserInterface;
use crate::domain::{
    address::address::Address,
    customer::customer::Id as CustomerId,
    draft_order::draft_order::{DraftOrder, Id as DraftOrderId},
    email::email::Email,
    error::error::DomainError,
    line_item::{discount::discount::Discount, line_item::LineItem},
    money::money::CurrencyCode,
};

#[derive(Debug, Clone, PartialEq)]
pub enum GetDraftOrdersQuery {
    Email(Email),
}

/// Interactor interface for draft orders.
#[automock]
#[async_trait]
pub trait DraftOrderInteractor {
    /// Get draft orders by query.
    ///
    /// # Arguments
    ///
    /// * `user` - The user interface.
    /// * `query` - The query to get draft orders.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<DraftOrder>, DomainError>` - The result of the operation.
    ///   - `Ok(Vec<DraftOrder>)` - The draft orders.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the draft order repository fails.
    async fn get_draft_orders(
        &self,
        user: Arc<dyn UserInterface>,
        query: &GetDraftOrdersQuery,
    ) -> Result<Vec<DraftOrder>, DomainError>;

    /// Create a draft order.
    ///
    /// # Arguments
    ///
    /// * `user` - The user interface.
    /// * `customer_id` - The customer id.
    /// * `billing_address` - The billing address.
    /// * `shipping_address` - The shipping address.
    /// * `note` - The note.
    /// * `line_items` - The line items.
    /// * `reserve_inventory_until` - The reserve inventory until.
    /// * `tax_exempt` - The tax exempt.
    /// * `presentment_currency_code` - Currency code to be applied to the order. If not specified, the store's default currency code is used.
    /// * `discount` - Discount applied per order.
    ///
    /// # Returns
    ///
    /// * `Result<DraftOrder, DomainError>` - The result of the operation.
    ///   - `Ok(DraftOrder)` - The draft order.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the draft order repository fails.
    async fn create_draft_order(
        &self,
        user: Arc<dyn UserInterface>,
        customer_id: Option<CustomerId>,
        billing_address: Option<Address>,
        shipping_address: Option<Address>,
        note: Option<String>,
        line_items: Vec<LineItem>,
        reserve_inventory_until: Option<DateTime<Utc>>,
        tax_exempt: Option<bool>,
        presentment_currency_code: Option<CurrencyCode>,
        discount: Option<Discount>,
    ) -> Result<DraftOrder, DomainError>;

    /// Complete a draft order.
    ///
    /// # Arguments
    ///
    /// * `user` - The user interface.
    /// * `id` - The draft order id.
    ///
    /// # Returns
    ///
    /// * `Result<DraftOrder, DomainError>` - The result of the operation.
    ///   - `Ok(DraftOrder)` - The draft order.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the draft order repository fails.
    /// * If a draft order has already been completed.
    async fn complete_draft_order(
        &self,
        user: Arc<dyn UserInterface>,
        id: &DraftOrderId,
    ) -> Result<DraftOrder, DomainError>;

    /// Delete a draft order.
    ///
    /// # Arguments
    ///
    /// * `user` - The user interface.
    /// * `id` - The draft order id.
    ///
    /// # Returns
    ///
    /// * `Result<DraftOrderId, DomainError>` - The result of the operation.
    ///   - `Ok(DraftOrder)` - ID of deleted draft order.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the draft order repository fails.
    async fn delete_draft_order(
        &self,
        user: Arc<dyn UserInterface>,
        id: &DraftOrderId,
    ) -> Result<DraftOrderId, DomainError>;
}
