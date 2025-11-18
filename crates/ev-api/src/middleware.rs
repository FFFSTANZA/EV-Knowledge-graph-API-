pub mod rate_limit;
pub mod auth;
pub mod metrics;

use axum::{
    body::Body,
    http::{Request, Response, HeaderValue},
    middleware::Next,
};
use uuid::Uuid;

/// Add Folonite branding headers to all responses
pub async fn add_branding_headers(
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    // Generate unique request ID
    let request_id = Uuid::new_v4().to_string();

    // Add request ID to extensions for logging
    let mut request = request;
    request.extensions_mut().insert(request_id.clone());

    // Process the request
    let mut response = next.run(request).await;

    // Add branding headers
    let headers = response.headers_mut();

    headers.insert("X-Powered-By", HeaderValue::from_static("Folonite"));
    headers.insert("X-Folonite-Version", HeaderValue::from_static("2026-01"));

    if let Ok(req_id) = HeaderValue::from_str(&request_id) {
        headers.insert("X-Folonite-Request-ID", req_id);
    }

    response
}
