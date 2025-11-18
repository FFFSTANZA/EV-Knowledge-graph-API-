use anyhow::Result;
use ev_graph::GraphDb;
use neo4rs::query;

pub async fn seed_connectors(db: &GraphDb) -> Result<()> {
    tracing::info!("Seeding connector standards...");

    let connectors = vec![
        ("TYPE2", "Type-2", "IEC 62196-2", 480.0, 80.0, 22.0, false),
        ("CCS2", "CCS Type 2 (Combined Charging System)", "IEC 62196-3", 920.0, 200.0, 350.0, true),
        ("CHADEMO", "CHAdeMO", "CHAdeMO", 500.0, 125.0, 62.5, true),
        ("GB_T", "GB/T", "GB/T 20234", 750.0, 250.0, 187.5, true),
        ("BHARAT_DC001", "Bharat DC001", "Bharat EV Specification", 500.0, 200.0, 100.0, true),
        ("BHARAT_AC001", "Bharat AC001", "Bharat EV Specification", 230.0, 32.0, 7.4, false),
    ];

    let connector_count = connectors.len();

    for (conn_type, name, standard, voltage, current, power, is_dc) in connectors {
        let cypher = "CREATE (c:Connector {
            id: randomUUID(),
            connector_type: $connector_type,
            name: $name,
            standard_name: $standard,
            max_voltage_v: $voltage,
            max_current_a: $current,
            max_power_kw: $power,
            is_dc: $is_dc
        })";

        db.graph()
            .run(query(cypher)
                .param("connector_type", conn_type)
                .param("name", name)
                .param("standard", standard)
                .param("voltage", voltage)
                .param("current", current)
                .param("power", power)
                .param("is_dc", is_dc))
            .await?;
    }

    tracing::info!("✓ Seeded {} connector types", connector_count);
    Ok(())
}
