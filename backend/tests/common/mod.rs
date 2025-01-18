use std::sync::Arc;

use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    test, App, Error,
};
use backend::infrastructure::{
    module::interact_provider_impl::InteractProviderImpl, router::actix_router,
};
use sea_orm::{DatabaseConnection, DatabaseTransaction};

pub async fn setup() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    test::init_service(App::new().configure(
        actix_router::configure_routes::<
            InteractProviderImpl,
            DatabaseTransaction,
            Arc<DatabaseConnection>,
        >,
    ))
    .await
}
