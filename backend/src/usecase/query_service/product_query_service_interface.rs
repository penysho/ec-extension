use async_trait::async_trait;

use crate::domain::{error::error::DomainError, product::category::category::Id};

use super::dto::product::ProductDTO;

pub struct RelatedProductFilter {
    pub category_id: Id,
}

#[async_trait]
pub trait ProductQueryService: Send + Sync {
    /// Obtains a list of related products for a specified product.
    async fn search_related_products(
        &self,
        filter: &RelatedProductFilter,
    ) -> Result<Vec<ProductDTO>, DomainError>;
}
