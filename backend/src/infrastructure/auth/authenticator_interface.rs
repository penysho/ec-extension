use async_trait::async_trait;

use crate::domain::error::error::DomainError;

pub struct IdpUser {
    pub id: String,
    pub email: String,
}

#[async_trait]
pub trait Authenticator: Send + Sync + Clone {
    /// Validate tokens issued by Idp.
    ///
    /// # Arguments
    ///
    /// * `id_token` - ID Token issued by Idp
    /// * `refresh_token` - Refresh Token issued by Idp
    /// * If both are `None`, an error occurs.
    ///
    /// # Returns
    ///
    /// * `Result<String, DomainError>` - The result of the operation.
    ///   - `Ok(String)` - ID Token. If an ID token is obtained using a refresh token, the updated ID token is returned, not the one received as an argument.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a error encountered in token validation
    async fn validate_token(
        &mut self,
        id_token: Option<String>,
        refresh_token: Option<String>,
    ) -> Result<(IdpUser, String), DomainError>;

    /// Obtain an ID Token by means of a refresh token.
    async fn get_id_token_by_refresh_token(
        &self,
        refresh_token: String,
    ) -> Result<String, DomainError>;
}
