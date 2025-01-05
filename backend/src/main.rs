use actix_cors::Cors;
use actix_web::middleware::{from_fn, Logger};
use actix_web::{http, web, App, HttpServer};
use env_logger::Env;
use infrastructure::auth::auth_middleware::AuthTransform;
use infrastructure::auth::cognito::cognito_authenticator::CognitoAuthenticator;
use infrastructure::config::config::ConfigProvider;
use infrastructure::db::sea_orm::sea_orm_manager::{
    SeaOrmConnectionProvider, SeaOrmTransactionManager,
};
use infrastructure::db::transaction_middleware;
use infrastructure::module::interact_provider_impl::InteractProviderImpl;
use interface::controller::controller::Controller;
use std::io;

mod domain;
mod infrastructure;
mod interface;
mod usecase;

use crate::infrastructure::router::actix_router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_provider = ConfigProvider::new().await.map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to load config: {}", e),
        )
    })?;
    let app_config = config_provider.app_config().clone();

    env_logger::init_from_env(Env::default().default_filter_or(app_config.log_level()));

    let controller = web::Data::new(Controller::new(InteractProviderImpl::new(
        config_provider.shopify_config().clone(),
        config_provider.cognito_config().clone(),
        config_provider.aws_sdk_config().clone(),
    )));

    let connection_provider =
        SeaOrmConnectionProvider::new(config_provider.database_config().clone())
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let transaction_manager = web::Data::new(SeaOrmTransactionManager::new(
        connection_provider.get_connection().clone(),
    ));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .supports_credentials()
            .max_age(0);

        App::new()
            // Definition of middleware
            // NOTE: Executed in the order of last written.
            .wrap(AuthTransform::new(CognitoAuthenticator::new(
                config_provider.cognito_config().clone(),
                config_provider.aws_sdk_config().clone(),
            )))
            .wrap(from_fn(
                transaction_middleware::transaction_middleware::<SeaOrmTransactionManager>,
            ))
            .wrap(Logger::default().exclude("/health"))
            .wrap(cors)
            // Definition of app data
            .app_data(transaction_manager.clone())
            .app_data(controller.clone())
            // Definition of routes
            .configure(actix_router::configure_routes::<InteractProviderImpl>)
    })
    .bind(format!("{}:{}", app_config.address(), app_config.port()))?
    .run()
    .await
}
