use actix_web::Error;
use actix_web::HttpMessage;
use std::time::Instant;
use tracing::Level;

/// Request start time holder for response time measurement
#[derive(Debug, Clone)]
struct RequestStartTime(Instant);

pub struct CustomRootSpanBuilder;

impl tracing_actix_web::RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(request: &actix_web::dev::ServiceRequest) -> tracing::Span {
        // Create a span with standard fields
        let span = tracing_actix_web::root_span!(request);

        // Store request start time in the request extensions
        request
            .extensions_mut()
            .insert(RequestStartTime(Instant::now()));

        span
    }

    fn on_request_end<B: actix_web::body::MessageBody>(
        span: tracing::Span,
        outcome: &Result<actix_web::dev::ServiceResponse<B>, Error>,
    ) {
        // Record standard fields when the request finishes
        tracing_actix_web::DefaultRootSpanBuilder::on_request_end(span.clone(), outcome);

        // Record additional information
        if let Ok(response) = outcome {
            // Record status code
            let status = response.status();
            let status_code = status.as_u16();
            span.record("status_code", &status_code);

            // Calculate response time if available
            let response_time = response
                .request()
                .extensions()
                .get::<RequestStartTime>()
                .map(|start_time| {
                    let duration = start_time.0.elapsed();
                    let ms = duration.as_millis();
                    span.record("response_time_ms", &(ms as i64));
                    format!("{:.2}ms", duration.as_secs_f64() * 1000.0)
                })
                .unwrap_or_else(|| "unknown".to_string());

            // Determine log level based on status code
            let level = match status_code {
                400..=499 => Level::WARN,
                500..=599 => Level::ERROR,
                _ => Level::INFO,
            };

            // Record response information
            let uri = response.request().uri().to_string();
            let method = response.request().method().to_string();
            let status_text = status.to_string();

            match level {
                Level::WARN => {
                    tracing::warn!(
                        target: "http_response",
                        status_code = status_code,
                        status_text = %status_text,
                        uri = %uri,
                        method = %method,
                        response_time = %response_time,
                        "Warning: Client error occurred"
                    );
                }
                Level::ERROR => {
                    tracing::error!(
                        target: "http_response",
                        status_code = status_code,
                        status_text = %status_text,
                        uri = %uri,
                        method = %method,
                        response_time = %response_time,
                        "Error: Server error occurred"
                    );
                }
                _ => {
                    tracing::info!(
                        target: "http_response",
                        status_code = status_code,
                        status_text = %status_text,
                        uri = %uri,
                        method = %method,
                        response_time = %response_time,
                        "Request successful: Response returned normally"
                    );
                }
            }
        } else if let Err(error) = outcome {
            // Record error information
            let error_message = error.to_string();
            span.record("error", &error_message);

            tracing::error!(
                target: "http_error",
                error = %error_message,
                "Error occurred during request processing"
            );
        }
    }
}
