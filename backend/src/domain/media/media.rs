use chrono::{DateTime, Utc};
use derive_getters::Getters;

use super::src::src::Src;
use crate::domain::error::error::DomainError;
use crate::domain::product::product::Id as ProductId;

pub type Id = String;

#[derive(Debug, Clone, PartialEq)]
pub enum MediaStatus {
    Active,
    Inactive,
    InPreparation,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AssociatedId {
    Product(ProductId),
}

/// Entity of Media.
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
            return Err(DomainError::ValidationError);
        }
        if let MediaStatus::Active = status {
            if published_src.is_none() {
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
