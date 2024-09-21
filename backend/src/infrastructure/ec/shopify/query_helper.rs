pub struct ShopifyGQLQueryHelper {}

#[allow(dead_code)]
impl ShopifyGQLQueryHelper {
    pub const SHOPIFY_QUERY_LIMIT: usize = 250;

    pub const SHOPIFY_PRODUCT_GID_PREFIX: &'static str = "gid://shopify/Product/";
    pub const SHOPIFY_PRODUCT_VARIANT_GID_PREFIX: &'static str = "gid://shopify/ProductVariant/";
    pub const SHOPIFY_MEDIA_IMAGE_GID_PREFIX: &'static str = "gid://shopify/MediaImage/";
    pub const SHOPIFY_INVENTORY_ITEM_GID_PREFIX: &'static str = "gid://shopify/InventoryItem/";

    /// Return first query with max limit.
    pub fn first_query() -> String {
        format!("first: {}", Self::SHOPIFY_QUERY_LIMIT)
    }

    /// Return pageInfo query.
    pub fn page_info() -> String {
        "pageInfo {
            hasPreviousPage
            hasNextPage
            startCursor
            endCursor
        }"
        .to_string()
    }

    /// Remove Shopify gid prefix.
    pub fn remove_product_gid_prefix(gid: &str) -> String {
        gid.replace(Self::SHOPIFY_PRODUCT_GID_PREFIX, "")
    }

    /// Remove Shopify gid prefix.
    pub fn remove_product_variant_gid_prefix(gid: &str) -> String {
        gid.replace(Self::SHOPIFY_PRODUCT_VARIANT_GID_PREFIX, "")
    }

    /// Remove Shopify gid prefix.
    pub fn remove_media_image_gid_prefix(gid: &str) -> String {
        gid.replace(Self::SHOPIFY_MEDIA_IMAGE_GID_PREFIX, "")
    }

    /// Remove Shopify gid prefix.
    pub fn remove_inventory_item_gid_prefix(gid: &str) -> String {
        gid.replace(Self::SHOPIFY_INVENTORY_ITEM_GID_PREFIX, "")
    }
}
