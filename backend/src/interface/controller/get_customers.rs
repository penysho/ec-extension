use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{
    domain::{email::email::Email, error::error::DomainError},
    infrastructure::auth::authorizer_interface::{Action, Resource},
    interface::presenter::{
        customer::customer_impl::CustomerPresenterImpl,
        customer_presenter_interface::CustomerPresenter,
    },
    usecase::interactor::customer_interactor_interface::GetCustomersQuery,
};

use super::controller::Controller;

#[derive(Deserialize)]
pub struct GetCustomersQueryParams {
    email: Option<String>,
}

impl Controller {
    /// Get a list of customers.
    pub async fn get_customers(
        &self,
        params: web::Query<GetCustomersQueryParams>,
        request: actix_web::HttpRequest,
    ) -> impl Responder {
        let presenter = CustomerPresenterImpl::new();

        let user_id = self.get_user_id(&request)?;
        self.authorizer
            .authorize(user_id.as_str(), &Resource::Customer, &Action::Read)
            .await?;

        let query = validate_query_params(&params)?;
        let interactor = self.interact_provider.provide_customer_interactor().await;
        let results = interactor.get_customers(&query).await;

        presenter.present_get_customers(results).await
    }
}

fn validate_query_params(
    params: &GetCustomersQueryParams,
) -> Result<GetCustomersQuery, DomainError> {
    if let Some(email) = params.email.clone() {
        if !email.is_empty() {
            return Ok(GetCustomersQuery::Email(Email::new(email)?));
        }
    }

    Err(DomainError::InvalidRequest)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::infrastructure::auth::authorizer_interface::MockAuthorizer;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::interface::mock::domain_mock::mock_customers;
    use crate::usecase::interactor::customer_interactor_interface::{
        CustomerInteractor, MockCustomerInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::{http::StatusCode, test, App, Error};
    use actix_web::{web, HttpMessage};
    use mockall::predicate::eq;

    const BASE_URL: &'static str = "/ec-extension/customers";

    async fn setup(
        interactor: MockCustomerInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interact_provider = MockInteractProvider::new();
        interact_provider
            .expect_provide_customer_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn CustomerInteractor>);

        let mut authorizer = MockAuthorizer::new();
        authorizer.expect_authorize().returning(|_, _, _| Ok(()));

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
    async fn test_get_customers_success() {
        let mut interactor = MockCustomerInteractor::new();
        interactor
            .expect_get_customers()
            .with(eq(GetCustomersQuery::Email(
                Email::new("john@example.com").expect("Failed to create email"),
            )))
            .returning(|_| Ok(mock_customers(10)));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        req.extensions_mut().insert("user_id".to_string());

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_customers_not_specified_email() {
        let interactor = MockCustomerInteractor::new();

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email="))
            .to_request();
        req.extensions_mut().insert("user_id".to_string());

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_customers_not_found() {
        let mut interactor = MockCustomerInteractor::new();
        interactor.expect_get_customers().returning(|_| Ok(vec![]));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        req.extensions_mut().insert("user_id".to_string());

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_customers_bad_request() {
        let mut interactor = MockCustomerInteractor::new();
        interactor
            .expect_get_customers()
            .returning(|_| Err(DomainError::ValidationError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        req.extensions_mut().insert("user_id".to_string());

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_customers_service_unavailable() {
        let mut interactor = MockCustomerInteractor::new();
        interactor
            .expect_get_customers()
            .returning(|_| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        req.extensions_mut().insert("user_id".to_string());

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
