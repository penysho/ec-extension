use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        media::{
            media::{AssociatedId, Media, MediaStatus},
            src::src::Src,
        },
    },
    infrastructure::ec::shopify::repository::common::schema::Edges,
};

#[derive(Debug, Deserialize)]
pub(super) struct MediaSchema {
    pub(super) id: String,
    pub(super) status: String,
    pub(super) alt: Option<String>,
    pub(super) src: Option<String>,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
}

impl From<MediaNode> for MediaSchema {
    fn from(node: MediaNode) -> Self {
        MediaSchema {
            id: node.id,
            status: node.file_status,
            alt: node.alt,
            src: node.preview.and_then(|p| p.image).map(|i| i.url),
            created_at: node.created_at,
            updated_at: node.updated_at,
        }
    }
}

impl MediaSchema {
    pub(super) fn to_domain(
        self,
        associated_id: Option<AssociatedId>,
    ) -> Result<Media, DomainError> {
        let status = match self.status.as_str() {
            "UPLOADED" | "READY" => MediaStatus::Active,
            "FAILED" => MediaStatus::Inactive,
            "PROCESSING" => MediaStatus::InPreparation,
            _ => MediaStatus::Inactive,
        };

        let published_src = self.src.map(Src::new).transpose()?;

        Media::new(
            self.id,
            associated_id,
            None::<String>,
            status,
            self.alt,
            None,
            published_src,
            self.created_at,
            self.updated_at,
        )
    }

    pub(super) fn to_domains(
        schemas: Vec<MediaSchema>,
        associated_ids: Vec<Option<AssociatedId>>,
    ) -> Result<Vec<Media>, DomainError> {
        schemas
            .into_iter()
            .zip(associated_ids.into_iter())
            .map(|(schema, associated_id)| schema.to_domain(associated_id))
            .collect()
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct Image {
    pub(super) url: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct MediaPreviewImage {
    pub(super) image: Option<Image>,
}

#[derive(Debug, Deserialize)]
pub(super) struct MediaNode {
    pub(super) id: String,
    #[serde(rename = "fileStatus")]
    pub(super) file_status: String,
    pub(super) alt: Option<String>,
    pub(super) preview: Option<MediaPreviewImage>,
    #[serde(rename = "createdAt")]
    pub(super) created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub(super) updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub(super) struct MediaData {
    pub(super) files: Edges<MediaNode>,
}
