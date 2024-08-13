use derive_more::{Display, Error};

/// Entity of Errors.
#[derive(Debug, Display, Error)]
pub enum DomainError {
    #[display(fmt = "System error.")]
    SystemError,
}
