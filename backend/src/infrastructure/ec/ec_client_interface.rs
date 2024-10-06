use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::domain::error::error::DomainError;

pub trait ECClientResponse: Send + Sync {}

#[automock]
#[async_trait]
pub trait ECClient: Send + Sync {
    async fn query<T>(&self, query: &str) -> Result<T, DomainError>
    where
        T: ECClientResponse + for<'de> Deserialize<'de> + Send + Sync + 'static;

    async fn mutation<T, U>(&self, query: &str, input: &T) -> Result<U, DomainError>
    where
        T: Serialize + Send + Sync + 'static,
        U: ECClientResponse + for<'de> Deserialize<'de> + Send + Sync + 'static;
}
