use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_aws::trace::{XrayIdGenerator, XrayPropagator};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use opentelemetry_semantic_conventions::resource;
use std::sync::LazyLock;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::infrastructure::config::config::AppConfig;

const APP_NAME: &str = "ec-extension-backend";

static RESOURCE: LazyLock<Resource> = LazyLock::new(|| {
    Resource::builder()
        .with_attribute(KeyValue::new(resource::SERVICE_NAME, APP_NAME))
        .build()
});

/// Initialize the OpenTelemetry tracing provider
/// https://github.com/LukeMathWalker/tracing-actix-web/blob/main/examples/opentelemetry/src/main.rs
///
/// This function sets up the OpenTelemetry tracing provider and the tracing subscriber.
/// It also creates an OTLP exporter and a tracer provider.
///
/// The tracing subscriber is configured to use the OTLP tracer and the default formatter.
///
/// The function returns the tracer provider.
///
/// # Returns
///
/// A `SdkTracerProvider` instance.
///
/// # Errors
///
/// This function will return an error if the OTLP exporter fails to be created.
pub fn init_telemetry(config: &AppConfig) -> SdkTracerProvider {
    global::set_text_map_propagator(XrayPropagator::new());

    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint(config.opentelemetry_endpoint())
        .build()
        .expect("Failed to create OTLP exporter");

    let provider = SdkTracerProvider::builder()
        .with_id_generator(XrayIdGenerator::default())
        .with_batch_exporter(otlp_exporter)
        .with_resource(RESOURCE.clone())
        .build();
    global::set_tracer_provider(provider.clone());
    let tracer = provider.tracer(APP_NAME);

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    // Create a `tracing` layer using the otlp tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .json()
        .with_current_span(true)
        .with_span_list(true);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    provider
}
