use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error,
    middleware::Next,
    web, Error, HttpMessage,
};

use super::transaction_manager_interface::TransactionManager;

// Fixed message is responded and no internal information is returned.
const TRANSACTION_ERROR_MESSAGE: &str = "System error";

pub async fn transaction_middleware<T>(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error>
where
    T: TransactionManager + 'static,
    T::Transaction: Send + 'static,
{
    let transaction_manager = req.app_data::<web::Data<T>>().cloned().ok_or_else(|| {
        log::error!("Failed to get transaction manager");
        error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE)
    })?;

    let transaction = transaction_manager.begin().await.map_err(|e| {
        log::error!("Failed to start transaction: {}", e);
        error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE)
    })?;

    req.extensions_mut().insert(transaction);

    let res = next.call(req).await;
    match res {
        Ok(response) => {
            if response.status().is_success() {
                if let Some(transaction) = response
                    .request()
                    .extensions_mut()
                    .remove::<<T as TransactionManager>::Transaction>()
                {
                    transaction_manager.commit(transaction).await.map_err(|e| {
                        log::error!("Failed to commit transaction: {}", e);
                        error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE)
                    })?;
                } else {
                    log::error!("Transaction already consumed");
                    return Err(error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE));
                }
            } else {
                if let Some(transaction) = response
                    .request()
                    .extensions_mut()
                    .remove::<<T as TransactionManager>::Transaction>()
                {
                    transaction_manager
                        .rollback(transaction)
                        .await
                        .map_err(|e| {
                            log::error!("Failed to rollback transaction: {}", e);
                            error::ErrorInternalServerError(TRANSACTION_ERROR_MESSAGE)
                        })?;
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
