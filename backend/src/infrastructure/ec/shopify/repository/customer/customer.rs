use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, email::email::Email, error::error::DomainError},
    infrastructure::ec::ec_client_interface::ECClient,
    usecase::repository::customer_repository_interface::CustomerRepository,
};

/// Repository for Customers for Shopify.
pub struct CustomerRepositoryImpl<C: ECClient> {
    client: C,
}

impl<C: ECClient> CustomerRepositoryImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: ECClient + Send + Sync> CustomerRepository for CustomerRepositoryImpl<C> {
    /// Retrieve customer information by email.
    async fn get_customer_by_email(&self, email: &Email) -> Result<Customer, DomainError> {
        Err(DomainError::ValidationError)
    }
}
