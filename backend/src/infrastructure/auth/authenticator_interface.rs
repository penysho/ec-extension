use async_trait::async_trait;

use crate::domain::error::error::DomainError;

#[async_trait]
pub trait Authenticator: Send + Sync {
    /// Validate tokens issued by Idp.
    async fn validate_token(
        &mut self,
        id_token: Option<String>,
        refresh_token: Option<String>,
    ) -> Result<String, DomainError>;

    /// Obtain an ID Token by means of a refresh token.
    async fn get_id_token_by_refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<String, DomainError>;
}
