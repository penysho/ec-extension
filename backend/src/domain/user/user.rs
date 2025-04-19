pub type Id = String;

/// User interfaces commonly used in applications.
pub trait UserInterface: Send + Sync {
    /// Get the user identifier.
    fn id(&self) -> &str;
    /// Get the email address of the user.
    fn email(&self) -> &str;
}
