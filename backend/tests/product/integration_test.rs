use actix_http::StatusCode;
use actix_web::test;

use crate::common;

#[actix_web::test]
async fn test_health_success() {
    let app = common::setup().await;
    let request = test::TestRequest::get().uri("/health").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(test::read_body(response).await, "ok");
}
