use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

use crate::interface::presenter::{
    auth::auth_impl::AuthPresenterImpl, auth_presenter_interface::AuthPresenter,
};

use super::{controller::Controller, interactor_provider_interface::InteractorProvider};

#[derive(Serialize, Deserialize)]
pub struct PostSignInRequest {
    id_token: Option<String>,
    refresh_token: Option<String>,
}

impl<I, T, C> Controller<I, T, C>
where
    I: InteractorProvider<T, C>,
    T: Send + Sync + 'static,
    C: Send + Sync + 'static,
{
    /// Perform back-end sign-in.
    /// Initiate session management with cookies.
    pub async fn post_sign_in(
        &self,
        request: actix_web::HttpRequest,
        body: web::Json<PostSignInRequest>,
    ) -> impl Responder {
        let body = body.into_inner();
        let id_token = body.id_token;
        let refresh_token = body.refresh_token;
        let transaction_manager = self.get_transaction_manager(&request)?;

        let interactor = self
            .interactor_provider
            .provide_auth_interactor(transaction_manager)
            .await;
        let result = interactor.authenticate(&id_token, &refresh_token).await;

        AuthPresenterImpl::new()
            .present_post_sign_in(result, refresh_token.as_deref())
            .await
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
    use crate::interface::mock::domain_mock::mock_customers;
    use crate::usecase::interactor::auth_interactor_interface::{
        AuthInteractor, MockAuthInteractor,
    };

    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::{http::StatusCode, test, App, Error};
    use actix_web::{web, HttpMessage};
    use sea_orm::{DatabaseConnection, DatabaseTransaction};

    const BASE_URL: &'static str = "/ec-extension/auth/sign-in";

    async fn setup(
        interactor: MockAuthInteractor,
    ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
        // Configure the mocks
        let mut interactor_provider =
            MockInteractorProvider::<DatabaseTransaction, Arc<DatabaseConnection>>::new();
        interactor_provider
            .expect_provide_auth_interactor()
            .return_once(move |_| Box::new(interactor) as Box<dyn AuthInteractor>);

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
    async fn test_post_sign_in_success() {
        let mut interactor = MockAuthInteractor::new();
        interactor
            .expect_authenticate()
            .returning(|_, _| Ok((mock_customers(1).remove(0), "updated-idtoken".to_string())));

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .set_json(PostSignInRequest {
                id_token: Some("idtoken".to_string()),
                refresh_token: Some("refreshtoken".to_string()),
            })
            .to_request();
        add_extensions(&req);

        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_post_sign_in_unauthorized() {
        let mut interactor = MockAuthInteractor::new();
        interactor
            .expect_authenticate()
            .returning(|_, _| Err(DomainError::SystemError));

        let req = test::TestRequest::post()
            .uri(&format!("{BASE_URL}"))
            .set_json(PostSignInRequest {
                id_token: Some("idtoken".to_string()),
                refresh_token: Some("refreshtoken".to_string()),
            })
            .to_request();
        let resp: ServiceResponse = test::call_service(&setup(interactor).await, req).await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
