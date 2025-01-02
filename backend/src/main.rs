use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, web, App, HttpServer};
use env_logger::Env;
use infrastructure::auth::auth_middleware::AuthTransform;
use infrastructure::auth::cognito::cognito_authenticator::CognitoAuthenticator;
use infrastructure::auth::rbac::rbac_authorizer::RbacAuthorizer;
use infrastructure::config::config::{AppConfig, CognitoConfig, ShopifyConfig};
use infrastructure::module::interact_provider_impl::InteractProviderImpl;
use interface::controller::controller::Controller;
use std::io;
use std::sync::Arc;

mod domain;
mod infrastructure;
mod interface;
mod usecase;

use crate::infrastructure::router::actix_router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = AppConfig::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let shopify_config =
        ShopifyConfig::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let cognito_config =
        CognitoConfig::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let aws_config = aws_config::load_from_env().await;

    env_logger::init_from_env(Env::default().default_filter_or(app_config.log_level()));

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
                cognito_config.clone(),
                aws_config.clone(),
            )))
            .wrap(Logger::default().exclude("/health"))
            .wrap(cors)
            // Definition of app data
            .app_data(web::Data::new(Arc::new(Controller::new(
                Box::new(InteractProviderImpl::new(
                    shopify_config.clone(),
                    cognito_config.clone(),
                    aws_config.clone(),
                )),
                Box::new(RbacAuthorizer::new()),
            ))))
            // Definition of routes
            .configure(actix_router::configure_routes)
    })
    .bind(format!("{}:{}", app_config.address(), app_config.port()))?
    .run()
    .await
}
