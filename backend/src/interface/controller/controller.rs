use actix_web::HttpMessage;

use crate::{
    domain::{error::error::DomainError, user::user::Id as UserId},
    infrastructure::auth::authorizer_interface::Authorizer,
};

use super::interact_provider_interface::InteractProvider;

/// Controller receives data from outside and calls usecase.
pub struct Controller {
    pub interact_provider: Box<dyn InteractProvider>,
    pub authorizer: Box<dyn Authorizer>,
}

impl Controller {
    pub fn new(
        interact_provider: Box<dyn InteractProvider>,
        authorizer: Box<dyn Authorizer>,
    ) -> Self {
        Controller {
            interact_provider,
            authorizer,
        }
    }

    /// Obtain the user ID used for authorization from the actix request.
    /// User ID is assumed to be always set if authenticated by middleware, and if it cannot be obtained, it is assumed to be a system error.
    pub fn get_user_id(&self, request: &actix_web::HttpRequest) -> Result<UserId, DomainError> {
        match request.extensions().get::<String>() {
            Some(user_id) => Ok(user_id.to_string()),
            None => {
                log::error!(target: "Controller::get_user_id", "user_id not found");
                Err(DomainError::SystemError)
            }
        }
    }
}
