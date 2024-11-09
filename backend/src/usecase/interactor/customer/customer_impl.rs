use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, error::error::DomainError},
    usecase::{
        interactor::customer_interactor_interface::{CustomerInteractor, GetCustomersQuery},
        repository::customer_repository_interface::CustomerRepository,
    },
};

/// Customer Interactor.
pub struct CustomerInteractorImpl {
    customer_repository: Box<dyn CustomerRepository>,
}

impl CustomerInteractorImpl {
    pub fn new(customer_repository: Box<dyn CustomerRepository>) -> Self {
        Self {
            customer_repository: customer_repository,
        }
    }
}

#[async_trait]
impl CustomerInteractor for CustomerInteractorImpl {
    async fn get_customers(&self, query: &GetCustomersQuery) -> Result<Vec<Customer>, DomainError> {
        match query {
            GetCustomersQuery::Email(email) => {
                let customer = self
                    .customer_repository
                    .find_customer_by_email(email)
                    .await?;
                Ok(vec![customer])
            }
        }
    }
}
