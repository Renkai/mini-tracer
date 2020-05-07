use rustracing_jaeger::Tracer;
use tracing::Subscriber;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::layer::{Context,Layer};
use tracing_core::span;

pub struct RustracingLayer {
    tracer: Tracer,
}

impl RustracingLayer {
    pub fn new(tracer: Tracer) -> Self {
        RustracingLayer {
            tracer
        }
    }
}

impl<S> Layer<S> for RustracingLayer where S: Subscriber + for<'span> LookupSpan<'span> {
    fn new_span(&self, _attrs: &span::Attributes<'_>, id: &span::Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).expect("Span not found, this is a bug");
        let mut extensions = span.extensions_mut();
        let span = self.tracer.span(span.name()).start();
        extensions.insert(span);
    }
}
