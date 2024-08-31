use async_trait::async_trait;
use reqwest::Response;
use serde::Serialize;

use crate::domain::error::error::DomainError;

#[async_trait]
pub trait ECClient: Send + Sync {
    async fn query<T>(&self, query: &T) -> Result<Response, DomainError>
    where
        T: Serialize + ?Sized + Send + Sync + 'async_trait;
}
