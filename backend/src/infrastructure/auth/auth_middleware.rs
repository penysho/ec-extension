use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
};
use futures_util::future::LocalBoxFuture;

use super::{authenticator::Authenticator, cognito::cognito_authenticator::CognitoAuthenticator};

const ID_TOKEN_COOKIE_NAME: &str = "ID_TOKEN";
const EXCLUDE_AUTH_PATHS: [&str; 1] = ["/health"];

pub struct AuthTransform;

impl<S, B> Transform<S, ServiceRequest> for AuthTransform
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S, CognitoAuthenticator>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let authenticator = CognitoAuthenticator::new();
        ready(Ok(AuthMiddleware {
            service,
            authenticator,
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

        authenticator.validate_token(id_token.unwrap().value().to_string());

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
