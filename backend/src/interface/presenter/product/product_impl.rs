use actix_web::web::{self, Json};

use crate::{
    entity::{error::error::DomainError, product::product::Product},
    interface::presenter::{
        product_presenter_interface::ProductPresenter,
        schema::product::{
            GetProductResponse, GetProductResponseError, GetProductsResponse,
            GetProductsResponseError, ProductSchema,
        },
    },
};

pub struct ProductPresenterImpl;
impl ProductPresenterImpl {
    pub fn new() -> Self {
        ProductPresenterImpl
    }
}
impl ProductPresenter for ProductPresenterImpl {
    type GetProductResponse = Json<GetProductResponse>;
    type GetProductResponseError = GetProductResponseError;
    type GetProductsResponse = Json<GetProductsResponse>;
    type GetProductsResponseError = GetProductsResponseError;

    async fn present_get_product(
        &self,
        result: Result<Option<Product>, DomainError>,
    ) -> Result<Self::GetProductResponse, Self::GetProductResponseError> {
        match result {
            Ok(Some(product)) => Ok(web::Json(GetProductResponse {
                product: ProductSchema {
                    id: product.get_id().to_string(),
                    name: product.get_name().to_string(),
                    price: product.get_price(),
                    description: product.get_description().to_string(),
                },
            })),
            Ok(None) => Err(GetProductResponseError::ProductNotFound),
            Err(_) => Err(GetProductResponseError::ServiceUnavailable),
        }
    }

    async fn present_get_products(
        &self,
        result: Result<Vec<Product>, DomainError>,
    ) -> Result<Self::GetProductsResponse, Self::GetProductsResponseError> {
        match result {
            Ok(products) => {
                let product_schemas: Vec<ProductSchema> = products
                    .into_iter()
                    .map(|product| ProductSchema {
                        id: product.get_id().to_string(),
                        name: product.get_name().to_string(),
                        price: product.get_price(),
                        description: product.get_description().to_string(),
                    })
                    .collect();

                Ok(web::Json(GetProductsResponse {
                    products: product_schemas,
                }))
            }
            Err(_) => Err(GetProductsResponseError::ServiceUnavailable),
        }
    }
}
