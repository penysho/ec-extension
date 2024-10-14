use crate::domain::draft_order::draft_order::{DraftOrder, DraftOrderStatus};

use super::schema::{DraftOrderSchema, DraftOrderStatusSchema};

impl From<DraftOrder> for DraftOrderSchema {
    fn from(draft_order: DraftOrder) -> Self {
        DraftOrderSchema {
            id: draft_order.id().to_string(),
            name: draft_order.name().to_string(),
            status: draft_order.status().to_owned().into(),
            line_items: draft_order
                .line_items()
                .iter()
                .map(|line_item| line_item.into())
                .collect(),
            reserve_inventory_until: *draft_order.reserve_inventory_until(),
            subtotal_price_set: draft_order.subtotal_price_set().to_owned().into(),
            taxes_included: *draft_order.taxes_included(),
            tax_exempt: *draft_order.tax_exempt(),
            total_tax_set: draft_order.total_tax_set().to_owned().into(),
            total_discounts_set: draft_order.total_discounts_set().to_owned().into(),
            total_shipping_price_set: draft_order.total_shipping_price_set().to_owned().into(),
            total_price_set: draft_order.total_price_set().to_owned().into(),
            customer_id: draft_order.customer_id().as_ref().map(|id| id.to_string()),
            billing_address: draft_order.billing_address().into(),
            shipping_address: draft_order.shipping_address().into(),
            note: draft_order.note().as_ref().map(|note| note.to_string()),
            order_id: draft_order.order_id().as_ref().map(|id| id.to_string()),
            completed_at: *draft_order.completed_at(),
            created_at: *draft_order.created_at(),
            update_at: *draft_order.update_at(),
        }
    }
}

impl From<DraftOrderStatus> for DraftOrderStatusSchema {
    fn from(status: DraftOrderStatus) -> Self {
        match status {
            DraftOrderStatus::Open => DraftOrderStatusSchema::Open,
            DraftOrderStatus::Completed => DraftOrderStatusSchema::Completed,
            DraftOrderStatus::Canceled => DraftOrderStatusSchema::Canceled,
        }
    }
}
