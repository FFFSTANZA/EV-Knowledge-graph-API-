# EV Knowledge Graph API - Examples

**Version:** 0.1.0
**Last Updated:** 2024-11-18

This document provides practical examples of using the EV Knowledge Graph API in various programming languages and real-world scenarios.

## Table of Contents

- [Getting Started](#getting-started)
- [Basic Examples](#basic-examples)
  - [cURL Examples](#curl-examples)
  - [Python Examples](#python-examples)
  - [JavaScript/Node.js Examples](#javascriptnodejs-examples)
  - [Rust Examples](#rust-examples)
- [Use Cases](#use-cases)
  - [Vehicle Discovery](#vehicle-discovery)
  - [Charging Infrastructure](#charging-infrastructure)
  - [Compatibility Checks](#compatibility-checks)
  - [Route Planning](#route-planning)
  - [Fleet Management](#fleet-management)
- [Advanced Examples](#advanced-examples)
- [Error Handling](#error-handling)
- [Rate Limiting](#rate-limiting)
- [Complete Applications](#complete-applications)

---

## Getting Started

**Base URL:** `http://localhost:8080`

**Common Headers:**
```
Content-Type: application/json
X-API-Key: your-api-key (if authentication is enabled)
```

**Response Format:**
```json
{
  "success": true,
  "data": { ... }
}
```

---

## Basic Examples

### cURL Examples

#### 1. Health Check

```bash
curl http://localhost:8080/health
```

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "environment": "development",
    "neo4j": "healthy",
    "redis": "healthy"
  }
}
```

#### 2. Get All Vehicles

```bash
curl "http://localhost:8080/api/v1/vehicles?page=1&limit=10"
```

#### 3. Search Vehicles

```bash
curl "http://localhost:8080/api/v1/vehicles/search?q=tata+nexon"
```

#### 4. Find Nearby Chargers

```bash
# Mumbai coordinates: 19.0760, 72.8777
curl "http://localhost:8080/api/v1/chargers/nearby?lat=19.0760&lon=72.8777&radius=5"
```

#### 5. Check Compatibility

```bash
curl -X POST http://localhost:8080/api/v1/compatibility/check \
  -H "Content-Type: application/json" \
  -d '{
    "vehicle_id": "550e8400-e29b-41d4-a716-446655440000",
    "charger_id": "660e8400-e29b-41d4-a716-446655440001"
  }'
```

#### 6. Batch Compatibility Check

```bash
curl -X POST http://localhost:8080/api/v1/batch/compatibility \
  -H "Content-Type: application/json" \
  -d '{
    "items": [
      {"vehicle_id": "550e8400-e29b-41d4-a716-446655440000"},
      {"vehicle_id": "550e8400-e29b-41d4-a716-446655440001"},
      {"vehicle_id": "550e8400-e29b-41d4-a716-446655440002"}
    ]
  }'
```

---

### Python Examples

#### Setup

```bash
pip install requests
```

#### 1. Basic Client

```python
import requests
from typing import Dict, Any

class EVKnowledgeGraphClient:
    def __init__(self, base_url: str = "http://localhost:8080"):
        self.base_url = base_url
        self.session = requests.Session()

    def _request(self, method: str, endpoint: str, **kwargs) -> Dict[str, Any]:
        """Make HTTP request and handle errors"""
        url = f"{self.base_url}{endpoint}"
        response = self.session.request(method, url, **kwargs)
        response.raise_for_status()
        return response.json()

    def health_check(self) -> Dict[str, Any]:
        """Check API health"""
        return self._request("GET", "/health")

    def search_vehicles(self, query: str) -> Dict[str, Any]:
        """Search for vehicles"""
        return self._request("GET", f"/api/v1/vehicles/search?q={query}")

    def find_nearby_chargers(self, lat: float, lon: float, radius: float = 10) -> Dict[str, Any]:
        """Find chargers near a location"""
        params = {"lat": lat, "lon": lon, "radius": radius}
        return self._request("GET", "/api/v1/chargers/nearby", params=params)

    def check_compatibility(self, vehicle_id: str, charger_id: str) -> Dict[str, Any]:
        """Check if vehicle and charger are compatible"""
        data = {"vehicle_id": vehicle_id, "charger_id": charger_id}
        return self._request("POST", "/api/v1/compatibility/check", json=data)

# Usage
client = EVKnowledgeGraphClient()

# Health check
health = client.health_check()
print(f"API Status: {health['data']['status']}")

# Search vehicles
vehicles = client.search_vehicles("tata nexon")
print(f"Found {len(vehicles['data'])} vehicles")

# Find chargers
chargers = client.find_nearby_chargers(lat=19.0760, lon=72.8777, radius=5)
print(f"Found {len(chargers['data']['chargers'])} chargers within 5km")
```

#### 2. Vehicle Discovery

```python
def find_affordable_long_range_evs(client: EVKnowledgeGraphClient,
                                   max_price: float = 2000000,
                                   min_range: int = 300):
    """Find affordable EVs with good range"""

    # Get all vehicles
    response = client._request("GET", "/api/v1/vehicles", params={"limit": 100})
    vehicles = response['data']['vehicles']

    # Filter by criteria
    filtered = [
        v for v in vehicles
        if v.get('range_km', 0) >= min_range and
           v.get('price_inr', float('inf')) <= max_price
    ]

    # Sort by range (descending)
    filtered.sort(key=lambda v: v.get('range_km', 0), reverse=True)

    return filtered

# Example usage
affordable_evs = find_affordable_long_range_evs(client)
for ev in affordable_evs[:5]:
    print(f"{ev['name']}: {ev['range_km']}km range at ₹{ev['price_inr']:,.0f}")
```

#### 3. Charging Route Planner

```python
def plan_charging_route(client: EVKnowledgeGraphClient,
                        vehicle_id: str,
                        waypoints: list):
    """Plan charging stops along a route"""

    charging_stops = []

    for waypoint in waypoints:
        lat, lon = waypoint['lat'], waypoint['lon']

        # Find nearby chargers
        chargers_response = client.find_nearby_chargers(lat, lon, radius=20)

        # Filter compatible chargers
        compatible = []
        for charger in chargers_response['data']['chargers']:
            compatibility = client.check_compatibility(vehicle_id, charger['id'])
            if compatibility['data']['is_compatible']:
                compatible.append({
                    'charger': charger,
                    'distance_km': waypoint.get('distance_km', 0)
                })

        if compatible:
            # Sort by power (prefer faster charging)
            compatible.sort(key=lambda x: x['charger']['max_power_kw'], reverse=True)
            charging_stops.append({
                'location': waypoint['name'],
                'recommended_charger': compatible[0],
                'alternatives': compatible[1:3]
            })

    return charging_stops

# Example: Mumbai to Pune route
waypoints = [
    {'name': 'Mumbai Start', 'lat': 19.0760, 'lon': 72.8777, 'distance_km': 0},
    {'name': 'Lonavala', 'lat': 18.7537, 'lon': 73.4086, 'distance_km': 83},
    {'name': 'Pune', 'lat': 18.5204, 'lon': 73.8567, 'distance_km': 148}
]

route = plan_charging_route(client, vehicle_id="your-vehicle-uuid", waypoints=waypoints)

for stop in route:
    print(f"\n{stop['location']}:")
    charger = stop['recommended_charger']['charger']
    print(f"  Recommended: {charger['name']} ({charger['max_power_kw']}kW)")
```

#### 4. Batch Processing

```python
def analyze_fleet_compatibility(client: EVKnowledgeGraphClient,
                                vehicle_ids: list):
    """Analyze charging compatibility for a fleet of vehicles"""

    # Batch request for all vehicles
    batch_request = {
        "items": [{"vehicle_id": vid} for vid in vehicle_ids]
    }

    response = client._request(
        "POST",
        "/api/v1/batch/compatibility",
        json=batch_request
    )

    results = response['data']['results']

    # Aggregate results
    total_chargers_per_vehicle = []
    for result in results:
        if result['success']:
            count = result['data']['compatible_chargers_count']
            total_chargers_per_vehicle.append(count)

    return {
        'total_vehicles': len(vehicle_ids),
        'avg_compatible_chargers': sum(total_chargers_per_vehicle) / len(total_chargers_per_vehicle),
        'min_compatible_chargers': min(total_chargers_per_vehicle),
        'max_compatible_chargers': max(total_chargers_per_vehicle)
    }

# Example
fleet_ids = ["uuid1", "uuid2", "uuid3", "uuid4", "uuid5"]
analysis = analyze_fleet_compatibility(client, fleet_ids)
print(f"Fleet Analysis: {analysis}")
```

#### 5. Error Handling

```python
import requests
import time
from typing import Optional

def robust_api_call(client: EVKnowledgeGraphClient,
                    endpoint: str,
                    max_retries: int = 3) -> Optional[Dict]:
    """Make API call with retry logic and error handling"""

    for attempt in range(max_retries):
        try:
            response = client._request("GET", endpoint)
            return response

        except requests.exceptions.HTTPError as e:
            if e.response.status_code == 429:  # Rate limited
                # Get retry-after from headers
                retry_after = int(e.response.headers.get('X-RateLimit-Reset', 60))
                print(f"Rate limited. Waiting {retry_after} seconds...")
                time.sleep(retry_after)
                continue

            elif e.response.status_code == 404:
                print(f"Resource not found: {endpoint}")
                return None

            elif e.response.status_code >= 500:
                # Server error, retry with backoff
                wait_time = 2 ** attempt
                print(f"Server error. Retrying in {wait_time}s...")
                time.sleep(wait_time)
                continue

            else:
                raise

        except requests.exceptions.ConnectionError:
            wait_time = 2 ** attempt
            print(f"Connection error. Retrying in {wait_time}s...")
            time.sleep(wait_time)
            continue

    print(f"Failed after {max_retries} attempts")
    return None

# Usage
result = robust_api_call(client, "/api/v1/vehicles")
if result:
    print(f"Success: {result['data']}")
```

---

### JavaScript/Node.js Examples

#### Setup

```bash
npm install axios
```

#### 1. Basic Client

```javascript
const axios = require('axios');

class EVKnowledgeGraphClient {
  constructor(baseURL = 'http://localhost:8080') {
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    // Track request IDs for debugging
    this.client.interceptors.response.use(
      response => {
        const requestId = response.headers['x-folonite-request-id'];
        console.log(`Request ID: ${requestId}`);
        return response;
      },
      error => {
        const requestId = error.response?.headers['x-folonite-request-id'];
        console.error(`Error (Request ID: ${requestId}):`, error.message);
        throw error;
      }
    );
  }

  async healthCheck() {
    const { data } = await this.client.get('/health');
    return data;
  }

  async searchVehicles(query) {
    const { data } = await this.client.get('/api/v1/vehicles/search', {
      params: { q: query },
    });
    return data;
  }

  async findNearbyChargers(lat, lon, radius = 10) {
    const { data } = await this.client.get('/api/v1/chargers/nearby', {
      params: { lat, lon, radius },
    });
    return data;
  }

  async checkCompatibility(vehicleId, chargerId) {
    const { data } = await this.client.post('/api/v1/compatibility/check', {
      vehicle_id: vehicleId,
      charger_id: chargerId,
    });
    return data;
  }

  async batchCompatibilityCheck(vehicleIds) {
    const { data } = await this.client.post('/api/v1/batch/compatibility', {
      items: vehicleIds.map(id => ({ vehicle_id: id })),
    });
    return data;
  }
}

// Usage
const client = new EVKnowledgeGraphClient();

// Search vehicles
client.searchVehicles('tata nexon')
  .then(result => console.log('Vehicles:', result.data))
  .catch(error => console.error('Error:', error.message));
```

#### 2. React Component Example

```javascript
import React, { useState, useEffect } from 'react';
import axios from 'axios';

function NearbyChargersMap() {
  const [chargers, setChargers] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const findChargers = async (lat, lon, radius) => {
    setLoading(true);
    setError(null);

    try {
      const response = await axios.get('http://localhost:8080/api/v1/chargers/nearby', {
        params: { lat, lon, radius },
      });

      if (response.data.success) {
        setChargers(response.data.data.chargers);
      }
    } catch (err) {
      setError(err.response?.data?.error || 'Failed to fetch chargers');
    } finally {
      setLoading(false);
    }
  };

  // Get user location and find chargers
  useEffect(() => {
    if (navigator.geolocation) {
      navigator.geolocation.getCurrentPosition(
        (position) => {
          findChargers(
            position.coords.latitude,
            position.coords.longitude,
            10
          );
        },
        (err) => setError('Unable to get location')
      );
    }
  }, []);

  if (loading) return <div>Loading chargers...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div>
      <h2>Nearby EV Chargers ({chargers.length})</h2>
      <ul>
        {chargers.map(charger => (
          <li key={charger.id}>
            {charger.name} - {charger.max_power_kw}kW
            <br />
            {charger.city}, {charger.state}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default NearbyChargersMap;
```

#### 3. Express.js Backend Integration

```javascript
const express = require('express');
const axios = require('axios');

const app = express();
app.use(express.json());

const EV_API_BASE = 'http://localhost:8080';

// Proxy endpoint with caching
const cache = new Map();
const CACHE_TTL = 60000; // 1 minute

app.get('/api/vehicles/search', async (req, res) => {
  const { q } = req.query;
  const cacheKey = `search:${q}`;

  // Check cache
  if (cache.has(cacheKey)) {
    const cached = cache.get(cacheKey);
    if (Date.now() - cached.timestamp < CACHE_TTL) {
      return res.json(cached.data);
    }
  }

  try {
    const response = await axios.get(`${EV_API_BASE}/api/v1/vehicles/search`, {
      params: { q },
    });

    // Cache result
    cache.set(cacheKey, {
      data: response.data,
      timestamp: Date.now(),
    });

    res.json(response.data);
  } catch (error) {
    res.status(error.response?.status || 500).json({
      error: error.response?.data?.error || 'Internal server error',
    });
  }
});

// Fleet management endpoint
app.post('/api/fleet/compatibility', async (req, res) => {
  const { vehicleIds } = req.body;

  try {
    const response = await axios.post(
      `${EV_API_BASE}/api/v1/batch/compatibility`,
      {
        items: vehicleIds.map(id => ({ vehicle_id: id })),
      }
    );

    // Process results
    const results = response.data.data.results;
    const summary = {
      total: results.length,
      successful: results.filter(r => r.success).length,
      failed: results.filter(r => !r.success).length,
      vehicles: results.map(r => ({
        vehicle_id: r.data?.vehicle_id,
        compatible_chargers: r.data?.compatible_chargers_count || 0,
      })),
    };

    res.json(summary);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

app.listen(3000, () => console.log('Server running on port 3000'));
```

---

### Rust Examples

#### Setup

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
```

#### 1. Basic Client

```rust
use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vehicle {
    id: String,
    name: String,
    range_km: f64,
    price_inr: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Charger {
    id: String,
    name: String,
    max_power_kw: f64,
    city: String,
    state: String,
}

struct EVKnowledgeGraphClient {
    base_url: String,
    client: reqwest::Client,
}

impl EVKnowledgeGraphClient {
    fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }

    async fn search_vehicles(&self, query: &str) -> Result<Vec<Vehicle>, Box<dyn Error>> {
        let url = format!("{}/api/v1/vehicles/search", self.base_url);
        let response: ApiResponse<Vec<Vehicle>> = self.client
            .get(&url)
            .query(&[("q", query)])
            .send()
            .await?
            .json()
            .await?;

        response.data.ok_or("No data in response".into())
    }

    async fn find_nearby_chargers(
        &self,
        lat: f64,
        lon: f64,
        radius: f64,
    ) -> Result<Vec<Charger>, Box<dyn Error>> {
        let url = format!("{}/api/v1/chargers/nearby", self.base_url);

        #[derive(Deserialize)]
        struct ChargerResponse {
            chargers: Vec<Charger>,
        }

        let response: ApiResponse<ChargerResponse> = self.client
            .get(&url)
            .query(&[("lat", lat), ("lon", lon), ("radius", radius)])
            .send()
            .await?
            .json()
            .await?;

        Ok(response.data.ok_or("No data")?.chargers)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = EVKnowledgeGraphClient::new("http://localhost:8080");

    // Search vehicles
    let vehicles = client.search_vehicles("tata nexon").await?;
    println!("Found {} vehicles", vehicles.len());

    // Find chargers
    let chargers = client.find_nearby_chargers(19.0760, 72.8777, 10.0).await?;
    println!("Found {} chargers", chargers.len());

    Ok(())
}
```

---

## Use Cases

### Vehicle Discovery

#### Use Case: Find best EV under budget

**Python:**
```python
def find_best_ev_under_budget(max_price: float, min_range: int = 200):
    client = EVKnowledgeGraphClient()

    # Get all vehicles
    response = client._request("GET", "/api/v1/vehicles", params={"limit": 100})
    vehicles = response['data']['vehicles']

    # Filter and rank
    candidates = [
        v for v in vehicles
        if v.get('price_inr', float('inf')) <= max_price
        and v.get('range_km', 0) >= min_range
    ]

    # Score: range per rupee
    for v in candidates:
        v['score'] = v['range_km'] / v['price_inr'] * 1000000

    # Sort by score
    candidates.sort(key=lambda v: v['score'], reverse=True)

    return candidates[0] if candidates else None

best_ev = find_best_ev_under_budget(max_price=1500000, min_range=250)
if best_ev:
    print(f"Best EV: {best_ev['name']}")
    print(f"Range: {best_ev['range_km']}km")
    print(f"Price: ₹{best_ev['price_inr']:,.0f}")
```

### Charging Infrastructure

#### Use Case: Charging density heatmap

**Python:**
```python
import json

def generate_charging_heatmap(cities: list):
    """Generate charging infrastructure density data"""
    client = EVKnowledgeGraphClient()
    heatmap_data = []

    for city in cities:
        state, city_name = city['state'], city['name']
        lat, lon = city['lat'], city['lon']

        # Get chargers in city
        chargers = client._request(
            "GET",
            f"/api/v1/chargers/city/{state}/{city_name}"
        )

        heatmap_data.append({
            'city': city_name,
            'state': state,
            'lat': lat,
            'lon': lon,
            'charger_count': len(chargers['data']['chargers']),
            'density': len(chargers['data']['chargers']) / city.get('area_km2', 1)
        })

    return heatmap_data

cities = [
    {'name': 'Mumbai', 'state': 'Maharashtra', 'lat': 19.0760, 'lon': 72.8777, 'area_km2': 603},
    {'name': 'Delhi', 'state': 'Delhi', 'lat': 28.7041, 'lon': 77.1025, 'area_km2': 1484},
    {'name': 'Bangalore', 'state': 'Karnataka', 'lat': 12.9716, 'lon': 77.5946, 'area_km2': 741},
]

heatmap = generate_charging_heatmap(cities)
print(json.dumps(heatmap, indent=2))
```

### Compatibility Checks

#### Use Case: Pre-purchase compatibility report

**Python:**
```python
def generate_compatibility_report(vehicle_id: str, user_location: dict):
    """Generate comprehensive compatibility report for vehicle purchase"""
    client = EVKnowledgeGraphClient()

    # Get vehicle details
    vehicle = client._request("GET", f"/api/v1/vehicles/{vehicle_id}")['data']

    # Find nearby chargers
    chargers = client.find_nearby_chargers(
        lat=user_location['lat'],
        lon=user_location['lon'],
        radius=50  # 50km radius
    )['data']['chargers']

    # Check compatibility with each
    compatible_chargers = []
    for charger in chargers:
        compat = client.check_compatibility(vehicle_id, charger['id'])
        if compat['data']['is_compatible']:
            compatible_chargers.append(charger)

    # Generate report
    report = {
        'vehicle': {
            'name': vehicle['name'],
            'range': vehicle['range_km'],
            'supported_connectors': vehicle.get('supported_connectors', [])
        },
        'location': user_location['name'],
        'search_radius_km': 50,
        'total_chargers_in_range': len(chargers),
        'compatible_chargers': len(compatible_chargers),
        'compatibility_rate': len(compatible_chargers) / len(chargers) * 100 if chargers else 0,
        'charger_breakdown': {
            'ac': sum(1 for c in compatible_chargers if c['charging_type'] == 'ac'),
            'dc': sum(1 for c in compatible_chargers if c['charging_type'] == 'dc'),
            'ultra_fast': sum(1 for c in compatible_chargers if c.get('max_power_kw', 0) > 100)
        },
        'recommendation': 'Good' if len(compatible_chargers) > 20 else 'Limited' if len(compatible_chargers) > 5 else 'Poor'
    }

    return report

# Example
report = generate_compatibility_report(
    vehicle_id="your-vehicle-uuid",
    user_location={'name': 'Mumbai', 'lat': 19.0760, 'lon': 72.8777}
)

print(f"Compatibility Report: {report['vehicle']['name']}")
print(f"Compatible chargers: {report['compatible_chargers']}/{report['total_chargers_in_range']}")
print(f"Recommendation: {report['recommendation']}")
```

### Route Planning

#### Use Case: Long distance trip planning

**JavaScript:**
```javascript
async function planLongDistanceTrip(vehicleId, route) {
  const client = new EVKnowledgeGraphClient();

  // Get vehicle range
  const vehicle = await client.client.get(`/api/v1/vehicles/${vehicleId}`);
  const maxRange = vehicle.data.data.range_km;
  const safeRange = maxRange * 0.8; // 80% of max range for safety

  const chargingPlan = [];
  let distanceTraveled = 0;

  for (const waypoint of route) {
    distanceTraveled += waypoint.distance_from_prev || 0;

    // Need charging?
    if (distanceTraveled >= safeRange) {
      // Find charging station
      const chargers = await client.findNearbyChargers(
        waypoint.lat,
        waypoint.lon,
        20 // 20km radius
      );

      // Filter compatible and sort by power
      const compatible = [];
      for (const charger of chargers.data.chargers) {
        const compat = await client.checkCompatibility(vehicleId, charger.id);
        if (compat.data.is_compatible) {
          compatible.push(charger);
        }
      }

      compatible.sort((a, b) => b.max_power_kw - a.max_power_kw);

      if (compatible.length > 0) {
        const bestCharger = compatible[0];

        // Estimate charging time (simplified)
        const batteryToCharge = (distanceTraveled / maxRange) * vehicle.data.data.battery_capacity_kwh;
        const chargingTimeHours = batteryToCharge / bestCharger.max_power_kw;

        chargingPlan.push({
          location: waypoint.name,
          charger: bestCharger.name,
          power_kw: bestCharger.max_power_kw,
          estimated_time_minutes: Math.ceil(chargingTimeHours * 60),
          distance_traveled: distanceTraveled
        });

        distanceTraveled = 0; // Reset after charging
      }
    }
  }

  return chargingPlan;
}

// Example: Mumbai to Goa trip
const route = [
  { name: 'Mumbai', lat: 19.0760, lon: 72.8777, distance_from_prev: 0 },
  { name: 'Ratnagiri', lat: 16.9902, lon: 73.3120, distance_from_prev: 330 },
  { name: 'Goa', lat: 15.2993, lon: 74.1240, distance_from_prev: 140 }
];

planLongDistanceTrip('vehicle-uuid', route)
  .then(plan => {
    console.log('Charging Plan:');
    plan.forEach(stop => {
      console.log(`${stop.location}: ${stop.estimated_time_minutes}min at ${stop.charger}`);
    });
  });
```

### Fleet Management

#### Use Case: Fleet electrification assessment

**Python:**
```python
def assess_fleet_electrification(current_fleet: list, depot_location: dict):
    """Assess feasibility of electrifying a fleet"""
    client = EVKnowledgeGraphClient()

    # Find available EVs matching fleet requirements
    all_vehicles = client._request("GET", "/api/v1/vehicles", params={"limit": 100})['data']['vehicles']

    # Find charging infrastructure near depot
    nearby_chargers = client.find_nearby_chargers(
        lat=depot_location['lat'],
        lon=depot_location['lon'],
        radius=10
    )['data']['chargers']

    recommendations = []

    for vehicle in current_fleet:
        # Match requirements
        suitable_evs = [
            ev for ev in all_vehicles
            if ev['vehicle_type'] == vehicle['type']
            and ev.get('range_km', 0) >= vehicle['daily_range_required']
            and ev.get('payload_kg', 0) >= vehicle.get('payload_required', 0)
        ]

        if suitable_evs:
            # Sort by total cost of ownership (simplified)
            for ev in suitable_evs:
                # Simplified TCO calculation
                ev['tco_5yr'] = (
                    ev.get('price_inr', 0) +
                    (vehicle['annual_km'] * 5 * 2)  # Electricity cost @ ₹2/km
                    - (vehicle['annual_km'] * 5 * 8)  # Savings vs diesel @ ₹8/km
                )

            suitable_evs.sort(key=lambda v: v['tco_5yr'])

            # Check charging infrastructure
            compat_chargers = []
            for charger in nearby_chargers:
                compat = client.check_compatibility(suitable_evs[0]['id'], charger['id'])
                if compat['data']['is_compatible']:
                    compat_chargers.append(charger)

            recommendations.append({
                'current_vehicle': vehicle['name'],
                'recommended_ev': suitable_evs[0]['name'],
                'range': suitable_evs[0]['range_km'],
                'price': suitable_evs[0]['price_inr'],
                'tco_5yr': suitable_evs[0]['tco_5yr'],
                'nearby_chargers': len(compat_chargers),
                'feasibility': 'High' if len(compat_chargers) > 3 else 'Medium' if len(compat_chargers) > 0 else 'Low'
            })
        else:
            recommendations.append({
                'current_vehicle': vehicle['name'],
                'recommended_ev': None,
                'feasibility': 'Not Feasible'
            })

    return {
        'total_vehicles': len(current_fleet),
        'can_electrify': sum(1 for r in recommendations if r.get('recommended_ev')),
        'high_feasibility': sum(1 for r in recommendations if r.get('feasibility') == 'High'),
        'recommendations': recommendations
    }

# Example fleet
fleet = [
    {'name': 'Delivery Van 1', 'type': 'four_wheeler', 'daily_range_required': 100, 'annual_km': 20000},
    {'name': 'Delivery Van 2', 'type': 'four_wheeler', 'daily_range_required': 120, 'annual_km': 25000},
    {'name': 'Executive Car', 'type': 'four_wheeler', 'daily_range_required': 80, 'annual_km': 15000},
]

assessment = assess_fleet_electrification(
    current_fleet=fleet,
    depot_location={'lat': 19.0760, 'lon': 72.8777}
)

print(f"Electrification Assessment:")
print(f"Can electrify: {assessment['can_electrify']}/{assessment['total_vehicles']} vehicles")
print(f"High feasibility: {assessment['high_feasibility']} vehicles")
```

---

## Advanced Examples

### Rate Limit Handling with Exponential Backoff

**Python:**
```python
import time
import random

def api_call_with_backoff(func, max_retries=5):
    """Execute API call with exponential backoff for rate limiting"""
    for attempt in range(max_retries):
        try:
            return func()
        except requests.exceptions.HTTPError as e:
            if e.response.status_code == 429:
                # Extract retry-after from headers
                retry_after = int(e.response.headers.get('X-RateLimit-Reset', 60))

                # Add jitter to prevent thundering herd
                jitter = random.uniform(0, 1)
                wait_time = retry_after + jitter

                print(f"Rate limited. Waiting {wait_time:.2f}s (attempt {attempt + 1}/{max_retries})")
                time.sleep(wait_time)
            else:
                raise

    raise Exception(f"Failed after {max_retries} attempts")

# Usage
result = api_call_with_backoff(
    lambda: client.search_vehicles("tata")
)
```

### Websocket-style Real-time Updates (Polling)

**JavaScript:**
```javascript
class RealtimeChargerMonitor {
  constructor(lat, lon, radius, callback) {
    this.lat = lat;
    this.lon = lon;
    this.radius = radius;
    this.callback = callback;
    this.client = new EVKnowledgeGraphClient();
    this.previousChargers = new Set();
    this.intervalId = null;
  }

  async poll() {
    const chargers = await this.client.findNearbyChargers(
      this.lat,
      this.lon,
      this.radius
    );

    const currentChargers = new Set(
      chargers.data.chargers.map(c => c.id)
    );

    // Find new chargers
    const newChargers = chargers.data.chargers.filter(
      c => !this.previousChargers.has(c.id)
    );

    if (newChargers.length > 0) {
      this.callback({
        type: 'new_chargers',
        chargers: newChargers
      });
    }

    this.previousChargers = currentChargers;
  }

  start(intervalMs = 60000) {
    this.poll(); // Initial poll
    this.intervalId = setInterval(() => this.poll(), intervalMs);
  }

  stop() {
    if (this.intervalId) {
      clearInterval(this.intervalId);
      this.intervalId = null;
    }
  }
}

// Usage
const monitor = new RealtimeChargerMonitor(
  19.0760, 72.8777, 5,
  (event) => {
    if (event.type === 'new_chargers') {
      console.log(`${event.chargers.length} new chargers detected!`);
    }
  }
);

monitor.start(30000); // Poll every 30 seconds
```

---

## Complete Applications

### CLI Tool (Python)

```python
#!/usr/bin/env python3
import click
from client import EVKnowledgeGraphClient

@click.group()
def cli():
    """EV Knowledge Graph CLI"""
    pass

@cli.command()
@click.argument('query')
def search(query):
    """Search for vehicles"""
    client = EVKnowledgeGraphClient()
    results = client.search_vehicles(query)

    for vehicle in results['data']:
        click.echo(f"{vehicle['name']}: {vehicle['range_km']}km range")

@cli.command()
@click.option('--lat', type=float, required=True)
@click.option('--lon', type=float, required=True)
@click.option('--radius', type=float, default=10)
def chargers(lat, lon, radius):
    """Find nearby chargers"""
    client = EVKnowledgeGraphClient()
    results = client.find_nearby_chargers(lat, lon, radius)

    click.echo(f"Found {len(results['data']['chargers'])} chargers")
    for charger in results['data']['chargers']:
        click.echo(f"  {charger['name']} ({charger['max_power_kw']}kW)")

if __name__ == '__main__':
    cli()
```

**Usage:**
```bash
python ev_cli.py search "tata nexon"
python ev_cli.py chargers --lat 19.0760 --lon 72.8777 --radius 5
```

---

## Summary

This examples guide covered:

- Basic API client implementations in multiple languages
- Real-world use cases (vehicle discovery, route planning, fleet management)
- Advanced patterns (rate limiting, error handling, batch processing)
- Complete application examples

For more information:
- [API Documentation](API.md)
- [Getting Started Guide](GETTING_STARTED.md)
- [Architecture Documentation](ARCHITECTURE.md)

---

**Last Updated:** 2024-11-18
**Version:** 0.1.0
**Powered by:** Folonite 2026-01
