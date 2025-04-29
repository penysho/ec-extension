use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;

use crate::domain::customer::customer::Customer;
use crate::domain::email::email::Email;
use crate::domain::error::error::DomainError;
use crate::domain::user::user::UserInterface;

#[derive(Debug, Clone, PartialEq)]
pub enum GetCustomersQuery {
    Email(Email),
}

/// Interactor interface for customer.
#[automock]
#[async_trait]
pub trait CustomerInteractor {
    /// Get a list of customer by query.
    ///
    /// # Arguments
    ///
    /// * `user` - The user interface.
    /// * `query` - The query to get customers.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Customer>, DomainError>` - The result of the operation.
    ///   - `Ok(Vec<Customer>)` - The customers.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the customer repository fails.
    async fn get_customers(
        &self,
        user: Arc<dyn UserInterface>,
        query: &GetCustomersQuery,
    ) -> Result<Vec<Customer>, DomainError>;
}
