use crate::domain::user::user::UserInterface;

/// Represent user information managed by Idp.
/// Since the model is infrastructure-dependent, it is defined here rather than at the domain layer.
///
/// # Fields
/// - `id` - User identifier issued by Idp.
/// - `email` - The email of the user registered in the Idp, which is unique.
#[derive(Debug, Clone)]
pub struct IdpUser {
    pub(super) id: String,
    pub(super) email: String,
}

impl Default for IdpUser {
    fn default() -> Self {
        Self {
            id: String::new(),
            email: String::new(),
        }
    }
}

impl UserInterface for IdpUser {
    fn id(&self) -> &str {
        &self.id
    }

    fn email(&self) -> &str {
        &self.email
    }
}
