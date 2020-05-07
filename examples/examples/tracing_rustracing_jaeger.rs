fn main() {
    use rustracing::sampler::AllSampler;
    use rustracing_jaeger::reporter::JaegerCompactReporter;
    use rustracing_jaeger::Tracer;
    use tracing_rustracing::RustracingLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::Registry;
    use tracing::{error, span};
    use std::{thread, time};

    // Creates a tracer
    let (span_tx, span_rx) = crossbeam_channel::bounded(10);
    let tracer = Tracer::with_sender(AllSampler, span_tx);

    let layer = RustracingLayer::new(tracer);

    let subscriber = Registry::default().with(layer);
    tracing::subscriber::with_default(subscriber, || {
        let root = span!(tracing::Level::TRACE, "tracing_rustracing", work_units = 2);
        let _enter = root.enter();
        let millis = time::Duration::from_millis(1000);
        thread::sleep(millis);
        error!("This event will be logged in the root span.");
    });

    let span = span_rx.try_recv().unwrap();
    assert_eq!(span.operation_name(), "tracing_rustracing");

    //TODO: Reports this span to the local jaeger agent
    let reporter = JaegerCompactReporter::new("sample_service").unwrap();
    reporter.report(&[span]).unwrap();
}
