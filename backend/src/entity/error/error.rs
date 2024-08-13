use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum DomainError {
    #[display(fmt = "System error.")]
    SystemError,
}
