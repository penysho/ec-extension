use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};

use crate::domain::error::error::DomainError;

pub trait ECClientResponse: Send + Sync {}

#[automock]
#[async_trait]
pub trait ECClient: Send + Sync {
    async fn query<T, U>(&self, query: &T) -> Result<U, DomainError>
    where
        T: Serialize + Send + Sync + 'static,
        U: ECClientResponse + for<'de> Deserialize<'de> + Send + Sync + 'static;

    async fn mutation<T>(&self, query: &str, input: &T) -> Result<(), DomainError>
    where
        T: Serialize + Send + Sync + 'static;
}
