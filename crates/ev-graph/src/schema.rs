/// Neo4j Graph Schema for EV Knowledge Graph
///
/// This module defines the complete graph schema including:
/// - Node labels
/// - Relationship types
/// - Properties
/// - Indexes and constraints

/// Node Labels
pub mod labels {
    pub const VEHICLE: &str = "Vehicle";
    pub const OEM: &str = "Oem";
    pub const BATTERY: &str = "Battery";
    pub const CHARGER: &str = "Charger";
    pub const CONNECTOR: &str = "Connector";
    pub const PROTOCOL: &str = "Protocol";
    pub const REGION: &str = "Region";
    pub const POLICY: &str = "Policy";
    pub const CHARGING_OPERATOR: &str = "ChargingOperator";
    pub const VARIANT: &str = "Variant";
}

/// Relationship Types
pub mod relationships {
    // Vehicle relationships
    pub const MANUFACTURED_BY: &str = "MANUFACTURED_BY";
    pub const HAS_BATTERY: &str = "HAS_BATTERY";
    pub const SUPPORTS_CONNECTOR: &str = "SUPPORTS_CONNECTOR";
    pub const HAS_VARIANT: &str = "HAS_VARIANT";
    pub const AVAILABLE_IN: &str = "AVAILABLE_IN";
    pub const ELIGIBLE_FOR: &str = "ELIGIBLE_FOR";

    // Charger relationships
    pub const OPERATED_BY: &str = "OPERATED_BY";
    pub const REQUIRES_CONNECTOR: &str = "REQUIRES_CONNECTOR";
    pub const USES_PROTOCOL: &str = "USES_PROTOCOL";
    pub const LOCATED_IN: &str = "LOCATED_IN";
    pub const COMPATIBLE_WITH: &str = "COMPATIBLE_WITH";

    // Policy relationships
    pub const APPLIES_TO_REGION: &str = "APPLIES_TO_REGION";
    pub const APPLIES_TO_VEHICLE_TYPE: &str = "APPLIES_TO_VEHICLE_TYPE";

    // Connector relationships
    pub const WORKS_WITH: &str = "WORKS_WITH";
    pub const REQUIRES_ADAPTER: &str = "REQUIRES_ADAPTER";
}

/// Cypher queries for schema initialization
pub const INIT_SCHEMA: &[&str] = &[
    // Constraints - Unique IDs
    "CREATE CONSTRAINT vehicle_id IF NOT EXISTS FOR (v:Vehicle) REQUIRE v.id IS UNIQUE",
    "CREATE CONSTRAINT oem_id IF NOT EXISTS FOR (o:Oem) REQUIRE o.id IS UNIQUE",
    "CREATE CONSTRAINT battery_id IF NOT EXISTS FOR (b:Battery) REQUIRE b.id IS UNIQUE",
    "CREATE CONSTRAINT charger_id IF NOT EXISTS FOR (c:Charger) REQUIRE c.id IS UNIQUE",
    "CREATE CONSTRAINT connector_id IF NOT EXISTS FOR (c:Connector) REQUIRE c.id IS UNIQUE",
    "CREATE CONSTRAINT protocol_id IF NOT EXISTS FOR (p:Protocol) REQUIRE p.id IS UNIQUE",
    "CREATE CONSTRAINT region_id IF NOT EXISTS FOR (r:Region) REQUIRE r.id IS UNIQUE",
    "CREATE CONSTRAINT policy_id IF NOT EXISTS FOR (p:Policy) REQUIRE p.id IS UNIQUE",

    // Indexes for performance
    "CREATE INDEX vehicle_name IF NOT EXISTS FOR (v:Vehicle) ON (v.name)",
    "CREATE INDEX vehicle_model_code IF NOT EXISTS FOR (v:Vehicle) ON (v.model_code)",
    "CREATE INDEX vehicle_type IF NOT EXISTS FOR (v:Vehicle) ON (v.vehicle_type)",
    "CREATE INDEX oem_name IF NOT EXISTS FOR (o:Oem) ON (o.name)",
    "CREATE INDEX charger_state IF NOT EXISTS FOR (c:Charger) ON (c.state)",
    "CREATE INDEX charger_city IF NOT EXISTS FOR (c:Charger) ON (c.city)",
    "CREATE INDEX connector_type IF NOT EXISTS FOR (c:Connector) ON (c.connector_type)",
    "CREATE INDEX region_state IF NOT EXISTS FOR (r:Region) ON (r.state)",
    "CREATE INDEX policy_active IF NOT EXISTS FOR (p:Policy) ON (p.is_active)",

    // Full-text search indexes
    "CREATE FULLTEXT INDEX vehicle_search IF NOT EXISTS FOR (v:Vehicle) ON EACH [v.name, v.model_code]",
    "CREATE FULLTEXT INDEX charger_search IF NOT EXISTS FOR (c:Charger) ON EACH [c.name, c.city, c.address]",
];

