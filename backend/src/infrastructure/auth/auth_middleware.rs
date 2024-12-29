use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures_util::future::LocalBoxFuture;

use super::authenticator::Authenticator;

const ID_TOKEN_COOKIE_NAME: &str = "ID_TOKEN";
const EXCLUDE_AUTH_PATHS: [&str; 1] = ["/health"];

pub struct AuthTransform<A>
where
    A: Authenticator,
{
    authenticator: A,
}

impl<A> AuthTransform<A>
where
    A: Authenticator,
{
    pub fn new(authenticator: A) -> Self {
        AuthTransform { authenticator }
    }
}

impl<S, B, A> Transform<S, ServiceRequest> for AuthTransform<A>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    A: Authenticator,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S, A>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service,
            authenticator: self.authenticator.clone(),
        }))
    }
}

pub struct AuthMiddleware<S, A>
where
    A: Authenticator,
{
    service: S,
    authenticator: A,
}

impl<S, B, A> Service<ServiceRequest> for AuthMiddleware<S, A>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    A: Authenticator,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let authenticator = &self.authenticator;

        if EXCLUDE_AUTH_PATHS.contains(&req.path()) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let id_token = req.cookie(ID_TOKEN_COOKIE_NAME);
        if id_token.is_none() {
            log::error!("IDTOKEN cannot be retrieved from cookie.");
            return Box::pin(async { Err(ErrorUnauthorized("Unauthorized")) });
        };

        let token_value = id_token.unwrap().value().to_string();
        let auth_fut = authenticator.validate_token(token_value);

        let fut = self.service.call(req);
        Box::pin(async move {
            if let Err(_) = auth_fut.await {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            }
            let res = fut.await?;
            Ok(res)
        })
    }
}
