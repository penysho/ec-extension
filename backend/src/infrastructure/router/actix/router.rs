use crate::interface::controller::controller::Controller;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let controller = web::Data::new(Controller::new());
    cfg.app_data(controller.clone());

    cfg.service(
        web::scope("/ec-extension")
            .route(
                "/products",
                web::get().to(|controller: web::Data<Controller>| async move {
                    controller.get_products().await
                }),
            )
            .route(
                "/products/{id}",
                web::get().to(|controller: web::Data<Controller>| async move {
                    controller.get_product().await
                }),
            ),
    );
}
