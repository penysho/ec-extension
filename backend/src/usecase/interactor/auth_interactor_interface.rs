use async_trait::async_trait;
use mockall::automock;

use crate::domain::customer::customer::Customer;
use crate::domain::error::error::DomainError;

/// Interactor interface for customer.
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
    /// * `Result<Customer, DomainError>` - The result of the operation.
    ///   - `Ok(Customer)` - The Authenticated customer.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the customer repository fails.
    async fn authenticate(
        &self,
        id_token: String,
        refresh_token: String,
    ) -> Result<Customer, DomainError>;
}
