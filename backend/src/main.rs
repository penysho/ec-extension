use actix_cors::Cors;
use actix_web::middleware::{from_fn, Logger};
use actix_web::{http, web, App, HttpServer};
use env_logger::Env;
use infrastructure::auth::auth_middleware::AuthTransform;
use infrastructure::auth::cognito::cognito_authenticator::CognitoAuthenticator;
use infrastructure::auth::rbac::rbac_authorizer::RbacAuthorizer;
use infrastructure::config::config::ConfigProvider;
use infrastructure::db::sea_orm::sea_orm_manager::{
    SeaOrmConnectionProvider, SeaOrmTransactionManager,
};
use infrastructure::db::sea_orm::sea_orm_transaction_middleware;
use infrastructure::module::interactor_provider_impl::InteractorProviderImpl;
use infrastructure::router::actix_router;
use interface::controller::controller::Controller;
use library::tracing::middleware::CustomRootSpanBuilder;
use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::io;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

mod domain;
mod infrastructure;
mod interface;
mod library;
mod usecase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config_provider = ConfigProvider::new()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let app_config = config_provider.app_config().clone();

    env_logger::init_from_env(Env::default().default_filter_or(app_config.log_level()));

    library::tracing::opentelemetry::init_telemetry(&app_config);

    let connection_provider = web::Data::new(
        SeaOrmConnectionProvider::new(config_provider.database_config().clone())
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
    );

    // No transaction is started because it is used globally.
    let transaction_manager = Arc::new(
        SeaOrmTransactionManager::new(Arc::clone(&connection_provider.get_connection()))
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
    );

    let controller = web::Data::new(Controller::new(InteractorProviderImpl::new(
        config_provider.shopify_config().clone(),
        config_provider.cognito_config().clone(),
        config_provider.aws_sdk_config().clone(),
    )));

    HttpServer::new(move || {
        let app_config = config_provider.app_config().clone();

        let mut cors = Cors::default();
        for origin in app_config.cors_allowed_origins() {
            cors = cors.allowed_origin(origin.as_str());
        }
        let cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
            .supports_credentials()
            .max_age(0);

        App::new()
            // Definition of middleware
            // NOTE: Executed in the order of last written.
            .wrap(AuthTransform::new(CognitoAuthenticator::new(
                config_provider.cognito_config().clone(),
                config_provider.aws_sdk_config().clone(),
                RbacAuthorizer::new(transaction_manager.clone()),
            )))
            .wrap(from_fn(
                sea_orm_transaction_middleware::sea_orm_transaction_middleware,
            ))
            .wrap(Logger::default().exclude("/health"))
            .wrap(cors)
            // .wrap(TracingLogger::<CustomRootSpanBuilder>::new())
            .wrap(TracingLogger::default())
            // Definition of app data
            .app_data(connection_provider.clone())
            .app_data(controller.clone())
            // Definition of routes
            .configure(
                actix_router::configure_routes::<
                    InteractorProviderImpl,
                    DatabaseTransaction,
                    Arc<DatabaseConnection>,
                >,
            )
    })
    .bind(format!("{}:{}", app_config.address(), app_config.port()))?
    .run()
    .await
}
