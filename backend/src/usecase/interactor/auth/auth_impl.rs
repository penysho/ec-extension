use async_trait::async_trait;

use crate::{
    domain::{customer::customer::Customer, email::email::Email, error::error::DomainError},
    infrastructure::auth::authenticator_interface::Authenticator,
    usecase::{
        interactor::auth_interactor_interface::AuthInteractor,
        repository::customer_repository_interface::CustomerRepository,
    },
};

/// Auth Interactor.
///
pub struct AuthInteractorImpl<A, C>
where
    A: Authenticator,
    C: CustomerRepository,
{
    authenticator: A,
    customer_repository: C,
}

impl<A, C> AuthInteractorImpl<A, C>
where
    A: Authenticator,
    C: CustomerRepository,
{
    pub fn new(authenticator: A, customer_repository: C) -> Self {
        Self {
            authenticator,
            customer_repository,
        }
    }
}

#[async_trait]
impl<A, C> AuthInteractor for AuthInteractorImpl<A, C>
where
    A: Authenticator,
    C: CustomerRepository,
{
    async fn authenticate(
        &self,
        id_token: &Option<String>,
        refresh_token: &Option<String>,
    ) -> Result<(Customer, String), DomainError> {
        let (idp_user, new_id_token) = self
            .authenticator
            .clone()
            .validate_token(id_token.as_deref(), refresh_token.as_deref())
            .await?;

        let customer = self
            .customer_repository
            .find_customer_by_email(&Email::new(idp_user.email)?)
            .await?;

        Ok((customer, new_id_token))
    }
}
