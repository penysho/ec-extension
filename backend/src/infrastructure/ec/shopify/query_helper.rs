pub struct ShopifyGQLQueryHelper {}

impl ShopifyGQLQueryHelper {
    pub const SHOPIFY_PRODUCT_GID_PREFIX: &'static str = "gid://shopify/Product/";
    pub const SHOPIFY_QUERY_LIMIT: usize = 250;

    /// Return first query with max limit.
    pub fn first_query() -> String {
        format!("first: {}", Self::SHOPIFY_QUERY_LIMIT)
    }

    /// Remove Shopify gid prefix.
    pub fn remove_product_gid_prefix(gid: &str) -> String {
        gid.replace(Self::SHOPIFY_PRODUCT_GID_PREFIX, "")
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
}
