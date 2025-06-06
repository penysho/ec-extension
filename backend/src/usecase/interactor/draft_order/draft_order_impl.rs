use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::{
    domain::{
        address::address::Address,
        authorized_resource::authorized_resource::{
            AuthorizedResource, Resource, ResourceAction, ResourceType,
        },
        customer::customer::Id as CustomerId,
        draft_order::draft_order::{DraftOrder, Id as DraftOrderId},
        error::error::DomainError,
        line_item::{discount::discount::Discount, line_item::LineItem},
        money::money::CurrencyCode,
        user::user::UserInterface,
    },
    usecase::{
        auth::authorizer_interface::Authorizer,
        interactor::draft_order_interactor_interface::{DraftOrderInteractor, GetDraftOrdersQuery},
        repository::{
            customer_repository_interface::CustomerRepository,
            draft_order_repository_interface::DraftOrderRepository,
        },
    },
};

/// Draft order Interactor.
pub struct DraftOrderInteractorImpl {
    draft_order_repository: Box<dyn DraftOrderRepository>,
    customer_repository: Box<dyn CustomerRepository>,
    authorizer: Arc<dyn Authorizer>,
}

impl DraftOrderInteractorImpl {
    pub fn new(
        draft_order_repository: Box<dyn DraftOrderRepository>,
        customer_repository: Box<dyn CustomerRepository>,
        authorizer: Arc<dyn Authorizer>,
    ) -> Self {
        Self {
            draft_order_repository,
            customer_repository,
            authorizer,
        }
    }
}

#[async_trait]
impl DraftOrderInteractor for DraftOrderInteractorImpl {
    async fn get_draft_orders(
        &self,
        user: Arc<dyn UserInterface>,
        query: &GetDraftOrdersQuery,
    ) -> Result<Vec<DraftOrder>, DomainError> {
        let draft_orders = match query {
            GetDraftOrdersQuery::Email(email) => {
                let customer = self
                    .customer_repository
                    .find_customer_by_email(&email)
                    .await?;
                self.draft_order_repository
                    .find_draft_orders_by_customer_id(customer.id())
                    .await
            }
        }?;

        self.authorizer
            .authorize(
                user,
                draft_orders
                    .iter()
                    .map(|d| d as &dyn AuthorizedResource)
                    .collect(),
                &ResourceAction::Read,
            )
            .await?;

        Ok(draft_orders)
    }

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
    ) -> Result<DraftOrder, DomainError> {
        self.authorizer
            .authorize(
                user.clone(),
                vec![&Resource::new(ResourceType::DraftOrder, None)],
                &ResourceAction::Write,
            )
            .await?;

        let draft_order = DraftOrder::create(
            user.id(),
            customer_id,
            billing_address,
            shipping_address,
            note,
            line_items,
            reserve_inventory_until,
            tax_exempt,
            presentment_currency_code,
            discount,
        )?;

        self.draft_order_repository.create(draft_order).await
    }

    async fn complete_draft_order(
        &self,
        user: Arc<dyn UserInterface>,
        id: &DraftOrderId,
    ) -> Result<DraftOrder, DomainError> {
        let mut draft_order = self
            .draft_order_repository
            .find_draft_order_by_id(id)
            .await?;

        self.authorizer
            .authorize(user.clone(), vec![&draft_order], &ResourceAction::Write)
            .await?;

        draft_order.complete()?;

        self.draft_order_repository.update(draft_order).await
    }

    async fn delete_draft_order(
        &self,
        user: Arc<dyn UserInterface>,
        id: &DraftOrderId,
    ) -> Result<DraftOrderId, DomainError> {
        let draft_order = self
            .draft_order_repository
            .find_draft_order_by_id(id)
            .await?;

        self.authorizer
            .authorize(user.clone(), vec![&draft_order], &ResourceAction::Delete)
            .await?;

        self.draft_order_repository.delete(draft_order).await
    }
}
