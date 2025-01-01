use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error, Error,
};
use futures_util::future::LocalBoxFuture;

use super::authenticator_interface::Authenticator;

const ID_TOKEN_COOKIE_NAME: &str = "ID_TOKEN";
const REFRESH_TOKEN_COOKIE_NAME: &str = "REFRESH_TOKEN";
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
    A: Authenticator + 'static,
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
    A: Authenticator + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mut authenticator = self.authenticator.clone();

        if EXCLUDE_AUTH_PATHS.contains(&req.path()) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let id_token = req.cookie(ID_TOKEN_COOKIE_NAME);
        let refresh_token = req.cookie(REFRESH_TOKEN_COOKIE_NAME);

        let id_token_string = if id_token.is_some() {
            Some(id_token.unwrap().value().to_string())
        } else {
            None
        };
        let refresh_token_string = if refresh_token.is_some() {
            Some(refresh_token.unwrap().value().to_string())
        } else {
            None
        };

        let fut = self.service.call(req);
        Box::pin(async move {
            if let Err(_) = authenticator
                .validate_token(id_token_string, refresh_token_string)
                .await
            {
                return Err(error::ErrorUnauthorized("Unauthorized"));
            }
            let res = fut.await?;
            Ok(res)
        })
    }
}
