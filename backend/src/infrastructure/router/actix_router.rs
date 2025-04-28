use crate::interface::controller::{
    controller::Controller, get_customers::GetCustomersQueryParams,
    get_draft_orders::GetDraftOrdersQueryParams, get_inventories::GetInventoriesQueryParams,
    get_locations::GetLocationsQueryParams, get_products::GetProductsQueryParams,
    interactor_provider_interface::InteractorProvider, post_draft_order::PostDraftOrderRequest,
    post_sign_in::PostSignInRequest,
    put_inventory_quantity_by_sku::PutInventoryQuantityBySkuRequest,
};
use actix_web::{web, HttpResponse};

/// Define actix routers.
pub fn configure_routes<I, T, C>(cfg: &mut web::ServiceConfig)
where
    I: InteractorProvider<T, C> + 'static,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    cfg.service(
        web::resource("/health").route(web::get().to(|| async { HttpResponse::Ok().body("ok") })),
    );

    cfg.service(
        web::scope("/ec-extension")
            .route(
                "/products",
                web::get().to(
                    |controller: web::Data<Controller<I, T, C>>, params: web::Query<GetProductsQueryParams>| async move {
                        controller.get_products(params).await
                    },
                ),
            )
            .route(
                "/products/{id}",
                web::get().to(
                    |controller: web::Data<Controller<I, T, C>>, path: web::Path<(String,)>| async move {
                        controller.get_product(path).await
                    },
                ),
            )
            .route(
                "/products/related/{id}",
                web::get().to(
                    |controller: web::Data<Controller<I, T, C>>, path: web::Path<(String,)>| async move {
                        controller.get_related_products(path).await
                    },
                ),
            )
            .route(
                "/inventories",
                web::get().to(
                    |controller: web::Data<Controller<I, T, C>>, params: web::Query<GetInventoriesQueryParams>| async move {
                        controller.get_inventories(params).await
                    },
                ),
            )
            .route(
                "/inventories/quantities/sku/{sku}",
                web::put().to(
                    |controller: web::Data<Controller<I, T, C>>, path: web::Path<(String,)>, body: web::Json<PutInventoryQuantityBySkuRequest>| async move {
                        controller.put_inventory_quantity_by_sku(path, body).await
                    },
                ),
            )
            .route(
                "/orders/draft",
                web::get().to(
                    |controller: web::Data<Controller<I, T, C>>, request: actix_web::HttpRequest, params: web::Query<GetDraftOrdersQueryParams>| async move {
                        controller.get_draft_orders(request,params).await
                    },
                ),
            )
            .route(
                "/orders/draft",
                web::post().to(
                    |controller: web::Data<Controller<I, T, C>>, request: actix_web::HttpRequest, body: web::Json<PostDraftOrderRequest>| async move {
                        controller.post_draft_order(request,body).await
                    },
                ),
            )
            .route(
                "/orders/draft/{id}",
                web::delete().to(
                    |controller: web::Data<Controller<I, T, C>>, request: actix_web::HttpRequest, path: web::Path<(String,)>| async move {
                        controller.delete_draft_order(request,path).await
                    },
                ),
            )
            .route(
                "/orders/draft/complete/{id}",
                web::put().to(
                    |controller: web::Data<Controller<I, T, C>>, request: actix_web::HttpRequest, path: web::Path<(String,)>| async move {
                        controller.complete_draft_order(request,path).await
                    },
                ),
            )
            .route(
                "/locations",
                web::get().to(
                    |controller: web::Data<Controller<I, T, C>>, params: web::Query<GetLocationsQueryParams>| async move {
                        controller.get_locations(params).await
                    },
                ),
            )
            .route(
                "/customers",
                web::get().to(
                    |controller: web::Data<Controller<I, T, C>>, request: actix_web::HttpRequest, params: web::Query<GetCustomersQueryParams>| async move {
                        controller.get_customers( request, params).await
                    },
                ),
            )
            .route(
                "/auth/sign-in",
                web::post().to(
                    |controller: web::Data<Controller<I, T, C>>, request: actix_web::HttpRequest, body: web::Json<PostSignInRequest>| async move {
                        controller.post_sign_in(request, body).await
                    },
                ),
            )
            .route(
                "/auth/sign-out",
                web::post().to(
                    |controller: web::Data<Controller<I, T, C>>| async move {
                        controller.post_sign_out().await
                    },
                ),
            ),
    );
}
