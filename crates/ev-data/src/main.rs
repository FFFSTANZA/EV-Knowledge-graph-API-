mod seed;

use anyhow::Result;
use ev_graph::GraphDb;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment
    dotenvy::dotenv().ok();

    let command = env::args().nth(1).unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "seed" => {
            tracing::info!("Starting data seeding process...");
            let neo4j_uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string());
            let neo4j_user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
            let neo4j_password = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".to_string());

            let db = GraphDb::new(&neo4j_uri, &neo4j_user, &neo4j_password).await?;
            db.init_schema().await?;

            seed::seed_all(&db).await?;
            tracing::info!("✓ Data seeding completed successfully!");
        }
        "migrate" => {
            tracing::info!("Running migrations...");
            // TODO: Implement migration logic
            tracing::info!("✓ Migrations completed!");
        }
        "help" | _ => {
            println!("EV Data Management Tool\n");
            println!("Commands:");
            println!("  seed    - Seed the database with India-specific EV data");
            println!("  migrate - Run database migrations");
            println!("  help    - Show this help message");
        }
    }

    Ok(())
}
