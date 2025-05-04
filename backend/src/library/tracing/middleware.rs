// Dependencies required for X-Ray and OpenTelemetry integration:
// Add the following dependencies to Cargo.toml:
// ```
// [dependencies]
// # Existing dependencies...
// opentelemetry = { version = "0.19", features = ["rt-tokio"] }
// opentelemetry-aws = "0.8"
// tracing-opentelemetry = "0.19"
// chrono = "0.4"
// ```

use actix_web::Error;
use actix_web::HttpMessage;
use chrono::Utc;
use opentelemetry::trace::TraceContextExt;
use opentelemetry::Context;
use std::collections::HashMap;
use std::time::Instant;
use tracing::Level;

/// Request start time holder for response time measurement
#[derive(Debug, Clone)]
struct RequestStartTime(Instant);

/// Error type for trace ID extraction
#[derive(Debug)]
enum TraceIdError {
    Missing,
}

// X-Ray trace header constants
const X_AMZN_TRACE_ID: &str = "X-Amzn-Trace-Id";
const ROOT_PREFIX: &str = "Root=";

/// Extracts X-Ray trace ID from request headers or OpenTelemetry context
/// X-Ray trace header format: "Root=1-5f84c596-5c35c1dba9b2147a1cce26b0;Parent=c2c789fe1929327f;Sampled=1"
fn extract_xray_trace_id(request: &actix_web::dev::ServiceRequest) -> Result<String, TraceIdError> {
    // Check for X-Ray trace header
    if let Some(xray_header) = request.headers().get(X_AMZN_TRACE_ID) {
        if let Ok(header_str) = xray_header.to_str() {
            // Extract the Root part (1-5f84c596-5c35c1dba9b2147a1cce26b0)
            for part in header_str.split(';') {
                if let Some(trace_id) = part.trim().strip_prefix(ROOT_PREFIX) {
                    // X-Ray trace ID format is "1-[8 hex digits for time]-[24 hex digits for random]"
                    // The total length should be 35 (including hyphens)
                    if trace_id.len() >= 35 && trace_id.starts_with("1-") {
                        // Additional validation could be done here (hex digits, etc.)
                        return Ok(trace_id.to_string());
                    }
                }
            }
        }
    }

    // Get trace ID from OpenTelemetry context
    let current_context = Context::current();
    let span = current_context.span();
    let span_context = span.span_context();
    if span_context.is_valid() {
        // Convert OpenTelemetry trace ID to X-Ray format
        // OpenTelemetry uses a 16-byte (32 hex chars) format, while X-Ray uses "1-timestamp-identifier"
        let otel_trace_id = span_context.trace_id().to_string();
        if otel_trace_id.len() >= 32 {
            // Extract timestamp (first 8 chars) and remaining identifier
            let timestamp = format!("{:08x}", Utc::now().timestamp());
            let identifier = &otel_trace_id[0..24]; // Use first 24 chars of OTel trace ID
            let xray_trace_id = format!("1-{}-{}", timestamp, identifier);
            return Ok(xray_trace_id);
        }
    }

    Err(TraceIdError::Missing)
}

/// Get a properly formatted trace ID to be included in logs and metrics
fn format_for_logging(trace_id: &str) -> String {
    // Ensure the trace ID is in X-Ray format when displayed in logs
    if trace_id.len() >= 35 && trace_id.starts_with("1-") {
        // Already in X-Ray format
        return trace_id.to_string();
    } else if trace_id.len() >= 32 {
        // It's an OpenTelemetry trace ID, convert to X-Ray format
        let timestamp = format!("{:08x}", Utc::now().timestamp());
        let identifier = &trace_id[0..24]; // Use first 24 chars
        return format!("1-{}-{}", timestamp, identifier);
    }

    // Return as is if we can't recognize the format
    trace_id.to_string()
}

pub struct CustomRootSpanBuilder;

impl tracing_actix_web::RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(request: &actix_web::dev::ServiceRequest) -> tracing::Span {
        // Extract X-Ray trace ID from headers if present
        let trace_id = extract_xray_trace_id(request).unwrap_or_else(|_| "unavailable".to_string());
        let formatted_trace_id = format_for_logging(&trace_id);

        // Create a span with standard fields and add X-Ray trace ID
        let span = tracing_actix_web::root_span!(
            request,
            xray.trace_id = %formatted_trace_id,
        );

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
            let headers = response
                .request()
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                .collect::<HashMap<String, String>>();

            match level {
                Level::WARN => {
                    tracing::warn!(
                        target: "http_response",
                        status_code = status_code,
                        status_text = %status_text,
                        uri = %uri,
                        method = %method,
                        headers = ?headers,
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
                        headers = ?headers,
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
                        headers = ?headers,
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
