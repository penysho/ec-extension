use async_trait::async_trait;

use crate::{
    domain::{draft_order::draft_order::DraftOrder, error::error::DomainError},
    usecase::{
        interactor::draft_order_interactor_interface::{DraftOrderInteractor, GetDraftOrdersQuery},
        repository::{
            customer_repository_interface::CustomerRepository,
            draft_order_repository_interface::DraftOrderRepository,
        },
    },
};

/// Draft order Interactor.
pub struct DraftOrderInteractorImpl {
    customer_repository: Box<dyn CustomerRepository>,
    draft_order_repository: Box<dyn DraftOrderRepository>,
}

impl DraftOrderInteractorImpl {
    pub fn new(
        customer_repository: Box<dyn CustomerRepository>,
        draft_order_repository: Box<dyn DraftOrderRepository>,
    ) -> Self {
        Self {
            customer_repository: customer_repository,
            draft_order_repository: draft_order_repository,
        }
    }
}

#[async_trait]
impl DraftOrderInteractor for DraftOrderInteractorImpl {
    async fn get_draft_orders(
        &self,
        query: &GetDraftOrdersQuery,
    ) -> Result<Vec<DraftOrder>, DomainError> {
        match query {
            GetDraftOrdersQuery::Email(email) => {
                let customer = self
                    .customer_repository
                    .find_customer_by_email(&email)
                    .await?;
                self.draft_order_repository
                    .find_draft_orders_by_customer_id(customer.id())
                    .await
            }
        }
    }
}
