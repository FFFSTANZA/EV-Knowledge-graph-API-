use crate::connection::GraphDb;
use crate::schema::{labels, relationships};
use ev_core::{models::*, Result, Error};
use neo4rs::query;
use uuid::Uuid;

pub struct ChargerRepository {
    db: GraphDb,
}

impl ChargerRepository {
    pub fn new(db: GraphDb) -> Self {
        Self { db }
    }

    /// Create a new charger in the graph
    pub async fn create(&self, charger: &Charger) -> Result<()> {
        let cypher = format!(
            "CREATE (c:{} {{
                id: $id,
                name: $name,
                operator_id: $operator_id,
                charging_type: $charging_type,
                max_power_kw: $max_power_kw,
                number_of_ports: $number_of_ports,
                is_operational: $is_operational,
                is_public: $is_public,
                access_type: $access_type,
                address: $address,
                city: $city,
                state: $state,
                pincode: $pincode,
                latitude: $latitude,
                longitude: $longitude,
                created_at: datetime($created_at),
                updated_at: datetime($updated_at)
            }})",
            labels::CHARGER
        );

        let q = query(&cypher)
            .param("id", charger.id.to_string())
            .param("name", charger.name.clone())
            .param("operator_id", charger.operator_id.to_string())
            .param("charging_type", format!("{:?}", charger.charging_type))
            .param("max_power_kw", charger.max_power_kw)
            .param("number_of_ports", charger.number_of_ports as i64)
            .param("is_operational", charger.is_operational)
            .param("is_public", charger.is_public)
            .param("access_type", format!("{:?}", charger.access_type))
            .param("address", charger.location.address.clone())
            .param("city", charger.location.city.clone())
            .param("state", format!("{:?}", charger.location.state))
            .param("pincode", charger.location.pincode.clone())
            .param("latitude", charger.location.latitude)
            .param("longitude", charger.location.longitude)
            .param("created_at", charger.created_at.to_rfc3339())
            .param("updated_at", charger.updated_at.to_rfc3339());

        self.db.graph()
            .run(q)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        // Link to connectors
        for connector_type in &charger.supported_connectors {
            self.link_to_connector(charger.id, connector_type).await?;
        }

        Ok(())
    }

    /// Link charger to connector
    async fn link_to_connector(&self, charger_id: Uuid, connector_type: &ev_core::types::ConnectorType) -> Result<()> {
        let cypher = format!(
            "MATCH (c:{} {{id: $charger_id}})
             MATCH (conn:{} {{connector_type: $connector_type}})
             CREATE (c)-[:{}]->(conn)",
            labels::CHARGER, labels::CONNECTOR, relationships::REQUIRES_CONNECTOR
        );

        let q = query(&cypher)
            .param("charger_id", charger_id.to_string())
            .param("connector_type", format!("{:?}", connector_type));

        self.db.graph()
            .run(q)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    /// Find chargers in a specific city
    pub async fn find_by_city(&self, city: &str, state: &str) -> Result<Vec<String>> {
        let cypher = format!(
            "MATCH (c:{})
             WHERE c.city = $city AND c.state = $state AND c.is_operational = true
             RETURN c.id as id, c.name as name, c.address as address, c.max_power_kw as power
             ORDER BY c.name",
            labels::CHARGER
        );

        let mut result = self.db.graph()
            .execute(query(&cypher)
                .param("city", city)
                .param("state", state))
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let mut chargers = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            if let (Ok(id), Ok(name), Ok(address), Ok(power)) = (
                row.get::<String>("id"),
                row.get::<String>("name"),
                row.get::<String>("address"),
                row.get::<f64>("power"),
            ) {
                chargers.push(format!("{}: {} - {} ({}kW)", id, name, address, power));
            }
        }

        Ok(chargers)
    }

    /// Find chargers near a location (within radius in km)
    pub async fn find_nearby(&self, latitude: f64, longitude: f64, radius_km: f64) -> Result<Vec<String>> {
        let cypher = format!(
            "MATCH (c:{})
             WHERE c.is_operational = true
             WITH c, point({{latitude: c.latitude, longitude: c.longitude}}) as charger_point,
                  point({{latitude: $lat, longitude: $lon}}) as search_point
             WHERE distance(charger_point, search_point) <= $radius
             RETURN c.id as id, c.name as name, c.city as city,
                    distance(charger_point, search_point) as distance
             ORDER BY distance
             LIMIT 50",
            labels::CHARGER
        );

        let mut result = self.db.graph()
            .execute(query(&cypher)
                .param("lat", latitude)
                .param("lon", longitude)
                .param("radius", radius_km * 1000.0)) // Convert to meters
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let mut chargers = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            if let (Ok(id), Ok(name), Ok(city), Ok(distance)) = (
                row.get::<String>("id"),
                row.get::<String>("name"),
                row.get::<String>("city"),
                row.get::<f64>("distance"),
            ) {
                chargers.push(format!("{}: {} in {} ({:.1}km away)", id, name, city, distance / 1000.0));
            }
        }

        Ok(chargers)
    }
}
