pub struct ShopifyGQLQueryHelper {}

#[allow(dead_code)]
impl ShopifyGQLQueryHelper {
    pub const SHOPIFY_QUERY_LIMIT: usize = 250;

    pub const SHOPIFY_PRODUCT_GID_PREFIX: &'static str = "gid://shopify/Product/";
    pub const SHOPIFY_PRODUCT_VARIANT_GID_PREFIX: &'static str = "gid://shopify/ProductVariant/";
    pub const SHOPIFY_MEDIA_IMAGE_GID_PREFIX: &'static str = "gid://shopify/MediaImage/";
    pub const SHOPIFY_INVENTORY_ITEM_GID_PREFIX: &'static str = "gid://shopify/InventoryItem/";
    pub const SHOPIFY_LOCATION_GID_PREFIX: &'static str = "gid://shopify/Location/";

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

    /// Add Shopify gid prefix for InventoryItem.
    pub fn add_inventory_item_gid_prefix(id: &str) -> String {
        if id.contains(Self::SHOPIFY_INVENTORY_ITEM_GID_PREFIX) {
            return id.to_string();
        }
        format!("{}{}", Self::SHOPIFY_INVENTORY_ITEM_GID_PREFIX, id)
    }

    /// Add Shopify gid prefix for Location.
    pub fn add_location_gid_prefix(id: &str) -> String {
        if id.contains(Self::SHOPIFY_LOCATION_GID_PREFIX) {
            return id.to_string();
        }
        format!("{}{}", Self::SHOPIFY_LOCATION_GID_PREFIX, id)
    }

    /// Remove Shopify gid prefix.
    pub fn remove_gid_prefix(gid: &str) -> String {
        let gid_without_query: &str = gid.split('?').next().unwrap_or(gid);
        let v: Vec<&str> = gid_without_query.rsplit('/').collect();
        v[0].to_string()
    }
}
