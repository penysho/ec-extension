use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    domain::{
        error::error::DomainError,
        media::{
            associated_id::associated_id::AssociatedId,
            media::{Media, MediaStatus},
            media_content::{image::image::Image, media_content::MediaContent},
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
            "UPLOADED" | "READY" => Ok(MediaStatus::Active),
            "FAILED" => Ok(MediaStatus::Inactive),
            "PROCESSING" => Ok(MediaStatus::InPreparation),
            _ => Err(DomainError::ConversionError),
        }?;

        let image = match self.preview.and_then(|p| p.image) {
            Some(i) => Some(MediaContent::Image(i.to_domain(associated_id)?)),
            None => None,
        };

        Media::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            None::<String>,
            status,
            image,
            self.created_at,
            self.updated_at,
        )
    }

    pub fn to_domains(
        schemas: Vec<Self>,
        associated_ids: Vec<Option<AssociatedId>>,
    ) -> Result<Vec<Media>, DomainError> {
        schemas
            .into_iter()
            .zip(associated_ids.into_iter())
            .map(|(schema, associated_id)| schema.to_domain(associated_id))
            .collect()
    }
}

impl ImageNode {
    pub fn to_domain(self, associated_id: Option<AssociatedId>) -> Result<Image, DomainError> {
        let src = Src::new(self.url)?;

        Image::new(
            ShopifyGQLQueryHelper::remove_gid_prefix(&self.id),
            associated_id,
            self.alt_text,
            None,
            Some(src),
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct MediaData {
    pub files: Edges<MediaNode>,
}

#[allow(dead_code)]
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
pub struct MediaPreviewImage {
    pub image: Option<ImageNode>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ImageNode {
    pub id: String,
    #[serde(rename = "altText")]
    pub alt_text: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub url: String,
}
