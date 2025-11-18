use crate::handlers::{ApiError, ApiResponse};
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use ev_cache::CacheKey;
use ev_graph::ChargerRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NearbyQuery {
    lat: f64,
    lon: f64,
    radius: Option<f64>,
}

#[derive(Serialize)]
pub struct ChargerListResponse {
    chargers: Vec<String>,
}

pub async fn list_chargers(
    State(_state): State<AppState>,
) -> Result<Json<ApiResponse<ChargerListResponse>>, ApiError> {
    let response = ChargerListResponse {
        chargers: vec!["Charger listing coming soon".to_string()],
    };
    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_charger(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    Ok(Json(ApiResponse::success("Charger details coming soon".to_string())))
}

pub async fn get_chargers_by_city(
    State(state): State<AppState>,
    Path((state_name, city)): Path<(String, String)>,
) -> Result<Json<ApiResponse<ChargerListResponse>>, ApiError> {
    let mut cache = state.cache();
    let cache_key = CacheKey::chargers_by_city(&city, &state_name);

    // Check cache
    if let Ok(Some(cached)) = cache.get::<Vec<String>>(&cache_key).await {
        return Ok(Json(ApiResponse::success(ChargerListResponse { chargers: cached })));
    }

    // Query from graph
    let repo = ChargerRepository::new(state.graph_db().clone());
    let chargers = repo.find_by_city(&city, &state_name).await?;

    // Cache the results
    let _ = cache.set(&cache_key, &chargers).await;

    Ok(Json(ApiResponse::success(ChargerListResponse { chargers })))
}

pub async fn get_chargers_nearby(
    State(state): State<AppState>,
    Query(query): Query<NearbyQuery>,
) -> Result<Json<ApiResponse<ChargerListResponse>>, ApiError> {
    let radius_km = query.radius.unwrap_or(10.0);

    let mut cache = state.cache();
    let cache_key = CacheKey::chargers_nearby(query.lat, query.lon, radius_km);

    // Check cache
    if let Ok(Some(cached)) = cache.get::<Vec<String>>(&cache_key).await {
        return Ok(Json(ApiResponse::success(ChargerListResponse { chargers: cached })));
    }

    // Query from graph
    let repo = ChargerRepository::new(state.graph_db().clone());
    let chargers = repo.find_nearby(query.lat, query.lon, radius_km).await?;

    // Cache the results
    let _ = cache.set(&cache_key, &chargers).await;

    Ok(Json(ApiResponse::success(ChargerListResponse { chargers })))
}
