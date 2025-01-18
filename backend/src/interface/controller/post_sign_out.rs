use actix_web::Responder;

use crate::interface::presenter::{
    auth::auth_impl::AuthPresenterImpl, auth_presenter_interface::AuthPresenter,
};

use super::{controller::Controller, interact_provider_interface::InteractProvider};

impl<I, T, C> Controller<I, T, C>
where
    I: InteractProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Perform back-end sign-out.
    /// Finish session management with cookies.
    pub async fn post_sign_out(&self) -> impl Responder {
        AuthPresenterImpl::new().present_post_sign_out().await
    }
}

#[cfg(test)]
mod tests {
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
        let mut interact_provider = MockInteractProvider::<(), ()>::new();
        interact_provider
            .expect_provide_auth_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn AuthInteractor>);

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
    async fn test_post_sign_out_success() {
        let interactor = MockAuthInteractor::new();

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
