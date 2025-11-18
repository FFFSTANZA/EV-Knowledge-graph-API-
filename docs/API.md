# EV Knowledge Graph API Documentation

**Version:** 0.1.0
**Base URL:** `http://localhost:8080`
**Powered by:** Folonite

## Table of Contents

- [Authentication](#authentication)
- [Response Format](#response-format)
- [Response Headers](#response-headers)
- [Rate Limiting](#rate-limiting)
- [Error Codes](#error-codes)
- [Endpoints](#endpoints)
  - [Health & Status](#health--status)
  - [Vehicles](#vehicles)
  - [Chargers](#chargers)
  - [Compatibility](#compatibility)
  - [Batch Operations](#batch-operations)
  - [Graph Queries](#graph-queries)
  - [Recommendations](#recommendations)
  - [Admin](#admin-endpoints)
  - [Metrics](#metrics)

---

## Authentication

Currently, the API does not require authentication for public endpoints. Admin endpoints may require API key authentication in production.

**API Key Header:**
```
X-API-Key: your_api_key_here
```

---

## Response Format

All API responses follow this standard format:

**Success Response:**
```json
{
  "success": true,
  "data": { ... }
}
```

**Error Response:**
```json
{
  "success": false,
  "error": "Error message",
  "error_code": "ERROR_CODE",
  "details": { ... }
}
```

---

## Response Headers

All API responses include Folonite branding headers:

```
X-Powered-By: Folonite
X-Folonite-Version: 2026-01
X-Folonite-Request-ID: <unique-uuid>
```

**Request ID:** Each request gets a unique UUID for tracking and debugging.

---

## Rate Limiting

The API implements Redis-based distributed rate limiting:

- **Per Minute:** 60 requests
- **Per Hour:** 1,000 requests

**Rate Limit Headers:**
```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 60
```

**Rate Limit Exceeded Response (429):**
```json
{
  "success": false,
  "error": "Rate limit exceeded",
  "error_code": "RATE_LIMIT_EXCEEDED",
  "details": {
    "limit": 60,
    "window_seconds": 60,
    "message": "You have exceeded the rate limit of 60 requests per 60 seconds"
  }
}
```

---

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `NOT_FOUND` | 404 | Resource not found |
| `BAD_REQUEST` | 400 | Invalid request parameters |
| `VALIDATION_ERROR` | 400 | Input validation failed |
| `UNAUTHORIZED` | 401 | Authentication required |
| `FORBIDDEN` | 403 | Access denied |
| `RATE_LIMIT_EXCEEDED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |
| `DATABASE_ERROR` | 500 | Database operation failed |
| `CACHE_ERROR` | 500 | Cache operation failed |

---

## Endpoints

### Health & Status

#### GET `/health`
Basic health check endpoint.

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

#### GET `/stats`
Get database statistics.

**Response:**
```json
{
  "success": true,
  "data": {
    "nodes": [
      ["Vehicle", 150],
      ["Charger", 500],
      ["Oem", 10]
    ]
  }
}
```

---

### Vehicles

#### GET `/api/v1/vehicles`
List all electric vehicles with pagination.

**Query Parameters:**
- `page` (integer, default: 1) - Page number
- `limit` (integer, default: 20, max: 100) - Items per page
- `sort_by` (string, default: "created_at") - Sort field
- `order` (string: "asc"|"desc", default: "desc") - Sort order

**Response:**
```json
{
  "success": true,
  "data": {
    "vehicles": ["List of vehicles"],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 150,
      "total_pages": 8,
      "has_next": true,
      "has_prev": false
    }
  }
}
```

#### GET `/api/v1/vehicles/:id`
Get vehicle details by ID.

**Parameters:**
- `id` (UUID) - Vehicle ID

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "uuid",
    "name": "Tata Nexon EV Max",
    "oem": "Tata Motors",
    "range_km": 437,
    "battery_capacity_kwh": 40.5
  }
}
```

#### GET `/api/v1/vehicles/search?q={query}`
Full-text search for vehicles.

**Query Parameters:**
- `q` (string, required) - Search term

**Example:**
```bash
curl "http://localhost:8080/api/v1/vehicles/search?q=nexon"
```

#### GET `/api/v1/vehicles/oem/:oem_id`
Get all vehicles from a specific manufacturer.

**Parameters:**
- `oem_id` (UUID) - OEM ID

---

### Chargers

#### GET `/api/v1/chargers`
List all charging stations with filtering.

**Query Parameters:**
- `page`, `limit`, `sort_by`, `order` - Pagination
- `charging_type` (string) - "ac", "dc", "dc_ultra_fast"
- `min_power_kw`, `max_power_kw` (float) - Power range
- `is_operational` (boolean) - Operational status
- `is_public` (boolean) - Public access

#### GET `/api/v1/chargers/:id`
Get charger details by ID.

#### GET `/api/v1/chargers/city/:state/:city`
Get chargers in a specific city.

**Parameters:**
- `state` (string) - State name (e.g., "Maharashtra")
- `city` (string) - City name (e.g., "Mumbai")

**Example:**
```bash
curl "http://localhost:8080/api/v1/chargers/city/Maharashtra/Mumbai"
```

**Response:**
```json
{
  "success": true,
  "data": {
    "chargers": [
      "uuid: Tata Power Fast Charger - Andheri East (60kW)",
      "uuid: Statiq Charger - BKC (50kW)"
    ]
  }
}
```

#### GET `/api/v1/chargers/nearby`
Find chargers near a location.

**Query Parameters:**
- `lat` (float, required) - Latitude
- `lon` (float, required) - Longitude
- `radius` (float, default: 10, max: 500) - Search radius in km

**Example:**
```bash
curl "http://localhost:8080/api/v1/chargers/nearby?lat=19.0760&lon=72.8777&radius=5"
```

---

### Compatibility

#### GET `/api/v1/compatibility/vehicle/:vehicle_id`
Find all chargers compatible with a vehicle.

**Response:**
```json
{
  "success": true,
  "data": {
    "vehicle_id": "uuid",
    "chargers": [
      {
        "charger_id": "uuid",
        "charger_name": "Tata Power Fast Charger",
        "connector_type": "CCS2",
        "max_power_kw": 60.0,
        "city": "Mumbai",
        "state": "Maharashtra"
      }
    ]
  }
}
```

#### GET `/api/v1/compatibility/charger/:charger_id`
Find all vehicles compatible with a charger.

#### POST `/api/v1/compatibility/check`
Check if a specific vehicle and charger are compatible.

**Request Body:**
```json
{
  "vehicle_id": "uuid",
  "charger_id": "uuid"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "vehicle_id": "uuid",
    "charger_id": "uuid",
    "is_compatible": true
  }
}
```

---

### Batch Operations

#### POST `/api/v1/batch/compatibility`
Batch check vehicle compatibility for multiple vehicles.

**Request Body:**
```json
{
  "items": [
    { "vehicle_id": "uuid1" },
    { "vehicle_id": "uuid2" },
    { "vehicle_id": "uuid3" }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "total": 3,
    "successful": 3,
    "failed": 0,
    "results": [
      {
        "index": 0,
        "success": true,
        "data": {
          "vehicle_id": "uuid1",
          "compatible_chargers_count": 45
        }
      }
    ]
  }
}
```

#### POST `/api/v1/batch/compatibility/check`
Batch compatibility check for vehicle-charger pairs.

**Request Body:**
```json
{
  "items": [
    { "vehicle_id": "uuid1", "charger_id": "uuid_a" },
    { "vehicle_id": "uuid2", "charger_id": "uuid_b" }
  ]
}
```

---

### Graph Queries

#### GET `/api/v1/query/fame-eligible/:state`
Get FAME-II eligible vehicles in a state.

**Parameters:**
- `state` (string) - State name (e.g., "Delhi", "Maharashtra")

**Example:**
```bash
curl "http://localhost:8080/api/v1/query/fame-eligible/Delhi"
```

#### GET `/api/v1/query/indian-oems`
Get all vehicles from Indian manufacturers.

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Indian OEM vehicles query"
  }
}
```

#### GET `/api/v1/query/network-coverage/:state`
Get charging network coverage by state.

**Parameters:**
- `state` (string) - State name

---

### Recommendations

#### GET `/api/v1/recommendations/vehicles`
Get vehicle recommendations based on criteria.

**Query Parameters:**
- `type` (string) - Vehicle type
- `min_range` (float) - Minimum range in km
- `max_price` (float) - Maximum price in INR

**Example:**
```bash
curl "http://localhost:8080/api/v1/recommendations/vehicles?type=four_wheeler&min_range=300&max_price=2000000"
```

#### GET `/api/v1/recommendations/chargers`
Get recommended chargers for a vehicle near a location.

**Query Parameters:**
- `lat`, `lon` (float) - Location
- `vehicle_id` (UUID) - Vehicle ID

---

### Admin Endpoints

**Note:** These endpoints should be protected with authentication in production.

#### GET `/admin/health/deep`
Deep system health check with component-level monitoring.

**Response:**
```json
{
  "success": true,
  "data": {
    "overall_status": "healthy",
    "components": {
      "neo4j": {
        "status": "up",
        "latency_ms": 12.5,
        "details": "bolt://localhost:7687"
      },
      "redis": {
        "status": "up",
        "latency_ms": 2.3,
        "details": "redis://localhost:6379"
      },
      "api": {
        "status": "up",
        "latency_ms": null,
        "details": "port 8080"
      }
    },
    "version": "0.1.0",
    "uptime_seconds": 0
  }
}
```

#### POST `/admin/cache/clear`
Clear cache (requires admin access).

**Response:**
```json
{
  "success": true,
  "data": "Cache cleared successfully"
}
```

#### GET `/admin/cache/stats`
Get cache statistics.

#### GET `/admin/graph/stats`
Get detailed graph database statistics.

**Response:**
```json
{
  "success": true,
  "data": {
    "nodes": [["Vehicle", 150], ["Charger", 500]],
    "relationships": [],
    "indexes": ["vehicle_name", "vehicle_search", "charger_location"]
  }
}
```

#### POST `/admin/graph/rebuild-indexes`
Rebuild Neo4j indexes (maintenance operation).

---

### Metrics

#### GET `/metrics`
Prometheus metrics endpoint.

**Response Format:** Prometheus text format

**Example Metrics:**
```
# HELP http_requests_total Total HTTP requests
# TYPE http_requests_total counter
http_requests_total 1234

# HELP http_request_duration_seconds HTTP request duration
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds_sum 45.2
http_request_duration_seconds_count 1234
```

---

## Examples

### Complete Workflow Example

```bash
# 1. Health check
curl http://localhost:8080/health

# 2. Search for vehicles
curl "http://localhost:8080/api/v1/vehicles/search?q=nexon"

# 3. Find nearby chargers
curl "http://localhost:8080/api/v1/chargers/nearby?lat=19.0760&lon=72.8777&radius=10"

# 4. Check compatibility
curl -X POST http://localhost:8080/api/v1/compatibility/check \
  -H "Content-Type: application/json" \
  -d '{
    "vehicle_id": "your-vehicle-uuid",
    "charger_id": "your-charger-uuid"
  }'

# 5. Get FAME-II eligible vehicles
curl "http://localhost:8080/api/v1/query/fame-eligible/Maharashtra"

# 6. Admin: Check system health
curl http://localhost:8080/admin/health/deep
```

---

## Pagination

All list endpoints support pagination:

**Query Parameters:**
- `page` (default: 1) - Page number
- `limit` (default: 20, max: 100) - Items per page

**Response includes pagination metadata:**
```json
{
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 150,
    "total_pages": 8,
    "has_next": true,
    "has_prev": false
  }
}
```

---

## Filtering

Vehicle and charger endpoints support filtering:

**Vehicle Filters:**
- `vehicle_type`, `segment`, `oem_id`
- `min_range_km`, `max_range_km`
- `min_price_inr`, `max_price_inr`
- `state`, `is_fame_eligible`

**Charger Filters:**
- `charging_type`, `connector_type`
- `min_power_kw`, `max_power_kw`
- `operator_id`, `is_operational`, `is_public`

---

## Sorting

List endpoints support sorting:

**Query Parameters:**
- `sort_by` (default: "created_at") - Field to sort by
- `order` ("asc" | "desc", default: "desc") - Sort order

---

## SDKs & Client Libraries

Coming soon:
- Python SDK
- JavaScript/TypeScript SDK
- Rust SDK
- Go SDK

---

## Support

- **Documentation:** [docs/](../docs/)
- **Issues:** [GitHub Issues](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues)
- **Discussions:** [GitHub Discussions](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/discussions)

---

**Last Updated:** 2024-11-18
**API Version:** 0.1.0
**Powered by:** Folonite 2026-01
