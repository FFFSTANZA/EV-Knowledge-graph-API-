use crate::types::{ChargingType, ConnectorType, IndianState};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Charging station
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Charger {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    pub operator_id: Uuid,
    pub charging_type: ChargingType,

    // Location
    pub location: Location,

    // Technical specs
    pub max_power_kw: f32,
    pub supported_connectors: Vec<ConnectorType>,
    pub number_of_ports: u8,

    // Protocol support
    pub supported_protocols: Vec<String>,

    // Operational
    pub is_operational: bool,
    pub is_public: bool,
    pub access_type: AccessType,

    // Network
    pub network_id: Option<Uuid>,

    // Pricing
    pub pricing_model: Option<PricingModel>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub address: String,
    pub city: String,
    pub state: IndianState,
    pub pincode: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccessType {
    Public,
    SemiPublic,  // Requires membership
    Private,
    Fleet,       // Fleet-only
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingModel {
    pub base_rate_inr_per_kwh: f32,
    pub time_based_rate_inr_per_min: Option<f32>,
    pub parking_fee_inr_per_min: Option<f32>,
    pub peak_hours_multiplier: Option<f32>,
}

/// Charging station operator/network
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ChargingOperator {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    pub brand_name: String,
    pub website: Option<String>,
    pub app_link: Option<String>,
    pub contact_number: Option<String>,
    pub total_stations: u32,
    pub states_present: Vec<IndianState>,
    pub created_at: DateTime<Utc>,
}

/// Charger availability status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargerAvailability {
    pub charger_id: Uuid,
    pub is_available: bool,
    pub ports_available: u8,
    pub ports_total: u8,
    pub last_updated: DateTime<Utc>,
}
