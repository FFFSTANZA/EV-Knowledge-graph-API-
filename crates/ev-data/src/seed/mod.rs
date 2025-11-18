mod oems;
mod vehicles;
mod chargers;
mod connectors;
mod policies;

use anyhow::Result;
use ev_graph::GraphDb;

pub async fn seed_all(db: &GraphDb) -> Result<()> {
    tracing::info!("Seeding Indian EV knowledge graph...");

    // Seed in dependency order
    connectors::seed_connectors(db).await?;
    oems::seed_indian_oems(db).await?;
    vehicles::seed_indian_vehicles(db).await?;
    chargers::seed_indian_chargers(db).await?;
    policies::seed_indian_policies(db).await?;

    Ok(())
}
