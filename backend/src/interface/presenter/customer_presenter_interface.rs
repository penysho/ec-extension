use async_trait::async_trait;

use crate::domain::{customer::customer::Customer, error::error::DomainError};

/// Interface to generate response schema for customers.
#[async_trait]
pub trait CustomerPresenter {
    type GetCustomersResponse;
    type GetCustomersErrorResponse;
    /// Generate a list response of customer information.
    async fn present_get_customers(
        &self,
        result: Result<Vec<Customer>, DomainError>,
    ) -> Result<Self::GetCustomersResponse, Self::GetCustomersErrorResponse>;
}
