use async_trait::async_trait;

use crate::domain::error::error::DomainError;

#[async_trait]
pub trait Authenticator: Send + Sync {
    /// Validate tokens issued by Idp.
    async fn validate_token(&self, token: String) -> Result<(), DomainError>;
}
