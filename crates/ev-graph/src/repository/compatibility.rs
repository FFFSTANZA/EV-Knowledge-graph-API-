use crate::connection::GraphDb;
use crate::schema::{labels, relationships};
use ev_core::{Result, Error, types::ConnectorType};
use neo4rs::query;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct CompatibilityRepository {
    db: GraphDb,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompatibilityResult {
    pub charger_id: String,
    pub charger_name: String,
    pub connector_type: String,
    pub max_power_kw: f64,
    pub city: String,
    pub state: String,
    pub distance_km: Option<f64>,
}

impl CompatibilityRepository {
    pub fn new(db: GraphDb) -> Self {
        Self { db }
    }

    /// Find all chargers compatible with a specific vehicle
    pub async fn find_compatible_chargers(&self, vehicle_id: Uuid) -> Result<Vec<CompatibilityResult>> {
        let cypher = format!(
            "MATCH (v:{} {{id: $vehicle_id}})-[:{}]->(conn:{})
             MATCH (c:{})-[:{}]->(conn)
             WHERE c.is_operational = true
             RETURN DISTINCT c.id as charger_id,
                    c.name as charger_name,
                    conn.connector_type as connector_type,
                    c.max_power_kw as max_power_kw,
                    c.city as city,
                    c.state as state
             ORDER BY c.city, c.max_power_kw DESC",
            labels::VEHICLE,
            relationships::SUPPORTS_CONNECTOR,
            labels::CONNECTOR,
            labels::CHARGER,
            relationships::REQUIRES_CONNECTOR
        );

        let mut result = self.db.graph()
            .execute(query(&cypher).param("vehicle_id", vehicle_id.to_string()))
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let mut chargers = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            if let (Ok(charger_id), Ok(charger_name), Ok(connector_type), Ok(max_power_kw), Ok(city), Ok(state)) = (
                row.get::<String>("charger_id"),
                row.get::<String>("charger_name"),
                row.get::<String>("connector_type"),
                row.get::<f64>("max_power_kw"),
                row.get::<String>("city"),
                row.get::<String>("state"),
            ) {
                chargers.push(CompatibilityResult {
                    charger_id,
                    charger_name,
                    connector_type,
                    max_power_kw,
                    city,
                    state,
                    distance_km: None,
                });
            }
        }

        Ok(chargers)
    }

    /// Find all vehicles compatible with a specific charger
    pub async fn find_compatible_vehicles(&self, charger_id: Uuid) -> Result<Vec<String>> {
        let cypher = format!(
            "MATCH (c:{} {{id: $charger_id}})-[:{}]->(conn:{})
             MATCH (v:{})-[:{}]->(conn)
             WHERE v.is_active = true
             RETURN DISTINCT v.id as id, v.name as name, v.model_code as model_code
             ORDER BY v.name",
            labels::CHARGER,
            relationships::REQUIRES_CONNECTOR,
            labels::CONNECTOR,
            labels::VEHICLE,
            relationships::SUPPORTS_CONNECTOR
        );

        let mut result = self.db.graph()
            .execute(query(&cypher).param("charger_id", charger_id.to_string()))
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let mut vehicles = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            if let (Ok(id), Ok(name), Ok(model_code)) = (
                row.get::<String>("id"),
                row.get::<String>("name"),
                row.get::<String>("model_code"),
            ) {
                vehicles.push(format!("{}: {} ({})", id, name, model_code));
            }
        }

        Ok(vehicles)
    }

    /// Find vehicles that support specific connector with minimum power
    pub async fn find_vehicles_by_connector(&self, connector_type: ConnectorType, min_power_kw: f32) -> Result<Vec<String>> {
        let cypher = format!(
            "MATCH (v:{})-[s:{}]->(conn:{} {{connector_type: $connector_type}})
             WHERE v.is_active = true
             RETURN v.id as id, v.name as name, v.power_kw as power
             ORDER BY v.power_kw DESC",
            labels::VEHICLE,
            relationships::SUPPORTS_CONNECTOR,
            labels::CONNECTOR
        );

        let mut result = self.db.graph()
            .execute(query(&cypher)
                .param("connector_type", format!("{:?}", connector_type)))
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let mut vehicles = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            if let (Ok(id), Ok(name), Ok(power)) = (
                row.get::<String>("id"),
                row.get::<String>("name"),
                row.get::<f64>("power"),
            ) {
                if power >= min_power_kw as f64 {
                    vehicles.push(format!("{}: {} ({}kW)", id, name, power));
                }
            }
        }

        Ok(vehicles)
    }

    /// Check if a specific vehicle-charger combination is compatible
    pub async fn check_compatibility(&self, vehicle_id: Uuid, charger_id: Uuid) -> Result<bool> {
        let cypher = format!(
            "MATCH (v:{} {{id: $vehicle_id}})-[:{}]->(conn:{})
             MATCH (c:{} {{id: $charger_id}})-[:{}]->(conn)
             RETURN count(*) > 0 as is_compatible",
            labels::VEHICLE,
            relationships::SUPPORTS_CONNECTOR,
            labels::CONNECTOR,
            labels::CHARGER,
            relationships::REQUIRES_CONNECTOR
        );

        let mut result = self.db.graph()
            .execute(query(&cypher)
                .param("vehicle_id", vehicle_id.to_string())
                .param("charger_id", charger_id.to_string()))
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        if let Ok(Some(row)) = result.next().await {
            if let Ok(is_compatible) = row.get::<bool>("is_compatible") {
                return Ok(is_compatible);
            }
        }

        Ok(false)
    }
}
