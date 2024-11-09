use crate::domain::media::{
    media::{Media, MediaStatus},
    media_content::media_content::MediaContent,
};

use super::schema::{MediaSchema, MediaStatusEnum};

impl From<Media> for MediaSchema {
    fn from(media: Media) -> Self {
        let image = match media.content() {
            Some(MediaContent::Image(image)) => Some(image),
            None => None,
        };

        MediaSchema {
            id: media.id().to_string(),
            name: media.name().to_owned(),
            status: match media.status() {
                MediaStatus::Active => MediaStatusEnum::Active,
                MediaStatus::Inactive => MediaStatusEnum::Inactive,
                MediaStatus::InPreparation => MediaStatusEnum::InPreparation,
            },
            alt: image.and_then(|image| image.alt().to_owned()),
            src: image
                .and_then(|image| image.published_src().to_owned())
                .map(|src| src.value().to_owned()),
            created_at: media.created_at().to_owned(),
            updated_at: media.updated_at().to_owned(),
        }
    }
}
