use chrono::{DateTime, Utc};
use derive_getters::Getters;

use crate::domain::error::error::DomainError;

use super::src::src::Src;

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
            return Err(DomainError::ValidationError);
        }
        if let MediaStatus::InPreparation = status {
            if uploaded_src.is_none() {
                return Err(DomainError::ValidationError);
            }
        }
        if let MediaStatus::Active = status {
            if published_src.is_none() {
                return Err(DomainError::ValidationError);
            }
        }

        Ok(Media {
            id,
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
