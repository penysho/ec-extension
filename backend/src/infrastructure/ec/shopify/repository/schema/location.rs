use serde::Deserialize;

impl From<LocationNode> for LocationSchema {
    fn from(node: LocationNode) -> Self {
        LocationSchema { id: node.id }
    }
}

#[derive(Debug, Deserialize)]
pub struct LocationSchema {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct LocationNode {
    pub id: String,
}
