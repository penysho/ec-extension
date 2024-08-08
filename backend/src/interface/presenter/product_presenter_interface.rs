use crate::entity::{error::error::DomainError, product::product::Product};

pub trait ProductPresenter<T, E> {
    async fn present_get_products(
        &self,
        result: Result<Option<Product>, DomainError>,
    ) -> Result<T, E>;
}
