use crate::types::IndianState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Regional data for states/cities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Region {
    pub id: Uuid,
    pub name: String,
    pub state: IndianState,
    pub region_type: RegionType,

    // EV Infrastructure
    pub total_charging_stations: u32,
    pub total_ev_registrations: Option<u32>,

    // Policy environment
    pub has_state_ev_policy: bool,
    pub has_local_incentives: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegionType {
    State,
    City,
    Metro,
    District,
}

/// Regional EV policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalPolicy {
    pub id: Uuid,
    pub region_id: Uuid,
    pub policy_name: String,
    pub description: String,
    pub incentive_amount_inr: Option<f32>,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub applicable_vehicle_types: Vec<String>,
}
