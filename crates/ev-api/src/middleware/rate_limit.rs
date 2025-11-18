use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use ev_cache::Cache;
use std::time::Duration;

/// Rate limit configuration
#[derive(Clone)]
pub struct RateLimitConfig {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 60,
            requests_per_hour: 1000,
        }
    }
}

/// Rate limiting middleware using Redis
pub async fn rate_limit(
    State(cache): State<Cache>,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    let config = RateLimitConfig::default();

    // Extract client identifier (IP address or API key)
    let client_id = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let path = request.uri().path().to_string();

    // Check rate limits
    if let Err(response) = check_rate_limit(cache.clone(), &client_id, &path, &config).await {
        return Err(response);
    }

    // Process request
    let mut response = next.run(request).await;

    // Add rate limit headers
    add_rate_limit_headers(&mut response, &config);

    Ok(response)
}

async fn check_rate_limit(
    mut cache: Cache,
    client_id: &str,
    path: &str,
    config: &RateLimitConfig,
) -> Result<(), Response> {
    // Minute-based rate limit
    let minute_key = format!("ratelimit:{}:{}:minute", path, client_id);
    let minute_count = cache
        .incr_with_ttl(&minute_key, Duration::from_secs(60))
        .await
        .unwrap_or(0);

    if minute_count > config.requests_per_minute as i64 {
        return Err(rate_limit_exceeded_response(
            config.requests_per_minute,
            60,
        ));
    }

    // Hour-based rate limit
    let hour_key = format!("ratelimit:{}:{}:hour", path, client_id);
    let hour_count = cache
        .incr_with_ttl(&hour_key, Duration::from_secs(3600))
        .await
        .unwrap_or(0);

    if hour_count > config.requests_per_hour as i64 {
        return Err(rate_limit_exceeded_response(
            config.requests_per_hour,
            3600,
        ));
    }

    Ok(())
}

fn add_rate_limit_headers(response: &mut Response, config: &RateLimitConfig) {
    let headers = response.headers_mut();

    if let Ok(limit) = HeaderValue::from_str(&config.requests_per_minute.to_string()) {
        headers.insert("X-RateLimit-Limit", limit);
    }

    // In production, you'd track actual remaining requests
    if let Ok(remaining) = HeaderValue::from_str("50") {
        headers.insert("X-RateLimit-Remaining", remaining);
    }

    if let Ok(reset) = HeaderValue::from_str("60") {
        headers.insert("X-RateLimit-Reset", reset);
    }
}

fn rate_limit_exceeded_response(limit: u32, window_secs: u64) -> Response {
    let error_body = serde_json::json!({
        "success": false,
        "error": "Rate limit exceeded",
        "error_code": "RATE_LIMIT_EXCEEDED",
        "details": {
            "limit": limit,
            "window_seconds": window_secs,
            "message": format!("You have exceeded the rate limit of {} requests per {} seconds", limit, window_secs)
        }
    });

    (
        StatusCode::TOO_MANY_REQUESTS,
        [(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        )],
        serde_json::to_string(&error_body).unwrap(),
    )
        .into_response()
}
