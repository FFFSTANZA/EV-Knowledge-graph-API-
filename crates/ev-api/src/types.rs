use serde::{Deserialize, Serialize};

/// Pagination parameters
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 20,
        }
    }
}

impl PaginationParams {
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.limit
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.limit > 100 {
            return Err("Limit cannot exceed 100".to_string());
        }
        if self.limit == 0 {
            return Err("Limit must be at least 1".to_string());
        }
        if self.page == 0 {
            return Err("Page must be at least 1".to_string());
        }
        Ok(())
    }
}

/// Paginated response
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationMeta,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

impl PaginationMeta {
    pub fn new(page: u32, limit: u32, total: u64) -> Self {
        let total_pages = ((total as f64) / (limit as f64)).ceil() as u32;
        Self {
            page,
            limit,
            total,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }
}

/// Sorting parameters
#[derive(Debug, Deserialize, Clone)]
pub struct SortParams {
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_sort_order")]
    pub order: SortOrder,
}

fn default_sort_by() -> String {
    "created_at".to_string()
}

fn default_sort_order() -> SortOrder {
    SortOrder::Desc
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Filter parameters for vehicles
#[derive(Debug, Deserialize, Clone)]
pub struct VehicleFilter {
    pub vehicle_type: Option<String>,
    pub segment: Option<String>,
    pub oem_id: Option<String>,
    pub min_range_km: Option<f32>,
    pub max_range_km: Option<f32>,
    pub min_price_inr: Option<f32>,
    pub max_price_inr: Option<f32>,
    pub state: Option<String>,
    pub is_fame_eligible: Option<bool>,
}

/// Filter parameters for chargers
#[derive(Debug, Deserialize, Clone)]
pub struct ChargerFilter {
    pub charging_type: Option<String>,
    pub min_power_kw: Option<f32>,
    pub max_power_kw: Option<f32>,
    pub connector_type: Option<String>,
    pub operator_id: Option<String>,
    pub is_operational: Option<bool>,
    pub is_public: Option<bool>,
}

/// Geospatial query parameters
#[derive(Debug, Deserialize, Clone)]
pub struct GeoQuery {
    pub lat: f64,
    pub lon: f64,
    #[serde(default = "default_radius")]
    pub radius_km: f64,
}

fn default_radius() -> f64 {
    10.0
}

impl GeoQuery {
    pub fn validate(&self) -> Result<(), String> {
        if self.lat < -90.0 || self.lat > 90.0 {
            return Err("Latitude must be between -90 and 90".to_string());
        }
        if self.lon < -180.0 || self.lon > 180.0 {
            return Err("Longitude must be between -180 and 180".to_string());
        }
        if self.radius_km <= 0.0 || self.radius_km > 500.0 {
            return Err("Radius must be between 0 and 500 km".to_string());
        }
        Ok(())
    }
}

/// Batch request
#[derive(Debug, Deserialize)]
pub struct BatchRequest<T> {
    pub items: Vec<T>,
}

/// Batch response
#[derive(Debug, Serialize)]
pub struct BatchResponse<T> {
    pub results: Vec<BatchResult<T>>,
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
}

#[derive(Debug, Serialize)]
pub struct BatchResult<T> {
    pub index: usize,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
