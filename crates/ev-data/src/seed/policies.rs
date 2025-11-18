use anyhow::Result;
use ev_graph::GraphDb;
use neo4rs::query;
use uuid::Uuid;

pub async fn seed_indian_policies(db: &GraphDb) -> Result<()> {
    tracing::info!("Seeding Indian EV policies...");

    // FAME-II (Faster Adoption and Manufacturing of Electric Vehicles)
    let fame_id = Uuid::new_v4();
    let cypher = "CREATE (p:Policy {
        id: $id,
        name: 'FAME-II',
        policy_type: 'subsidy',
        level: 'national',
        description: 'Central Government scheme to promote electric mobility in India',
        incentive_per_kwh_inr: 10000.0,
        max_incentive_cap_inr: 150000.0,
        min_battery_capacity_kwh: 15.0,
        min_range_km: 80.0,
        requires_local_manufacturing: true,
        is_active: true,
        valid_from: datetime('2019-04-01T00:00:00Z'),
        created_at: datetime(),
        updated_at: datetime()
    })";

    db.graph()
        .run(query(cypher).param("id", fame_id.to_string()))
        .await?;

    // Delhi EV Policy
    let delhi_id = Uuid::new_v4();
    let cypher_delhi = "CREATE (p:Policy {
        id: $id,
        name: 'Delhi EV Policy 2020',
        policy_type: 'subsidy',
        level: 'state',
        description: 'Delhi state incentive for electric vehicles',
        incentive_amount_inr: 30000.0,
        max_incentive_cap_inr: 150000.0,
        is_active: true,
        valid_from: datetime('2020-08-07T00:00:00Z'),
        created_at: datetime(),
        updated_at: datetime()
    })";

    db.graph()
        .run(query(cypher_delhi).param("id", delhi_id.to_string()))
        .await?;

    // TODO: Add more state-level policies:
    // - Maharashtra EV Policy
    // - Gujarat EV Policy
    // - Karnataka EV Policy
    // - Tamil Nadu EV Policy

    tracing::info!("✓ Seeded FAME-II and state policies");
    Ok(())
}
