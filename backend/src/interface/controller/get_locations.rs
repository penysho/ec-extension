use actix_web::{web, Responder};
use serde::Deserialize;

use crate::interface::presenter::{
    location::location_impl::LocationPresenterImpl, location_presenter_interface::LocationPresenter,
};

use super::{controller::Controller, interact_provider_interface::InteractProvider};

#[derive(Deserialize)]
pub struct GetLocationsQueryParams {
    limit: Option<u32>,
    offset: Option<u32>,
}

impl<I, T, C> Controller<I, T, C>
where
    I: InteractProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Get a list of locations.
    pub async fn get_locations(
        &self,
        params: web::Query<GetLocationsQueryParams>,
    ) -> impl Responder {
        let presenter = LocationPresenterImpl::new();

        let interactor = self.interact_provider.provide_location_interactor().await;
        let results = interactor
            .get_locations(&params.limit, &params.offset)
            .await;

        presenter.present_get_locations(results).await
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::error::error::DomainError;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use crate::interface::mock::domain_mock::mock_locations;
    use crate::usecase::interactor::location_interactor_interface::{
        LocationInteractor, MockLocationInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::{http::StatusCode, test, App, Error};

    const BASE_URL: &'static str = "/ec-extension/locations";

    async fn setup(
        interactor: MockLocationInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interact_provider = MockInteractProvider::<(), ()>::new();
        interact_provider
            .expect_provide_location_interactor()
            .return_once(move || Box::new(interactor) as Box<dyn LocationInteractor>);

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
    async fn test_get_locations_success() {
        let mut interactor = MockLocationInteractor::new();
        interactor
            .expect_get_locations()
            .returning(|_, _| Ok(mock_locations(10)));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_get_locations_bad_request() {
        let mut interactor = MockLocationInteractor::new();
        interactor
            .expect_get_locations()
            .returning(|_, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_get_locations_service_unavailable() {
        let mut interactor = MockLocationInteractor::new();
        interactor
            .expect_get_locations()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::get().uri(BASE_URL).to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
