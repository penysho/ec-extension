use async_trait::async_trait;
use mockall::automock;

use crate::domain::{error::error::DomainError, product::product::Id as ProductId};

use crate::domain::media::media::Media;

/// Interactor interface for media.

#[automock]
#[async_trait]
pub trait MediaInteractor {
    async fn get_media_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<Media>, DomainError>;
}
