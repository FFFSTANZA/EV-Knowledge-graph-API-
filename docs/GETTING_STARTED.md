# Getting Started with EV Knowledge Graph API

This guide will help you get the EV Knowledge Graph API up and running on your local machine.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Installation Methods](#installation-methods)
  - [Docker (Recommended)](#docker-recommended)
  - [Local Development](#local-development)
- [Verification](#verification)
- [Seeding Data](#seeding-data)
- [Running Tests](#running-tests)
- [Next Steps](#next-steps)

---

## Prerequisites

### Required

- **Docker** 20.10+ and **Docker Compose** 2.0+
  - Install: https://docs.docker.com/get-docker/
- **Git**
  - Install: https://git-scm.com/downloads

### For Local Development (Optional)

- **Rust** 1.75+
  - Install via rustup: https://rustup.rs/
- **Neo4j** 5.x
- **Redis** 7.x

### System Requirements

- **RAM:** 4GB minimum, 8GB recommended
- **Disk:** 10GB free space
- **OS:** Linux, macOS, or Windows (with WSL2)

---

## Quick Start

The fastest way to get started is using Docker Compose:

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
```

That's it! The API is now running at http://localhost:8080

---

## Installation Methods

### Docker (Recommended)

Docker Compose handles all dependencies automatically.

#### Step 1: Start Services

```bash
# Start all services in detached mode
docker-compose up -d

# View logs
docker-compose logs -f

# View logs for specific service
docker-compose logs -f api
docker-compose logs -f neo4j
docker-compose logs -f redis
```

#### Step 2: Check Status

```bash
# Check running containers
docker-compose ps

# Expected output:
# NAME       SERVICE   STATUS    PORTS
# ev-api     api       Up        0.0.0.0:8080->8080/tcp
# ev-neo4j   neo4j     Up        0.0.0.0:7474->7474/tcp, 0.0.0.0:7687->7687/tcp
# ev-redis   redis     Up        0.0.0.0:6379->6379/tcp
```

#### Step 3: Access Services

- **API:** http://localhost:8080
- **API Docs:** http://localhost:8080/docs
- **Neo4j Browser:** http://localhost:7474 (username: `neo4j`, password: `password123`)
- **Metrics:** http://localhost:8080/metrics

#### Useful Docker Commands

```bash
# Stop services
docker-compose stop

# Stop and remove containers
docker-compose down

# Stop and remove containers + volumes (clean slate)
docker-compose down -v

# Rebuild images
docker-compose build

# View resource usage
docker stats ev-api ev-neo4j ev-redis
```

---

### Local Development

For active development, run services locally:

#### Step 1: Install Dependencies

**On macOS:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Neo4j
brew install neo4j

# Install Redis
brew install redis
```

**On Ubuntu/Debian:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Neo4j
sudo apt install default-jre
wget -O - https://debian.neo4j.com/neotechnology.gpg.key | sudo apt-key add -
echo 'deb https://debian.neo4j.com stable latest' | sudo tee /etc/apt/sources.list.d/neo4j.list
sudo apt update && sudo apt install neo4j

# Install Redis
sudo apt install redis-server
```

#### Step 2: Start Infrastructure Services

**Option A: Use Docker for infrastructure only**
```bash
docker-compose up -d neo4j redis
```

**Option B: Start services locally**
```bash
# Start Neo4j
neo4j start

# Start Redis
redis-server
```

#### Step 3: Configure Environment

```bash
# Copy environment template
cp .env.example .env

# Edit .env with your settings
nano .env
```

**`.env` file:**
```env
ENVIRONMENT=development
SERVER_PORT=8080

NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=password123

REDIS_URL=redis://localhost:6379

CACHE_TTL_SECS=3600
RUST_LOG=info,ev_api=debug
```

#### Step 4: Build and Run

```bash
# Build the project
cargo build --release

# Initialize database schema
cargo run --bin ev-data -- migrate

# Seed with Indian EV data
cargo run --bin ev-data -- seed

# Run the API server
cargo run --release --bin ev-api
```

#### Alternative: Use Makefile

```bash
# Start development environment
make dev

# This does:
# 1. Starts Neo4j and Redis with Docker
# 2. Builds the project
# 3. Runs the API server
```

---

## Verification

### Health Check

```bash
# Basic health check
curl http://localhost:8080/health

# Expected response:
# {
#   "success": true,
#   "data": {
#     "status": "healthy",
#     "environment": "development",
#     "neo4j": "healthy",
#     "redis": "healthy"
#   }
# }
```

### Deep Health Check

```bash
curl http://localhost:8080/admin/health/deep

# Shows detailed component status with latencies
```

### Stats Check

```bash
curl http://localhost:8080/stats

# Shows database statistics
```

### API Documentation

Open in browser: http://localhost:8080/docs

---

## Seeding Data

The API comes with seed data for Indian EVs, chargers, and policies.

### Using Docker

```bash
# Seed data
docker-compose exec api /app/ev-data seed

# Or rebuild and seed
docker-compose down -v
docker-compose up -d
docker-compose exec api /app/ev-data seed
```

### Using Local Build

```bash
# Seed all data
cargo run --bin ev-data -- seed

# Seed specific data
cargo run --bin ev-data -- seed --env development
```

### Verify Seeded Data

```bash
# Check database stats
curl http://localhost:8080/stats

# Search for a vehicle
curl "http://localhost:8080/api/v1/vehicles/search?q=tata"

# Check connector standards
curl "http://localhost:8080/api/v1/query/indian-oems"
```

---

## Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace -- --nocapture

# Run specific test
cargo test --package ev-core test_name

# Run tests for a specific crate
cargo test -p ev-api
cargo test -p ev-graph
cargo test -p ev-cache
```

### Integration Tests

```bash
# Start services
docker-compose up -d

# Wait for readiness
sleep 10

# Run integration tests
cargo test --test integration_tests
```

---

## Next Steps

### Explore the API

1. **View Documentation**
   - Browser: http://localhost:8080/docs
   - Markdown: [docs/API.md](API.md)

2. **Try Example Queries**
   ```bash
   # Search vehicles
   curl "http://localhost:8080/api/v1/vehicles/search?q=nexon"

   # Find nearby chargers
   curl "http://localhost:8080/api/v1/chargers/nearby?lat=19.0760&lon=72.8777&radius=10"

   # Check compatibility
   curl -X POST http://localhost:8080/api/v1/compatibility/check \
     -H "Content-Type: application/json" \
     -d '{"vehicle_id": "uuid", "charger_id": "uuid"}'
   ```

3. **Explore Neo4j**
   - Open Neo4j Browser: http://localhost:7474
   - Username: `neo4j`
   - Password: `password123`
   - Try: `MATCH (v:Vehicle) RETURN v LIMIT 10`

### Development Workflow

1. **Make Changes**
   ```bash
   # Edit code in crates/
   nano crates/ev-api/src/handlers/vehicles.rs
   ```

2. **Test Changes**
   ```bash
   cargo check
   cargo test
   ```

3. **Run Locally**
   ```bash
   cargo run --bin ev-api
   ```

4. **Build for Production**
   ```bash
   cargo build --release
   ```

### Learn More

- [Architecture Documentation](ARCHITECTURE.md)
- [API Reference](API.md)
- [Deployment Guide](DEPLOYMENT.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Troubleshooting](TROUBLESHOOTING.md)

---

## Troubleshooting

### Services Won't Start

```bash
# Check ports are available
lsof -i :8080  # API
lsof -i :7687  # Neo4j
lsof -i :6379  # Redis

# Kill conflicting processes
kill -9 <PID>

# Or use different ports in docker-compose.yml
```

### Connection Errors

```bash
# Test Neo4j connection
docker-compose exec api nc -zv neo4j 7687

# Test Redis connection
docker-compose exec api nc -zv redis 6379

# Check logs
docker-compose logs neo4j
docker-compose logs redis
```

### Build Errors

```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update

# Check Rust version
rustc --version  # Should be 1.75+
```

For more troubleshooting tips, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md)

---

## Quick Reference

```bash
# Common Commands
make help              # Show all make targets
make build             # Build the project
make run               # Run the API
make test              # Run tests
make docker-up         # Start Docker services
make docker-down       # Stop Docker services
make seed              # Seed database

# Docker Commands
docker-compose up -d   # Start services
docker-compose ps      # Check status
docker-compose logs -f # View logs
docker-compose down    # Stop services

# Cargo Commands
cargo build            # Build debug
cargo build --release  # Build release
cargo test             # Run tests
cargo run --bin ev-api # Run API
cargo check            # Quick check
```

---

## Support

Need help? Check these resources:

- **Documentation:** [docs/](.)
- **Examples:** [docs/EXAMPLES.md](EXAMPLES.md)
- **Troubleshooting:** [docs/TROUBLESHOOTING.md](TROUBLESHOOTING.md)
- **Issues:** [GitHub Issues](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues)
- **Discussions:** [GitHub Discussions](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/discussions)

Happy coding! 🚀
