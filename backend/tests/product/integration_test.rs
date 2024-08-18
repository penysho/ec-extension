use actix_http::StatusCode;
use actix_web::test;

use crate::common;

#[actix_web::test]
async fn test_health_success() {
    let app = common::setup().await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(test::read_body(resp).await, "ok");
}
