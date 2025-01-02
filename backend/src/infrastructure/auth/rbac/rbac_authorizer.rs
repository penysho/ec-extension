use async_trait::async_trait;

use crate::{
    domain::error::error::DomainError,
    infrastructure::auth::{
        authorizer_interface::{Action, Authorizer, Resource},
        idp_user::IdpUser,
    },
};

/// Authorization by RBAC.
pub struct RbacAuthorizer {}

impl RbacAuthorizer {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Authorizer for RbacAuthorizer {
    async fn authorize(
        &self,
        user: &IdpUser,
        resource: &Resource,
        action: &Action,
    ) -> Result<(), DomainError> {
        todo!()
    }
}
