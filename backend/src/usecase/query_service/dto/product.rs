use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDTO {
    pub id: String,
    pub name: String,
    pub handle: String,
    pub vendor: Option<String>,
    pub price: f64,
    pub featured_media_url: Option<String>,
}
