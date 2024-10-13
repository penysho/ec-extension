use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{
    domain::{email::email::Email, error::error::DomainError},
    interface::presenter::{
        draft_order::draft_order_impl::DraftOrderPresenterImpl,
        draft_order_presenter_interface::DraftOrderPresenter,
    },
    usecase::interactor::draft_order_interactor_interface::GetDraftOrdersQuery,
};

use super::controller::Controller;

#[derive(Deserialize)]
pub struct GetDraftOrdersQueryParams {
    email: Option<String>,
}

impl Controller {
    /// Get a list of draft orders.
    pub async fn get_draft_orders(
        &self,
        params: web::Query<GetDraftOrdersQueryParams>,
    ) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        let query = match validate_query_params(&params) {
            Ok(query) => query,
            Err(error) => return presenter.present_get_draft_orders(Err(error)).await,
        };

        let interactor = self
            .interact_provider
            .provide_draft_order_interactor()
            .await;
        let results = interactor.get_draft_orders(&query).await;

        presenter.present_get_draft_orders(results).await
    }
}

fn validate_query_params(
    params: &GetDraftOrdersQueryParams,
) -> Result<GetDraftOrdersQuery, DomainError> {
    if let Some(email) = params.email.clone() {
        if !email.is_empty() {
            return Ok(GetDraftOrdersQuery::Email(Email::new(email)?));
        }
    }

    Err(DomainError::InvalidRequest)
}
