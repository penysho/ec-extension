use async_trait::async_trait;

use crate::domain::error::error::DomainError;

use super::idp_user::IdpUser;

/// Authentication interface.
#[async_trait]
pub trait Authenticator: Send + Sync + Clone {
    /// Verify tokens issued by Idp.
    ///
    /// # Arguments
    ///
    /// * `id_token` - ID Token issued by Idp
    /// * `refresh_token` - Refresh Token issued by Idp
    /// * If both are `None`, an error occurs.
    ///
    /// # Returns
    ///
    /// * `Result<(IdpUser, String), DomainError>` - The result of the operation.
    ///   - `Ok((IdpUser, String))` - Idp user information and ID Token. If an ID token is obtained using a refresh token, the updated ID token is returned, not the one received as an argument.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a error encountered in token validation
    async fn verify_token(
        &mut self,
        id_token: Option<&str>,
        refresh_token: Option<&str>,
    ) -> Result<(IdpUser, String), DomainError>;

    /// Obtain an ID Token by means of a refresh token.
    async fn get_id_token_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<String, DomainError>;
}
