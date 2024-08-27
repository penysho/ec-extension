use derive_getters::Getters;

use crate::domain::{error::error::DomainError, media::media::Media};

use super::category::category::CategoryId;

pub type Id = String;

#[derive(Debug)]
pub enum ProductStatus {
    Active,
    Inactive,
    Draft,
}

/// Entity of Products.
#[derive(Debug, Getters)]
pub struct Product {
    id: String,
    name: String,
    price: u32,
    description: String,
    status: ProductStatus,
    category_id: Option<CategoryId>,
    media: Vec<Media>,
}

impl Product {
    pub const MAX_DESCRIPTION_LENGTH: u32 = 10000;

    pub fn new(
        id: Id,
        name: String,
        price: u32,
        description: String,
        status: ProductStatus,
        category_id: Option<CategoryId>,
        media: Vec<Media>,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError);
        }
        if description.len() as u32 > Self::MAX_DESCRIPTION_LENGTH {
            return Err(DomainError::ValidationError);
        }

        Ok(Product {
            id,
            name,
            price,
            description,
            status,
            category_id,
            media,
        })
    }
}
