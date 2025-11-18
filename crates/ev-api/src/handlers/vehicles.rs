use crate::handlers::{ApiError, ApiResponse};
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use ev_cache::CacheKey;
use ev_graph::VehicleRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SearchQuery {
    q: String,
}

#[derive(Serialize)]
pub struct VehicleListResponse {
    vehicles: Vec<String>,
}

pub async fn list_vehicles(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<VehicleListResponse>>, ApiError> {
    // TODO: Implement pagination and actual vehicle listing
    let response = VehicleListResponse {
        vehicles: vec!["Vehicle listing coming soon".to_string()],
    };
    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_vehicle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Option<String>>>, ApiError> {
    let mut cache = state.cache();
    let cache_key = CacheKey::vehicle(id);

    // Check cache first
    if let Ok(Some(cached)) = cache.get::<String>(&cache_key).await {
        return Ok(Json(ApiResponse::success(Some(cached))));
    }

    // Query from graph
    let repo = VehicleRepository::new(state.graph_db().clone());
    let vehicle = repo.find_by_id(id).await?;

    // Cache the result
    if let Some(ref v) = vehicle {
        let _ = cache.set(&cache_key, v).await;
    }

    Ok(Json(ApiResponse::success(vehicle)))
}

pub async fn search_vehicles(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<ApiResponse<VehicleListResponse>>, ApiError> {
    let mut cache = state.cache();
    let cache_key = CacheKey::vehicle_search(&query.q);

    // Check cache
    if let Ok(Some(cached)) = cache.get::<Vec<String>>(&cache_key).await {
        return Ok(Json(ApiResponse::success(VehicleListResponse { vehicles: cached })));
    }

    // Search from graph
    let repo = VehicleRepository::new(state.graph_db().clone());
    let vehicles = repo.search_by_name(&query.q).await?;

    // Cache the results
    let _ = cache.set(&cache_key, &vehicles).await;

    Ok(Json(ApiResponse::success(VehicleListResponse { vehicles })))
}

pub async fn get_vehicles_by_oem(
    State(state): State<AppState>,
    Path(oem_id): Path<Uuid>,
) -> Result<Json<ApiResponse<VehicleListResponse>>, ApiError> {
    let mut cache = state.cache();
    let cache_key = CacheKey::vehicle_by_oem(oem_id);

    // Check cache
    if let Ok(Some(cached)) = cache.get::<Vec<String>>(&cache_key).await {
        return Ok(Json(ApiResponse::success(VehicleListResponse { vehicles: cached })));
    }

    // Query from graph
    let repo = VehicleRepository::new(state.graph_db().clone());
    let vehicles = repo.find_by_oem(oem_id).await?;

    // Cache the results
    let _ = cache.set(&cache_key, &vehicles).await;

    Ok(Json(ApiResponse::success(VehicleListResponse { vehicles })))
}
