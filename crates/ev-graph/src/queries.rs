/// Common Cypher queries for the EV Knowledge Graph

/// Find all FAME-II eligible vehicles in a specific region
pub const FIND_FAME_ELIGIBLE_VEHICLES: &str = r#"
MATCH (v:Vehicle)-[:ELIGIBLE_FOR]->(p:Policy {name: 'FAME-II'})
MATCH (v)-[:AVAILABLE_IN]->(r:Region {state: $state})
WHERE p.is_active = true AND v.is_active = true
RETURN v.id as id, v.name as name, v.range_km as range,
       v.price_inr as price
ORDER BY v.price_inr
"#;

/// Find charging route between two cities
pub const FIND_CHARGING_ROUTE: &str = r#"
MATCH (start:Region {city: $start_city})
MATCH (end:Region {city: $end_city})
MATCH path = shortestPath((start)-[:HAS_CHARGER*]-(end))
RETURN path
"#;

/// Find fastest chargers for a vehicle type
pub const FIND_FASTEST_CHARGERS: &str = r#"
MATCH (v:Vehicle {vehicle_type: $vehicle_type})-[:SUPPORTS_CONNECTOR]->(conn:Connector)
MATCH (c:Charger)-[:REQUIRES_CONNECTOR]->(conn)
WHERE c.is_operational = true
RETURN c.id as charger_id, c.name as name, c.max_power_kw as power,
       conn.connector_type as connector_type
ORDER BY c.max_power_kw DESC
LIMIT $limit
"#;

/// Find vehicles in a price range with minimum range
pub const FIND_VEHICLES_BY_PRICE_RANGE: &str = r#"
MATCH (v:Vehicle)-[:AVAILABLE_IN]->(r:Region {state: $state})
WHERE v.is_active = true
  AND v.price_inr >= $min_price
  AND v.price_inr <= $max_price
  AND v.range_km >= $min_range
RETURN v.id as id, v.name as name, v.price_inr as price,
       v.range_km as range, v.vehicle_type as type
ORDER BY v.price_inr
"#;

/// Find all vehicles from Indian OEMs
pub const FIND_INDIAN_OEM_VEHICLES: &str = r#"
MATCH (v:Vehicle)-[:MANUFACTURED_BY]->(o:Oem)
WHERE o.is_indian_company = true AND v.is_active = true
RETURN v.id as id, v.name as name, o.name as oem_name,
       v.price_inr as price, v.range_km as range
ORDER BY o.name, v.name
"#;

/// Get complete vehicle profile with all relationships
pub const GET_VEHICLE_PROFILE: &str = r#"
MATCH (v:Vehicle {id: $vehicle_id})
OPTIONAL MATCH (v)-[:MANUFACTURED_BY]->(oem:Oem)
OPTIONAL MATCH (v)-[:HAS_BATTERY]->(battery:Battery)
OPTIONAL MATCH (v)-[:SUPPORTS_CONNECTOR]->(connector:Connector)
OPTIONAL MATCH (v)-[:ELIGIBLE_FOR]->(policy:Policy)
RETURN v, oem, battery, collect(DISTINCT connector) as connectors,
       collect(DISTINCT policy) as policies
"#;

/// Find charging network coverage in a state
pub const FIND_NETWORK_COVERAGE: &str = r#"
MATCH (c:Charger)-[:OPERATED_BY]->(op:ChargingOperator)
MATCH (c)-[:LOCATED_IN]->(r:Region {state: $state})
WHERE c.is_operational = true
RETURN op.name as operator, count(c) as charger_count,
       collect(DISTINCT r.city) as cities
ORDER BY charger_count DESC
"#;

/// Recommend vehicles based on use case
pub const RECOMMEND_VEHICLES: &str = r#"
MATCH (v:Vehicle)
WHERE v.is_active = true
  AND v.vehicle_type = $vehicle_type
  AND v.range_km >= $min_range
  AND ($max_price IS NULL OR v.price_inr <= $max_price)
OPTIONAL MATCH (v)-[:MANUFACTURED_BY]->(oem:Oem)
OPTIONAL MATCH (v)-[:ELIGIBLE_FOR]->(policy:Policy {is_active: true})
RETURN v.id as id, v.name as name, oem.name as oem_name,
       v.price_inr as price, v.range_km as range,
       count(policy) as incentive_count
ORDER BY incentive_count DESC, v.range_km DESC, v.price_inr ASC
LIMIT $limit
"#;
