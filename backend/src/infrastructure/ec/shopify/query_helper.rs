pub struct ShopifyGQLQueryHelper {}

impl ShopifyGQLQueryHelper {
    const SHOPIFY_PRODUCT_GID_PREFIX: &'static str = "gid://shopify/Product/";
    /// Remove Shopify gid prefix.
    pub fn remove_product_gid_prefix(gid: &str) -> String {
        gid.replace(Self::SHOPIFY_PRODUCT_GID_PREFIX, "")
    }

    pub fn page_info() -> String {
        "pageInfo {
            hasPreviousPage
            hasNextPage
            startCursor
            endCursor
        }"
        .to_string()
    }
}
