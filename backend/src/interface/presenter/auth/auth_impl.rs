use actix_web::{cookie::Cookie, HttpResponse};
use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, error::error::DomainError},
    interface::presenter::auth_presenter_interface::AuthPresenter,
};

use super::schema::PostSingInErrorResponse;

/// Generate a response schema for the auth.
pub struct AuthPresenterImpl;
impl AuthPresenterImpl {
    pub fn new() -> Self {
        AuthPresenterImpl
    }
}

const ID_TOKEN_COOKIES_NAME: &str = "ID_TOKEN";
const REFRESH_TOKEN_COOKIES_NAME: &str = "REFRESH_TOKEN";
const CUSTOMER_ID_COOKIES_NAME: &str = "CUSTOMER_ID";

#[async_trait]
impl AuthPresenter for AuthPresenterImpl {
    type PostSiginInResponse = HttpResponse;
    type PostSiginInErrorResponse = PostSingInErrorResponse;
    async fn present_post_sign_in(
        &self,
        result: Result<(Customer, String), DomainError>,
        refresh_token: Option<&str>,
    ) -> Result<Self::PostSiginInResponse, Self::PostSiginInErrorResponse> {
        match result {
            Ok((customer, new_id_token)) => {
                let cookie_id_token = Cookie::build(ID_TOKEN_COOKIES_NAME, new_id_token)
                    .secure(true)
                    .http_only(true)
                    .path("/")
                    .finish();

                let mut cookie_refresh_token = Cookie::build(REFRESH_TOKEN_COOKIES_NAME, "")
                    .secure(true)
                    .http_only(true)
                    .path("/")
                    .finish();
                if refresh_token.is_some() {
                    cookie_refresh_token.set_value(refresh_token.unwrap());
                }

                let cookie_customer_id = Cookie::build(CUSTOMER_ID_COOKIES_NAME, customer.id())
                    .secure(true)
                    .http_only(false)
                    .path("/")
                    .finish();

                return Ok(HttpResponse::Ok()
                    .cookie(cookie_id_token)
                    .cookie(cookie_refresh_token)
                    .cookie(cookie_customer_id)
                    .finish());
            }
            Err(_) => Err(PostSingInErrorResponse::Unauthorized),
        }
    }

    type PostSignOutResponse = HttpResponse;
    async fn present_post_sign_out(&self) -> Self::PostSignOutResponse {
        let cookie_id_token = Cookie::build(ID_TOKEN_COOKIES_NAME, "")
            .secure(true)
            .http_only(true)
            .path("/")
            .finish();

        let cookie_refresh_token = Cookie::build(REFRESH_TOKEN_COOKIES_NAME, "")
            .secure(true)
            .http_only(true)
            .path("/")
            .finish();

        let cookie_customer_id = Cookie::build(CUSTOMER_ID_COOKIES_NAME, "")
            .secure(true)
            .http_only(false)
            .path("/")
            .finish();

        HttpResponse::Ok()
            .cookie(cookie_id_token)
            .cookie(cookie_refresh_token)
            .cookie(cookie_customer_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::interface::mock::domain_mock::mock_customers;

    use super::*;

    #[actix_web::test]
    async fn test_present_post_sign_in_success() {
        let presenter = AuthPresenterImpl::new();
        let customer = mock_customers(1).remove(0);

        let result = presenter
            .present_post_sign_in(Ok((customer, "idtoken".to_string())), Some("refreshtoken"))
            .await
            .unwrap();

        assert_eq!(
            result
                .cookies()
                .find(|cookie| cookie.name() == "ID_TOKEN")
                .unwrap()
                .value(),
            "idtoken"
        );
        assert_eq!(
            result
                .cookies()
                .find(|cookie| cookie.name() == "REFRESH_TOKEN")
                .unwrap()
                .value(),
            "refreshtoken"
        );
        assert_eq!(
            result
                .cookies()
                .find(|cookie| cookie.name() == "CUSTOMER_ID")
                .unwrap()
                .value(),
            "0"
        );
    }

    #[actix_web::test]
    async fn test_present_post_sign_in_unauthorize() {
        let presenter = AuthPresenterImpl::new();

        let result = presenter
            .present_post_sign_in(Err(DomainError::SystemError), Some("refreshtoken"))
            .await;

        assert!(matches!(result, Err(PostSingInErrorResponse::Unauthorized)));
    }
}
