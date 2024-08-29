use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct Node<T> {
    pub node: T,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct PageInfo {
    #[serde(rename = "hasPreviousPage")]
    pub has_previous_pages: bool,
    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,
    #[serde(rename = "startCursor")]
    pub start_cursor: Option<String>,
    #[serde(rename = "endCursor")]
    pub end_cursor: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Edges<T> {
    pub edges: Vec<Node<T>>,
    #[serde(rename = "pageInfo")]
    pub page_info: PageInfo,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    message: String,
    extensions: Option<GraphQLErrorExtensions>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GraphQLErrorExtensions {
    code: Option<String>,
}
