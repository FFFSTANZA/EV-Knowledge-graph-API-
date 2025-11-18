use serde::{Deserialize, Serialize};
use std::fmt;

/// Vehicle types in the Indian EV market
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VehicleType {
    TwoWheeler,      // Scooters, motorcycles
    ThreeWheeler,    // Auto-rickshaws
    FourWheeler,     // Cars, SUVs
    CommercialVan,   // Delivery vans
    Bus,             // Electric buses
    Truck,           // Commercial trucks
}

/// Vehicle segment classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VehicleSegment {
    Entry,      // Budget EVs
    Mid,        // Mid-range
    Premium,    // Premium segment
    Luxury,     // Luxury EVs
    Performance,// High-performance
}

/// Charging types supported in India
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ChargingType {
    Ac,         // AC charging
    Dc,         // DC fast charging
    DcUltraFast,// Ultra-fast DC (>150kW)
}

/// Connector types used in India
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectorType {
    Type2,         // Type-2 (AC)
    Ccs2,          // CCS2 (DC fast charging)
    Chademo,       // CHAdeMO
    GbT,           // GB/T (Chinese standard)
    BharatDc001,   // Bharat DC001 (Indian standard)
    BharatAc001,   // Bharat AC001
}

impl fmt::Display for ConnectorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectorType::Type2 => write!(f, "Type-2"),
            ConnectorType::Ccs2 => write!(f, "CCS2"),
            ConnectorType::Chademo => write!(f, "CHAdeMO"),
            ConnectorType::GbT => write!(f, "GB/T"),
            ConnectorType::BharatDc001 => write!(f, "Bharat DC001"),
            ConnectorType::BharatAc001 => write!(f, "Bharat AC001"),
        }
    }
}

/// Charging protocols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Protocol {
    Ocpp16,    // OCPP 1.6
    Ocpp20,    // OCPP 2.0
    Ccs,       // CCS protocol
    Chademo,   // CHAdeMO protocol
}

/// Battery chemistry types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BatteryChemistry {
    #[serde(rename = "NMC")]
    Nmc,       // Nickel Manganese Cobalt
    #[serde(rename = "LFP")]
    Lfp,       // Lithium Iron Phosphate
    #[serde(rename = "NCA")]
    Nca,       // Nickel Cobalt Aluminum
    #[serde(rename = "LTO")]
    Lto,       // Lithium Titanate Oxide
}

/// Indian states for regional mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IndianState {
    AndhraPradesh,
    ArunachalPradesh,
    Assam,
    Bihar,
    Chhattisgarh,
    Goa,
    Gujarat,
    Haryana,
    HimachalPradesh,
    Jharkhand,
    Karnataka,
    Kerala,
    MadhyaPradesh,
    Maharashtra,
    Manipur,
    Meghalaya,
    Mizoram,
    Nagaland,
    Odisha,
    Punjab,
    Rajasthan,
    Sikkim,
    TamilNadu,
    Telangana,
    Tripura,
    UttarPradesh,
    Uttarakhand,
    WestBengal,
    // Union Territories
    Delhi,
    Chandigarh,
    Puducherry,
    JammuKashmir,
    Ladakh,
}

/// Power output levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PowerOutput {
    pub value: f32,
    pub unit: PowerUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PowerUnit {
    Kw,  // Kilowatts
    Hp,  // Horsepower
}

/// Battery capacity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatteryCapacity {
    pub value: f32,
    pub unit: CapacityUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CapacityUnit {
    Kwh,  // Kilowatt-hours
}

/// Range specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Range {
    pub value: f32,
    pub unit: RangeUnit,
    pub test_cycle: Option<TestCycle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RangeUnit {
    Km,  // Kilometers
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TestCycle {
    Arai,    // ARAI (India)
    Wltp,    // WLTP (Europe)
    Epa,     // EPA (US)
    Nedc,    // NEDC (older)
}

/// Charging speed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChargingSpeed {
    pub max_power_kw: f32,
    pub time_0_to_80_percent_mins: Option<u32>,
    pub time_0_to_100_percent_mins: Option<u32>,
}

/// Price in Indian Rupees
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Price {
    pub ex_showroom_inr: f32,
    pub state: Option<IndianState>,
    pub includes_fame_subsidy: bool,
    pub includes_state_subsidy: bool,
}
