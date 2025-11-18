use crate::types::{BatteryCapacity, BatteryChemistry};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Battery pack specification
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Battery {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    pub manufacturer: String,
    pub chemistry: BatteryChemistry,
    pub capacity: BatteryCapacity,

    // Technical specs
    pub voltage_v: f32,
    pub cell_configuration: String,  // e.g., "96s2p"
    pub energy_density_wh_per_kg: Option<f32>,

    // Warranty
    pub warranty_years: u8,
    pub warranty_km: u32,
    pub warranty_capacity_retention_percent: u8,

    // Performance
    pub charging_cycles: Option<u32>,
    pub max_charging_rate_kw: f32,
    pub thermal_management: ThermalManagement,

    // Safety & Certifications
    pub ip_rating: Option<String>,  // e.g., "IP67"
    pub certifications: Vec<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ThermalManagement {
    AirCooled,
    LiquidCooled,
    PassiveCooled,
}

/// Battery health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryHealth {
    pub battery_id: Uuid,
    pub state_of_health_percent: f32,
    pub cycles_completed: u32,
    pub estimated_remaining_capacity: BatteryCapacity,
}
