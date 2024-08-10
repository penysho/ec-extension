use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use env_logger::Env;
use infrastructure::module::interact_provider_impl::InteractProviderImpl;
use interface::controller::controller::Controller;
use std::env;
use std::sync::Arc;

mod entity;
mod infrastructure;
mod interface;
mod usecase;

use crate::infrastructure::router::actix_router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());
    match log_level.as_str() {
        "error" | "warn" | "info" | "debug" | "trace" | "off" => (),
        _ => {
            eprintln!("LOG_LEVELに不正な値が定義されています: {}", log_level);
            std::process::exit(1);
        }
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    let controller = web::Data::new(Arc::new(Controller::new(Box::new(
        InteractProviderImpl::new(),
    ))));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            // .wrap(Logger::default().exclude("/health"))
            .app_data(controller.clone())
            .configure(actix_router::configure_routes)
            .route(
                "/health",
                web::get().to(|| async { HttpResponse::Ok().body("ok") }),
            )
    })
    .bind("0.0.0.0:8011")?
    .run()
    .await
}
