use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error,
    middleware::Next,
    web, Error, HttpMessage,
};

use super::{
    sea_orm::sea_orm_manager::{SeaOrmConnectionProvider, SeaOrmTransactionManager},
    transaction_manager_interface::TransactionManager,
};

// Fixed message is responded and no internal information is returned.
const TRANSACTION_ERROR_MESSAGE: &str = "System error";

pub async fn transaction_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let connection_provider = req
        .app_data::<web::Data<SeaOrmConnectionProvider>>()
        .cloned()
        .ok_or_else(|| {
            log::error!("Failed to get connection provider");
            error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE)
        })?;

    let transaction_manager =
        SeaOrmTransactionManager::new(connection_provider.get_connection().clone())
            .await
            .map_err(|e| {
                log::error!("Failed to get transaction manager: {}", e);
                error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE)
            });

    req.extensions_mut().insert(transaction_manager);

    let res = next.call(req).await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                if let Some(mut transaction_manager) = response
                    .request()
                    .extensions_mut()
                    .remove::<SeaOrmTransactionManager>()
                {
                    transaction_manager.commit().await.map_err(|e| {
                        log::error!("Failed to commit transaction: {}", e);
                        error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE)
                    })?;
                } else {
                    log::error!("Failed to get transaction manager");
                    return Err(error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE));
                }
            } else {
                if let Some(mut transaction_manager) = response
                    .request()
                    .extensions_mut()
                    .remove::<SeaOrmTransactionManager>()
                {
                    transaction_manager.rollback().await.map_err(|e| {
                        log::error!("Failed to rollback transaction: {}", e);
                        error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE)
                    })?;
                } else {
                    log::error!("Failed to get transaction manager");
                    return Err(error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE));
                }
            }
            Ok(response)
        }
        Err(err) => {
            // This branch assumes an error before the application logic is called, so there is no need to explicitly roll back
            // TODO: Consideration when the program panics
            log::error!("Transaction cannot be rolled back because a response cannot be obtained");
            Err(err)
        }
    }
}
