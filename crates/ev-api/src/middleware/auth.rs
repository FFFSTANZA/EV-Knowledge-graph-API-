use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

/// API key authentication middleware
/// For now, this is a placeholder. In production, validate against database/cache
pub async fn require_api_key(request: Request, next: Next) -> Result<Response, impl IntoResponse> {
    // Check for API key in header
    let api_key = request
        .headers()
        .get("X-API-Key")
        .and_then(|h| h.to_str().ok());

    // For now, accept any key (TODO: implement proper validation)
    if api_key.is_some() {
        Ok(next.run(request).await)
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            "Missing API key. Include X-API-Key header.",
        ))
    }
}

/// Optional API key authentication (allows anonymous access)
pub async fn optional_api_key(mut request: Request, next: Next) -> Response {
    // Extract and validate API key if present
    if let Some(api_key) = request.headers().get("X-API-Key").and_then(|h| h.to_str().ok()).map(|s| s.to_string()) {
        // TODO: Validate API key and load user context
        request.extensions_mut().insert(api_key);
    }

    next.run(request).await
}
