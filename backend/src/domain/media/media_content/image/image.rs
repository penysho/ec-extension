use derive_getters::Getters;

use crate::domain::{
    error::error::DomainError,
    media::{associated_id::associated_id::AssociatedId, src::src::Src},
};

pub type Id = String;

#[derive(Debug, Getters)]
pub struct Image {
    id: Id,
    associated_id: Option<AssociatedId>,
    alt: Option<String>,
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
            log::error!("Id cannot be empty");
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
