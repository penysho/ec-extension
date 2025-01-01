use async_trait::async_trait;
use mockall::automock;

use crate::domain::customer::customer::Customer;
use crate::domain::error::error::DomainError;

/// Interactor interface for auth.
#[automock]
#[async_trait]
pub trait AuthInteractor {
    /// Authentication by token.
    ///
    /// # Arguments
    ///
    /// * `id_token` - ID Token issued by Idp
    /// * `refresh_token` - Refresh Token issued by Idp
    ///
    /// # Returns
    ///
    /// * `Result<(Customer, String), DomainError>` - The result of the operation.
    ///   - `Ok((Customer, String))` - The Authenticated customer and ID Token. If an ID token is obtained using a refresh token, the updated ID token is returned, not the one received as an argument.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the customer repository fails.
    async fn authenticate(
        &self,
        id_token: &Option<String>,
        refresh_token: &Option<String>,
    ) -> Result<(Customer, String), DomainError>;
}
