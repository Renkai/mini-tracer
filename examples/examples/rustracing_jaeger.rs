fn main() {
    use rustracing::sampler::AllSampler;
    use rustracing_jaeger::Tracer;
    use rustracing_jaeger::reporter::JaegerCompactReporter;

    // Creates a tracer
    let (span_tx, span_rx) = crossbeam_channel::bounded(10);
    let tracer = Tracer::with_sender(AllSampler, span_tx);
    {
        let span = tracer.span("sample_op").start();
        // Do something

    } // The dropped span will be sent to `span_rx`

    let span = span_rx.try_recv().unwrap();
    assert_eq!(span.operation_name(), "sample_op");

    // Reports this span to the local jaeger agent
    let reporter = JaegerCompactReporter::new("sample_service").unwrap();
    reporter.report(&[span]).unwrap();
}
