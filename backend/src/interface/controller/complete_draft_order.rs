use actix_web::{web::Path, Responder};

use crate::interface::presenter::{
    draft_order::draft_order_impl::DraftOrderPresenterImpl,
    draft_order_presenter_interface::DraftOrderPresenter,
};

use super::{controller::Controller, interactor_provider_interface::InteractorProvider};

impl<I, T, C> Controller<I, T, C>
where
    I: InteractorProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Complete a draft order.
    pub async fn complete_draft_order(
        &self,
        request: actix_web::HttpRequest,
        path: Path<(String,)>,
    ) -> impl Responder {
        let presenter = DraftOrderPresenterImpl::new();

        let user = self.get_user(&request)?;
        let transaction_manager = self.get_transaction_manager(&request)?;

        let interactor = self
            .interactor_provider
            .provide_draft_order_interactor(transaction_manager)
            .await;

        let result = interactor
            .complete_draft_order(user, &path.into_inner().0)
            .await;

        presenter.present_complete_draft_order(result).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::domain::error::error::DomainError;
    use crate::domain::user::user::UserInterface;
    use crate::infrastructure::auth::idp_user::IdpUser;
    use crate::infrastructure::db::sea_orm::sea_orm_manager::SeaOrmTransactionManager;
    use crate::infrastructure::db::transaction_manager_interface::TransactionManager;
    use crate::infrastructure::router::actix_router;
    use crate::interface::controller::interactor_provider_interface::MockInteractorProvider;
    use crate::interface::mock::domain_mock::mock_draft_orders;
    use crate::usecase::interactor::draft_order_interactor_interface::DraftOrderInteractor;
    use crate::usecase::interactor::draft_order_interactor_interface::MockDraftOrderInteractor;

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::web;
    use actix_web::HttpMessage;
    use actix_web::{http::StatusCode, test, App, Error};
    use sea_orm::DatabaseConnection;
    use sea_orm::DatabaseTransaction;

    const BASE_URL: &'static str = "/ec-extension/orders/draft/complete";

    async fn setup(
        interactor: MockDraftOrderInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interactor_provider =
            MockInteractorProvider::<DatabaseTransaction, Arc<DatabaseConnection>>::new();
        interactor_provider
            .expect_provide_draft_order_interactor()
            .return_once(move |_| Box::new(interactor) as Box<dyn DraftOrderInteractor>);

        let controller = web::Data::new(Controller::new(interactor_provider));

        // Create an application for testing
        test::init_service(App::new().app_data(controller).configure(
            actix_router::configure_routes::<
                MockInteractorProvider<DatabaseTransaction, Arc<DatabaseConnection>>,
                DatabaseTransaction,
                Arc<DatabaseConnection>,
            >,
        ))
        .await
    }

    fn add_extensions(req: &Request) {
        req.extensions_mut()
            .insert(Arc::new(IdpUser::default()) as Arc<dyn UserInterface>);
        req.extensions_mut()
            .insert(Arc::new(SeaOrmTransactionManager::default())
                as Arc<
                    dyn TransactionManager<DatabaseTransaction, Arc<DatabaseConnection>>,
                >);
    }

    #[actix_web::test]
    async fn test_complete_draft_order_success() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_complete_draft_order()
            .returning(|_, _| Ok(mock_draft_orders(1).remove(0)));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_complete_draft_order_bad_request() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_complete_draft_order()
            .returning(|_, _| Err(DomainError::ValidationError));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_complete_draft_order_service_unavailable() {
        let mut interactor = MockDraftOrderInteractor::new();
        interactor
            .expect_complete_draft_order()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::put()
            .uri(&format!("{BASE_URL}/1"))
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
