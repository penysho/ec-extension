use crate::entity::{error::error::DomainError, product::product::Product};

pub trait ProductPresenter<T, E> {
    async fn present_get_products(&self, result: Result<Product, DomainError>) -> Result<T, E>;
}
