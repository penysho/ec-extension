use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use infrastructure::config::config::{AppConfig, ShopifyConfig};
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

    env_logger::init_from_env(Env::default().default_filter_or(app_config.log_level()));

    let controller = web::Data::new(Arc::new(Controller::new(Box::new(
        InteractProviderImpl::new(shopify_config),
    ))));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default().exclude("/health"))
            .app_data(controller.clone())
            .configure(actix_router::configure_routes)
    })
    .bind(format!("{}:{}", app_config.address(), app_config.port()))?
    .run()
    .await
}
