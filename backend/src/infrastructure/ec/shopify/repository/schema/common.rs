use serde::Deserialize;

use crate::infrastructure::ec::ec_client_interface::ECClientResponse;

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

impl<T> ECClientResponse for GraphQLResponse<T> where T: Send + Sync {}

#[derive(Debug, Deserialize)]
pub struct Node<T> {
    pub node: T,
}

#[derive(Debug, Deserialize)]
pub struct Edges<T> {
    pub edges: Vec<Node<T>>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_page: bool,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "startCursor")]
    pub start_cursor: Option<String>,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub extensions: Option<GraphQLErrorExtensions>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GraphQLErrorExtensions {
    pub code: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UserError {
    pub field: Vec<String>,
    pub message: String,
}
