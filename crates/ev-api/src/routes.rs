use crate::handlers;
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health & Status
        .route("/health", get(handlers::health::health_check))
        .route("/stats", get(handlers::health::stats))

        // API v1 routes
        .nest("/api/v1", api_v1_routes())

        // Documentation
        .route("/docs", get(handlers::docs::api_docs))

        .with_state(state)
}

fn api_v1_routes() -> Router<AppState> {
    Router::new()
        // Vehicles
        .route("/vehicles", get(handlers::vehicles::list_vehicles))
        .route("/vehicles/:id", get(handlers::vehicles::get_vehicle))
        .route("/vehicles/search", get(handlers::vehicles::search_vehicles))
        .route("/vehicles/oem/:oem_id", get(handlers::vehicles::get_vehicles_by_oem))

        // Chargers
        .route("/chargers", get(handlers::chargers::list_chargers))
        .route("/chargers/:id", get(handlers::chargers::get_charger))
        .route("/chargers/city/:state/:city", get(handlers::chargers::get_chargers_by_city))
        .route("/chargers/nearby", get(handlers::chargers::get_chargers_nearby))

        // Compatibility
        .route("/compatibility/vehicle/:vehicle_id", get(handlers::compatibility::get_compatible_chargers))
        .route("/compatibility/charger/:charger_id", get(handlers::compatibility::get_compatible_vehicles))
        .route("/compatibility/check", post(handlers::compatibility::check_compatibility))

        // Graph queries
        .route("/query/fame-eligible/:state", get(handlers::queries::get_fame_eligible_vehicles))
        .route("/query/indian-oems", get(handlers::queries::get_indian_oem_vehicles))
        .route("/query/network-coverage/:state", get(handlers::queries::get_network_coverage))

        // Recommendations & Inference
        .route("/recommendations/vehicles", get(handlers::recommendations::recommend_vehicles))
        .route("/recommendations/chargers", get(handlers::recommendations::recommend_chargers))
}
