# Deployment Guide

## Prerequisites

- Docker & Docker Compose
- Rust 1.75+ (for local development)
- 4GB+ RAM
- 10GB+ disk space

## Quick Start with Docker

1. **Clone the repository**
   ```bash
   git clone https://github.com/FFFSTANZA/EV-Knowledge-graph-API-.git
   cd EV-Knowledge-graph-API-
   ```

2. **Configure environment**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start all services**
   ```bash
   docker-compose up -d
   ```

4. **Seed the database**
   ```bash
   docker-compose exec api /app/ev-data seed
   ```

5. **Access the API**
   - API: http://localhost:8080
   - Neo4j Browser: http://localhost:7474
   - Documentation: http://localhost:8080/docs

## Local Development

1. **Start infrastructure services**
   ```bash
   docker-compose up -d neo4j redis
   ```

2. **Build the project**
   ```bash
   cargo build
   ```

3. **Run the API**
   ```bash
   cargo run --bin ev-api
   ```

4. **Seed data**
   ```bash
   cargo run --bin ev-data -- seed
   ```

## Production Deployment

### Environment Variables

Required environment variables for production:

```bash
ENVIRONMENT=production
SERVER_PORT=8080
NEO4J_URI=bolt://neo4j:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=<strong-password>
REDIS_URL=redis://redis:6379
CACHE_TTL_SECS=3600
RUST_LOG=info
```

### Security Considerations

1. **Change default passwords**
   - Neo4j: Change from `password123` to a strong password
   - Use environment-specific secrets

2. **Enable authentication**
   - Add API key authentication (TODO)
   - Configure CORS properly for production

3. **Enable HTTPS**
   - Use a reverse proxy (nginx, traefik)
   - Configure TLS certificates

4. **Rate limiting**
   - Implement rate limiting (TODO)
   - Use Redis for distributed rate limiting

### Kubernetes Deployment

See `k8s/` directory for Kubernetes manifests (TODO).

### Scaling

**Neo4j:**
- Use Neo4j Enterprise for clustering
- Configure read replicas for read-heavy workloads
- Tune heap and pagecache settings

**Redis:**
- Use Redis Cluster for high availability
- Configure persistence (AOF + RDB)
- Set up replication

**API:**
- Horizontal scaling: run multiple API instances
- Load balancer: nginx, traefik, or cloud LB
- Auto-scaling based on CPU/memory

### Monitoring

1. **Metrics**
   - Prometheus + Grafana (TODO)
   - Neo4j metrics
   - Redis metrics
   - Application metrics

2. **Logging**
   - Centralized logging (ELK, Loki)
   - Structured logging with tracing

3. **Health checks**
   - `/health` endpoint
   - Neo4j health check
   - Redis health check

### Backup & Recovery

**Neo4j:**
```bash
# Backup
docker exec ev-neo4j neo4j-admin database dump neo4j --to=/backups/neo4j-backup.dump

# Restore
docker exec ev-neo4j neo4j-admin database load neo4j --from=/backups/neo4j-backup.dump
```

**Redis:**
```bash
# Backup (RDB snapshot)
docker exec ev-redis redis-cli BGSAVE

# Copy backup
docker cp ev-redis:/data/dump.rdb ./backup/
```

## Performance Tuning

### Neo4j

```
NEO4J_dbms_memory_heap_initial__size=2G
NEO4J_dbms_memory_heap_max__size=4G
NEO4J_dbms_memory_pagecache_size=2G
```

### Redis

```
maxmemory 2gb
maxmemory-policy allkeys-lru
```

### API

- Adjust connection pool sizes
- Tune cache TTL based on usage
- Enable HTTP/2
- Use compression

## Troubleshooting

### Neo4j connection issues

```bash
# Check Neo4j logs
docker-compose logs neo4j

# Verify Neo4j is running
curl http://localhost:7474
```

### Redis connection issues

```bash
# Check Redis
docker-compose exec redis redis-cli ping

# View Redis logs
docker-compose logs redis
```

### API issues

```bash
# View API logs
docker-compose logs api

# Check health
curl http://localhost:8080/health
```

## Maintenance

### Update dependencies

```bash
cargo update
cargo outdated
```

### Database maintenance

```bash
# Rebuild indexes
MATCH (n) CALL db.index.fulltext.queryNodes('vehicle_search', '*') YIELD node RETURN count(node)

# Analyze query performance
PROFILE MATCH (v:Vehicle) RETURN v
```
