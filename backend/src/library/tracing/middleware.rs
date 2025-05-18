// Dependencies required for X-Ray and OpenTelemetry integration:
// Add the following dependencies to Cargo.toml:
// ```
// [dependencies]
// # Existing dependencies...
// opentelemetry = { version = "0.29", features = ["rt-tokio"] }
// opentelemetry-aws = "0.8"
// tracing-opentelemetry = "0.30.0"
// chrono = "0.4"
// ```
use actix_http::header::HeaderMap;
use actix_http::header::HeaderName;
use actix_http::header::HeaderValue;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::Next;
use actix_web::Error;
use actix_web::HttpMessage;
use chrono::Utc;
use opentelemetry::global;
use opentelemetry::propagation::Extractor;
use opentelemetry::trace::{TraceContextExt, TraceId};
use std::collections::HashMap;
use std::time::Instant;
use tracing::Level;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::infrastructure::config::config::AppConfig;

/// Request start time holder for response time measurement
#[derive(Debug, Clone)]
struct RequestStartTime(Instant);

/// Error type for trace ID extraction
#[derive(Debug)]
enum TraceIdError {
    Missing,
}

// X-Ray trace header constants
const X_AMZN_TRACE_ID: &str = "x-amzn-trace-id";
const ROOT_PREFIX: &str = "Root=";

/// OpenTelemetry header Extractor implementation
struct HeaderExtractor<'a> {
    headers: &'a HeaderMap,
}

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.headers.keys().map(|k| k.as_str()).collect()
    }
}

/// Extracts X-Ray trace ID from request headers or OpenTelemetry context
/// X-Ray trace header format: "Root=1-5f84c596-5c35c1dba9b2147a1cce26b0;Parent=c2c789fe1929327f;Sampled=1"
fn extract_xray_trace_id(headers: &HeaderMap) -> Result<String, TraceIdError> {
    let extractor = HeaderExtractor { headers };

    let context = global::get_text_map_propagator(|propagater| propagater.extract(&extractor));
    let binding = context.span();
    let span_context = binding.span_context();

    // Try to extract directly from X-Ray header
    if let Some(xray_header) = headers.get(X_AMZN_TRACE_ID) {
        if let Ok(header_str) = xray_header.to_str() {
            tracing::debug!("X-Ray header found: {}", header_str);
            for part in header_str.split(';') {
                if let Some(trace_id) = part.trim().strip_prefix(ROOT_PREFIX) {
                    // X-Ray trace ID format is "1-[8 hex digits for time]-[24 hex digits for random]"
                    // The total length should be 35 (including hyphens)
                    if trace_id.len() >= 35 && trace_id.starts_with("1-") {
                        tracing::debug!("Valid X-Ray trace ID found: {}", trace_id);
                        return Ok(trace_id.to_string());
                    }
                }
            }
        }
    }

    // Get trace ID from OTel context
    if span_context.is_valid() {
        let trace_id = span_context.trace_id();
        tracing::debug!("OTel trace ID: {}", trace_id);
        if trace_id != TraceId::INVALID {
            // Convert to X-Ray format
            let timestamp = format!("{:08x}", Utc::now().timestamp());
            let identifier = &trace_id.to_string()[0..24]; // Use first 24 characters
            let xray_trace_id = format!("1-{}-{}", timestamp, identifier);
            tracing::debug!("Formatted X-Ray trace ID: {}", xray_trace_id);
            return Ok(xray_trace_id);
        }
    }

    Err(TraceIdError::Missing)
}

/// Get a properly formatted trace ID to be included in logs and metrics
fn format_for_logging(trace_id: &str) -> String {
    if trace_id.len() >= 35 && trace_id.starts_with("1-") {
        // Already in X-Ray format
        return trace_id.to_string();
    } else if trace_id.len() >= 32 {
        // It's an OpenTelemetry trace ID, convert to X-Ray format
        let timestamp = format!("{:08x}", Utc::now().timestamp());
        let identifier = &trace_id[0..24]; // Use first 24 chars
        return format!("1-{}-{}", timestamp, identifier);
    }

    trace_id.to_string()
}

pub struct XRayRootSpanBuilder;

impl tracing_actix_web::RootSpanBuilder for XRayRootSpanBuilder {
    fn on_request_start(request: &actix_web::dev::ServiceRequest) -> tracing::Span {
        let trace_id =
            extract_xray_trace_id(request.headers()).unwrap_or_else(|_| "unavailable".to_string());
        let formatted_trace_id = format_for_logging(&trace_id);

        let span = tracing_actix_web::root_span!(
            request,
            xray.trace_id = %formatted_trace_id,
        );

        // Trace IDs set in the previous stage (ELB, etc.) are set as parents so that they can be tied together
        let extractor = HeaderExtractor {
            headers: request.headers(),
        };
        let parent_context =
            global::get_text_map_propagator(|propagater| propagater.extract(&extractor));
        OpenTelemetrySpanExt::set_parent(&span, parent_context);

        request
            .extensions_mut()
            .insert(RequestStartTime(Instant::now()));
        request.extensions_mut().insert(trace_id);

        span
    }

    fn on_request_end<B: actix_web::body::MessageBody>(
        span: tracing::Span,
        outcome: &Result<actix_web::dev::ServiceResponse<B>, Error>,
    ) {
        tracing_actix_web::DefaultRootSpanBuilder::on_request_end(span.clone(), outcome);

        if let Ok(response) = outcome {
            let health_check_path = response
                .request()
                .extensions()
                .get::<AppConfig>()
                .map(|config| config.health_check_path().clone())
                .unwrap_or_else(|| "/health".to_string());

            if response.request().path() == health_check_path {
                return;
            }

            let status = response.status();
            let status_code = status.as_u16();
            span.record("status_code", &status_code);

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

            let level = match status_code {
                400..=499 => Level::WARN,
                500..=599 => Level::ERROR,
                _ => Level::INFO,
            };

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

const TRACE_ID_HEADER_NAME: &str = "x-trace-id";

/// Sets the trace ID in the response headers
pub async fn set_trace_id_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let mut response = next.call(req).await?;

    let trace_id = response
        .request()
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_else(|| "unavailable".to_string());
    let formatted_trace_id = format_for_logging(&trace_id);

    response.response_mut().headers_mut().insert(
        HeaderName::from_static(TRACE_ID_HEADER_NAME),
        HeaderValue::from_str(&formatted_trace_id).unwrap_or(HeaderValue::from_static("")),
    );

    Ok(response)
}
