use serde::Deserialize;

use super::common::Edges;

#[derive(Debug, Deserialize)]
pub struct ProductSchema {
    pub id: String,
    pub title: String,
    pub price: f64,
    pub description: String,
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
