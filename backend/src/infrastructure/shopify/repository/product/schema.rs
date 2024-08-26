use serde::Deserialize;

use crate::{
    entity::{
        error::error::DomainError,
        media::media::Media,
        product::product::{Product, ProductCategory, ProductStatus},
    },
    infrastructure::shopify::repository::common::schema::Edges,
};

#[derive(Debug, Deserialize)]
pub(super) struct ProductSchema {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) price: f64,
    pub(super) description: String,
    pub(super) status: String,
    pub(super) category: String,
}

impl From<ProductNode> for ProductSchema {
    fn from(node: ProductNode) -> Self {
        ProductSchema {
            id: node.id,
            title: node.title,
            price: node
                .price
                .max_variant_price
                .amount
                .parse::<f64>()
                .unwrap_or(0.0),
            description: node.description,
            status: node.status,
            category: node.category.name,
        }
    }
}

impl ProductSchema {
    pub(super) fn to_domain(self) -> Result<Product, DomainError> {
        let status = match self.status.as_str() {
            "ACTIVE" => ProductStatus::Active,
            "ARCHIVED" => ProductStatus::Inactive,
            "DRAFT" => ProductStatus::Draft,
            _ => ProductStatus::Inactive,
        };
        let category = match self.category.as_str() {
            "Tops" => ProductCategory::Tops,
            "Bottoms" => ProductCategory::Bottoms,
            "Shoes" => ProductCategory::Shoes,
            "Accessories" => ProductCategory::Accessories,
            "Other" => ProductCategory::Other,
            _ => ProductCategory::Other,
        };

        let media: Vec<Media> = Vec::new();

        Product::new(
            self.id,
            self.title,
            self.price as u32,
            self.description,
            status,
            category,
            media,
        )
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct TaxonomyCategory {
    pub(super) name: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct MaxVariantPrice {
    pub(super) amount: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct PriceRangeV2 {
    #[serde(rename = "maxVariantPrice")]
    pub(super) max_variant_price: MaxVariantPrice,
}

#[derive(Debug, Deserialize)]
pub(super) struct ProductNode {
    pub(super) id: String,
    pub(super) title: String,
    #[serde(rename = "priceRangeV2")]
    pub(super) price: PriceRangeV2,
    pub(super) description: String,
    pub(super) status: String,
    pub(super) category: TaxonomyCategory,
}

#[derive(Debug, Deserialize)]
pub(super) struct ProductsData {
    pub(super) products: Edges<ProductNode>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ProductData {
    pub(super) product: Option<ProductNode>,
}
