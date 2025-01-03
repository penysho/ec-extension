use actix_web::HttpMessage;

use crate::domain::{error::error::DomainError, user::user::Id as UserId};

use super::{authorizer_interface::Authorizer, interact_provider_interface::InteractProvider};

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

#[cfg(test)]
mod test {
    use crate::interface::controller::authorizer_interface::MockAuthorizer;
    use crate::interface::controller::controller::Controller;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use actix_web::test::TestRequest;
    use actix_web::HttpMessage;

    #[test]
    fn test_get_user_id_success() {
        let interact_provider = MockInteractProvider::new();
        let authorizer = MockAuthorizer::new();

        let controller = Controller::new(Box::new(interact_provider), Box::new(authorizer));

        let request = TestRequest::default().to_http_request();
        request.extensions_mut().insert("user_id".to_string());
        let user_id = controller.get_user_id(&request);
        assert!(user_id.is_ok());
    }

    #[test]
    fn test_get_user_id_error() {
        let interact_provider = MockInteractProvider::new();
        let authorizer = MockAuthorizer::new();

        let controller = Controller::new(Box::new(interact_provider), Box::new(authorizer));

        let request = TestRequest::default().to_http_request();
        let user_id = controller.get_user_id(&request);
        assert!(user_id.is_err());
    }
}
