# EV Knowledge Graph API - Folonite

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com)
[![Powered by](https://img.shields.io/badge/powered_by-Folonite-purple.svg)](https://folonite.com)

A production-grade intelligence layer that connects, understands, and organizes every EV-related entity in India into one unified machine-readable graph.

## Overview

The EV Knowledge Graph API acts as the core EV brain for startups, OEMs, charging platforms, mapping apps, fleet systems, and developers. It provides:

- **Unified Database**: Normalized data of all Indian EV models, OEMs, and variants
- **Comprehensive Mapping**: Battery specs, charging capabilities, connector types, protocols
- **Regional Intelligence**: FAME-II data, regional regulations, compliance requirements
- **Compatibility Engine**: Vehicle ↔ Charger, Connector, Protocol matching
- **Smart Inference**: Auto-suggestions, missing relationships, fallback options
- **Production-Ready API**: Clean, structured output with rate limiting, caching, and metrics

## ✨ Features

### Core Capabilities

- 🚗 **Complete EV Database** - All Indian EV models, OEMs, variants
- 🔋 **Battery Intelligence** - Specs, chemistries, performance characteristics
- ⚡ **Charging Network** - AC/DC, slow/fast/ultrafast mapping
- 🔌 **Connector Standards** - Type-2, CCS2, CHAdeMO, GB/T, Bharat DC001
- 🏢 **Operator Networks** - Tata, Statiq, Zeon, ChargeZone, and more
- 📡 **Protocol Support** - OCPP, CCS, CHAdeMO protocol capabilities
- 🗺️ **Regional Rules** - State-wise compliance, FAME-II eligibility
- 🧠 **Smart Matching** - Compatibility inference and recommendations

### Production Features

- 🚀 **High Performance** - Sub-millisecond responses with Redis caching
- 🛡️ **Rate Limiting** - Distributed Redis-based rate limiting (60/min, 1000/hour)
- 📊 **Metrics & Monitoring** - Prometheus metrics endpoint
- ⚙️ **Admin Endpoints** - Cache management, system health, graph statistics
- 🔄 **Batch Operations** - Efficient bulk processing
- 🏷️ **Branding Headers** - Folonite-branded response headers with request tracking
- ❌ **Advanced Error Handling** - Structured errors with codes
- 📝 **Comprehensive Docs** - API, architecture, deployment, troubleshooting guides

### Graph Queries

```
- "Which chargers support this vehicle?"
- "What connector types are compatible for this region?"
- "Which EV models support CCS2 + 22kW AC?"
- "Show me all FAME-II eligible vehicles in Maharashtra"
- "Find charging stations compatible with Tata Nexon EV Max"
```

## 🛠️ Tech Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| **Backend** | Rust 1.75+ | High-performance, memory-safe API |
| **Web Framework** | Axum 0.7 | Async HTTP server |
| **Graph Database** | Neo4j 5.x | Relationship-first data model |
| **Cache** | Redis 7.x | Sub-millisecond query responses |
| **Metrics** | Prometheus | Performance monitoring |
| **Deployment** | Docker + Kubernetes | Container orchestration |

## 🏗️ Architecture

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

### Middleware Pipeline

```
Request → Branding Headers → Metrics → Rate Limiting → Auth → CORS → Handler → Response
```

## 📁 Project Structure

```
EV-Knowledge-graph-API/
├── crates/
│   ├── ev-api/          # REST API server (Axum)
│   │   ├── handlers/    # Request handlers
│   │   ├── middleware/  # Rate limiting, auth, metrics
│   │   └── routes.rs    # Route definitions
│   ├── ev-core/         # Domain models & business logic
│   ├── ev-graph/        # Neo4j graph operations
│   ├── ev-cache/        # Redis caching layer
│   └── ev-data/         # Data seeding & migrations
├── docs/                # Comprehensive documentation
│   ├── API.md           # API reference
│   ├── ARCHITECTURE.md  # System architecture
│   ├── GETTING_STARTED.md
│   ├── DEPLOYMENT.md    # Production deployment
│   ├── TROUBLESHOOTING.md
│   ├── EXAMPLES.md      # Code examples
│   └── CONTRIBUTING.md  # Developer guidelines
├── docker/              # Docker configurations
├── scripts/             # Deployment & utility scripts
└── Cargo.toml           # Workspace configuration
```

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.75+ (install via [rustup](https://rustup.rs/))
- **Docker** & **Docker Compose**
- **Neo4j** 5.x (via Docker)
- **Redis** 7.x (via Docker)

### Quick Start (Docker - Recommended)

```bash
# 1. Clone the repository
git clone https://github.com/FFFSTANZA/EV-Knowledge-graph-API-.git
cd EV-Knowledge-graph-API-

# 2. Start all services
docker-compose up -d

# 3. Wait for services to be ready (30-60 seconds)
docker-compose logs -f api

# 4. Verify the API is running
curl http://localhost:8080/health

# 5. (Optional) Seed with Indian EV data
docker-compose exec api ev-data seed
```

**Access Points:**
- **API**: http://localhost:8080
- **API Docs**: http://localhost:8080/docs
- **Neo4j Browser**: http://localhost:7474 (neo4j/password123)
- **Metrics**: http://localhost:8080/metrics
- **Admin Health**: http://localhost:8080/admin/health/deep

### Local Development

```bash
# 1. Start infrastructure services
docker-compose up -d neo4j redis

# 2. Configure environment
cp .env.example .env
# Edit .env with your configuration

# 3. Build the project
cargo build --release

# 4. Run migrations and seed data
cargo run --bin ev-data -- migrate
cargo run --bin ev-data -- seed

# 5. Run the API
cargo run --release --bin ev-api
```

See [Getting Started Guide](docs/GETTING_STARTED.md) for detailed instructions.

## 📚 API Endpoints

### Core Endpoints

```bash
# Health & Status
GET  /health                           # Basic health check
GET  /admin/health/deep                # Detailed component health
GET  /stats                            # Database statistics

# Vehicles
GET  /api/v1/vehicles                  # List all vehicles
GET  /api/v1/vehicles/:id              # Get vehicle details
GET  /api/v1/vehicles/search?q=nexon   # Search vehicles

# Chargers
GET  /api/v1/chargers                  # List all chargers
GET  /api/v1/chargers/:id              # Get charger details
GET  /api/v1/chargers/nearby           # Find nearby chargers
GET  /api/v1/chargers/city/:state/:city  # Chargers by city

# Compatibility
GET  /api/v1/compatibility/vehicle/:id # Compatible chargers
POST /api/v1/compatibility/check       # Check specific pair

# Batch Operations
POST /api/v1/batch/compatibility       # Batch compatibility check

# Admin Endpoints
POST /admin/cache/clear                # Clear cache
GET  /admin/cache/stats                # Cache statistics
GET  /admin/graph/stats                # Graph database stats
POST /admin/graph/rebuild-indexes      # Rebuild indexes

# Metrics
GET  /metrics                          # Prometheus metrics
```

See complete [API Documentation](docs/API.md) for detailed endpoint reference.

## 💡 Quick Examples

### cURL Examples

```bash
# Search for vehicles
curl "http://localhost:8080/api/v1/vehicles/search?q=tata+nexon"

# Find chargers near Mumbai
curl "http://localhost:8080/api/v1/chargers/nearby?lat=19.0760&lon=72.8777&radius=5"

# Check compatibility
curl -X POST http://localhost:8080/api/v1/compatibility/check \
  -H "Content-Type: application/json" \
  -d '{"vehicle_id": "uuid", "charger_id": "uuid"}'
```

### Python Example

```python
import requests

class EVKnowledgeGraphClient:
    def __init__(self, base_url="http://localhost:8080"):
        self.base_url = base_url

    def search_vehicles(self, query):
        response = requests.get(f"{self.base_url}/api/v1/vehicles/search",
                               params={"q": query})
        return response.json()

    def find_nearby_chargers(self, lat, lon, radius=10):
        response = requests.get(f"{self.base_url}/api/v1/chargers/nearby",
                               params={"lat": lat, "lon": lon, "radius": radius})
        return response.json()

# Usage
client = EVKnowledgeGraphClient()
vehicles = client.search_vehicles("tata nexon")
chargers = client.find_nearby_chargers(19.0760, 72.8777)
```

See more examples in [EXAMPLES.md](docs/EXAMPLES.md).

## 🗄️ Data Model

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

```
(Vehicle)-[:MANUFACTURED_BY]->(OEM)
(Vehicle)-[:HAS_BATTERY]->(Battery)
(Vehicle)-[:SUPPORTS_CONNECTOR]->(Connector)
(Vehicle)-[:ELIGIBLE_FOR]->(Policy)
(Charger)-[:COMPATIBLE_WITH]->(Vehicle)
(Charger)-[:LOCATED_IN]->(Region)
(Charger)-[:REQUIRES_CONNECTOR]->(Connector)
```

See [Architecture Documentation](docs/ARCHITECTURE.md) for complete schema.

## 🇮🇳 Indian EV Data Coverage

### Vehicles
- **Tata**: Nexon EV, Tigor EV, Punch EV
- **Mahindra**: XUV400, e-Verito
- **MG**: ZS EV, Comet EV
- **Hyundai**: Kona, Ioniq 5
- **Ola**: S1, S1 Pro, S1 Air
- **Ather**: 450X, 450 Plus, Rizta
- **TVS**: iQube
- **Bajaj**: Chetak

### Charging Networks
- Tata Power EZ Charge
- Statiq
- Zeon
- ChargeZone
- Ather Grid
- Fortum
- Kazam

### Connector Standards (India)
- **Type-2** (AC charging)
- **CCS2** (DC fast charging)
- **CHAdeMO** (older standard)
- **GB/T** (Chinese standard, some imports)
- **Bharat DC001** (Indian standard for DC)
- **Bharat AC001** (Indian standard for AC)

## 🛡️ Production Features

### Rate Limiting

- **Per Minute**: 60 requests per client
- **Per Hour**: 1,000 requests per client
- Distributed Redis-based implementation
- Customizable limits via environment variables

### Caching Strategy

- **Hot Data Cache**: 1 hour TTL (vehicle/charger details)
- **Query Cache**: 5 minutes TTL (search results, compatibility)
- **Pattern-based invalidation**: Admin endpoints for cache management

### Monitoring

- **Prometheus metrics** at `/metrics`
- **Health checks**: Basic and deep component monitoring
- **Request tracking**: Unique request IDs via `X-Folonite-Request-ID`
- **Structured logging**: tracing with debug/info/error levels

### Branding

All API responses include Folonite branding headers:
```
X-Powered-By: Folonite
X-Folonite-Version: 2026-01
X-Folonite-Request-ID: <unique-uuid>
```

## 🧪 Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p ev-api

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Lint code
cargo clippy --all-targets --all-features -- -D warnings

# Check compilation
cargo check --workspace
```

### Database Operations

```bash
# Run migrations
cargo run --bin ev-data -- migrate

# Seed test data
cargo run --bin ev-data -- seed --env development

# Seed production data
cargo run --bin ev-data -- seed --env production
```

## 📦 Deployment

### Docker Deployment

```bash
# Build production image
docker build -t ev-api:latest .

# Run with docker-compose
docker-compose -f docker-compose.prod.yml up -d
```

### Kubernetes Deployment

```bash
# Apply Kubernetes manifests
kubectl apply -f k8s/

# Scale API instances
kubectl scale deployment ev-api --replicas=3
```

### Cloud Platforms

- **AWS**: ECS/EKS deployment guides
- **GCP**: Cloud Run / GKE deployment guides
- **Azure**: ACI / AKS deployment guides

See [Deployment Guide](docs/DEPLOYMENT.md) for detailed production deployment instructions.

## 📖 Documentation

Comprehensive documentation available:

- **[Getting Started](docs/GETTING_STARTED.md)** - Installation and quick start
- **[API Reference](docs/API.md)** - Complete endpoint documentation
- **[Architecture](docs/ARCHITECTURE.md)** - System design and technical details
- **[Deployment](docs/DEPLOYMENT.md)** - Production deployment guide
- **[Troubleshooting](docs/TROUBLESHOOTING.md)** - Common issues and solutions
- **[Examples](docs/EXAMPLES.md)** - Code examples in Python, JavaScript, Rust
- **[Contributing](docs/CONTRIBUTING.md)** - Developer guidelines

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for:

- Code of conduct
- Development setup
- Coding standards
- Testing guidelines
- Pull request process

### Quick Start for Contributors

```bash
# Fork the repository
# Clone your fork
git clone https://github.com/YOUR_USERNAME/EV-Knowledge-graph-API-.git

# Create a feature branch
git checkout -b feature/your-feature-name

# Make changes and test
cargo test --workspace
cargo clippy --all-targets

# Commit and push
git commit -m "feat: add your feature"
git push origin feature/your-feature-name

# Create a pull request on GitHub
```

## 🗺️ Roadmap

- [x] **Phase 1**: Core API & Graph Schema
- [x] **Phase 2**: Production features (rate limiting, caching, metrics)
- [ ] **Phase 3**: Indian EV Data Seeding (in progress)
- [ ] **Phase 4**: Advanced Inference Engine
- [ ] **Phase 5**: Real-time Updates & WebSockets
- [ ] **Phase 6**: GraphQL Support
- [ ] **Phase 7**: ML-based Recommendations
- [ ] **Phase 8**: Mobile SDKs (iOS, Android)
- [ ] **Phase 9**: OpenAPI/Swagger Documentation
- [ ] **Phase 10**: Multi-region Deployment

## 🐛 Troubleshooting

Common issues and solutions:

**Services won't start:**
```bash
docker-compose logs
docker-compose ps
```

**Connection errors:**
```bash
curl http://localhost:8080/health
curl http://localhost:8080/admin/health/deep
```

**Performance issues:**
```bash
curl http://localhost:8080/metrics
curl http://localhost:8080/admin/cache/stats
```

See [Troubleshooting Guide](docs/TROUBLESHOOTING.md) for detailed solutions.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💬 Support

Need help?

- **Documentation**: [docs/](docs/)
- **Issues**: [GitHub Issues](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues)
- **Discussions**: [GitHub Discussions](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/discussions)

## 🙏 Acknowledgments

- Folonite for powering the API
- Indian EV ecosystem contributors
- Open-source community

## 📊 Stats

- **Vehicles**: 150+ Indian EV models
- **Chargers**: 500+ charging stations
- **OEMs**: 10+ manufacturers
- **Coverage**: Pan-India
- **API Response Time**: < 50ms (cached), < 200ms (uncached)
- **Uptime**: 99.9% target

---

**Built with ❤️ for the Indian EV ecosystem**

**Powered by Folonite 2026-01**
