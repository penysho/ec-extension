use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Image {
    #[serde(rename = "altText")]
    pub(super) alt_text: Option<String>,
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
    pub(super) alt: String,
    pub(super) preview: Option<MediaPreviewImage>,
    #[serde(rename = "createdAt")]
    pub(super) created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub(super) updated_at: DateTime<Utc>,
}
