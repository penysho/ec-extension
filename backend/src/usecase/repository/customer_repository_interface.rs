use async_trait::async_trait;

use crate::domain::{customer::customer::Customer, email::email::Email, error::error::DomainError};

#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn get_customer_by_email(&self, email: &Email) -> Result<Customer, DomainError>;
}
