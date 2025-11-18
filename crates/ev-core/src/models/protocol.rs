use crate::types::Protocol;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Charging protocol specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingProtocol {
    pub id: Uuid,
    pub protocol: Protocol,
    pub name: String,
    pub version: String,
    pub description: String,

    // Capabilities
    pub supports_bidirectional: bool,
    pub supports_plug_and_charge: bool,
    pub supports_smart_charging: bool,

    // Communication
    pub communication_type: CommunicationType,

    // Standards
    pub standard_body: String,
    pub specification_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CommunicationType {
    Powerline,     // PLC
    Wireless,      // Wi-Fi, cellular
    Can,           // CAN bus
    Iso15118,      // ISO 15118
}
