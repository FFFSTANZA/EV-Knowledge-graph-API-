use crate::types::ConnectorType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Connector standard specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub id: Uuid,
    pub connector_type: ConnectorType,
    pub name: String,
    pub description: String,

    // Electrical specs
    pub max_voltage_v: f32,
    pub max_current_a: f32,
    pub max_power_kw: f32,

    // Physical
    pub is_dc: bool,
    pub pin_configuration: String,

    // Standards
    pub standard_name: String,  // e.g., "IEC 62196"
    pub region_of_use: Vec<String>,
}

/// Connector compatibility rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorCompatibility {
    pub connector_a: ConnectorType,
    pub connector_b: ConnectorType,
    pub is_compatible: bool,
    pub requires_adapter: bool,
    pub adapter_name: Option<String>,
}
