use derive_getters::Getters;

use crate::entity::{error::error::DomainError, media::media::Media};

pub type Id = String;

#[derive(Debug)]
pub enum ProductStatus {
    Active,
    Inactive,
    Draft,
}

#[derive(Debug)]
pub enum ProductCategory {
    Tops,
    Bottoms,
    Shoes,
    Accessories,
    Other,
}

/// Entity of Products.
#[derive(Debug, Getters)]
pub struct Product {
    id: String,
    name: String,
    price: u32,
    description: String,
    status: ProductStatus,
    category: ProductCategory,
    media: Vec<Media>,
}

impl Product {
    pub fn new(
        id: Id,
        name: String,
        price: u32,
        description: String,
        status: ProductStatus,
        category: ProductCategory,
        media: Vec<Media>,
    ) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::ValidationError);
        }

        Ok(Product {
            id,
            name,
            price,
            description,
            status,
            category,
            media,
        })
    }
}
