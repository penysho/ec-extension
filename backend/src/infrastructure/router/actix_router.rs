use std::sync::Arc;

use crate::interface::controller::controller::Controller;
use actix_web::{web, HttpResponse};

/// Define actix routers.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health").route(web::get().to(|| async { HttpResponse::Ok().body("ok") })),
    );
    cfg.service(
        web::scope("/ec-extension")
            .route(
                "/products",
                web::get().to(|controller: web::Data<Arc<Controller>>| async move {
                    controller.get_products().await
                }),
            )
            .route(
                "/products/{id}",
                web::get().to(
                    |controller: web::Data<Arc<Controller>>, path: web::Path<(String,)>| async move {
                        controller.get_product(path).await
                    },
                ),
            ),
    );
}
