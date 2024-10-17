use async_trait::async_trait;
use mockall::automock;

use crate::domain::{error::error::DomainError, product::product::Id as ProductId};

use crate::domain::media::media::Media;

/// Interactor interface for media.
#[automock]
#[async_trait]
pub trait MediaInteractor {
    /// Get a list of media by product id.
    ///
    /// # Arguments
    ///
    /// * `product_id` - The product id.
    ///
    /// # Returns
    ///
    /// A list of media.
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the media repository fails.
    async fn get_media_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<Media>, DomainError>;
}
