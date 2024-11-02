use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, location::location::Location},
    usecase::{
        interactor::location_interactor_interface::LocationInteractor,
        repository::location_repository_interface::LocationRepository,
    },
};

/// Location Interactor.
pub struct LocationInteractorImpl {
    location_repository: Box<dyn LocationRepository>,
}

impl LocationInteractorImpl {
    pub fn new(location_repository: Box<dyn LocationRepository>) -> Self {
        Self {
            location_repository,
        }
    }
}

#[async_trait]
impl LocationInteractor for LocationInteractorImpl {
    async fn get_locations(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<Vec<Location>, DomainError> {
        self.location_repository.find_locations(limit, offset).await
    }
}
