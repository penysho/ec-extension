use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaSchema {
    pub id: String,
    pub name: Option<String>,
    pub status: MediaStatusEnum,
    pub alt: Option<String>,
    pub src: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MediaStatusEnum {
    Active,
    Inactive,
    InPreparation,
}
