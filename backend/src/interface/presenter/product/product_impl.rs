use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{error::error::DomainError, product::product::Product},
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
        result: Result<Option<Product>, DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductResponseError> {
        match result {
            Ok(Some(product)) => Ok(web::Json(GetProductResponse {
                product: ProductSchema::from(product),
            })),
            Ok(None) => Err(GetProductResponseError::ProductNotFound),
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
                    .map(|product| ProductSchema::from(product))
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
    use crate::domain::{
        media::media::{Media, MediaStatus},
        product::{barcode::barcode::Barcode, product::ProductStatus, sku::sku::Sku},
    };

    use super::*;

    fn mock_product() -> Product {
        Product::new(
            "gid://shopify/Product/1".to_string(),
            "Test Product",
            100,
            "This is a test product description.",
            ProductStatus::Active,
            Some(Sku::new("TESTSKU123")),
            Some(Barcode::new("123456789012")),
            Some(50),
            1,
            Some("gid://shopify/Category/111".to_string()),
        )
        .unwrap()
    }

    fn mock_media() -> Media {
        Media::new(
            "1".to_string(),
            "Test Media".to_string(),
            MediaStatus::Active,
            Some("https://example.com/image.jpg".to_string()),
        )
        .unwrap()
    }

    #[actix_web::test]
    async fn test_present_get_product_success() {
        let presenter = ProductPresenterImpl::new();
        let product = mock_product();

        let result = presenter
            .present_get_product(Ok(Some(product)))
            .await
            .unwrap();

        assert_eq!(result.product.name, "Test Product");
        assert_eq!(result.product.price, 100);
        assert_eq!(result.product.description, "Description");
    }

    #[actix_web::test]
    async fn test_present_get_product_not_found() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter.present_get_product(Ok(None)).await;

        assert!(matches!(
            result,
            Err(GetProductResponseError::ProductNotFound)
        ));
    }

    #[actix_web::test]
    async fn test_present_get_product_error() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_product(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetProductResponseError::ServiceUnavailable)
        ));
    }

    fn mock_products() -> Vec<Product> {
        vec![
            Product::new(
                "gid://shopify/Product/1".to_string(),
                "Test Product",
                100,
                "This is a test product description.",
                ProductStatus::Active,
                Some(Sku::new("TESTSKU123")),
                Some(Barcode::new("123456789012")),
                Some(50),
                1,
                Some("gid://shopify/Category/111".to_string()),
            )
            .unwrap(),
            Product::new(
                "gid://shopify/Product/2".to_string(),
                "Test Product",
                100,
                "This is a test product description.",
                ProductStatus::Active,
                Some(Sku::new("TESTSKU123")),
                Some(Barcode::new("123456789012")),
                Some(50),
                1,
                Some("gid://shopify/Category/111".to_string()),
            )
            .unwrap(),
        ]
    }

    #[actix_web::test]
    async fn test_present_get_products_success() {
        let presenter = ProductPresenterImpl::new();
        let products = mock_products();

        let result = presenter
            .present_get_products(Ok(products))
            .await
            .unwrap()
            .into_inner();

        assert_eq!(result.products.len(), 2);
        assert_eq!(result.products[0].name, "Test Product 1");
        assert_eq!(result.products[1].name, "Test Product 2");
        assert_eq!(result.products[0].price, 100);
        assert_eq!(result.products[1].price, 200);
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
