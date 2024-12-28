use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};

async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let id_token = req.cookie("ID_TOKEN");
    if id_token.is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
    }

    next.call(req).await
    // post-processing
}
