# EV Knowledge Graph API

A production-grade intelligence layer that connects, understands, and organizes every EV-related entity in India into one unified machine-readable graph.

## Overview

The EV Knowledge Graph API acts as the core EV brain for startups, OEMs, charging platforms, mapping apps, fleet systems, and developers. It provides:

- **Unified Database**: Normalized data of all Indian EV models, OEMs, and variants
- **Comprehensive Mapping**: Battery specs, charging capabilities, connector types, protocols
- **Regional Intelligence**: FAME-II data, regional regulations, compliance requirements
- **Compatibility Engine**: Vehicle ↔ Charger, Connector, Protocol matching
- **Smart Inference**: Auto-suggestions, missing relationships, fallback options
- **Real-time API**: Clean, structured output for developers

## Features

### Core Capabilities

- 🚗 **Complete EV Database** - All Indian EV models, OEMs, variants
- 🔋 **Battery Intelligence** - Specs, chemistries, performance characteristics
- ⚡ **Charging Network** - AC/DC, slow/fast/ultrafast mapping
- 🔌 **Connector Standards** - Type-2, CCS2, CHAdeMO, GB/T, Bharat DC001
- 🏢 **Operator Networks** - Tata, Statiq, Zeon, ChargeZone, and more
- 📡 **Protocol Support** - OCPP, CCS, CHAdeMO protocol capabilities
- 🗺️ **Regional Rules** - State-wise compliance, FAME-II eligibility
- 🧠 **Smart Matching** - Compatibility inference and recommendations

### Graph Queries

```
- "Which chargers support this vehicle?"
- "What connector types are compatible for this region?"
- "Which EV models support CCS2 + 22kW AC?"
- "Show me all FAME-II eligible vehicles in Maharashtra"
- "Find charging stations compatible with Tata Nexon EV Max"
```

## Tech Stack

- **Backend**: Rust (high-performance, memory-safe)
- **Graph Database**: Neo4j (relationship-first data model)
- **Cache**: Redis (sub-millisecond query responses)
- **API**: REST + GraphQL endpoints
- **Deployment**: Docker, Kubernetes-ready

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     API Layer (Axum)                    │
│  REST Endpoints │ GraphQL │ WebSocket (Real-time)       │
└─────────────────────────────────────────────────────────┘
                           │
┌─────────────────────────────────────────────────────────┐
│                   Business Logic Layer                  │
│  Query Engine │ Inference Engine │ Compatibility Rules  │
└─────────────────────────────────────────────────────────┘
                           │
        ┌──────────────────┴──────────────────┐
        │                                     │
┌───────────────────┐              ┌──────────────────┐
│   Redis Cache     │              │   Neo4j Graph    │
│  (Hot Data)       │              │  (Source of      │
│  - Query Results  │              │   Truth)         │
│  - Compatibility  │              │  - Nodes         │
│  - Sessions       │              │  - Relationships │
└───────────────────┘              └──────────────────┘
```

## Project Structure

```
EV-Knowledge-graph-API/
├── crates/
│   ├── ev-api/          # REST API server (Axum)
│   ├── ev-core/         # Domain models & business logic
│   ├── ev-graph/        # Neo4j graph operations
│   ├── ev-cache/        # Redis caching layer
│   └── ev-data/         # Data seeding & migrations
├── data/                # Seed data (Indian EV specs)
├── docker/              # Docker configurations
├── docs/                # API documentation
└── scripts/             # Deployment & utility scripts
```

## Getting Started

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs/))
- Docker & Docker Compose
- Neo4j 5.x
- Redis 7.x

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/FFFSTANZA/EV-Knowledge-graph-API-.git
   cd EV-Knowledge-graph-API-
   ```

2. **Start services with Docker**
   ```bash
   docker-compose up -d
   ```

3. **Configure environment**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Build and run**
   ```bash
   cargo build --release
   cargo run --bin ev-api
   ```

5. **Seed data**
   ```bash
   cargo run --bin ev-data -- seed
   ```

### API Endpoints

Once running, the API will be available at `http://localhost:8080`

#### Core Endpoints

