use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        media::{
            associated_id::associated_id::AssociatedId,
            media::{Media, MediaStatus},
            src::src::Src,
        },
    },
    infrastructure::ec::shopify::{
        query_helper::ShopifyGQLQueryHelper, repository::schema::common::Edges,
    },
};

impl MediaNode {
    pub fn to_domain(self, associated_id: Option<AssociatedId>) -> Result<Media, DomainError> {
        let status = match self.file_status.as_str() {
            "UPLOADED" | "READY" => MediaStatus::Active,
            "FAILED" => MediaStatus::Inactive,
            "PROCESSING" => MediaStatus::InPreparation,
            _ => MediaStatus::Inactive,
        };

        let published_src = self
            .preview
            .and_then(|p| p.image)
            .map(|i| i.url)
            .map(Src::new)
            .transpose()?;

        Media::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
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

    pub fn to_domains(
        schemas: Vec<MediaNode>,
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
pub struct Image {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct MediaPreviewImage {
    pub image: Option<Image>,
}

#[derive(Debug, Deserialize)]
pub struct MediaNode {
    pub id: String,
    #[serde(rename = "fileStatus")]
    pub file_status: String,
    pub alt: Option<String>,
    pub preview: Option<MediaPreviewImage>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct MediaData {
    pub files: Edges<MediaNode>,
}
