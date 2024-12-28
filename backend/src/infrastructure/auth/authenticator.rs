pub trait Authenticator: Send + Sync {
    fn validate_token(&self, token: String) -> bool;
}
