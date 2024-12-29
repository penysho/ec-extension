use futures_util::future::LocalBoxFuture;

use crate::domain::error::error::DomainError;

pub trait Authenticator: Send + Sync + Clone {
    fn validate_token(&self, token: String) -> LocalBoxFuture<'static, Result<(), DomainError>>;
}
