use crate::types::{IndianState, VehicleType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Government policy/incentive (FAME-II, state subsidies)
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Policy {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    pub policy_type: PolicyType,
    pub description: String,

    // Scope
    pub level: PolicyLevel,
    pub applicable_states: Option<Vec<IndianState>>,
    pub applicable_vehicle_types: Vec<VehicleType>,

    // Incentive details
    pub incentive_amount_inr: Option<f32>,
    pub incentive_per_kwh_inr: Option<f32>,
    pub max_incentive_cap_inr: Option<f32>,

    // Eligibility
    pub min_battery_capacity_kwh: Option<f32>,
    pub min_range_km: Option<f32>,
    pub max_vehicle_cost_inr: Option<f32>,
    pub requires_local_manufacturing: bool,

    // Validity
    pub valid_from: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyType {
    Subsidy,
    TaxBenefit,
    RoadTaxExemption,
    RegistrationFeeWaiver,
    ChargingInfrastructure,
    ScrapageBonus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicyLevel {
    National,     // FAME-II
    State,        // State EV policies
    Municipal,    // City-level
}

/// Vehicle eligibility for a policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEligibility {
    pub policy_id: Uuid,
    pub vehicle_id: Uuid,
    pub is_eligible: bool,
    pub incentive_amount_inr: f32,
    pub eligibility_criteria_met: Vec<String>,
    pub eligibility_criteria_failed: Vec<String>,
}
