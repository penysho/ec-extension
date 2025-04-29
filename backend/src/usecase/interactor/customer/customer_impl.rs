use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        authorized_resource::authorized_resource::ResourceAction, customer::customer::Customer,
        error::error::DomainError, user::user::UserInterface,
    },
    usecase::{
        auth::authorizer_interface::Authorizer,
        interactor::customer_interactor_interface::{CustomerInteractor, GetCustomersQuery},
        repository::customer_repository_interface::CustomerRepository,
    },
};

/// Customer Interactor.
pub struct CustomerInteractorImpl {
    customer_repository: Box<dyn CustomerRepository>,
    authorizer: Arc<dyn Authorizer>,
}

impl CustomerInteractorImpl {
    pub fn new(
        customer_repository: Box<dyn CustomerRepository>,
        authorizer: Arc<dyn Authorizer>,
    ) -> Self {
        Self {
            customer_repository,
            authorizer,
        }
    }
}

#[async_trait]
impl CustomerInteractor for CustomerInteractorImpl {
    async fn get_customers(
        &self,
        user: Arc<dyn UserInterface>,
        query: &GetCustomersQuery,
    ) -> Result<Vec<Customer>, DomainError> {
        match query {
            GetCustomersQuery::Email(email) => {
                let customer = self
                    .customer_repository
                    .find_customer_by_email(email)
                    .await?;

                self.authorizer
                    .authorize(user, vec![&customer], &ResourceAction::Read)
                    .await?;
                Ok(vec![customer])
            }
        }
    }
}
