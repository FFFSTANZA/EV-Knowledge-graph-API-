use crate::connection::GraphDb;
use crate::schema::{labels, relationships};
use ev_core::{models::*, types::*, Result, Error};
use neo4rs::query;
use uuid::Uuid;

pub struct VehicleRepository {
    db: GraphDb,
}

impl VehicleRepository {
    pub fn new(db: GraphDb) -> Self {
        Self { db }
    }

    /// Create a new vehicle in the graph
    pub async fn create(&self, vehicle: &Vehicle) -> Result<()> {
        let cypher = format!(
            "CREATE (v:{} {{
                id: $id,
                name: $name,
                model_code: $model_code,
                oem_id: $oem_id,
                vehicle_type: $vehicle_type,
                segment: $segment,
                is_active: $is_active,
                range_km: $range_km,
                top_speed_kmph: $top_speed_kmph,
                power_kw: $power_kw,
                torque_nm: $torque_nm,
                created_at: datetime($created_at),
                updated_at: datetime($updated_at)
            }})",
            labels::VEHICLE
        );

        let q = query(&cypher)
            .param("id", vehicle.id.to_string())
            .param("name", vehicle.name.clone())
            .param("model_code", vehicle.model_code.clone())
            .param("oem_id", vehicle.oem_id.to_string())
            .param("vehicle_type", format!("{:?}", vehicle.vehicle_type))
            .param("segment", format!("{:?}", vehicle.segment))
            .param("is_active", vehicle.is_active)
            .param("range_km", vehicle.range.value)
            .param("top_speed_kmph", vehicle.top_speed_kmph.unwrap_or(0.0))
            .param("power_kw", vehicle.power_output.value)
            .param("torque_nm", vehicle.torque_nm.unwrap_or(0.0))
            .param("created_at", vehicle.created_at.to_rfc3339())
            .param("updated_at", vehicle.updated_at.to_rfc3339());

        self.db.graph()
            .run(q)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        // Link to OEM
        self.link_to_oem(vehicle.id, vehicle.oem_id).await?;

        // Link to battery if specified
        if let Some(battery_id) = vehicle.battery_id {
            self.link_to_battery(vehicle.id, battery_id).await?;
        }

        // Link to connectors
        for connector_type in &vehicle.supported_connectors {
            self.link_to_connector(vehicle.id, connector_type).await?;
        }

        Ok(())
    }

    /// Link vehicle to OEM
    async fn link_to_oem(&self, vehicle_id: Uuid, oem_id: Uuid) -> Result<()> {
        let cypher = format!(
            "MATCH (v:{} {{id: $vehicle_id}})
             MATCH (o:{} {{id: $oem_id}})
             CREATE (v)-[:{}]->(o)",
            labels::VEHICLE, labels::OEM, relationships::MANUFACTURED_BY
        );

        let q = query(&cypher)
            .param("vehicle_id", vehicle_id.to_string())
            .param("oem_id", oem_id.to_string());

        self.db.graph()
            .run(q)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    /// Link vehicle to battery
    async fn link_to_battery(&self, vehicle_id: Uuid, battery_id: Uuid) -> Result<()> {
        let cypher = format!(
            "MATCH (v:{} {{id: $vehicle_id}})
             MATCH (b:{} {{id: $battery_id}})
             CREATE (v)-[:{}]->(b)",
            labels::VEHICLE, labels::BATTERY, relationships::HAS_BATTERY
        );

        let q = query(&cypher)
            .param("vehicle_id", vehicle_id.to_string())
            .param("battery_id", battery_id.to_string());

        self.db.graph()
            .run(q)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    /// Link vehicle to connector
    async fn link_to_connector(&self, vehicle_id: Uuid, connector_type: &ConnectorType) -> Result<()> {
        let cypher = format!(
            "MATCH (v:{} {{id: $vehicle_id}})
             MATCH (c:{} {{connector_type: $connector_type}})
             CREATE (v)-[:{}]->(c)",
            labels::VEHICLE, labels::CONNECTOR, relationships::SUPPORTS_CONNECTOR
        );

        let q = query(&cypher)
            .param("vehicle_id", vehicle_id.to_string())
            .param("connector_type", format!("{:?}", connector_type));

        self.db.graph()
            .run(q)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }

    /// Find vehicle by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<String>> {
        let cypher = format!(
            "MATCH (v:{} {{id: $id}}) RETURN v",
            labels::VEHICLE
        );

        let mut result = self.db.graph()
            .execute(query(&cypher).param("id", id.to_string()))
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        if let Ok(Some(row)) = result.next().await {
            if let Ok(vehicle_data) = row.get::<neo4rs::Node>("v") {
                return Ok(Some(format!("{:?}", vehicle_data)));
            }
        }

        Ok(None)
    }

    /// Find vehicles by OEM
    pub async fn find_by_oem(&self, oem_id: Uuid) -> Result<Vec<String>> {
        let cypher = format!(
            "MATCH (v:{})-[:{}]->(o:{} {{id: $oem_id}})
             WHERE v.is_active = true
             RETURN v.id as id, v.name as name, v.model_code as model_code
             ORDER BY v.name",
            labels::VEHICLE, relationships::MANUFACTURED_BY, labels::OEM
        );

        let mut result = self.db.graph()
            .execute(query(&cypher).param("oem_id", oem_id.to_string()))
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

    /// Search vehicles by name
    pub async fn search_by_name(&self, search_term: &str) -> Result<Vec<String>> {
        let cypher = "CALL db.index.fulltext.queryNodes('vehicle_search', $search_term)
                      YIELD node, score
                      RETURN node.id as id, node.name as name, score
                      ORDER BY score DESC
                      LIMIT 20";

        let mut result = self.db.graph()
            .execute(query(cypher).param("search_term", search_term))
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let mut vehicles = Vec::new();

        while let Ok(Some(row)) = result.next().await {
            if let (Ok(id), Ok(name)) = (
                row.get::<String>("id"),
                row.get::<String>("name"),
            ) {
                vehicles.push(format!("{}: {}", id, name));
            }
        }

        Ok(vehicles)
    }
}
