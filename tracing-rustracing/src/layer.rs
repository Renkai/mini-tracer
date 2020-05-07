use rustracing_jaeger::Tracer;
use tracing::Subscriber;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

pub struct RustracingLayer {
    tracer: Tracer,
}

impl<S> Layer<S> for RustracingLayer where S: Subscriber + for<'span> LookupSpan<'span> {}
