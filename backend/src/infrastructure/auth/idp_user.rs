/// Represent user information managed by Idp.
/// Since the model is infrastructure-dependent, it is defined here rather than at the domain layer.
///
/// # Fields
/// - `id` - User identifier issued by Idp.
/// - `email` - The email of the user registered in the Idp, which is unique.
#[derive(Debug, Clone)]
pub struct IdpUser {
    pub id: String,
    pub email: String,
}
