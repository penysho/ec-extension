use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, media::media::Media, product::product::Id as ProductId},
    usecase::{
        interactor::media_interactor_interface::MediaInteractor,
        repository::media_repository_interface::MediaRepository,
    },
};

/// Media Interactor.
pub struct MediaInteractorImpl {
    media_repository: Box<dyn MediaRepository>,
}

impl MediaInteractorImpl {
    pub fn new(media_repository: Box<dyn MediaRepository>) -> Self {
        Self {
            media_repository: media_repository,
        }
    }
}

#[async_trait]
impl MediaInteractor for MediaInteractorImpl {
    async fn get_media_by_product_id(
        &self,
        product_id: &ProductId,
    ) -> Result<Vec<Media>, DomainError> {
        self.media_repository
            .find_media_by_product_id(product_id)
            .await
    }
}
