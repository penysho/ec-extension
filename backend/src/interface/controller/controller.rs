use super::interact_provider_interface::InteractProvider;

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
