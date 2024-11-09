use crate::domain::media::{
    media::{Media, MediaStatus},
    media_content::{image::image::Image, media_content::MediaContent},
};

use super::schema::{ImageSchema, MediaContentSchema, MediaSchema, MediaStatusEnum};

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
            content: Some(MediaContentSchema {
                image: image.map(|image| ImageSchema::from(image)),
            }),
            created_at: media.created_at().to_owned(),
            updated_at: media.updated_at().to_owned(),
        }
    }
}

impl From<&Image> for ImageSchema {
    fn from(image: &Image) -> Self {
        ImageSchema {
            id: image.id().to_string(),
            alt: image.alt().to_owned(),
            src: image
                .published_src()
                .to_owned()
                .map(|src| src.value().to_string()),
        }
    }
}
