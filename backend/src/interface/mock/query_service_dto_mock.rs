/// Define functions for common use in interface layer tests.
/// Generate a mock of the dto for query service.
use crate::usecase::query_service::dto::product::ProductDTO;

pub fn mock_products_dto(count: usize) -> Vec<ProductDTO> {
    (0..count)
        .map(|i| ProductDTO {
            id: format!("product-{}", i),
            name: format!("Product {}", i),
            handle: format!("product-{}", i),
            vendor: "Vendor".to_string(),
            price: 100.0,
            featured_media_url: Some("https://example.com/image.jpg".to_string()),
        })
        .collect()
}
