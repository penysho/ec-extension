pub struct ShopifyGQLHelper {}

impl ShopifyGQLHelper {
    pub const SHOPIFY_QUERY_LIMIT: usize = 250;

    pub const SHOPIFY_PRODUCT_VARIANT_GID_PREFIX: &'static str = "gid://shopify/ProductVariant/";
    pub const SHOPIFY_INVENTORY_ITEM_GID_PREFIX: &'static str = "gid://shopify/InventoryItem/";
    pub const SHOPIFY_LOCATION_GID_PREFIX: &'static str = "gid://shopify/Location/";
    pub const SHOPIFY_DRAFT_ORDER_GID_PREFIX: &'static str = "gid://shopify/DraftOrder/";
    pub const SHOPIFY_CUSTOMER_GID_PREFIX: &'static str = "gid://shopify/Customer/";

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

    /// Return userErrors query.
    pub fn user_errors() -> String {
        "userErrors {
            field
            message
        }"
        .to_string()
    }

    /// Return address fields.
    pub fn address_fields() -> String {
        "address1
        address2
        city
        coordinatesValidated
        country
        firstName
        id
        lastName
        name
        phone
        province
        zip"
        .to_string()
    }

    /// Return money bag fields.
    pub fn money_bag_fields() -> String {
        "shopMoney {
            amount
            currencyCode
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

    /// Add Shopify gid prefix for DraftOrder.
    pub fn add_draft_order_gid_prefix(id: &str) -> String {
        if id.contains(Self::SHOPIFY_DRAFT_ORDER_GID_PREFIX) {
            return id.to_string();
        }
        format!("{}{}", Self::SHOPIFY_DRAFT_ORDER_GID_PREFIX, id)
    }

    /// Add Shopify gid prefix for Customer.
    pub fn add_customer_gid_prefix(id: &str) -> String {
        if id.contains(Self::SHOPIFY_CUSTOMER_GID_PREFIX) {
            return id.to_string();
        }
        format!("{}{}", Self::SHOPIFY_CUSTOMER_GID_PREFIX, id)
    }

    /// Add Shopify gid prefix for Product variant.
    pub fn add_product_variant_gid_prefix(id: &str) -> String {
        if id.contains(Self::SHOPIFY_PRODUCT_VARIANT_GID_PREFIX) {
            return id.to_string();
        }
        format!("{}{}", Self::SHOPIFY_PRODUCT_VARIANT_GID_PREFIX, id)
    }

    /// Remove Shopify gid prefix.
    pub fn remove_gid_prefix(gid: &str) -> String {
        let gid_without_query: &str = gid.split('?').next().unwrap_or(gid);
        let v: Vec<&str> = gid_without_query.rsplit('/').collect();
        v[0].to_string()
    }
}
