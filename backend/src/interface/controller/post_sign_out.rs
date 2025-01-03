use actix_web::Responder;

use crate::interface::presenter::{
    auth::auth_impl::AuthPresenterImpl, auth_presenter_interface::AuthPresenter,
};

use super::controller::Controller;

impl Controller {
    /// Perform back-end sign-out.
    /// Finish session management with cookies.
    pub async fn post_sign_out(&self) -> impl Responder {
        AuthPresenterImpl::new().present_post_sign_out().await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::interface::controller::authorizer_interface::MockAuthorizer;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::usecase::interactor::auth_interactor_interface::{
        AuthInteractor, MockAuthInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};

    const BASE_URL: &'static str = "/ec-extension/auth/sign-out";

    async fn setup(
        interactor: MockAuthInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interact_provider = MockInteractProvider::new();
        interact_provider
            .expect_provide_auth_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn AuthInteractor>);

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
    async fn test_post_sign_out_success() {
        let interactor = MockAuthInteractor::new();

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
