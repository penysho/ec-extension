use super::interact_provider_interface::InteractProvider;

/// Controller receives data from outside and calls usecase.
pub struct Controller {
    pub interact_provider: Box<dyn InteractProvider>,
}

impl Controller {
    pub fn new(interact_provider: Box<dyn InteractProvider>) -> Self {
        Controller {
            interact_provider: interact_provider,
        }
    }
}
