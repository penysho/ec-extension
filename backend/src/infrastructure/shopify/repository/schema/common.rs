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

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub extensions: Option<GraphQLErrorExtensions>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLErrorExtensions {
    pub code: Option<String>,
}
