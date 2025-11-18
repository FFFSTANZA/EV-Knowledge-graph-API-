use crate::types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Electric Vehicle model
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Vehicle {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub model_code: String,
    pub oem_id: Uuid,
    pub vehicle_type: VehicleType,
    pub segment: VehicleSegment,
    pub battery_id: Option<Uuid>,
    pub launch_date: Option<DateTime<Utc>>,
    pub discontinuation_date: Option<DateTime<Utc>>,
    pub is_active: bool,

    // Performance specs
    pub range: Range,
    pub top_speed_kmph: Option<f32>,
    pub acceleration_0_to_100_secs: Option<f32>,
    pub power_output: PowerOutput,
    pub torque_nm: Option<f32>,

    // Charging capabilities
    pub supported_connectors: Vec<ConnectorType>,
    pub ac_charging_speed: Option<ChargingSpeed>,
    pub dc_charging_speed: Option<ChargingSpeed>,

    // Pricing
    pub price_range: Option<PriceRange>,

    // Features
    pub seating_capacity: Option<u8>,
    pub boot_space_liters: Option<u32>,
    pub ground_clearance_mm: Option<u32>,

    // Metadata
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRange {
    pub min: Price,
    pub max: Price,
}

/// Vehicle variant (different battery/feature options)
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VehicleVariant {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    #[validate(length(min = 1))]
    pub variant_name: String,
    pub battery_id: Uuid,
    pub range: Range,
    pub price: Price,
    pub features: Vec<String>,
    pub created_at: DateTime<Utc>,
}

/// Vehicle performance profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub vehicle_id: Uuid,
    pub efficiency_wh_per_km: f32,
    pub regenerative_braking: bool,
    pub drive_modes: Vec<DriveMode>,
    pub real_world_range_km: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DriveMode {
    Eco,
    Normal,
    Sport,
    Custom,
}

/// Vehicle specifications for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleSpecs {
    pub vehicle: Vehicle,
    pub oem_name: String,
    pub battery_specs: Option<String>,
    pub compatible_chargers: Vec<String>,
    pub eligible_policies: Vec<String>,
    pub available_regions: Vec<String>,
}
