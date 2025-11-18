use crate::types::VehicleType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Original Equipment Manufacturer (EV maker)
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Oem {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub name: String,
    pub brand_name: String,
    pub country_of_origin: String,

    // Contact & Web
    pub website: Option<String>,
    pub headquarters_location: Option<String>,

    // Business info
    pub established_year: Option<u16>,
    pub parent_company: Option<String>,
    pub is_indian_company: bool,

    // Product portfolio
    pub vehicle_types: Vec<VehicleType>,
    pub total_models: u32,

    // Manufacturing
    pub manufacturing_plants_india: Vec<String>,
    pub has_indian_manufacturing: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// OEM market presence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OemMarketPresence {
    pub oem_id: Uuid,
    pub market_share_percent: Option<f32>,
    pub total_sales_ytd: Option<u32>,
    pub states_present: Vec<String>,
    pub service_centers_count: u32,
}
