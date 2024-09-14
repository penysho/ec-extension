use async_trait::async_trait;

use crate::domain::{
    error::error::DomainError, media::media::Media, product::product::Id as ProductId,
};

#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn get_media_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<Media>, DomainError>;

    async fn get_media_by_product_ids(
        &self,
        id: Vec<&ProductId>,
    ) -> Result<Vec<Media>, DomainError>;
}
