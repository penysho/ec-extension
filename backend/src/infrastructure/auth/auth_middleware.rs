use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

const EXCLUDE_AUTH_PATHS: [&str; 1] = ["/health"];

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if EXCLUDE_AUTH_PATHS.contains(&req.path()) {
        return next.call(req).await;
    }

    let id_token = req.cookie("ID_TOKEN");
    if id_token.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }
    println!("ID_TOKEN: {:?}", id_token.unwrap().value());

    next.call(req).await
    // post-processing
}
