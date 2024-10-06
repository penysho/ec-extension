use chrono::{DateTime, Utc};
use derive_getters::Getters;

use super::associated_id::associated_id::AssociatedId;
use super::src::src::Src;
use crate::domain::error::error::DomainError;

pub type Id = String;

#[derive(Debug, Clone, PartialEq)]
pub enum MediaStatus {
    Active,
    Inactive,
    InPreparation,
}

/// Represents media associated with an entity such as a product or category.
///
/// The `Media` struct contains information related to media files, including
/// details such as its ID, name, status, and sources for uploaded and published media.
/// It also records the creation and last updated timestamps for the media.
///
/// # Fields
/// - `id`: The unique identifier for the media.
/// - `associated_id`: An optional ID for the entity (e.g., product) that this media is associated with.
/// - `name`: An optional name for the media file.
/// - `status`: The current status of the media (e.g., `Uploaded`, `Published`).
/// - `alt`: An optional alternative text description for accessibility purposes.
/// - `uploaded_src`: An optional source URL for the uploaded media file.
/// - `published_src`: An optional source URL for the published media file.
/// - `created_at`: The timestamp indicating when the media was created.
/// - `updated_at`: The timestamp indicating when the media was last updated.
#[derive(Debug, Getters)]
pub struct Media {
    id: Id,
    associated_id: Option<AssociatedId>,
    name: Option<String>,
    status: MediaStatus,
    alt: Option<String>,
    uploaded_src: Option<Src>,
    published_src: Option<Src>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Media {
    pub fn new(
        id: impl Into<String>,
        associated_id: Option<impl Into<AssociatedId>>,
        name: Option<impl Into<String>>,
        status: MediaStatus,
        alt: Option<impl Into<String>>,
        uploaded_src: Option<Src>,
        published_src: Option<Src>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log::error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }
        if let MediaStatus::Active = status {
            if published_src.is_none() {
                log::error!("Published src cannot be empty when status is active");
                return Err(DomainError::ValidationError);
            }
        }

        Ok(Media {
            id,
            associated_id: associated_id.map(|i| i.into()),
            name: name.map(|n| n.into()),
            status,
            alt: alt.map(|a| a.into()),
            uploaded_src,
            published_src,
            created_at,
            updated_at,
        })
    }
}
