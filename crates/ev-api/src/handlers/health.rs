use crate::handlers::{ApiError, ApiResponse};
use crate::state::AppState;
use axum::{extract::State, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub environment: String,
    pub neo4j: String,
    pub redis: String,
}

#[derive(Serialize)]
pub struct StatsResponse {
    pub nodes: Vec<(String, i64)>,
}

pub async fn health_check(State(state): State<AppState>) -> Result<Json<ApiResponse<HealthResponse>>, ApiError> {
    // Check Neo4j
    let neo4j_status = match state.graph_db().health_check().await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    // Check Redis
    let mut cache = state.cache();
    let redis_status = match cache.health_check().await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let response = HealthResponse {
        status: if neo4j_status == "healthy" && redis_status == "healthy" {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        environment: state.config().environment.clone(),
        neo4j: neo4j_status.to_string(),
        redis: redis_status.to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

pub async fn stats(State(state): State<AppState>) -> Result<Json<ApiResponse<StatsResponse>>, ApiError> {
    let stats = state.graph_db()
        .get_stats()
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let response = StatsResponse {
        nodes: stats.node_counts,
    };

    Ok(Json(ApiResponse::success(response)))
}
