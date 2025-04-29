use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;

use crate::domain::authorized_resource::authorized_resource::{AuthorizedResource, ResourceAction};
use crate::domain::error::error::DomainError;
use crate::domain::user::user::UserInterface;

/// Authorization interface.
#[automock]
#[async_trait]
pub trait Authorizer: Send + Sync {
    /// Authorize the given request.
    ///
    /// # Arguments
    ///
    /// * `user` - Users who manipulate resources.
    /// * `resources` - Vector of the resource to be manipulated.
    /// * `action` - Action on Resources.
    ///
    /// # Returns
    ///
    /// * `Result<(), DomainError>` - The result of the operation.
    ///   - `Ok()` - That the resource operation was authorized.
    ///   - `Err(DomainError)` - The error.
    ///
    /// # Errors
    ///
    /// * If do not hold authorization to the subject.
    async fn authorize<'a>(
        &self,
        user: Arc<dyn UserInterface>,
        resources: Vec<&'a dyn AuthorizedResource>,
        action: &ResourceAction,
    ) -> Result<(), DomainError>;
}
