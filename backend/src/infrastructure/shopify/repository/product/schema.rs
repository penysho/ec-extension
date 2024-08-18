use serde::Deserialize;

use crate::{
    entity::product::product::Product, infrastructure::shopify::repository::common::schema::Edges,
};

#[derive(Debug, Deserialize)]
pub(super) struct ProductSchema {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) price: f64,
    pub(super) description: String,
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

impl ProductSchema {
    pub(super) fn to_domain(self) -> Product {
        Product::new(self.id, self.title, self.price as u32, self.description)
    }
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
}

#[derive(Debug, Deserialize)]
pub(super) struct ProductsData {
    pub(super) products: Edges<ProductNode>,
}

#[derive(Debug, Deserialize)]
pub(super) struct ProductData {
    pub(super) product: Option<ProductNode>,
}
