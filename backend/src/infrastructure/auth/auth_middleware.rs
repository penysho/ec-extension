use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error, Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use super::authenticator_interface::Authenticator;

const ID_TOKEN_COOKIE_NAME: &str = "ID_TOKEN";
const REFRESH_TOKEN_COOKIE_NAME: &str = "REFRESH_TOKEN";
const USER_ID_COOKIE_NAME: &str = "USER_ID";
const EXCLUDE_AUTH_PATHS: [&str; 2] = ["/health", "/ec-extension/auth/sign-in"];
// Fixed message is responded and no internal information is returned.
const UNAUTHORIZED_MESSAGE: &str = "Unauthorized";

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
        let user_id = req.cookie(USER_ID_COOKIE_NAME);

        let id_token_string = id_token.map(|cookie| cookie.value().to_string());
        let refresh_token_string = refresh_token.map(|cookie| cookie.value().to_string());
        let user_id_string = user_id.map(|cookie| cookie.value().to_string());

        match user_id_string.clone() {
            Some(value) => {
                req.extensions_mut().insert(value.clone());
            }
            None => {
                log::error!("User ID cookie not found");
                return Box::pin(
                    async move { Err(error::ErrorUnauthorized(UNAUTHORIZED_MESSAGE)) },
                );
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let (idp_user, _) = authenticator
                .validate_token(id_token_string.as_deref(), refresh_token_string.as_deref())
                .await
                .map_err(|e| error::ErrorUnauthorized(e))?;

            // NOTE: Since ServiceRequest.extensions_mut() cannot be called within this block, obtain the user ID from the cookie and set it outside this block
            // Then, compare the ID here with the ID obtained from the ID token to validate the request before processing it.
            let cookie_user_id = user_id_string.unwrap();
            if idp_user.id != cookie_user_id {
                log::error!(
                    "User ID mismatch. cookie User ID: {}, ID token sub: {}",
                    cookie_user_id,
                    idp_user.id
                );
                return Err(error::ErrorUnauthorized(UNAUTHORIZED_MESSAGE));
            }

            let res = fut.await?;
            Ok(res)
        })
    }
}
