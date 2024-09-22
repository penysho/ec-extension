use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LocationSchema {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct LocationNode {
    pub id: String,
}
