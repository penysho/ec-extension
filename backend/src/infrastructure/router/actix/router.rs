use actix_web::{web, Responder};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/ec-extension")
            .route("/health",web::get().to(health)));
}

pub async fn health() -> impl Responder {
    "ok"
}
