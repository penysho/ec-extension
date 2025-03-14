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
            GetProductErrorResponse, GetProductResponse, GetProductsErrorResponse,
            GetProductsResponse, ProductSchema,
        },
        product_presenter_interface::ProductPresenter,
    },
    usecase::query_service::dto::product::ProductDTO,
};

use super::schema::{GetRelatedProductsErrorResponse, GetRelatedProductsResponse};

/// Generate a response schema for the product.
pub struct ProductPresenterImpl;
impl ProductPresenterImpl {
    pub fn new() -> Self {
        ProductPresenterImpl
    }
}

#[async_trait]
impl ProductPresenter for ProductPresenterImpl {
    type GetProductResponse = Json<GetProductResponse>;
    type GetProductErrorResponse = GetProductErrorResponse;
    async fn present_get_product(
        &self,
        result: Result<(Product, Vec<Media>), DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductErrorResponse> {
        let (product, media) = result?;

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
    type GetProductsErrorResponse = GetProductsErrorResponse;
    async fn present_get_products(
        &self,
        result: Result<(Vec<Product>, Vec<Media>), DomainError>,
    ) -> Result<Self::GetProductsResponse, Self::GetProductsErrorResponse> {
        let (products, media) = result?;

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

    type GetRelatedProductsResponse = Json<GetRelatedProductsResponse>;
    type GetRelatedProductsErrorResponse = GetRelatedProductsErrorResponse;
    async fn present_get_related_products(
        &self,
        result: Result<Vec<ProductDTO>, DomainError>,
    ) -> Result<Self::GetRelatedProductsResponse, Self::GetRelatedProductsErrorResponse> {
        Ok(web::Json(GetRelatedProductsResponse { products: result? }))
    }
}

#[cfg(test)]
mod tests {
    use crate::interface::mock::{
        domain_mock::{mock_media, mock_products},
        query_service_dto_mock::mock_products_dto,
    };

    use super::*;

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

        assert!(matches!(
            result,
            Err(GetProductErrorResponse::NotFound { .. })
        ));
    }

    #[actix_web::test]
    async fn test_present_get_product_bad_request() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_product(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(result, Err(GetProductErrorResponse::BadRequest)));
    }

    #[actix_web::test]
    async fn test_present_get_product_service_unavailable() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_product(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetProductErrorResponse::ServiceUnavailable)
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
    async fn test_present_get_products_bad_request() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_products(Err(DomainError::ValidationError))
            .await;

        assert!(matches!(result, Err(GetProductsErrorResponse::BadRequest)));
    }

    #[actix_web::test]
    async fn test_present_get_products_service_unavailable() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_products(Err(DomainError::SystemError))
            .await;

        assert!(matches!(
            result,
            Err(GetProductsErrorResponse::ServiceUnavailable)
        ));
    }

    #[actix_web::test]
    async fn test_present_get_related_products_success() {
        let presenter = ProductPresenterImpl::new();

        let result = presenter
            .present_get_related_products(Ok(mock_products_dto(5)))
            .await
            .unwrap()
            .into_inner();

        assert_eq!(result.products.len(), 5);
        assert_eq!(result.products[0].id, "0");
        assert_eq!(result.products[4].id, "4");
    }
}
