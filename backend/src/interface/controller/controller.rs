use std::{marker::PhantomData, sync::Arc};

use actix_web::HttpMessage;

use crate::{
    domain::{error::error::DomainError, user::user::Id as UserId},
    infrastructure::db::transaction_manager_interface::TransactionManager,
};

use super::interact_provider_interface::InteractProvider;

/// Controller receives data from outside and calls usecase.
pub struct Controller<I, T>
where
    I: InteractProvider<T>,
    T: Send + Sync + 'static,
{
    pub interact_provider: I,
    _marker: PhantomData<T>,
}

impl<I, T> Controller<I, T>
where
    I: InteractProvider<T>,
    T: Send + Sync + 'static,
{
    pub fn new(interact_provider: I) -> Self {
        Controller {
            interact_provider,
            _marker: PhantomData,
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

    pub fn get_transaction_manager(
        &self,
        request: &actix_web::HttpRequest,
    ) -> Result<Arc<dyn TransactionManager<T>>, DomainError> {
        let manager = request
            .extensions()
            .get::<Arc<dyn TransactionManager<T>>>()
            .cloned();

        match manager {
            Some(manager) => Ok(manager),
            None => {
                log::error!(target: "Controller::get_transaction_manager", "transaction_manager not found");
                Err(DomainError::SystemError)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::infrastructure::db::sea_orm::sea_orm_manager::SeaOrmTransactionManager;
    use crate::infrastructure::db::transaction_manager_interface::TransactionManager;
    use crate::interface::controller::controller::Controller;
    use crate::interface::controller::interact_provider_interface::MockInteractProvider;
    use actix_web::test::TestRequest;
    use actix_web::HttpMessage;
    use sea_orm::DatabaseTransaction;

    #[test]
    fn test_get_user_id_success() {
        let interact_provider = MockInteractProvider::<()>::new();

        let controller = Controller::new(interact_provider);

        let request = TestRequest::default().to_http_request();
        request.extensions_mut().insert("user_id".to_string());
        let user_id = controller.get_user_id(&request);
        assert!(user_id.is_ok());
    }

    #[test]
    fn test_get_user_id_error() {
        let interact_provider = MockInteractProvider::<()>::new();

        let controller = Controller::new(interact_provider);

        let request = TestRequest::default().to_http_request();
        let user_id = controller.get_user_id(&request);
        assert!(user_id.is_err());
    }

    #[test]
    fn test_get_transaction_manager_success() {
        let interact_provider = MockInteractProvider::<DatabaseTransaction>::new();

        let controller = Controller::new(interact_provider);

        let request = TestRequest::default().to_http_request();
        let mock = Arc::new(SeaOrmTransactionManager::default())
            as Arc<dyn TransactionManager<DatabaseTransaction>>;
        request.extensions_mut().insert(mock.clone());

        let transaction_manager = controller.get_transaction_manager(&request);
        assert!(transaction_manager.is_ok());
        assert!(Arc::ptr_eq(transaction_manager.as_ref().unwrap(), &mock));
    }

    #[test]
    fn test_get_transaction_manager_error() {
        let interact_provider = MockInteractProvider::<DatabaseTransaction>::new();

        let controller = Controller::new(interact_provider);

        let request = TestRequest::default().to_http_request();

        let transaction_manager = controller.get_transaction_manager(&request);
        assert!(transaction_manager.is_err());
    }
}
