use crate::domain::line_item::line_item::LineItem;

use super::schema::LineItemSchema;

impl From<&LineItem> for LineItemSchema {
    fn from(line_item: &LineItem) -> Self {
        LineItemSchema {
            id: line_item.id().to_string(),
            is_custom: *line_item.is_custom(),
            variant_id: line_item.variant_id().as_ref().map(|id| id.to_string()),
            quantity: *line_item.quantity(),
            discounted_total_set: line_item.discounted_total_set().to_owned().into(),
            original_total_set: line_item.original_total_set().to_owned().into(),
        }
    }
}
