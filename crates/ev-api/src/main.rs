mod config;
mod handlers;
mod middleware;
mod routes;
mod state;
mod types;

use anyhow::Result;
use config::AppConfig;
use state::AppState;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ev_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::load()?;

    tracing::info!("Starting EV Knowledge Graph API...");
    tracing::info!("Environment: {}", config.environment);

    // Initialize app state
    let state = AppState::new(config.clone()).await?;

    // Initialize database schema
    state.graph_db().init_schema().await
        .expect("Failed to initialize graph schema");

    // Build router
    let app = routes::create_router(state)
        .layer(axum::middleware::from_fn(middleware::add_branding_headers))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    tracing::info!("🚀 Server listening on http://{}", addr);
    tracing::info!("📚 API documentation: http://{}/docs", addr);
    tracing::info!("🏥 Health check: http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
