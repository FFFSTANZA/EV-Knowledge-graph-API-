use crate::handlers::{ApiError, ApiResponse};
use crate::state::AppState;
use axum::{extract::State, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct CacheStatsResponse {
    pub status: String,
    pub operations: Vec<String>,
}

#[derive(Serialize)]
pub struct GraphStatsResponse {
    pub nodes: Vec<(String, i64)>,
    pub relationships: Vec<(String, i64)>,
    pub indexes: Vec<String>,
}

/// Clear specific cache patterns
pub async fn clear_cache(State(state): State<AppState>) -> Result<Json<ApiResponse<String>>, ApiError> {
    let mut cache = state.cache();

    // Clear all vehicle caches
    let _ = cache.delete_pattern("ev:vehicle:*").await;
    let _ = cache.delete_pattern("ev:charger:*").await;
    let _ = cache.delete_pattern("ev:compatibility:*").await;

    Ok(Json(ApiResponse::success(
        "Cache cleared successfully".to_string(),
    )))
}

/// Get cache statistics
pub async fn get_cache_stats(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<CacheStatsResponse>>, ApiError> {
    // TODO: Implement actual cache stats from Redis
    let response = CacheStatsResponse {
        status: "operational".to_string(),
        operations: vec![
            "vehicle caching".to_string(),
            "charger caching".to_string(),
            "compatibility caching".to_string(),
        ],
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Get detailed graph statistics
pub async fn get_graph_stats(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<GraphStatsResponse>>, ApiError> {
    let stats = state
        .graph_db()
        .get_stats()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let response = GraphStatsResponse {
        nodes: stats.node_counts,
        relationships: vec![], // TODO: Implement relationship counts
        indexes: vec![
            "vehicle_name".to_string(),
            "vehicle_search".to_string(),
            "charger_location".to_string(),
        ],
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Rebuild graph indexes
pub async fn rebuild_indexes(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    // Reinitialize schema (which includes indexes)
    state
        .graph_db()
        .init_schema()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    Ok(Json(ApiResponse::success(
        "Indexes rebuilt successfully".to_string(),
    )))
}

/// System health check (deep)
#[derive(Serialize)]
pub struct SystemHealthResponse {
    pub overall_status: String,
    pub components: ComponentHealth,
    pub version: String,
    pub uptime_seconds: u64,
}

#[derive(Serialize)]
pub struct ComponentHealth {
    pub neo4j: HealthStatus,
    pub redis: HealthStatus,
    pub api: HealthStatus,
}

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub latency_ms: Option<f64>,
    pub details: Option<String>,
}

pub async fn system_health(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SystemHealthResponse>>, ApiError> {
    use std::time::Instant;

    // Check Neo4j
    let neo4j_start = Instant::now();
    let neo4j_healthy = state.graph_db().health_check().await.is_ok();
    let neo4j_latency = neo4j_start.elapsed().as_millis() as f64;

    // Check Redis
    let redis_start = Instant::now();
    let mut cache = state.cache();
    let redis_healthy = cache.health_check().await.is_ok();
    let redis_latency = redis_start.elapsed().as_millis() as f64;

    let response = SystemHealthResponse {
        overall_status: if neo4j_healthy && redis_healthy {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        components: ComponentHealth {
            neo4j: HealthStatus {
                status: if neo4j_healthy { "up" } else { "down" }.to_string(),
                latency_ms: Some(neo4j_latency),
                details: Some(state.config().neo4j.uri.clone()),
            },
            redis: HealthStatus {
                status: if redis_healthy { "up" } else { "down" }.to_string(),
                latency_ms: Some(redis_latency),
                details: Some(state.config().redis.url.clone()),
            },
            api: HealthStatus {
                status: "up".to_string(),
                latency_ms: None,
                details: Some(format!("port {}", state.config().server_port)),
            },
        },
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0, // TODO: Track actual uptime
    };

    Ok(Json(ApiResponse::success(response)))
}
