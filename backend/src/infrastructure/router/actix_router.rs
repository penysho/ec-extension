use std::sync::Arc;

use crate::interface::controller::{
    controller::Controller, get_customers::GetCustomersQueryParams,
    get_draft_orders::GetDraftOrdersQueryParams, get_inventories::GetInventoriesQueryParams,
    get_locations::GetLocationsQueryParams, get_products::GetProductsQueryParams,
    post_draft_order::PostDraftOrderRequest, post_sign_in::PostSignInRequest,
    put_inventory_quantity_by_sku::PutInventoryQuantityBySkuRequest,
};
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
                web::get().to(|controller: web::Data<Arc<Controller>>, params: web::Query<GetProductsQueryParams>| async move {
                    controller.get_products(params).await
                }),
            )
            .route(
                "/products/{id}",
                web::get().to(
                    |controller: web::Data<Arc<Controller>>, path: web::Path<(String,)>| async move {
                        controller.get_product(path).await
                    },
                ),
            )
            .route(
                "/products/related/{id}",
                web::get().to(
                    |controller: web::Data<Arc<Controller>>, path: web::Path<(String,)>| async move {
                        controller.get_related_products(path).await
                    },
                ),
            )
            .route(
                "/inventories",
                 web::get().to(|controller: web::Data<Arc<Controller>>, params: web::Query<GetInventoriesQueryParams>| async move {
                    controller.get_inventories(params).await
                }),
            )
            .route(
                "/inventories/quantities/sku/{sku}",
                 web::put().to(|controller: web::Data<Arc<Controller>>, path: web::Path<(String,)>,
        body: web::Json<PutInventoryQuantityBySkuRequest>| async move {
                    controller.put_inventory_quantity_by_sku(path, body).await
                }),
            )
            .route(
                "/orders/draft",
                 web::get().to(|controller: web::Data<Arc<Controller>>, params: web::Query<GetDraftOrdersQueryParams>| async move {
                    controller.get_draft_orders(params).await
                }),
            )
            .route(
                "/orders/draft",
                 web::post().to(|controller: web::Data<Arc<Controller>>, body: web::Json<PostDraftOrderRequest>| async move {
                    controller.post_draft_order(body).await
                }),
            )
            .route(
                "/orders/draft/{id}",
                 web::delete().to(|controller: web::Data<Arc<Controller>>, path: web::Path<(String,)>| async move {
                    controller.delete_draft_order(path).await
                }),
            )
            .route(
                "/orders/draft/complete/{id}",
                 web::put().to(|controller: web::Data<Arc<Controller>>, path: web::Path<(String,)>| async move {
                    controller.complete_draft_order(path).await
                }),
            )
            .route(
                "/locations",
                 web::get().to(|controller: web::Data<Arc<Controller>>, params: web::Query<GetLocationsQueryParams>| async move {
                    controller.get_locations(params).await
                }),
            )
            .route(
                "/customers",
                 web::get().to(|controller: web::Data<Arc<Controller>>, params: web::Query<GetCustomersQueryParams>| async move {
                    controller.get_customers(params).await
                }),
            )
            .route(
                "/auth/sign-in",
                 web::post().to(|controller: web::Data<Arc<Controller>>, body: web::Json<PostSignInRequest>| async move {
                    controller.post_sign_in(body).await
                }),
            )
            .route(
                "/auth/sign-out",
                 web::post().to(|controller: web::Data<Arc<Controller>>| async move {
                    controller.post_sign_out().await
                }),
            )
    );
}
