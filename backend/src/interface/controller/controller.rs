use crate::infrastructure::auth::authorizer_interface::Authorizer;

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
}
