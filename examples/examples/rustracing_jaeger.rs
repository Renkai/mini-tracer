fn main() {
    use rustracing::sampler::AllSampler;
    use rustracing_jaeger::reporter::JaegerCompactReporter;
    use rustracing_jaeger::Tracer;
    use std::{thread, time};

    // Creates a tracer
    let (span_tx, span_rx) = crossbeam_channel::bounded(10);
    let tracer = Tracer::with_sender(AllSampler, span_tx);
    {
        let _span = tracer.span("sample_op").start();
        // Do something
        let millis = time::Duration::from_millis(1000);
        thread::sleep(millis);
    } // The dropped span will be sent to `span_rx`

    let span = span_rx.try_recv().unwrap();
    assert_eq!(span.operation_name(), "sample_op");

    // Reports this span to the local jaeger agent
    let reporter = JaegerCompactReporter::new("sample_service").unwrap();
    reporter.report(&[span]).unwrap();
}
