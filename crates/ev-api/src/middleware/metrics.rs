use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use metrics::{counter, histogram};
use std::time::Instant;

/// Metrics middleware to track request metrics
pub async fn track_metrics(request: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();

    // Process request
    let response = next.run(request).await;

    // Record metrics
    let duration = start.elapsed();
    let status = response.status().as_u16();

    // Log metrics (TODO: implement proper Prometheus metrics)
    tracing::debug!(
        method = ?method,
        path = %path,
        status = status,
        duration_ms = duration.as_millis(),
        "Request completed"
    );

    response
}

/// Initialize metrics exporter
pub fn init_metrics() {
    let builder = metrics_exporter_prometheus::PrometheusBuilder::new();
    builder
        .install()
        .expect("Failed to install Prometheus exporter");

    tracing::info!("Metrics exporter initialized");
}
