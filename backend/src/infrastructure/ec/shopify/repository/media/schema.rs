use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        media::{
            media::{Media, MediaStatus},
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
    pub(super) fn to_domain(self) -> Result<Media, DomainError> {
        let status = match self.status.as_str() {
            "UPLOADED" => MediaStatus::Active,
            "FAILED" => MediaStatus::Inactive,
            "READY" => MediaStatus::InPreparation,
            "PROCESSING" => MediaStatus::InPreparation,
            _ => MediaStatus::Inactive,
        };
        let published_src = match self.src {
            Some(src) => Some(Src::new(src)?),
            None => None,
        };

        Media::new(
            self.id,
            None::<String>,
            status,
            self.alt,
            None,
            published_src,
            self.created_at,
            self.updated_at,
        )
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