```
GET  /api/v1/vehicles              - List all EV models
GET  /api/v1/vehicles/:id          - Get vehicle details
GET  /api/v1/chargers              - List all chargers
GET  /api/v1/compatibility/check   - Check compatibility
POST /api/v1/query/graph           - Execute graph query
GET  /api/v1/inference/suggest     - Get smart suggestions
```

See [API Documentation](docs/API.md) for complete endpoint reference.

## Data Model

### Core Entities (Nodes)

- **Vehicle**: EV models with specs, range, performance
- **OEM**: Manufacturers (Tata, Mahindra, Ola, Ather, etc.)
- **Battery**: Battery tech, chemistry, capacity, warranty
- **Charger**: Charging stations, operators, locations
- **Connector**: Physical connector types and standards
- **Protocol**: Communication protocols (OCPP, CCS, etc.)
- **Region**: States, cities, compliance zones
- **Policy**: FAME-II, subsidies, regional incentives

### Relationships (Edges)

- `MANUFACTURED_BY`: Vehicle → OEM
- `HAS_BATTERY`: Vehicle → Battery
- `SUPPORTS_CONNECTOR`: Vehicle → Connector
- `COMPATIBLE_WITH`: Charger ↔ Vehicle
- `USES_PROTOCOL`: Charger → Protocol
- `LOCATED_IN`: Charger → Region
- `ELIGIBLE_FOR`: Vehicle → Policy
- `REQUIRES_CONNECTOR`: Charger → Connector

## Example Queries

### Find compatible chargers for a vehicle

```rust
// REST API
GET /api/v1/compatibility/vehicle/tata-nexon-ev-max

// Response
{
  "vehicle_id": "tata-nexon-ev-max",
  "compatible_chargers": [
    {
      "charger_id": "ccs2-60kw-fast",
      "connector_type": "CCS2",
      "max_power": "60kW",
      "charging_time": "45 minutes (0-80%)"
    }
  ]
}
```

### Graph query: Find all FAME-II eligible EVs in Maharashtra

```cypher
MATCH (v:Vehicle)-[:ELIGIBLE_FOR]->(p:Policy {name: "FAME-II"})
MATCH (v)-[:AVAILABLE_IN]->(r:Region {state: "Maharashtra"})
RETURN v.name, v.price, v.range
```

## Development

### Running Tests

```bash
cargo test --workspace
```

### Database Migrations

```bash
cargo run --bin ev-data -- migrate
```

### Seeding Test Data

```bash
cargo run --bin ev-data -- seed --env development
```

## Deployment

### Docker

```bash
docker build -t ev-knowledge-graph-api .
docker run -p 8080:8080 ev-knowledge-graph-api
```

### Kubernetes

```bash
kubectl apply -f k8s/
```

## Indian EV Data Coverage

### Vehicles
- Tata (Nexon EV, Tigor EV, Punch EV)
- Mahindra (XUV400, e-Verito)
- MG (ZS EV, Comet EV)
- Hyundai (Kona, Ioniq 5)
- Ola (S1, S1 Pro, S1 Air)
- Ather (450X, 450 Plus, Rizta)
- TVS (iQube)
- Bajaj (Chetak)

### Charging Networks
- Tata Power EZ Charge
- Statiq
- Zeon
- ChargeZone
- Ather Grid
- Fortum
- Kazam

### Connector Standards (India)
- Type-2 (AC charging)
- CCS2 (DC fast charging)
- CHAdeMO (older standard)
- GB/T (Chinese standard, some imports)
- Bharat DC001 (Indian standard)

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- Documentation: [docs/](docs/)
- Issues: [GitHub Issues](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues)
- Discussions: [GitHub Discussions](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/discussions)

## Roadmap

- [ ] Phase 1: Core API & Graph Schema ✅
- [ ] Phase 2: Indian EV Data Seeding
- [ ] Phase 3: Inference Engine
- [ ] Phase 4: Real-time Updates
- [ ] Phase 5: GraphQL Support
- [ ] Phase 6: ML-based Recommendations
- [ ] Phase 7: Mobile SDKs

---

Built with ❤️ for the Indian EV ecosystem
