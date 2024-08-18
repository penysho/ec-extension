use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    test, App, Error,
};
use backend::infrastructure::router::actix_router;

pub async fn setup() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    test::init_service(App::new().configure(actix_router::configure_routes)).await
}
