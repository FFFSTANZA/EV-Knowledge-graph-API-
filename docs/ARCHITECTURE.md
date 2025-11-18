# Architecture Documentation

This document provides a comprehensive overview of the EV Knowledge Graph API architecture, design decisions, and implementation details.

## Table of Contents

- [Overview](#overview)
- [System Architecture](#system-architecture)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Data Model](#data-model)
- [API Design](#api-design)
- [Caching Strategy](#caching-strategy)
- [Middleware Pipeline](#middleware-pipeline)
- [Error Handling](#error-handling)
- [Security](#security)
- [Performance](#performance)
- [Scalability](#scalability)

---

## Overview

The EV Knowledge Graph API is a production-grade intelligence layer that connects, understands, and organizes every EV-related entity in India into one unified machine-readable graph.

### Key Design Principles

1. **Graph-First:** Leverages Neo4j for relationship-rich queries
2. **High Performance:** Redis caching for sub-millisecond responses
3. **Type Safety:** Rust's compile-time guarantees
4. **Modularity:** Clear separation of concerns
5. **Scalability:** Horizontal scaling ready
6. **Observability:** Built-in metrics and tracing

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                       Load Balancer                          │
│                     (nginx/traefik)                          │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
┌───────▼─────┐     ┌───────▼─────┐     ┌───────▼─────┐
│  API Server │     │  API Server │     │  API Server │
│  (ev-api)   │     │  (ev-api)   │     │  (ev-api)   │
└───────┬─────┘     └───────┬─────┘     └───────┬─────┘
        │                   │                   │
        └───────────────────┼───────────────────┘
                            │
        ┌───────────────────┴───────────────────┐
        │                                       │
┌───────▼──────────┐                  ┌─────────▼────────┐
│  Redis Cluster   │                  │  Neo4j Cluster   │
│  (Cache Layer)   │                  │  (Graph DB)      │
│                  │                  │                  │
│  • Hot Data      │                  │  • Vehicles      │
│  • Rate Limits   │                  │  • Chargers      │
│  • Sessions      │                  │  • Relationships │
└──────────────────┘                  └──────────────────┘
```

### Request Flow

```
1. Client Request
   ↓
2. Load Balancer (nginx)
   ↓
3. API Server
   │
   ├─→ Middleware Pipeline
   │   ├─ Branding Headers
   │   ├─ Metrics Tracking
   │   ├─ Rate Limiting (Redis)
   │   └─ Authentication (optional)
   │
   ├─→ Route Handler
   │   ├─ Parameter Validation
   │   ├─ Check Cache (Redis)
   │   │   ├─ Cache Hit → Return cached data
   │   │   └─ Cache Miss → Query graph
   │   ├─ Query Neo4j
   │   ├─ Process Results
   │   └─ Update Cache
   │
   └─→ Response
       ├─ Success (200/201)
       └─ Error (4xx/5xx)
```

---

## Technology Stack

### Core Technologies

| Component | Technology | Version | Purpose |
|-----------|-----------|---------|---------|
| **Runtime** | Rust | 1.75+ | High-performance, memory-safe backend |
| **Web Framework** | Axum | 0.7 | Async HTTP server with excellent performance |
| **Graph Database** | Neo4j | 5.x | Relationship-first data model |
| **Cache** | Redis | 7.x | Sub-millisecond query responses |
| **Serialization** | Serde | 1.0 | Type-safe JSON handling |
| **Async Runtime** | Tokio | 1.35 | Async I/O and concurrency |

### Supporting Libraries

- **neo4rs:** Neo4j driver for Rust
- **redis:** Redis client with async support
- **tower/tower-http:** Middleware and HTTP utilities
- **metrics:** Prometheus metrics collection
- **tracing:** Structured logging
- **validator:** Input validation
- **uuid:** Unique identifiers

---

## Project Structure

```
EV-Knowledge-graph-API/
├── crates/
│   ├── ev-api/          # REST API server
│   │   ├── src/
│   │   │   ├── handlers/       # Request handlers
│   │   │   │   ├── vehicles.rs
│   │   │   │   ├── chargers.rs
│   │   │   │   ├── compatibility.rs
│   │   │   │   ├── admin.rs
│   │   │   │   └── batch.rs
│   │   │   ├── middleware/     # Middleware components
│   │   │   │   ├── rate_limit.rs
│   │   │   │   ├── auth.rs
│   │   │   │   └── metrics.rs
│   │   │   ├── config.rs       # Configuration management
│   │   │   ├── routes.rs       # Route definitions
│   │   │   ├── state.rs        # Application state
│   │   │   ├── types.rs        # Request/response types
│   │   │   └── main.rs         # Entry point
│   │   └── Cargo.toml
│   │
│   ├── ev-core/         # Domain models & business logic
│   │   ├── src/
│   │   │   ├── models/         # Data models
│   │   │   │   ├── vehicle.rs
│   │   │   │   ├── battery.rs
│   │   │   │   ├── charger.rs
│   │   │   │   ├── connector.rs
│   │   │   │   ├── oem.rs
│   │   │   │   ├── region.rs
│   │   │   │   ├── protocol.rs
│   │   │   │   └── policy.rs
│   │   │   ├── types.rs        # Shared types
│   │   │   └── error.rs        # Error types
│   │   └── Cargo.toml
│   │
│   ├── ev-graph/        # Neo4j graph operations
│   │   ├── src/
│   │   │   ├── repository/     # Data access layer
│   │   │   │   ├── vehicle.rs
│   │   │   │   ├── charger.rs
│   │   │   │   └── compatibility.rs
│   │   │   ├── connection.rs   # Database connection
│   │   │   ├── schema.rs       # Graph schema
│   │   │   └── queries.rs      # Cypher queries
│   │   └── Cargo.toml
│   │
│   ├── ev-cache/        # Redis caching layer
│   │   ├── src/
│   │   │   ├── cache.rs        # Cache operations
│   │   │   └── keys.rs         # Cache key generation
│   │   └── Cargo.toml
│   │
│   └── ev-data/         # Data seeding & migrations
│       ├── src/
│       │   ├── seed/           # Seed data modules
│       │   │   ├── connectors.rs
│       │   │   ├── oems.rs
│       │   │   ├── vehicles.rs
│       │   │   ├── chargers.rs
│       │   │   └── policies.rs
│       │   └── main.rs
│       └── Cargo.toml
│
├── docs/                # Documentation
├── docker/              # Docker configurations
├── scripts/             # Utility scripts
├── Cargo.toml           # Workspace configuration
├── docker-compose.yml   # Docker Compose setup
└── Makefile            # Build automation
```

### Crate Responsibilities

| Crate | Responsibility | Dependencies |
|-------|---------------|--------------|
| **ev-api** | HTTP API, routing, middleware | ev-core, ev-graph, ev-cache, axum |
| **ev-core** | Domain models, business logic | serde, chrono, uuid, validator |
| **ev-graph** | Neo4j operations, repositories | ev-core, neo4rs |
| **ev-cache** | Redis operations, caching | ev-core, redis |
| **ev-data** | Data seeding, migrations | ev-core, ev-graph, neo4rs |

---

## Data Model

### Graph Schema

#### Nodes

```
Vehicle {
  id: UUID
  name: String
  model_code: String
  vehicle_type: Enum
  segment: Enum
  range_km: Float
  power_kw: Float
  is_active: Boolean
  created_at: DateTime
  updated_at: DateTime
}

Oem {
  id: UUID
  name: String
  brand_name: String
  country_of_origin: String
  is_indian_company: Boolean
  has_indian_manufacturing: Boolean
}

Charger {
  id: UUID
  name: String
  charging_type: Enum
  max_power_kw: Float
  is_operational: Boolean
  latitude: Float
  longitude: Float
  city: String
  state: String
}

Connector {
  id: UUID
  connector_type: Enum (TYPE2, CCS2, CHADEMO, GB_T, BHARAT_DC001)
  max_voltage_v: Float
  max_current_a: Float
  max_power_kw: Float
  is_dc: Boolean
}

Battery {
  id: UUID
  chemistry: Enum (NMC, LFP, NCA, LTO)
  capacity_kwh: Float
  warranty_years: Integer
  warranty_km: Integer
}

Policy {
  id: UUID
  name: String (e.g., "FAME-II")
  policy_type: Enum
  level: Enum (national, state, municipal)
  incentive_amount_inr: Float
  is_active: Boolean
}
```

#### Relationships

```
(Vehicle)-[:MANUFACTURED_BY]->(Oem)
(Vehicle)-[:HAS_BATTERY]->(Battery)
(Vehicle)-[:SUPPORTS_CONNECTOR]->(Connector)
(Vehicle)-[:AVAILABLE_IN]->(Region)
(Vehicle)-[:ELIGIBLE_FOR]->(Policy)

(Charger)-[:OPERATED_BY]->(ChargingOperator)
(Charger)-[:REQUIRES_CONNECTOR]->(Connector)
(Charger)-[:LOCATED_IN]->(Region)
(Charger)-[:USES_PROTOCOL]->(Protocol)

(Charger)-[:COMPATIBLE_WITH]->(Vehicle)
```

### Indexes

```cypher
// Unique constraints
CREATE CONSTRAINT vehicle_id FOR (v:Vehicle) REQUIRE v.id IS UNIQUE;
CREATE CONSTRAINT oem_id FOR (o:Oem) REQUIRE o.id IS UNIQUE;
CREATE CONSTRAINT charger_id FOR (c:Charger) REQUIRE c.id IS UNIQUE;

// Performance indexes
CREATE INDEX vehicle_name FOR (v:Vehicle) ON (v.name);
CREATE INDEX vehicle_type FOR (v:Vehicle) ON (v.vehicle_type);
CREATE INDEX charger_state FOR (c:Charger) ON (c.state);
CREATE INDEX charger_city FOR (c:Charger) ON (c.city);

// Full-text search
CREATE FULLTEXT INDEX vehicle_search FOR (v:Vehicle) ON EACH [v.name, v.model_code];
CREATE FULLTEXT INDEX charger_search FOR (c:Charger) ON EACH [c.name, c.city, c.address];
```

---

## API Design

### RESTful Principles

1. **Resource-based URLs:** `/api/v1/vehicles`, `/api/v1/chargers`
2. **HTTP Methods:** GET (read), POST (create), PUT (update), DELETE (delete)
3. **Status Codes:** Appropriate HTTP status codes (200, 201, 400, 404, 500)
4. **Versioning:** URL-based versioning (`/api/v1/`)

### Response Format

```json
{
  "success": boolean,
  "data": object | null,
  "error": string | null,
  "error_code": string | null,
  "details": object | null
}
```

### Error Handling

- Structured errors with codes
- HTTP status code mapping
- No sensitive data in errors
- Request ID for tracing

### Pagination

```
Query Parameters:
- page: 1-based page number
- limit: Items per page (max 100)
- sort_by: Field to sort by
- order: "asc" | "desc"

Response includes:
- data: Results array
- pagination: Metadata (total, pages, has_next, has_prev)
```

---

## Caching Strategy

### Cache Layers

1. **Hot Data Cache** - Redis, TTL: 1 hour
   - Vehicle details
   - Charger details
   - OEM information

2. **Query Cache** - Redis, TTL: 5 minutes
   - Compatibility results
   - Search results
   - Nearby chargers

3. **Rate Limit Cache** - Redis, TTL: 1 hour
   - Per-minute counters
   - Per-hour counters

### Cache Keys

```
Pattern: ev:{resource}:{identifier}

Examples:
- ev:vehicle:uuid
- ev:charger:uuid
- ev:vehicles:page:1:20
- ev:compatibility:vehicle_uuid:chargers
- ev:chargers:Maharashtra:Mumbai
- ev:ratelimit:endpoint:client_id
```

### Cache Invalidation

1. **TTL-based:** Automatic expiration
2. **Manual:** Admin endpoints for cache clear
3. **Pattern-based:** Clear specific patterns
4. **Event-driven:** On data updates

---

## Middleware Pipeline

Order matters! Middleware executes in order:

```
1. Branding Headers     - Add X-Powered-By, X-Folonite-* headers
2. Metrics Tracking     - Record request duration, count
3. Rate Limiting        - Check Redis, enforce limits
4. Authentication       - Validate API keys (if required)
5. CORS                 - Handle cross-origin requests
6. Tracing              - Structured logging
7. Route Handler        - Process request
8. Error Handler        - Convert errors to responses
```

### Middleware Implementation

```rust
// Simplified middleware structure
Router::new()
    .route("/api/v1/...", handler)
    .layer(branding_headers)
    .layer(metrics_tracking)
    .layer(rate_limiting)
    .layer(cors)
    .layer(tracing)
```

---

## Error Handling

### Error Types

```rust
pub enum Error {
    NotFound(String),           // 404
    Validation(String),         // 400
    BadRequest(String),         // 400
    Unauthorized(String),       // 401
    Forbidden(String),          // 403
    RateLimitExceeded,          // 429
    Database(String),           // 500
    Cache(String),              // 500
    Internal(String),           // 500
}
```

### Error Flow

```
1. Error occurs in handler/repository
2. Error propagated with `?` operator
3. Converted to ApiError via From trait
4. ApiError implements IntoResponse
5. Response with appropriate status code and error body
```

---

## Security

### Current Features

1. **Rate Limiting** - Prevents abuse
2. **Input Validation** - Sanitizes user input
3. **Error Handling** - No sensitive data leakage
4. **Request Tracing** - Audit trail

### Planned Features

1. **API Key Authentication** - In progress
2. **OAuth2/JWT** - Planned
3. **Role-Based Access Control** - Planned
4. **HTTPS Only** - Production deployment
5. **SQL Injection Prevention** - Parameterized queries (Neo4j)

---

## Performance

### Optimizations

1. **Async I/O** - Tokio runtime for concurrent requests
2. **Connection Pooling** - Neo4j and Redis pools
3. **Caching** - Redis for hot data
4. **Indexing** - Neo4j indexes for fast lookups
5. **Batch Operations** - Reduce round trips

### Benchmarks

```
Single Request:
- Cached: <5ms
- Non-cached: <50ms
- Complex graph query: <200ms

Throughput:
- Simple queries: 10,000+ req/sec
- Complex queries: 1,000+ req/sec
```

---

## Scalability

### Horizontal Scaling

```
API Servers:
- Stateless design
- Can run multiple instances
- Load balanced
- Shared Redis & Neo4j

Database:
- Neo4j: Read replicas
- Redis: Cluster mode
```

### Vertical Scaling

```
- Increase Neo4j heap/pagecache
- Increase Redis memory
- More CPU cores for API
```

### Bottlenecks & Solutions

| Bottleneck | Solution |
|------------|----------|
| Neo4j writes | Batch operations, queue writes |
| Neo4j reads | Read replicas, caching |
| Redis memory | Cluster mode, optimize TTLs |
| API CPU | Horizontal scaling |
| Network I/O | CDN for static assets |

---

## Design Decisions

### Why Rust?

- Memory safety without garbage collection
- Zero-cost abstractions
- Excellent async support
- Strong type system
- High performance

### Why Neo4j?

- Native graph database
- Cypher query language
- ACID transactions
- Excellent for relationship queries
- Scales well for read-heavy workloads

### Why Redis?

- Sub-millisecond latency
- Rich data structures
- Pub/sub support
- Cluster mode for scaling
- Battle-tested in production

### Why Axum?

- Built on Tokio + Tower
- Excellent performance
- Type-safe extractors
- Modular middleware
- Active development

---

## Future Enhancements

1. **GraphQL API** - Flexible queries
2. **WebSocket Support** - Real-time updates
3. **ML Recommendations** - Predictive analytics
4. **Multi-tenancy** - Isolated data per tenant
5. **OpenAPI/Swagger** - Auto-generated docs
6. **Event Sourcing** - Audit trail
7. **CQRS** - Separate read/write models

---

## References

- [Axum Documentation](https://docs.rs/axum)
- [Neo4j Cypher Manual](https://neo4j.com/docs/cypher-manual)
- [Redis Documentation](https://redis.io/documentation)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Documentation](https://tokio.rs/)

---

**Last Updated:** 2024-11-18
**Version:** 0.1.0
