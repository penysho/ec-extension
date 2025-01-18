use actix_web::{web::Path, Responder};

use crate::interface::presenter::{
    draft_order::draft_order_impl::DraftOrderPresenterImpl,
    draft_order_presenter_interface::DraftOrderPresenter,
};

use super::{controller::Controller, interact_provider_interface::InteractProvider};

impl<I, T, C> Controller<I, T, C>
where
    I: InteractProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Delete a draft order.
    pub async fn delete_draft_order(&self, path: Path<(String,)>) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        let interactor = self
            .interact_provider
            .provide_draft_order_interactor()
            .await;

        let result = interactor.delete_draft_order(&path.into_inner().0).await;

        presenter.present_delete_draft_order(result).await
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::error::error::DomainError;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::usecase::interactor::draft_order_interactor_interface::DraftOrderInteractor;
    use crate::usecase::interactor::draft_order_interactor_interface::MockDraftOrderInteractor;

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};
    use mockall::predicate::eq;

    const BASE_URL: &'static str = "/ec-extension/orders/draft";

    async fn setup(
        interactor: MockDraftOrderInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interact_provider = MockInteractProvider::<(), ()>::new();
        interact_provider
            .expect_provide_draft_order_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn DraftOrderInteractor>);

        let controller = web::Data::new(Controller::new(interact_provider));

        // Create an application for testing
        test::init_service(
            App::new()
                .app_data(controller)
                .configure(actix_router::configure_routes::<MockInteractProvider<(), ()>, (), ()>),
        )
        .await
    }

    #[actix_web::test]
    async fn test_delete_draft_order_success() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_delete_draft_order()
            .with(eq(format!("1")))
            .returning(|_| Ok(format!("1")));

        let req = test::TestRequest::delete()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_delete_draft_order_bad_request() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_delete_draft_order()
            .returning(|_| Err(DomainError::ValidationError));

        let req = test::TestRequest::delete()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_delete_draft_order_service_unavailable() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_delete_draft_order()
            .returning(|_| Err(DomainError::SystemError));

        let req = test::TestRequest::delete()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
