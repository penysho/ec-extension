use actix_web::{web, Responder};
use serde::Deserialize;

use crate::{
    domain::{email::email::Email, error::error::DomainError},
    interface::presenter::{
        customer::customer_impl::CustomerPresenterImpl,
        customer_presenter_interface::CustomerPresenter,
    },
    usecase::interactor::customer_interactor_interface::GetCustomersQuery,
};

use super::{controller::Controller, interactor_provider_interface::InteractorProvider};

#[derive(Deserialize)]
pub struct GetCustomersQueryParams {
    email: Option<String>,
}

impl<I, T, C> Controller<I, T, C>
where
    I: InteractorProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Get a list of customers.
    pub async fn get_customers(
        &self,
        request: actix_web::HttpRequest,
        params: web::Query<GetCustomersQueryParams>,
    ) -> impl Responder {
        let presenter = CustomerPresenterImpl::new();

        let query = validate_query_params(&params)?;
        let user = self.get_user(&request)?;
        let transaction_manager = self.get_transaction_manager(&request)?;

        let interactor = self
            .interactor_provider
            .provide_customer_interactor(transaction_manager)
            .await;
        let results = interactor.get_customers(user, &query).await;

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

    use crate::domain::user::user::UserInterface;
    use crate::infrastructure::auth::idp_user::IdpUser;
    use crate::infrastructure::db::sea_orm::sea_orm_manager::SeaOrmTransactionManager;
    use crate::infrastructure::db::transaction_manager_interface::TransactionManager;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interactor_provider_interface::MockInteractorProvider;
    use crate::interface::mock::domain_mock::mock_customers;
    use crate::usecase::interactor::customer_interactor_interface::{
        CustomerInteractor, MockCustomerInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::{http::StatusCode, test, App, Error};
    use actix_web::{web, HttpMessage};
    use mockall::predicate::{always, eq};
    use sea_orm::{DatabaseConnection, DatabaseTransaction};

    const BASE_URL: &'static str = "/ec-extension/customers";

    async fn setup(
        interactor: MockCustomerInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interactor_provider =
            MockInteractorProvider::<DatabaseTransaction, Arc<DatabaseConnection>>::new();
        interactor_provider
            .expect_provide_customer_interactor()
            .return_once(move |_| Box::new(interactor) as Box<dyn CustomerInteractor>);

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
    async fn test_get_customers_success() {
        let mut interactor = MockCustomerInteractor::new();
        interactor
            .expect_get_customers()
            .with(
                always(),
                eq(GetCustomersQuery::Email(
                    Email::new("john@example.com").expect("Failed to create email"),
                )),
            )
            .returning(|_, _| Ok(mock_customers(10)));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_customers_not_specified_email() {
        let interactor = MockCustomerInteractor::new();

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email="))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_customers_not_found() {
        let mut interactor = MockCustomerInteractor::new();
        interactor
            .expect_get_customers()
            .returning(|_, _| Ok(vec![]));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_customers_bad_request() {
        let mut interactor = MockCustomerInteractor::new();
        interactor
            .expect_get_customers()
            .returning(|_, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_customers_service_unavailable() {
        let mut interactor = MockCustomerInteractor::new();
        interactor
            .expect_get_customers()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::get()
            .uri(&format!("{BASE_URL}?email=john@example.com"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
