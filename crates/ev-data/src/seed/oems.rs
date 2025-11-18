use anyhow::Result;
use ev_graph::GraphDb;
use neo4rs::query;
use uuid::Uuid;

pub async fn seed_indian_oems(db: &GraphDb) -> Result<()> {
    tracing::info!("Seeding Indian OEMs...");

    let oems = vec![
        ("Tata Motors", "Tata", "India", true, true, 2021, vec!["Pune", "Sanand"]),
        ("Mahindra Electric", "Mahindra", "India", true, true, 2013, vec!["Bengaluru", "Chakan"]),
        ("Ola Electric", "Ola", "India", true, true, 2021, vec!["Bengaluru"]),
        ("Ather Energy", "Ather", "India", true, true, 2013, vec!["Hosur"]),
        ("TVS Motor", "TVS iQube", "India", true, true, 2020, vec!["Hosur"]),
        ("Bajaj Auto", "Bajaj Chetak", "India", true, true, 2019, vec!["Pune"]),
        ("MG Motor India", "MG", "UK/China", false, true, 2017, vec!["Halol"]),
        ("Hyundai", "Hyundai", "South Korea", false, true, 2019, vec!["Chennai"]),
        ("BYD India", "BYD", "China", false, false, 2023, vec![]),
    ];

    let oem_count = oems.len();

    for (name, brand, country, is_indian, has_mfg, year, plants) in oems {
        let id = Uuid::new_v4();
        let cypher = "CREATE (o:Oem {
            id: $id,
            name: $name,
            brand_name: $brand,
            country_of_origin: $country,
            is_indian_company: $is_indian,
            has_indian_manufacturing: $has_mfg,
            established_year: $year,
            manufacturing_plants_india: $plants,
            total_models: 0,
            created_at: datetime(),
            updated_at: datetime()
        })";

        db.graph()
            .run(query(cypher)
                .param("id", id.to_string())
                .param("name", name)
                .param("brand", brand)
                .param("country", country)
                .param("is_indian", is_indian)
                .param("has_mfg", has_mfg)
                .param("year", year as i64)
                .param("plants", plants))
            .await?;
    }

    tracing::info!("✓ Seeded {} OEMs", oem_count);
    Ok(())
}
