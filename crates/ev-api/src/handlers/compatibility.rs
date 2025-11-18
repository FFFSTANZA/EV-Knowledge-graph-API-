use crate::handlers::{ApiError, ApiResponse};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use ev_cache::CacheKey;
use ev_graph::{repository::CompatibilityRepository, repository::compatibility::CompatibilityResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CompatibleChargersResponse {
    vehicle_id: String,
    chargers: Vec<CompatibilityResult>,
}

#[derive(Serialize)]
pub struct CompatibleVehiclesResponse {
    charger_id: String,
    vehicles: Vec<String>,
}

#[derive(Deserialize)]
pub struct CompatibilityCheckRequest {
    vehicle_id: Uuid,
    charger_id: Uuid,
}

#[derive(Serialize)]
pub struct CompatibilityCheckResponse {
    vehicle_id: String,
    charger_id: String,
    is_compatible: bool,
}

pub async fn get_compatible_chargers(
    State(state): State<AppState>,
    Path(vehicle_id): Path<Uuid>,
) -> Result<Json<ApiResponse<CompatibleChargersResponse>>, ApiError> {
    let mut cache = state.cache();
    let cache_key = CacheKey::compatible_chargers(vehicle_id);

    // Check cache
    if let Ok(Some(cached)) = cache.get::<Vec<CompatibilityResult>>(&cache_key).await {
        return Ok(Json(ApiResponse::success(CompatibleChargersResponse {
            vehicle_id: vehicle_id.to_string(),
            chargers: cached,
        })));
    }

    // Query from graph
    let repo = CompatibilityRepository::new(state.graph_db().clone());
    let chargers = repo.find_compatible_chargers(vehicle_id).await?;

    // Cache the results
    let _ = cache.set(&cache_key, &chargers).await;

    Ok(Json(ApiResponse::success(CompatibleChargersResponse {
        vehicle_id: vehicle_id.to_string(),
        chargers,
    })))
}

pub async fn get_compatible_vehicles(
    State(state): State<AppState>,
    Path(charger_id): Path<Uuid>,
) -> Result<Json<ApiResponse<CompatibleVehiclesResponse>>, ApiError> {
    let mut cache = state.cache();
    let cache_key = CacheKey::compatible_vehicles(charger_id);

    // Check cache
    if let Ok(Some(cached)) = cache.get::<Vec<String>>(&cache_key).await {
        return Ok(Json(ApiResponse::success(CompatibleVehiclesResponse {
            charger_id: charger_id.to_string(),
            vehicles: cached,
        })));
    }

    // Query from graph
    let repo = CompatibilityRepository::new(state.graph_db().clone());
    let vehicles = repo.find_compatible_vehicles(charger_id).await?;

    // Cache the results
    let _ = cache.set(&cache_key, &vehicles).await;

    Ok(Json(ApiResponse::success(CompatibleVehiclesResponse {
        charger_id: charger_id.to_string(),
        vehicles,
    })))
}

pub async fn check_compatibility(
    State(state): State<AppState>,
    Json(payload): Json<CompatibilityCheckRequest>,
) -> Result<Json<ApiResponse<CompatibilityCheckResponse>>, ApiError> {
    let repo = CompatibilityRepository::new(state.graph_db().clone());
    let is_compatible = repo.check_compatibility(payload.vehicle_id, payload.charger_id).await?;

    Ok(Json(ApiResponse::success(CompatibilityCheckResponse {
        vehicle_id: payload.vehicle_id.to_string(),
        charger_id: payload.charger_id.to_string(),
        is_compatible,
    })))
}
