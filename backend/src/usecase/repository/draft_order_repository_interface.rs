use async_trait::async_trait;

use crate::domain::{
    customer::customer::Id as CustomerId, draft_order::draft_order::DraftOrder,
    error::error::DomainError,
};

#[async_trait]
pub trait DraftOrderRepository: Send + Sync {
    async fn get_draft_order_by_customer_id(
        &self,
        customer_id: &CustomerId,
    ) -> Result<DraftOrder, DomainError>;
}
