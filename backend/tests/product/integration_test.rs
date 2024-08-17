use actix_web::{http::StatusCode, test, App};
use backend::infrastructure::router::actix_router;

#[actix_web::test]
async fn test_health_success() {
    let app = test::init_service(App::new().configure(actix_router::configure_routes)).await;
    let request = test::TestRequest::get().uri("/health").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(test::read_body(response).await, "ok");
}
