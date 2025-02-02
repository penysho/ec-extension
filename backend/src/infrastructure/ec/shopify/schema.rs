use serde::{Deserialize, Serialize};

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
#[serde(rename_all = "camelCase")]
pub struct Edges<T> {
    pub edges: Vec<Node<T>>,
    pub page_info: PageInfo,
}

impl<T> Default for Edges<T> {
    fn default() -> Self {
        Edges {
            edges: Vec::new(),
            page_info: PageInfo {
                has_previous_page: false,
                has_next_page: false,
                start_cursor: None,
                end_cursor: None,
            },
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool,
    pub start_cursor: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct Metafield<T> {
    pub value: T,
}

#[derive(Debug, Serialize)]
pub struct MetafieldInput<T> {
    pub key: String,
    pub namespace: String,
    pub value: T,
}
