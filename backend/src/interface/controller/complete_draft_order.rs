use actix_web::{web::Path, Responder};

use crate::interface::presenter::{
    draft_order::draft_order_impl::DraftOrderPresenterImpl,
    draft_order_presenter_interface::DraftOrderPresenter,
};

use super::controller::Controller;

impl Controller {
    /// Complete a draft order.
    pub async fn complete_draft_order(&self, path: Path<(String,)>) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        let interactor = self
            .interact_provider
            .provide_draft_order_interactor()
            .await;

        let result = interactor.complete_draft_order(&path.into_inner().0).await;

        presenter.present_complete_draft_order(result).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::error::error::DomainError;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::authorizer_interface::MockAuthorizer;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::interface::mock::domain_mock::mock_draft_orders;
    use crate::usecase::interactor::draft_order_interactor_interface::DraftOrderInteractor;
    use crate::usecase::interactor::draft_order_interactor_interface::MockDraftOrderInteractor;

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};

    const BASE_URL: &'static str = "/ec-extension/orders/draft/complete";

    async fn setup(
        interactor: MockDraftOrderInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interact_provider = MockInteractProvider::new();
        interact_provider
            .expect_provide_draft_order_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn DraftOrderInteractor>);

        let authorizer = MockAuthorizer::new();

        let controller = web::Data::new(Arc::new(Controller::new(
            Box::new(interact_provider),
            Box::new(authorizer),
        )));

        // Create an application for testing
        test::init_service(
            App::new()
                .app_data(controller)
                .configure(actix_router::configure_routes),
        )
        .await
    }

    #[actix_web::test]
    async fn test_complete_draft_order_success() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_complete_draft_order()
            .returning(|_| Ok(mock_draft_orders(1).remove(0)));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_complete_draft_order_bad_request() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_complete_draft_order()
            .returning(|_| Err(DomainError::ValidationError));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_complete_draft_order_service_unavailable() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_complete_draft_order()
            .returning(|_| Err(DomainError::SystemError));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
