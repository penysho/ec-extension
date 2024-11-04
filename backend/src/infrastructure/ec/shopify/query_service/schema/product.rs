use serde::Deserialize;

use crate::{
    infrastructure::ec::shopify::{gql_helper::ShopifyGQLHelper, schema::Edges},
    usecase::query_service::dto::product::ProductDTO,
};

impl From<ProductNode> for ProductDTO {
    fn from(node: ProductNode) -> Self {
        Self {
            id: ShopifyGQLHelper::remove_gid_prefix(&node.id),
            name: node.title,
            handle: node.handle,
            vendor: node.vender,
            price: node
                .price_range_v2
                .max_variant_price
                .amount
                .parse()
                .unwrap_or(0.0),
            featured_media_url: node.featured_media.and_then(|media| {
                media
                    .preview
                    .and_then(|preview| preview.image.map(|image| image.url))
            }),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RelatedProductsData {
    pub products: Edges<ProductNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductNode {
    pub id: String,
    pub title: String,
    pub handle: String,
    pub vender: String,
    pub price_range_v2: PriceRangeV2Node,
    pub featured_media: Option<MediaNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceRangeV2Node {
    pub max_variant_price: MoneyV2Node,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyV2Node {
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct MediaNode {
    pub preview: Option<MediaPreviewImageNode>,
}

#[derive(Debug, Deserialize)]
pub struct MediaPreviewImageNode {
    pub image: Option<ImageNode>,
}

#[derive(Debug, Deserialize)]
pub struct ImageNode {
    pub url: String,
}
