use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaSchema {
    pub id: String,
    pub name: Option<String>,
    pub status: MediaStatusEnum,
    pub content: Option<MediaContentSchema>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaStatusEnum {
    Active,
    Inactive,
    InPreparation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaContentSchema {
    pub image: Option<ImageSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSchema {
    pub id: String,
    pub alt: Option<String>,
    pub src: Option<String>,
}
