use async_trait::async_trait;

use crate::domain::{customer::customer::Customer, email::email::Email, error::error::DomainError};

/// Repository interface for customers.
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    /// Retrieve customer information by email.
    async fn find_customer_by_email(&self, email: &Email) -> Result<Customer, DomainError>;
}
