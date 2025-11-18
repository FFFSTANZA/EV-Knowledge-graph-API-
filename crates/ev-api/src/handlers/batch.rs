use crate::handlers::{ApiError, ApiResponse};
use crate::state::AppState;
use crate::types::{BatchRequest, BatchResponse, BatchResult};
use axum::{extract::State, Json};
use ev_graph::CompatibilityRepository;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct VehicleCompatibilityRequest {
    pub vehicle_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct VehicleCompatibilityResponse {
    pub vehicle_id: String,
    pub compatible_chargers_count: usize,
}

/// Batch check vehicle compatibility
pub async fn batch_check_compatibility(
    State(state): State<AppState>,
    Json(request): Json<BatchRequest<VehicleCompatibilityRequest>>,
) -> Result<Json<ApiResponse<BatchResponse<VehicleCompatibilityResponse>>>, ApiError> {
    let repo = CompatibilityRepository::new(state.graph_db().clone());
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;

    for (index, item) in request.items.iter().enumerate() {
        match repo.find_compatible_chargers(item.vehicle_id).await {
            Ok(chargers) => {
                results.push(BatchResult {
                    index,
                    success: true,
                    data: Some(VehicleCompatibilityResponse {
                        vehicle_id: item.vehicle_id.to_string(),
                        compatible_chargers_count: chargers.len(),
                    }),
                    error: None,
                });
                successful += 1;
            }
            Err(e) => {
                results.push(BatchResult {
                    index,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                });
                failed += 1;
            }
        }
    }

    let response = BatchResponse {
        total: request.items.len(),
        successful,
        failed,
        results,
    };

    Ok(Json(ApiResponse::success(response)))
}

#[derive(Debug, Deserialize)]
pub struct CompatibilityCheckRequest {
    pub vehicle_id: Uuid,
    pub charger_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CompatibilityCheckResponse {
    pub vehicle_id: String,
    pub charger_id: String,
    pub is_compatible: bool,
}

/// Batch compatibility check
pub async fn batch_compatibility_check(
    State(state): State<AppState>,
    Json(request): Json<BatchRequest<CompatibilityCheckRequest>>,
) -> Result<Json<ApiResponse<BatchResponse<CompatibilityCheckResponse>>>, ApiError> {
    let repo = CompatibilityRepository::new(state.graph_db().clone());
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;

    for (index, item) in request.items.iter().enumerate() {
        match repo
            .check_compatibility(item.vehicle_id, item.charger_id)
            .await
        {
            Ok(is_compatible) => {
                results.push(BatchResult {
                    index,
                    success: true,
                    data: Some(CompatibilityCheckResponse {
                        vehicle_id: item.vehicle_id.to_string(),
                        charger_id: item.charger_id.to_string(),
                        is_compatible,
                    }),
                    error: None,
                });
                successful += 1;
            }
            Err(e) => {
                results.push(BatchResult {
                    index,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                });
                failed += 1;
            }
        }
    }

    let response = BatchResponse {
        total: request.items.len(),
        successful,
        failed,
        results,
    };

    Ok(Json(ApiResponse::success(response)))
}
