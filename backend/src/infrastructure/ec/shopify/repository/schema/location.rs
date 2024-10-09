use serde::Deserialize;

use super::common::Edges;

#[derive(Debug, Deserialize)]
pub struct LocationsData {
    pub locations: Edges<LocationNode>,
}

#[derive(Debug, Deserialize)]
pub struct LocationNode {
    pub id: String,
}
