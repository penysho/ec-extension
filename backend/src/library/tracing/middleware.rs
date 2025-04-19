use actix_web::Error;

pub struct CustomRootSpanBuilder;

impl tracing_actix_web::RootSpanBuilder for CustomRootSpanBuilder {
    fn on_request_start(request: &actix_web::dev::ServiceRequest) -> tracing::Span {
        // Create a span with the standard fields when the request starts.
        tracing_actix_web::root_span!(request)
    }

    fn on_request_end<B: actix_web::body::MessageBody>(
        span: tracing::Span,
        outcome: &Result<actix_web::dev::ServiceResponse<B>, Error>,
    ) {
        // Capture the standard fields when the request finishes.
        tracing_actix_web::DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}
