use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::domain::error::error::DomainError;

pub trait ECClientResponse: Send + Sync {}

#[automock]
#[async_trait]
pub trait ECClient: Send + Sync {
    /// Execute the query.
    async fn query<T>(&self, query: &str) -> Result<T, DomainError>
    where
        T: ECClientResponse + for<'de> Deserialize<'de> + Send + Sync + 'static;

    /// Perform mutation.
    async fn mutation<T, U>(&self, query: &str, input: &T) -> Result<U, DomainError>
    where
        T: Serialize + Send + Sync + fmt::Display + 'static,
        U: ECClientResponse + for<'de> Deserialize<'de> + Send + Sync + 'static;
}
