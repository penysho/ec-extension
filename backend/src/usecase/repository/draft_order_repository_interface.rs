use async_trait::async_trait;

use crate::domain::{
    customer::customer::Id as CustomerId,
    draft_order::draft_order::{DraftOrder, Id as DraftOrderId},
    error::error::DomainError,
};

#[async_trait]
pub trait DraftOrderRepository: Send + Sync {
    async fn find_draft_order_by_id(&self, id: &DraftOrderId) -> Result<DraftOrder, DomainError>;

    async fn find_draft_orders_by_customer_id(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Vec<DraftOrder>, DomainError>;

    async fn create(&self, draft_order: DraftOrder) -> Result<DraftOrder, DomainError>;

    async fn update(&self, draft_order: DraftOrder) -> Result<DraftOrder, DomainError>;

    async fn delete(&self, draft_order: DraftOrder) -> Result<DraftOrderId, DomainError>;
}
