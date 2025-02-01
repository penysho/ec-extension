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

use super::{controller::Controller, interactor_provider_interface::InteractorProvider};

#[derive(Deserialize)]
pub struct GetDraftOrdersQueryParams {
    email: Option<String>,
}

impl<I, T, C> Controller<I, T, C>
where
    I: InteractorProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Get a list of draft orders.
    pub async fn get_draft_orders(
        &self,
        request: actix_web::HttpRequest,
        params: web::Query<GetDraftOrdersQueryParams>,
    ) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        let query = match validate_query_params(&params) {
            Ok(query) => query,
            Err(error) => return presenter.present_get_draft_orders(Err(error)).await,
        };
        let user = self.get_user(&request)?;
        let transaction_manager = self.get_transaction_manager(&request)?;

        let interactor = self
            .interactor_provider
            .provide_draft_order_interactor(transaction_manager)
            .await;
        let results = interactor.get_draft_orders(user, &query).await;

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::error::error::DomainError;
    use crate::infrastructure::auth::idp_user::IdpUser;
    use crate::infrastructure::db::sea_orm::sea_orm_manager::SeaOrmTransactionManager;
    use crate::infrastructure::db::transaction_manager_interface::TransactionManager;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interactor_provider_interface::MockInteractorProvider;
    use crate::interface::mock::domain_mock::mock_draft_orders;
    use crate::usecase::interactor::draft_order_interactor_interface::DraftOrderInteractor;
    use crate::usecase::interactor::draft_order_interactor_interface::MockDraftOrderInteractor;
    use crate::usecase::user::UserInterface;

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::HttpMessage;
    use actix_web::{http::StatusCode, test, App, Error};
    use mockall::predicate::always;
    use mockall::predicate::eq;
    use sea_orm::DatabaseConnection;
    use sea_orm::DatabaseTransaction;

    const BASE_URL: &'static str = "/ec-extension/orders/draft";

    async fn setup(
        interactor: MockDraftOrderInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interactor_provider =
            MockInteractorProvider::<DatabaseTransaction, Arc<DatabaseConnection>>::new();
        interactor_provider
            .expect_provide_draft_order_interactor()
            .return_once(move |_| Box::new(interactor) as Box<dyn DraftOrderInteractor>);

        let controller = web::Data::new(Controller::new(interactor_provider));

        // Create an application for testing
        test::init_service(App::new().app_data(controller).configure(
            actix_router::configure_routes::<
                MockInteractorProvider<DatabaseTransaction, Arc<DatabaseConnection>>,
                DatabaseTransaction,
                Arc<DatabaseConnection>,
            >,
        ))
        .await
    }

    fn add_extensions(req: &Request) {
        req.extensions_mut()
            .insert(Arc::new(IdpUser::default()) as Arc<dyn UserInterface>);
        req.extensions_mut()
            .insert(Arc::new(SeaOrmTransactionManager::default())
                as Arc<
                    dyn TransactionManager<DatabaseTransaction, Arc<DatabaseConnection>>,
                >);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_success() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_get_draft_orders()
            .with(
                always(),
                eq(GetDraftOrdersQuery::Email(
                    Email::new("john@example.com").expect("Failed to create email"),
                )),
            )
            .returning(|_, _| Ok(mock_draft_orders(10)));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_not_specified_email() {
        let interactor = MockDraftOrderInteractor::new();

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email="))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_not_found() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_get_draft_orders()
            .returning(|_, _| Ok(vec![]));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_bad_request() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_get_draft_orders()
            .returning(|_, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_draft_orders_service_unavailable() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_get_draft_orders()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
