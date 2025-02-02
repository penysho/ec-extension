use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{customer::customer::Id as CustomerId, draft_order::draft_order::DraftOrder},
    infrastructure::ec::shopify::{
        gql_helper::ShopifyGQLHelper,
        schema::{MetafieldInput, UserError},
    },
};

use super::{
    address_input::AddressInput,
    draft_order::DraftOrderNode,
    line_item_input::{DiscountInput, LineItemInput},
};

impl From<DraftOrder> for DraftOrderInput {
    fn from(draft_order: DraftOrder) -> Self {
        Self {
            purchasing_entity: draft_order.customer_id().to_owned().map(|p| p.into()),
            billing_address: draft_order.billing_address().to_owned().map(|a| a.into()),
            shipping_address: draft_order.shipping_address().to_owned().map(|a| a.into()),
            note: draft_order.note().to_owned(),
            line_items: draft_order.line_items().iter().map(|l| l.into()).collect(),
            reserve_inventory_until: draft_order.reserve_inventory_until().to_owned(),
            applied_discount: draft_order.discount().to_owned().map(|d| d.into()),
            tax_exempt: Some(*draft_order.tax_exempt()),
            metafields: vec![{
                MetafieldInput {
                    key: "owner_user_id".to_string(),
                    namespace: "custom".to_string(),
                    value: draft_order.owner_user_id().to_owned(),
                }
            }],
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderInput {
    pub purchasing_entity: Option<PurchasingEntityInput>,
    pub billing_address: Option<AddressInput>,
    pub shipping_address: Option<AddressInput>,
    pub note: Option<String>,

    pub line_items: Vec<LineItemInput>,
    pub reserve_inventory_until: Option<DateTime<Utc>>,

    pub applied_discount: Option<DiscountInput>,

    pub tax_exempt: Option<bool>,

    pub metafields: Vec<MetafieldInput<String>>,
}

impl From<CustomerId> for PurchasingEntityInput {
    fn from(customer_id: CustomerId) -> Self {
        Self {
            customer_id: Some(ShopifyGQLHelper::add_customer_gid_prefix(&customer_id)),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchasingEntityInput {
    pub customer_id: Option<String>,
}

impl From<DraftOrder> for DraftOrderDeleteInput {
    fn from(draft_order: DraftOrder) -> Self {
        Self {
            id: ShopifyGQLHelper::add_draft_order_gid_prefix(&draft_order.id()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderDeleteInput {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderCreateData {
    pub draft_order_create: DraftOrderCreate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderCreate {
    pub draft_order: Option<DraftOrderNode>,
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderUpdateData {
    pub draft_order_update: DraftOrderUpdate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderUpdate {
    pub draft_order: Option<DraftOrderNode>,
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderCompleteData {
    pub draft_order_complete: DraftOrderComplete,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderComplete {
    pub draft_order: Option<DraftOrderNode>,
    pub user_errors: Vec<UserError>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderDeleteData {
    pub draft_order_delete: DraftOrderDelete,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftOrderDelete {
    pub deleted_id: Option<String>,
    pub user_errors: Vec<UserError>,
}
