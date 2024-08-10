use crate::entity::error::error::DomainError;
use crate::entity::product::product::Product;

pub trait ProductInteractorInterface {
    async fn get_products(&self) -> Result<Vec<Product>, DomainError>;
}
