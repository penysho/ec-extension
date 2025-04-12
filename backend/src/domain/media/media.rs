use chrono::{DateTime, Utc};
use derive_getters::Getters;

use super::media_content::media_content::MediaContent;
use crate::{domain::error::error::DomainError, log_error};

pub type Id = String;

#[derive(Debug, Clone, PartialEq)]
pub enum MediaStatus {
    Active,
    Inactive,
    InPreparation,
}

/// Represents media associated with an entity such as a product or customer.
///
/// Hold multiple media contents such as videos and images by content field.
///
/// # Fields
/// - `id` - The unique identifier for the media.
/// - `name` - An optional name for the media file.
/// - `status` - The current status of the media (e.g., `Active`, `Inactive`, `InPreparation`).
/// - `content` - The media content, which can be an image, video, or other media types.
/// - `created_at` - The timestamp indicating when the media was created.
/// - `updated_at` - The timestamp indicating when the media was last updated.
#[derive(Debug, Getters)]
pub struct Media {
    id: Id,
    name: Option<String>,
    status: MediaStatus,
    content: Option<MediaContent>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Media {
    pub fn new(
        id: impl Into<Id>,
        name: Option<impl Into<String>>,
        status: MediaStatus,
        content: Option<MediaContent>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log_error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }

        Ok(Media {
            id,
            name: name.map(|n| n.into()),
            status,
            content,
            created_at,
            updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::media::{
        associated_id::associated_id::AssociatedId, media_content::image::image::Image,
        src::src::Src,
    };

    use super::*;
    use chrono::Utc;

    #[test]
    fn test_new_media() {
        let id = "media_id";
        let name = Some("media_name".to_string());
        let status = MediaStatus::Active;
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let associated_id = Some(AssociatedId::Product("product_id".to_string()));
        let alt = Some("alternative_text".to_string());
        let uploaded_src = Some(Src::new("https://example.com/uploaded.jpg").unwrap());
        let published_src = Some(Src::new("https://example.com/published.jpg").unwrap());
        let content = Some(MediaContent::Image(
            Image::new(
                "image_id".to_string(),
                associated_id.to_owned(),
                alt.to_owned(),
                uploaded_src.to_owned(),
                published_src.to_owned(),
            )
            .unwrap(),
        ));

        let media = Media::new(
            id,
            name.to_owned(),
            status.to_owned(),
            content,
            created_at,
            updated_at,
        );

        assert!(media.is_ok());
        let media = media.unwrap();
        assert_eq!(media.id(), id);
        assert_eq!(media.name(), &name);
        assert_eq!(media.status(), &status);
        assert_eq!(media.created_at(), &created_at);
        assert_eq!(media.updated_at(), &updated_at);

        let image = match &media.content() {
            Some(MediaContent::Image(image)) => image,
            _ => panic!("Expected MediaContent::Image"),
        };
        assert_eq!(image.id(), "image_id");
        assert_eq!(image.associated_id(), &associated_id);
        assert_eq!(image.alt(), &alt);
        assert_eq!(image.uploaded_src(), &uploaded_src);
        assert_eq!(image.published_src(), &published_src);
    }

    #[test]
    fn test_new_media_invalid_id() {
        let media = Media::new(
            "",
            Some("media_name".to_string()),
            MediaStatus::Active,
            None,
            Utc::now(),
            Utc::now(),
        );
        assert!(media.is_err());
    }
}
