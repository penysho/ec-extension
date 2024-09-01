use derive_getters::Getters;

use crate::domain::error::error::DomainError;

pub type Id = String;

#[derive(Debug, Clone)]
pub enum MediaStatus {
    Active,
    Inactive,
    InPreparation,
}

/// Entity of Media.
#[derive(Debug, Getters, Clone)]
pub struct Media {
    id: Id,
    name: String,
    status: MediaStatus,
    upload_src: Option<String>,
}

impl Media {
    pub fn new(
        id: Id,
        name: String,
        status: MediaStatus,
        upload_src: Option<String>,
    ) -> Result<Self, DomainError> {
        if let MediaStatus::InPreparation = status {
            if upload_src.is_none() {
                return Err(DomainError::ValidationError);
            }
        }

        Ok(Media {
            id,
            name,
            status,
            upload_src,
        })
    }
}
