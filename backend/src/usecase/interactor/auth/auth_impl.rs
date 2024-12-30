use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, error::error::DomainError},
    usecase::{
        interactor::auth_interactor_interface::AuthInteractor,
        repository::customer_repository_interface::CustomerRepository,
    },
};

/// Auth Interactor.
pub struct AuthInteractorImpl {
    customer_repository: Box<dyn CustomerRepository>,
}

impl AuthInteractorImpl {
    pub fn new(customer_repository: Box<dyn CustomerRepository>) -> Self {
        Self {
            customer_repository: customer_repository,
        }
    }
}

#[async_trait]
impl AuthInteractor for AuthInteractorImpl {
    async fn authenticate(
        &self,
        id_token: String,
        refresh_token: String,
    ) -> Result<Customer, DomainError> {
        // TODO: implement
        unimplemented!()
    }
}
