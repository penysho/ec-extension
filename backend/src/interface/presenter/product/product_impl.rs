use std::collections::HashMap;

use actix_web::web::{self, Json};
use async_trait::async_trait;

use crate::{
    domain::{
        error::error::DomainError,
        media::{
            associated_id::associated_id::AssociatedId, media::Media,
            media_content::media_content::MediaContent,
        },
        product::product::Product,
    },
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
        result: Result<(Product, Vec<Media>), DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductResponseError> {
        let (product, media) = match result {
            Ok((product, media)) => (product, media),
            Err(DomainError::NotFound) => return Err(GetProductResponseError::NotFound),
            Err(DomainError::ValidationError) => return Err(GetProductResponseError::BadRequest),
            Err(_) => return Err(GetProductResponseError::ServiceUnavailable),
        };

        for medium in media.iter() {
            let image = match medium.content() {
                Some(MediaContent::Image(image)) => image,
                None => continue,
            };

            let _ = match image.associated_id() {
                Some(id) if id.clone() != AssociatedId::Product(product.id().clone()) => {
                    Err(DomainError::SystemError)
                }
                None => Err(DomainError::SystemError),
                _ => Ok(medium),
            };
        }

        let schema = ProductSchema::to_schema(product, media);
        Ok(web::Json(GetProductResponse { product: schema }))
    }

    type GetProductsResponse = Json<GetProductsResponse>;
    type GetProductsResponseError = GetProductsResponseError;
    /// Generate a response for the product list.
    async fn present_get_products(
        &self,
        result: Result<(Vec<Product>, Vec<Media>), DomainError>,
    ) -> Result<Self::GetProductsResponse, Self::GetProductsResponseError> {
        let (products, media) = match result {
            Ok((products, media)) => (products, media),
            Err(DomainError::ValidationError) => return Err(GetProductsResponseError::BadRequest),
            Err(_) => return Err(GetProductsResponseError::ServiceUnavailable),
        };
        if products.is_empty() {
            return Err(GetProductsResponseError::NotFound);
        }

        let mut media_map: HashMap<AssociatedId, Vec<Media>> =
            media.into_iter().fold(HashMap::new(), |mut accum, medium| {
                let image = match medium.content() {
                    Some(MediaContent::Image(image)) => Some(image),
                    None => None,
                };
                if image.is_none() {
                    return accum;
                }

                if let Some(associated_id) = image.unwrap().associated_id() {
                    accum
                        .entry(associated_id.to_owned())
                        .or_insert_with(Vec::new)
                        .push(medium);
                }
                accum
            });

        let product_schemas: Vec<ProductSchema> = products
            .into_iter()
            .map(|product| {
                let media = media_map
                    .remove(&AssociatedId::Product(product.id().to_owned()))
                    .unwrap_or_else(Vec::new);

                ProductSchema::to_schema(product, media)
            })
            .collect();

        Ok(web::Json(GetProductsResponse {
            products: product_schemas,
        }))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::domain::{
        media::{
            media::{Media, MediaStatus},
            media_content::image::image::Image,
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
                    format!("{i}"),
                    format!("Test Product {i}"),
                    "This is a test product description.",
                    ProductStatus::Active,
                    vec![Variant::new(
                        format!("{i}"),
                        Some(format!("Test Variant {i}")),
                        100,
                        Some(Sku::new("TESTSKU123").unwrap()),
                        Some(Barcode::new("123456789012").unwrap()),
                        Some(50),
                        1,
                        Utc::now(),
                        Utc::now(),
                    )
                    .unwrap()],
                    Some("111"),
                )
                .unwrap()
            })
            .collect()
    }

    fn mock_media(count: usize) -> Vec<Media> {
        (0..count)
            .map(|i| {
                Media::new(
                    format!("{i}"),
                    Some(format!("Test Media {i}")),
                    MediaStatus::Active,
                    Some(MediaContent::Image(
                        Image::new(
                            format!("{i}"),
                            Some(AssociatedId::Product(format!("{i}"))),
                            Some(format!("Alt Text {i}")),
                            Some(
                                Src::new(format!("https://example.com/uploaded_{}.jpg", i))
                                    .unwrap(),
                            ),
                            Some(
                                Src::new(format!("https://example.com/published_{}.jpg", i))
                                    .unwrap(),
                            ),
                        )
                        .unwrap(),
                    )),
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
        let product = mock_products(1).remove(0);
        let mut media = mock_media(1);
        media.extend(mock_media(1));

        let result = presenter
            .present_get_product(Ok((product, media)))
            .await
            .unwrap();

        assert_eq!(result.product.id, "0");
        assert_eq!(result.product.name, "Test Product 0");
        assert_eq!(
            result.product.description,
            "This is a test product description."
        );
        assert_eq!(result.product.media.len(), 2);
        assert_eq!(result.product.media[0].id, "0");
        assert_eq!(result.product.variants.len(), 1);
        assert_eq!(result.product.variants[0].id, "0");
    }

    #[actix_web::test]
    async fn test_present_get_product_not_found() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_product(Err(DomainError::NotFound))
            .await;

        assert!(matches!(result, Err(GetProductResponseError::NotFound)));
    }

    #[actix_web::test]
    async fn test_present_get_product_bad_request() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_product(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(result, Err(GetProductResponseError::BadRequest)));
    }

    #[actix_web::test]
    async fn test_present_get_product_service_unavailable() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_product(Err(DomainError::SystemError))
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
        let media = mock_media(5);

        let result = presenter
            .present_get_products(Ok((products, media)))
            .await
            .unwrap()
            .into_inner();

        assert_eq!(result.products.len(), 5);
        assert_eq!(result.products[0].name, "Test Product 0");
        assert_eq!(result.products[4].name, "Test Product 4");
    }

    #[actix_web::test]
    async fn test_present_get_products_not_found() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter.present_get_products(Ok((vec![], vec![]))).await;

        assert!(matches!(result, Err(GetProductsResponseError::NotFound)));
    }

    #[actix_web::test]
    async fn test_present_get_products_bad_request() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_products(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(result, Err(GetProductsResponseError::BadRequest)));
    }

    #[actix_web::test]
    async fn test_present_get_products_service_unavailable() {
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
