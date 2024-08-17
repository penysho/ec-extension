use serde::Deserialize;

use crate::entity::product::product::Product;

use super::common::Edges;

#[derive(Debug, Deserialize)]
pub struct ProductSchema {
    pub id: String,
    pub title: String,
    pub price: f64,
    pub description: String,
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
        }
    }
}

impl From<Product> for ProductSchema {
    fn from(domain: Product) -> Self {
        ProductSchema {
            id: domain.id().to_string(),
            title: domain.name().to_string(),
            price: *(domain.price()) as f64,
            description: domain.description().to_string(),
        }
    }
}

impl ProductSchema {
    pub fn to_domain(self) -> Product {
        Product::new(self.id, self.title, self.price as u32, self.description)
    }
}

#[derive(Debug, Deserialize)]
pub struct MaxVariantPrice {
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct PriceRangeV2 {
    #[serde(rename = "maxVariantPrice")]
    pub max_variant_price: MaxVariantPrice,
}

#[derive(Debug, Deserialize)]
pub struct ProductNode {
    pub id: String,
    pub title: String,
    #[serde(rename = "priceRangeV2")]
    pub price: PriceRangeV2,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductsData {
    pub products: Edges<ProductNode>,
}

#[derive(Debug, Deserialize)]
pub struct ProductData {
    pub product: ProductNode,
}
