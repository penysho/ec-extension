use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

use crate::interface::presenter::{
    auth::auth_impl::AuthPresenterImpl, auth_presenter_interface::AuthPresenter,
};

use super::controller::Controller;

#[derive(Serialize, Deserialize)]
pub struct PostSignInRequest {
    id_token: Option<String>,
    refresh_token: Option<String>,
}

impl Controller {
    /// Perform back-end sign-in.
    /// Initiate session management with cookies.
    pub async fn post_sign_in(&self, body: web::Json<PostSignInRequest>) -> impl Responder {
        let body = body.into_inner();
        let id_token = body.id_token;
        let refresh_token = body.refresh_token;

        let interactor = self.interact_provider.provide_auth_interactor().await;
        let result = interactor.authenticate(&id_token, &refresh_token).await;

        AuthPresenterImpl::new()
            .present_post_sign_in(result, refresh_token.as_deref())
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::error::error::DomainError;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::interface::mock::domain_mock::mock_customers;
    use crate::usecase::interactor::auth_interactor_interface::{
        AuthInteractor, MockAuthInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};

    const BASE_URL: &'static str = "/ec-extension/auth/sign-in";

    async fn setup(
        interactor: MockAuthInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interact_provider = MockInteractProvider::new();
        interact_provider
            .expect_provide_auth_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn AuthInteractor>);

        let controller = web::Data::new(Controller::new(Box::new(interact_provider)));

        // Create an application for testing
        test::init_service(
            App::new()
                .app_data(controller)
                .configure(actix_router::configure_routes),
        )
        .await
    }

    #[actix_web::test]
    async fn test_post_sign_in_success() {
        let mut interactor = MockAuthInteractor::new();
        interactor
            .expect_authenticate()
            .returning(|_, _| Ok((mock_customers(1).remove(0), "updated-idtoken".to_string())));

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .set_json(PostSignInRequest {
                id_token: Some("idtoken".to_string()),
                refresh_token: Some("refreshtoken".to_string()),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_post_sign_in_unauthorized() {
        let mut interactor = MockAuthInteractor::new();
        interactor
            .expect_authenticate()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .set_json(PostSignInRequest {
                id_token: Some("idtoken".to_string()),
                refresh_token: Some("refreshtoken".to_string()),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
