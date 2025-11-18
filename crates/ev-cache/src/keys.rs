use uuid::Uuid;

/// Cache key generator for consistent key naming
pub struct CacheKey;

impl CacheKey {
    const PREFIX: &'static str = "ev";

    // Vehicle keys
    pub fn vehicle(id: Uuid) -> String {
        format!("{}:vehicle:{}", Self::PREFIX, id)
    }

    pub fn vehicle_list(page: u32, limit: u32) -> String {
        format!("{}:vehicles:page:{}:{}", Self::PREFIX, page, limit)
    }

    pub fn vehicle_by_oem(oem_id: Uuid) -> String {
        format!("{}:vehicles:oem:{}", Self::PREFIX, oem_id)
    }

    pub fn vehicle_search(query: &str) -> String {
        format!("{}:vehicles:search:{}", Self::PREFIX, query)
    }

    // Charger keys
    pub fn charger(id: Uuid) -> String {
        format!("{}:charger:{}", Self::PREFIX, id)
    }

    pub fn chargers_by_city(city: &str, state: &str) -> String {
        format!("{}:chargers:{}:{}", Self::PREFIX, state, city)
    }

    pub fn chargers_nearby(lat: f64, lon: f64, radius_km: f64) -> String {
        format!("{}:chargers:nearby:{:.4}:{:.4}:{}", Self::PREFIX, lat, lon, radius_km)
    }

    // Compatibility keys
    pub fn compatible_chargers(vehicle_id: Uuid) -> String {
        format!("{}:compatibility:vehicle:{}:chargers", Self::PREFIX, vehicle_id)
    }

    pub fn compatible_vehicles(charger_id: Uuid) -> String {
        format!("{}:compatibility:charger:{}:vehicles", Self::PREFIX, charger_id)
    }

    pub fn compatibility_check(vehicle_id: Uuid, charger_id: Uuid) -> String {
        format!("{}:compatibility:{}:{}", Self::PREFIX, vehicle_id, charger_id)
    }

    // OEM keys
    pub fn oem(id: Uuid) -> String {
        format!("{}:oem:{}", Self::PREFIX, id)
    }

    pub fn oem_list() -> String {
        format!("{}:oems:all", Self::PREFIX)
    }

    // Policy keys
    pub fn policy(id: Uuid) -> String {
        format!("{}:policy:{}", Self::PREFIX, id)
    }

    pub fn fame_eligible_vehicles(state: &str) -> String {
        format!("{}:fame:{}:vehicles", Self::PREFIX, state)
    }

    // Region keys
    pub fn region(id: Uuid) -> String {
        format!("{}:region:{}", Self::PREFIX, id)
    }

    pub fn regional_stats(state: &str) -> String {
        format!("{}:stats:region:{}", Self::PREFIX, state)
    }

    // Inference/recommendation keys
    pub fn vehicle_recommendations(vehicle_type: &str, min_range: f32, max_price: f32) -> String {
        format!("{}:recommendations:{}:{}:{}", Self::PREFIX, vehicle_type, min_range, max_price)
    }

    // Analytics keys
    pub fn popular_vehicles(timeframe: &str) -> String {
        format!("{}:analytics:popular:vehicles:{}", Self::PREFIX, timeframe)
    }

    pub fn charging_network_stats(state: &str) -> String {
        format!("{}:analytics:network:{}", Self::PREFIX, state)
    }

    // Session/rate limiting
    pub fn rate_limit(ip: &str, endpoint: &str) -> String {
        format!("{}:ratelimit:{}:{}", Self::PREFIX, endpoint, ip)
    }

    // Pattern for bulk deletion
    pub fn pattern_all() -> String {
        format!("{}:*", Self::PREFIX)
    }

    pub fn pattern_vehicle() -> String {
        format!("{}:vehicle:*", Self::PREFIX)
    }

    pub fn pattern_charger() -> String {
        format!("{}:charger:*", Self::PREFIX)
    }

    pub fn pattern_compatibility() -> String {
        format!("{}:compatibility:*", Self::PREFIX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_keys() {
        let vehicle_id = Uuid::new_v4();
        assert_eq!(
            CacheKey::vehicle(vehicle_id),
            format!("ev:vehicle:{}", vehicle_id)
        );

        assert_eq!(
            CacheKey::vehicle_list(1, 20),
            "ev:vehicles:page:1:20"
        );

        assert_eq!(
            CacheKey::chargers_by_city("Mumbai", "Maharashtra"),
            "ev:chargers:Maharashtra:Mumbai"
        );
    }
}
