use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, media::media::Media, product::product::Product},
    interface::presenter::{
        product::schema::{
            GetProductResponse, GetProductResponseError, GetProductsResponse,
            GetProductsResponseError, ProductSchema,
        },
        product_presenter_interface::ProductPresenter,
    },
};

/// Generate a response schema for the product
pub struct ProductPresenterImpl;
impl ProductPresenterImpl {
    pub fn new() -> Self {
        ProductPresenterImpl
    }
}

#[async_trait]
impl ProductPresenter for ProductPresenterImpl {
    type GetProductResponse = Json<GetProductResponse>;
    type GetProductResponseError = GetProductResponseError;
    /// Generate a response with detailed product information.
    async fn present_get_product(
        &self,
        product_result: Result<Product, DomainError>,
        media_result: Result<Vec<Media>, DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductResponseError> {
        let media = match media_result {
            Ok(media) => media,
            Err(DomainError::NotFound) => vec![],
            Err(_) => return Err(GetProductResponseError::ServiceUnavailable),
        };
        match product_result {
            Ok(product) => Ok(web::Json(GetProductResponse {
                product: ProductSchema::to_response(product, media),
            })),
            Err(DomainError::NotFound) => Err(GetProductResponseError::ProductNotFound),
            Err(_) => Err(GetProductResponseError::ServiceUnavailable),
        }
    }

    type GetProductsResponse = Json<GetProductsResponse>;
    type GetProductsResponseError = GetProductsResponseError;
    /// Generate a response for the product list.
    async fn present_get_products(
        &self,
        result: Result<Vec<Product>, DomainError>,
    ) -> Result<Self::GetProductsResponse, Self::GetProductsResponseError> {
        match result {
            Ok(products) => {
                let product_schemas: Vec<ProductSchema> = products
                    .into_iter()
                    .map(|product| ProductSchema::to_response(product, vec![]))
                    .collect();

                Ok(web::Json(GetProductsResponse {
                    products: product_schemas,
                }))
            }
            Err(_) => Err(GetProductsResponseError::ServiceUnavailable),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::domain::{
        media::{
            media::{Media, MediaStatus},
            src::src::Src,
        },
        product::{
            product::ProductStatus,
            variant::{barcode::barcode::Barcode, sku::sku::Sku, variant::Variant},
        },
    };

    use super::*;

    fn mock_products(count: usize) -> Vec<Product> {
        (0..count)
            .map(|i| {
                Product::new(
                    format!("gid://shopify/Product/{i}"),
                    format!("Test Product {i}"),
                    "This is a test product description.",
                    ProductStatus::Active,
                    vec![Variant::new(
                        "gid://shopify/ProductVariant/1".to_string(),
                        Some("Test Variant 1"),
                        100,
                        Some(Sku::new("TESTSKU123").unwrap()),
                        Some(Barcode::new("123456789012").unwrap()),
                        Some(50),
                        1,
                        Utc::now(),
                        Utc::now(),
                    )
                    .unwrap()],
                    Some("gid://shopify/Category/111".to_string()),
                )
                .unwrap()
            })
            .collect()
    }

    fn mock_media(count: usize) -> Vec<Media> {
        (0..count)
            .map(|i| {
                Media::new(
                    format!("gid://shopify/ProductMedia/{}", i),
                    Some(format!("Test Media {}", i)),
                    MediaStatus::Active,
                    Some(format!("gid://shopify/Product/{}", i)),
                    Some(Src::new(format!("https://example.com/uploaded{}.jpg", i)).unwrap()),
                    Some(Src::new(format!("https://example.com/published{}.jpg", i)).unwrap()),
                    Utc::now(),
                    Utc::now(),
                )
                .unwrap()
            })
            .collect()
    }

    #[actix_web::test]
    async fn test_present_get_product_success() {
        let presenter = ProductPresenterImpl::new();
        let product = mock_products(1)[0].clone();
        let media = mock_media(5);

        let result = presenter
            .present_get_product(Ok(product), Ok(media))
            .await
            .unwrap();

        assert_eq!(result.product.name, "Test Product 0");
        assert_eq!(
            result.product.description,
            "This is a test product description."
        );
    }

    #[actix_web::test]
    async fn test_present_get_product_not_found() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_product(Err(DomainError::NotFound), Err(DomainError::NotFound))
            .await;

        assert!(matches!(
            result,
            Err(GetProductResponseError::ProductNotFound)
        ));
    }

    #[actix_web::test]
    async fn test_present_get_product_error() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_product(Err(DomainError::SystemError), Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetProductResponseError::ServiceUnavailable)
        ));
    }

    #[actix_web::test]
    async fn test_present_get_products_success() {
        let presenter = ProductPresenterImpl::new();
        let products = mock_products(5);

        let result = presenter
            .present_get_products(Ok(products))
            .await
            .unwrap()
            .into_inner();

        assert_eq!(result.products.len(), 5);
        assert_eq!(result.products[0].name, "Test Product 0");
        assert_eq!(result.products[4].name, "Test Product 4");
    }

    #[actix_web::test]
    async fn test_present_get_products_error() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_products(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetProductsResponseError::ServiceUnavailable)
        ));
    }
}
