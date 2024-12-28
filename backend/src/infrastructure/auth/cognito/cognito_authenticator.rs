use crate::infrastructure::auth::authenticator::Authenticator;

pub struct CognitoAuthenticator;
impl CognitoAuthenticator {
    pub fn new() -> Self {
        CognitoAuthenticator {}
    }
}

impl Authenticator for CognitoAuthenticator {
    fn validate_token(&self, token: String) -> bool {
        true
    }
}
