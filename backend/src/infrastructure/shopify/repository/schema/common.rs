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

#[derive(Debug, Deserialize)]
pub struct Edges<T> {
    pub edges: Vec<Node<T>>,
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
