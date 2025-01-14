use async_trait::async_trait;

use crate::domain::{customer::customer::Customer, error::error::DomainError};

/// Interface to generate response schema for auth.
#[async_trait]
pub trait AuthPresenter {
    type PostSiginInResponse;
    type PostSiginInErrorResponse;
    /// Generate a response of sign in.
    async fn present_post_sign_in(
        &self,
        result: Result<(Customer, String), DomainError>,
        refresh_token: Option<&str>,
    ) -> Result<Self::PostSiginInResponse, Self::PostSiginInErrorResponse>;

    type PostSignOutResponse;
    /// Generate a response of sign out.
    async fn present_post_sign_out(&self) -> Self::PostSignOutResponse;
}
