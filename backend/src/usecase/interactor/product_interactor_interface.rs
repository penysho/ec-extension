use crate::domain::error::error::DomainError;
use crate::domain::media::media::Media;
use crate::domain::product::product::{Id as ProductId, Product};
use async_trait::async_trait;
use mockall::automock;

/// Interactor interface for products.
#[automock]
#[async_trait]
pub trait ProductInteractor {
    /// Get detailed product information.
    ///
    /// # Arguments
    ///
    /// * `id` - Product ID
    ///
    /// # Returns
    ///
    /// * `Result<(Product, Vec<Media>), DomainError>` - Product and its media
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the media repository fails.
    async fn get_product_with_media(
        &self,
        id: &ProductId,
    ) -> Result<(Product, Vec<Media>), DomainError>;

    /// Get a list of products.
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of products to return
    /// * `offset` - Number of products to skip
    ///
    /// # Returns
    ///
    /// * `Result<(Vec<Product>, Vec<Media>), DomainError>` - List of products and their media
    ///
    /// # Errors
    ///
    /// * Returns a domain error if the product or media repository fails.
    async fn get_products_with_media(
        &self,
        limit: &Option<u32>,
        offset: &Option<u32>,
    ) -> Result<(Vec<Product>, Vec<Media>), DomainError>;
}
