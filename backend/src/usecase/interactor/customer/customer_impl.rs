use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, error::error::DomainError, user::user::Id as UserId},
    usecase::{
        authorizer::authorizer_interface::{Action, Authorizer, Resource},
        interactor::customer_interactor_interface::{CustomerInteractor, GetCustomersQuery},
        repository::customer_repository_interface::CustomerRepository,
    },
};

/// Customer Interactor.
pub struct CustomerInteractorImpl {
    customer_repository: Box<dyn CustomerRepository>,
    authorizer: Box<dyn Authorizer>,
}

impl CustomerInteractorImpl {
    pub fn new(
        customer_repository: Box<dyn CustomerRepository>,
        authorizer: Box<dyn Authorizer>,
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
        user_id: &UserId,
        query: &GetCustomersQuery,
    ) -> Result<Vec<Customer>, DomainError> {
        self.authorizer
            .authorize(user_id, &Resource::Customer, &Action::Read)
            .await?;

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
