use std::collections::{HashMap, HashSet};

use crate::domain::{
    authorized_resource::authorized_resource::ResourceType,
    user::user::{Role, UserAction, UserInterface},
};
/// Represent user information managed by Idp.
/// Since the model is infrastructure-dependent, it is defined here rather than at the domain layer.
///
/// # Fields
/// - `id` - User identifier issued by Idp.
/// - `email` - The email of the user registered in the Idp, which is unique.
/// - `roles` - The roles of the user.
/// - `permissions` - The permissions of the user.
#[derive(Debug, Clone)]
pub struct IdpUser {
    id: String,
    email: String,
    roles: Vec<Role>,
    permissions: HashMap<ResourceType, HashSet<UserAction>>,
}

impl Default for IdpUser {
    fn default() -> Self {
        Self {
            id: String::new(),
            email: String::new(),
            roles: Vec::new(),
            permissions: HashMap::new(),
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

    fn roles(&self) -> Vec<Role> {
        self.roles.clone()
    }

    fn permissions(&self) -> HashMap<ResourceType, HashSet<UserAction>> {
        self.permissions.clone()
    }
}

impl IdpUser {
    /// Create a new IdpUser.
    pub(super) fn new(
        id: String,
        email: String,
        roles: Vec<Role>,
        permissions: HashMap<ResourceType, HashSet<UserAction>>,
    ) -> Self {
        Self {
            id,
            email,
            roles,
            permissions,
        }
    }
}
