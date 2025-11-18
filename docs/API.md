# EV Knowledge Graph API Documentation

## Base URL

```
http://localhost:8080
```

## Authentication

Currently, the API does not require authentication. This will be added in future versions.

## Response Format

All API responses follow this standard format:

```json
{
  "success": true,
  "data": { ... },
  "error": null
}
```

Error responses:

```json
{
  "success": false,
  "data": null,
  "error": "Error message here"
}
```

## Response Headers

All API responses include the following branding headers:

```
X-Powered-By: Folonite
X-Folonite-Version: 2026-01
X-Folonite-Request-ID: <unique-uuid>
```

The `X-Folonite-Request-ID` header contains a unique UUID for each request, which can be used for tracking and debugging.

## Endpoints

### Health & Status

#### GET /health

Health check endpoint.

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

#### GET /stats

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

### Vehicles

#### GET /api/v1/vehicles

List all vehicles.

#### GET /api/v1/vehicles/:id

Get vehicle by ID.

**Parameters:**
- `id` (UUID): Vehicle ID

#### GET /api/v1/vehicles/search?q={query}

Search vehicles by name.

**Query Parameters:**
- `q` (string): Search term

#### GET /api/v1/vehicles/oem/:oem_id

Get all vehicles from a specific OEM.

**Parameters:**
- `oem_id` (UUID): OEM ID

### Chargers

#### GET /api/v1/chargers/city/:state/:city

Get chargers in a specific city.

**Parameters:**
- `state` (string): State name
- `city` (string): City name

#### GET /api/v1/chargers/nearby

Find nearby chargers.

**Query Parameters:**
- `lat` (float): Latitude
- `lon` (float): Longitude
- `radius` (float, optional): Search radius in km (default: 10)

### Compatibility

#### GET /api/v1/compatibility/vehicle/:vehicle_id

Find all chargers compatible with a vehicle.

**Response:**
```json
{
  "success": true,
  "data": {
    "vehicle_id": "uuid-here",
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

#### POST /api/v1/compatibility/check

Check if a vehicle and charger are compatible.

**Request:**
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

### Graph Queries

#### GET /api/v1/query/fame-eligible/:state

Get FAME-II eligible vehicles in a state.

**Parameters:**
- `state` (string): State name

#### GET /api/v1/query/indian-oems

Get all vehicles from Indian manufacturers.

#### GET /api/v1/query/network-coverage/:state

Get charging network coverage by state.

### Recommendations

#### GET /api/v1/recommendations/vehicles

Get vehicle recommendations.

**Query Parameters:**
- `type` (string): Vehicle type
- `min_range` (float): Minimum range in km
- `max_price` (float): Maximum price in INR

## Error Codes

- `200 OK`: Success
- `400 Bad Request`: Invalid request
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server error

## Rate Limiting

Currently no rate limiting is enforced. This will be added in production.

## Caching

Responses are cached using Redis with a default TTL of 1 hour (3600 seconds).

## Examples

### Find compatible chargers for a vehicle

```bash
curl http://localhost:8080/api/v1/compatibility/vehicle/{vehicle-uuid}
```

### Search for vehicles

```bash
curl http://localhost:8080/api/v1/vehicles/search?q=nexon
```

### Find chargers near a location

```bash
curl "http://localhost:8080/api/v1/chargers/nearby?lat=19.0760&lon=72.8777&radius=5"
```
