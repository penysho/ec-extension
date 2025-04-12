use derive_getters::Getters;

use crate::{
    domain::{
        error::error::DomainError,
        media::{associated_id::associated_id::AssociatedId, src::src::Src},
    },
    log_error,
};

pub type Id = String;

/// Represents an image associated with an entity, typically used in the context of product
/// or media-related operations.
///
/// This struct provides information about the image, such as its unique identifier (`id`),
/// an optional `associated_id` to indicate any linked entities, an optional `alt` text for
/// accessibility purposes, and two potential source URLs: `uploaded_src` for the initially
/// uploaded image source and `published_src` for the published image URL.
///
/// # Fields
///
/// * `id` - A unique identifier for the image. This field is required and cannot be empty.
/// * `associated_id` - An optional identifier for an entity associated with this image (e.g., product or user).
/// * `alt` - Optional alternative text for the image, typically used for accessibility purposes.
/// * `uploaded_src` - A URL to the image as originally uploaded. This field is currently not in use, as the
///                    image registration endpoint has not been implemented.
/// * `published_src` - The URL to the published version of the image.
#[derive(Debug, Getters)]
pub struct Image {
    id: Id,
    associated_id: Option<AssociatedId>,
    alt: Option<String>,
    // Currently not in use because the image registration endpoint is not implemented.
    #[allow(dead_code)]
    uploaded_src: Option<Src>,
    published_src: Option<Src>,
}

impl Image {
    pub fn new(
        id: impl Into<String>,
        associated_id: Option<impl Into<AssociatedId>>,
        alt: Option<impl Into<String>>,
        uploaded_src: Option<Src>,
        published_src: Option<Src>,
    ) -> Result<Self, DomainError> {
        let id = id.into();
        if id.is_empty() {
            log_error!("Id cannot be empty");
            return Err(DomainError::ValidationError);
        }

        Ok(Self {
            id,
            associated_id: associated_id.map(|i| i.into()),
            alt: alt.map(|a| a.into()),
            uploaded_src,
            published_src,
        })
    }
}
