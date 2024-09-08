use crate::domain::error::error::DomainError;
use async_trait::async_trait;
use mockall::automock;

/// Interactor interface for media.

#[automock]
#[async_trait]
pub trait MediaInteractor {
    async fn get_media_by_product_id(
        &self,
        product_id: crate::domain::product::product::Id,
    ) -> Result<Vec<crate::domain::media::media::Media>, DomainError>;
}