/// Graph schema documentation
pub const SCHEMA_DOC: &str = r#"
# EV Knowledge Graph Schema

## Nodes

### Vehicle
Properties:
- id: UUID (unique)
- name: String
- model_code: String
- vehicle_type: String (two_wheeler, three_wheeler, four_wheeler, etc.)
- segment: String (entry, mid, premium, luxury, performance)
- is_active: Boolean
- range_km: Float
- top_speed_kmph: Float
- power_kw: Float
- created_at: DateTime
- updated_at: DateTime

### Oem
Properties:
- id: UUID (unique)
- name: String
- brand_name: String
- country_of_origin: String
- is_indian_company: Boolean
- has_indian_manufacturing: Boolean
- total_models: Integer
- created_at: DateTime

### Battery
Properties:
- id: UUID (unique)
- name: String
- manufacturer: String
- chemistry: String (NMC, LFP, NCA, LTO)
- capacity_kwh: Float
- voltage_v: Float
- warranty_years: Integer
- warranty_km: Integer
- max_charging_rate_kw: Float
- created_at: DateTime

### Charger
Properties:
- id: UUID (unique)
- name: String
- charging_type: String (ac, dc, dc_ultra_fast)
- max_power_kw: Float
- number_of_ports: Integer
- is_operational: Boolean
- is_public: Boolean
- access_type: String
- address: String
- city: String
- state: String
- latitude: Float
- longitude: Float
- created_at: DateTime

### Connector
Properties:
- id: UUID (unique)
- connector_type: String (TYPE2, CCS2, CHADEMO, GB_T, BHARAT_DC001, BHARAT_AC001)
- name: String
- max_voltage_v: Float
- max_current_a: Float
- max_power_kw: Float
- is_dc: Boolean
- standard_name: String

### Protocol
Properties:
- id: UUID (unique)
- protocol: String (OCPP16, OCPP20, CCS, CHADEMO)
- name: String
- version: String
- supports_bidirectional: Boolean
- supports_plug_and_charge: Boolean
- supports_smart_charging: Boolean

### Region
Properties:
- id: UUID (unique)
- name: String
- state: String
- region_type: String (state, city, metro, district)
- total_charging_stations: Integer
- has_state_ev_policy: Boolean

### Policy
Properties:
- id: UUID (unique)
- name: String (e.g., "FAME-II")
- policy_type: String (subsidy, tax_benefit, etc.)
- level: String (national, state, municipal)
- incentive_amount_inr: Float
- is_active: Boolean
- valid_from: DateTime
- valid_until: DateTime

## Relationships

### MANUFACTURED_BY
Vehicle -[MANUFACTURED_BY]-> Oem

### HAS_BATTERY
Vehicle -[HAS_BATTERY]-> Battery

### SUPPORTS_CONNECTOR
Vehicle -[SUPPORTS_CONNECTOR {charging_type, max_power_kw}]-> Connector

### COMPATIBLE_WITH
Charger -[COMPATIBLE_WITH {compatibility_score}]-> Vehicle

### REQUIRES_CONNECTOR
Charger -[REQUIRES_CONNECTOR {port_number}]-> Connector

### USES_PROTOCOL
Charger -[USES_PROTOCOL]-> Protocol

### LOCATED_IN
Charger -[LOCATED_IN]-> Region

### AVAILABLE_IN
Vehicle -[AVAILABLE_IN {price_inr, includes_fame_subsidy}]-> Region

### ELIGIBLE_FOR
Vehicle -[ELIGIBLE_FOR {incentive_amount_inr}]-> Policy

### OPERATED_BY
Charger -[OPERATED_BY]-> ChargingOperator

## Example Queries

1. Find compatible chargers for a vehicle:
```cypher
MATCH (v:Vehicle {id: $vehicle_id})-[:SUPPORTS_CONNECTOR]->(conn:Connector)
MATCH (c:Charger)-[:REQUIRES_CONNECTOR]->(conn)
WHERE c.is_operational = true
RETURN c
```

2. Find all FAME-II eligible vehicles in Maharashtra:
```cypher
MATCH (v:Vehicle)-[:ELIGIBLE_FOR]->(p:Policy {name: 'FAME-II'})
MATCH (v)-[:AVAILABLE_IN]->(r:Region {state: 'Maharashtra'})
WHERE p.is_active = true AND v.is_active = true
RETURN v
```

3. Find vehicles compatible with CCS2 and 22kW AC:
```cypher
MATCH (v:Vehicle)-[s:SUPPORTS_CONNECTOR]->(conn:Connector {connector_type: 'CCS2'})
WHERE s.max_power_kw >= 22
RETURN v
```
"#;
