# Production Deployment Guide

**Version:** 0.1.0
**Last Updated:** 2024-11-18

This guide covers deploying the EV Knowledge Graph API to production environments, including Docker, cloud platforms, and Kubernetes.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start with Docker](#quick-start-with-docker)
- [Production Deployment](#production-deployment)
- [Environment Configuration](#environment-configuration)
- [Security Hardening](#security-hardening)
- [Monitoring & Observability](#monitoring--observability)
- [Scaling Strategies](#scaling-strategies)
- [Backup & Recovery](#backup--recovery)
- [Performance Tuning](#performance-tuning)
- [Cloud Platform Deployment](#cloud-platform-deployment)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Maintenance](#maintenance)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Infrastructure Requirements

- **Docker** 20.10+ and **Docker Compose** 2.0+
- **Rust** 1.75+ (for building from source)
- **RAM:** 4GB minimum, 8GB+ recommended for production
- **Disk:** 10GB+ free space (more for production data)
- **CPU:** 2+ cores (4+ cores for production)
- **OS:** Linux (Ubuntu 20.04+, RHEL 8+), macOS, or Windows with WSL2

### Production Infrastructure

- **Neo4j** 5.x (Community or Enterprise)
- **Redis** 7.x (Standalone or Cluster)
- **Load Balancer** (nginx, traefik, or cloud LB)
- **SSL/TLS Certificates** (Let's Encrypt, AWS ACM, etc.)
- **Monitoring Stack** (Prometheus, Grafana)

---

## Quick Start with Docker

Perfect for development and testing:

```bash
# 1. Clone the repository
git clone https://github.com/FFFSTANZA/EV-Knowledge-graph-API-.git
cd EV-Knowledge-graph-API-

# 2. Configure environment
cp .env.example .env
# Edit .env with your configuration

# 3. Start all services
docker-compose up -d

# 4. Wait for services to be ready (30-60 seconds)
docker-compose logs -f api

# 5. Seed the database
docker-compose exec api /app/ev-data seed

# 6. Access the API
curl http://localhost:8080/health
```

**Services:**
- **API:** http://localhost:8080
- **API Docs:** http://localhost:8080/docs
- **Neo4j Browser:** http://localhost:7474 (neo4j/password123)
- **Metrics:** http://localhost:8080/metrics
- **Admin Health:** http://localhost:8080/admin/health/deep

---

## Production Deployment

### Step 1: Build Production Image

```bash
# Build optimized production image
docker build -t ev-api:latest -f Dockerfile.prod .

# Or use multi-stage build
docker build --target production -t ev-api:latest .
```

**Dockerfile.prod:**
```dockerfile
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release --workspace

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/ev-api /usr/local/bin/
COPY --from=builder /app/target/release/ev-data /usr/local/bin/
EXPOSE 8080
CMD ["ev-api"]
```

### Step 2: Production Docker Compose

Create `docker-compose.prod.yml`:

```yaml
version: '3.8'

services:
  neo4j:
    image: neo4j:5.15-enterprise
    environment:
      NEO4J_AUTH: neo4j/${NEO4J_PASSWORD}
      NEO4J_ACCEPT_LICENSE_AGREEMENT: "yes"
      NEO4J_dbms_memory_heap_initial__size: 2G
      NEO4J_dbms_memory_heap_max__size: 4G
      NEO4J_dbms_memory_pagecache_size: 2G
      NEO4J_dbms_security_procedures_unrestricted: apoc.*
    volumes:
      - neo4j_data:/data
      - neo4j_logs:/logs
      - neo4j_backups:/backups
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "cypher-shell", "-u", "neo4j", "-p", "${NEO4J_PASSWORD}", "RETURN 1"]
      interval: 30s
      timeout: 10s
      retries: 5

  redis:
    image: redis:7-alpine
    command: redis-server --requirepass ${REDIS_PASSWORD} --maxmemory 2gb --maxmemory-policy allkeys-lru
    volumes:
      - redis_data:/data
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  api:
    image: ev-api:latest
    environment:
      ENVIRONMENT: production
      SERVER_PORT: 8080
      NEO4J_URI: bolt://neo4j:7687
      NEO4J_USER: neo4j
      NEO4J_PASSWORD: ${NEO4J_PASSWORD}
      REDIS_URL: redis://:${REDIS_PASSWORD}@redis:6379
      CACHE_TTL_SECS: 3600
      RUST_LOG: info,ev_api=debug
      RATE_LIMIT_PER_MINUTE: 60
      RATE_LIMIT_PER_HOUR: 1000
    ports:
      - "8080:8080"
    depends_on:
      neo4j:
        condition: service_healthy
      redis:
        condition: service_healthy
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  neo4j_data:
  neo4j_logs:
  neo4j_backups:
  redis_data:
```

### Step 3: Deploy

```bash
# Set environment variables
export NEO4J_PASSWORD="your-strong-password"
export REDIS_PASSWORD="your-strong-password"

# Start production stack
docker-compose -f docker-compose.prod.yml up -d

# Verify services
docker-compose -f docker-compose.prod.yml ps
docker-compose -f docker-compose.prod.yml logs -f api

# Seed production data
docker-compose -f docker-compose.prod.yml exec api ev-data seed

# Check system health
curl http://localhost:8080/admin/health/deep
```

---

## Environment Configuration

### Required Environment Variables

```bash
# Application Settings
ENVIRONMENT=production              # Environment name
SERVER_PORT=8080                    # API server port

# Neo4j Configuration
NEO4J_URI=bolt://neo4j:7687        # Neo4j connection URI
NEO4J_USER=neo4j                   # Neo4j username
NEO4J_PASSWORD=<strong-password>   # Neo4j password (CHANGE THIS!)

# Redis Configuration
REDIS_URL=redis://:password@redis:6379  # Redis connection URL
CACHE_TTL_SECS=3600                     # Cache TTL in seconds

# Rate Limiting (implemented and active)
RATE_LIMIT_PER_MINUTE=60           # Requests per minute per client
RATE_LIMIT_PER_HOUR=1000           # Requests per hour per client

# Logging
RUST_LOG=info,ev_api=debug         # Log level configuration

# Optional: API Authentication (framework ready)
# API_KEY_REQUIRED=true
# ADMIN_API_KEY=your-admin-key
```

### .env File Example

```bash
# Production .env
ENVIRONMENT=production
SERVER_PORT=8080

NEO4J_URI=bolt://neo4j-cluster.internal:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=super_secure_neo4j_password_2024!

REDIS_URL=redis://:super_secure_redis_password@redis-cluster.internal:6379
CACHE_TTL_SECS=3600

RATE_LIMIT_PER_MINUTE=60
RATE_LIMIT_PER_HOUR=1000

RUST_LOG=info,ev_api=debug,ev_graph=info

# Optional
# API_KEY_REQUIRED=true
# ADMIN_API_KEY=folonite-admin-key-2024
```

---

## Security Hardening

### 1. Change Default Passwords

**Critical:** Never use default passwords in production!

```bash
# Neo4j: Change password immediately after first login
docker-compose exec neo4j cypher-shell -u neo4j -p password123 \
  "ALTER CURRENT USER SET PASSWORD FROM 'password123' TO 'new-strong-password'"

# Redis: Configure requirepass in redis.conf
redis-server --requirepass your-strong-password
```

### 2. Enable HTTPS with Reverse Proxy

**nginx configuration:**

```nginx
upstream ev_api {
    least_conn;
    server api-1:8080;
    server api-2:8080;
    server api-3:8080;
}

server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    ssl_certificate /etc/ssl/certs/your-cert.pem;
    ssl_certificate_key /etc/ssl/private/your-key.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    location / {
        proxy_pass http://ev_api;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Request ID preservation
        proxy_set_header X-Request-ID $request_id;
    }

    location /health {
        access_log off;
        proxy_pass http://ev_api/health;
    }
}

# Redirect HTTP to HTTPS
server {
    listen 80;
    server_name api.yourdomain.com;
    return 301 https://$server_name$request_uri;
}
```

### 3. Rate Limiting (Implemented)

The API has Redis-based distributed rate limiting:

- **Per Minute:** 60 requests per client
- **Per Hour:** 1,000 requests per client
- **Response:** HTTP 429 with retry-after header

**Response headers:**
```
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 60
```

**Customize limits via environment:**
```bash
RATE_LIMIT_PER_MINUTE=100
RATE_LIMIT_PER_HOUR=5000
```

### 4. API Key Authentication (Framework Ready)

The authentication middleware is implemented and ready:

```bash
# Enable API key authentication
API_KEY_REQUIRED=true
ADMIN_API_KEY=your-secret-admin-key

# Client requests must include:
# X-API-Key: your-api-key
```

### 5. Network Security

```bash
# Firewall rules (UFW example)
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw deny 7687/tcp   # Neo4j (internal only)
sudo ufw deny 6379/tcp   # Redis (internal only)
sudo ufw enable
```

### 6. Secrets Management

Use Docker secrets or cloud secret managers:

```yaml
# Docker Swarm secrets
services:
  api:
    secrets:
      - neo4j_password
      - redis_password
    environment:
      NEO4J_PASSWORD_FILE: /run/secrets/neo4j_password
      REDIS_PASSWORD_FILE: /run/secrets/redis_password

secrets:
  neo4j_password:
    external: true
  redis_password:
    external: true
```

---

## Monitoring & Observability

### 1. Prometheus Metrics (Implemented)

**Metrics endpoint:** `http://localhost:8080/metrics`

**Available metrics:**
- HTTP request count and duration
- Rate limit violations
- Cache hit/miss ratios
- Database query latencies
- System health status

**Prometheus configuration:**

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'ev-api'
    static_configs:
      - targets: ['api:8080']
    metrics_path: '/metrics'
```

### 2. Health Checks (Implemented)

**Basic health check:**
```bash
curl http://localhost:8080/health
```

**Deep health check with component monitoring:**
```bash
curl http://localhost:8080/admin/health/deep
```

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
        "details": "bolt://neo4j:7687"
      },
      "redis": {
        "status": "up",
        "latency_ms": 2.3,
        "details": "redis://redis:6379"
      },
      "api": {
        "status": "up",
        "details": "port 8080"
      }
    },
    "version": "0.1.0",
    "uptime_seconds": 86400
  }
}
```

### 3. Structured Logging

Logs are structured with tracing:

```bash
# View logs
docker-compose logs -f api

# Example log output
2024-11-18T10:30:45.123Z INFO ev_api::middleware method=GET path=/api/v1/vehicles status=200 duration_ms=45 request_id=550e8400-e29b-41d4-a716-446655440000
```

**Configure log level:**
```bash
RUST_LOG=info,ev_api=debug,ev_graph=trace
```

### 4. Admin Endpoints (Implemented)

**Cache management:**
```bash
# Clear all cache
curl -X POST http://localhost:8080/admin/cache/clear

# Get cache statistics
curl http://localhost:8080/admin/cache/stats
```

**Graph database management:**
```bash
# Get graph statistics
curl http://localhost:8080/admin/graph/stats

# Rebuild indexes
curl -X POST http://localhost:8080/admin/graph/rebuild-indexes
```

### 5. Branding & Request Tracking

All responses include Folonite branding headers:

```
X-Powered-By: Folonite
X-Folonite-Version: 2026-01
X-Folonite-Request-ID: 550e8400-e29b-41d4-a716-446655440000
```

Use `X-Folonite-Request-ID` for distributed tracing and debugging.

---

## Scaling Strategies

### Horizontal Scaling (API Layer)

The API is stateless and can be scaled horizontally:

```yaml
# docker-compose.scale.yml
services:
  api:
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '1'
          memory: 1G
```

**Run multiple instances:**
```bash
docker-compose up -d --scale api=3
```

**Load balancer configuration** (see nginx example in Security section)

### Neo4j Scaling

**Read Replicas (Enterprise Edition):**
```cypher
// Configure read replicas
CALL dbms.cluster.overview()
```

**Connection pooling:**
```bash
NEO4J_MAX_CONNECTION_POOL_SIZE=100
NEO4J_CONNECTION_TIMEOUT=30
```

**Performance tuning:**
```bash
NEO4J_dbms_memory_heap_initial__size=2G
NEO4J_dbms_memory_heap_max__size=4G
NEO4J_dbms_memory_pagecache_size=2G
NEO4J_dbms_query_cache_size=100
```

### Redis Scaling

**Redis Cluster Mode:**
```bash
# Deploy Redis cluster with 3 masters and 3 replicas
docker-compose -f docker-compose.redis-cluster.yml up -d
```

**Persistence configuration:**
```bash
# RDB + AOF for durability
save 900 1
save 300 10
save 60 10000
appendonly yes
appendfsync everysec
```

### Auto-scaling

**Kubernetes HPA example:**
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: ev-api-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: ev-api
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

---

## Backup & Recovery

### Neo4j Backup

**Online backup (Enterprise):**
```bash
# Full backup
neo4j-admin database backup neo4j \
  --to-path=/backups/$(date +%Y%m%d_%H%M%S)

# Incremental backup
neo4j-admin database backup neo4j \
  --to-path=/backups/incremental \
  --incremental
```

**Community Edition backup:**
```bash
# Stop database
docker-compose stop api

# Dump database
docker exec ev-neo4j neo4j-admin database dump neo4j \
  --to=/backups/neo4j-$(date +%Y%m%d_%H%M%S).dump

# Restart
docker-compose start api
```

**Restore:**
```bash
# Stop database
docker-compose stop neo4j api

# Restore from dump
docker exec ev-neo4j neo4j-admin database load neo4j \
  --from=/backups/neo4j-20241118_120000.dump \
  --overwrite-destination=true

# Start services
docker-compose start neo4j api
```

### Redis Backup

**RDB snapshot:**
```bash
# Trigger background save
docker exec ev-redis redis-cli BGSAVE

# Copy snapshot
docker cp ev-redis:/data/dump.rdb ./backups/redis-$(date +%Y%m%d_%H%M%S).rdb
```

**AOF backup:**
```bash
# Trigger AOF rewrite
docker exec ev-redis redis-cli BGREWRITEAOF

# Copy AOF file
docker cp ev-redis:/data/appendonly.aof ./backups/
```

**Restore:**
```bash
# Copy backup to Redis data directory
docker cp ./backups/dump.rdb ev-redis:/data/dump.rdb

# Restart Redis
docker-compose restart redis
```

### Automated Backup Script

```bash
#!/bin/bash
# backup.sh - Automated backup script

BACKUP_DIR="/backups/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Neo4j backup
docker exec ev-neo4j neo4j-admin database dump neo4j \
  --to="$BACKUP_DIR/neo4j.dump"

# Redis backup
docker exec ev-redis redis-cli BGSAVE
sleep 10  # Wait for BGSAVE to complete
docker cp ev-redis:/data/dump.rdb "$BACKUP_DIR/redis.rdb"

# Compress
tar -czf "$BACKUP_DIR.tar.gz" "$BACKUP_DIR"

# Upload to S3 (optional)
aws s3 cp "$BACKUP_DIR.tar.gz" s3://your-bucket/backups/

# Cleanup old backups (keep last 7 days)
find /backups -name "*.tar.gz" -mtime +7 -delete

echo "Backup completed: $BACKUP_DIR.tar.gz"
```

**Cron job:**
```bash
# Run daily at 2 AM
0 2 * * * /path/to/backup.sh >> /var/log/ev-api-backup.log 2>&1
```

---

## Performance Tuning

### Application Tuning

**Connection pools:**
```rust
// Configure in config.rs
pub struct Config {
    pub neo4j_max_connections: u32,  // Default: 100
    pub redis_max_connections: u32,   // Default: 50
    pub cache_ttl_secs: u64,          // Default: 3600
}
```

**Cache optimization:**
```bash
# Adjust cache TTL based on data volatility
CACHE_TTL_SECS=7200  # 2 hours for hot data
QUERY_CACHE_TTL_SECS=300  # 5 minutes for query results
```

### Neo4j Performance

**Heap and pagecache:**
```bash
# For 16GB RAM server
NEO4J_dbms_memory_heap_initial__size=4G
NEO4J_dbms_memory_heap_max__size=4G
NEO4J_dbms_memory_pagecache_size=8G
```

**Query tuning:**
```cypher
// Use EXPLAIN to analyze queries
EXPLAIN MATCH (v:Vehicle)-[:COMPATIBLE_WITH]->(c:Charger)
WHERE v.id = $vehicle_id
RETURN c

// Use PROFILE for detailed metrics
PROFILE MATCH (v:Vehicle)-[:COMPATIBLE_WITH]->(c:Charger)
WHERE v.id = $vehicle_id
RETURN c
```

**Indexes:**
```cypher
// Ensure all indexes exist (auto-created on startup)
SHOW INDEXES
```

### Redis Performance

**Memory management:**
```bash
maxmemory 2gb
maxmemory-policy allkeys-lru  # Evict least recently used
```

**Persistence vs Performance:**
```bash
# Fast writes, less durability
appendfsync no

# Balanced
appendfsync everysec

# Slow writes, maximum durability
appendfsync always
```

### OS-level Tuning

**TCP settings:**
```bash
# /etc/sysctl.conf
net.core.somaxconn = 65535
net.ipv4.tcp_max_syn_backlog = 8192
net.ipv4.tcp_tw_reuse = 1
```

**File descriptors:**
```bash
# /etc/security/limits.conf
* soft nofile 65535
* hard nofile 65535
```

---

## Cloud Platform Deployment

### AWS Deployment

**Architecture:**
```
┌─────────────────────────────────────────┐
│  Route 53 → CloudFront → ALB            │
└─────────────────┬───────────────────────┘
                  │
      ┌───────────┴───────────┐
      │                       │
┌─────▼──────┐       ┌────────▼─────┐
│  ECS/EKS   │       │  ECS/EKS     │
│  (API)     │       │  (API)       │
└─────┬──────┘       └────────┬─────┘
      │                       │
      └───────────┬───────────┘
                  │
    ┌─────────────┴─────────────┐
    │                           │
┌───▼──────┐         ┌──────────▼───┐
│  Neo4j   │         │  ElastiCache │
│  (EC2)   │         │  (Redis)     │
└──────────┘         └──────────────┘
```

**ECS Task Definition:**
```json
{
  "family": "ev-api",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "1024",
  "memory": "2048",
  "containerDefinitions": [
    {
      "name": "ev-api",
      "image": "your-ecr-repo/ev-api:latest",
      "portMappings": [{"containerPort": 8080}],
      "environment": [
        {"name": "ENVIRONMENT", "value": "production"},
        {"name": "NEO4J_URI", "value": "bolt://neo4j.internal:7687"}
      ],
      "secrets": [
        {"name": "NEO4J_PASSWORD", "valueFrom": "arn:aws:secretsmanager:..."}
      ],
      "healthCheck": {
        "command": ["CMD-SHELL", "curl -f http://localhost:8080/health || exit 1"],
        "interval": 30,
        "timeout": 5,
        "retries": 3
      }
    }
  ]
}
```

### Google Cloud Deployment

**Cloud Run:**
```bash
# Build and push
gcloud builds submit --tag gcr.io/PROJECT_ID/ev-api

# Deploy
gcloud run deploy ev-api \
  --image gcr.io/PROJECT_ID/ev-api \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars NEO4J_URI=bolt://neo4j:7687 \
  --set-secrets NEO4J_PASSWORD=neo4j-password:latest
```

### Azure Deployment

**Azure Container Instances:**
```bash
az container create \
  --resource-group ev-api-rg \
  --name ev-api \
  --image your-acr.azurecr.io/ev-api:latest \
  --cpu 2 \
  --memory 4 \
  --ports 8080 \
  --environment-variables \
    ENVIRONMENT=production \
    NEO4J_URI=bolt://neo4j:7687 \
  --secure-environment-variables \
    NEO4J_PASSWORD=your-password
```

---

## Kubernetes Deployment

### Deployment Manifest

**api-deployment.yaml:**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ev-api
  namespace: ev-system
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ev-api
  template:
    metadata:
      labels:
        app: ev-api
        version: v1
    spec:
      containers:
      - name: ev-api
        image: your-registry/ev-api:latest
        ports:
        - containerPort: 8080
          name: http
        env:
        - name: ENVIRONMENT
          value: "production"
        - name: NEO4J_URI
          value: "bolt://neo4j-service:7687"
        - name: NEO4J_PASSWORD
          valueFrom:
            secretKeyRef:
              name: neo4j-credentials
              key: password
        - name: REDIS_URL
          value: "redis://redis-service:6379"
        resources:
          requests:
            cpu: "500m"
            memory: "1Gi"
          limits:
            cpu: "2"
            memory: "2Gi"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
```

**Service:**
```yaml
apiVersion: v1
kind: Service
metadata:
  name: ev-api-service
  namespace: ev-system
spec:
  selector:
    app: ev-api
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

**Ingress with TLS:**
```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: ev-api-ingress
  namespace: ev-system
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
spec:
  tls:
  - hosts:
    - api.yourdomain.com
    secretName: ev-api-tls
  rules:
  - host: api.yourdomain.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: ev-api-service
            port:
              number: 80
```

---

## Maintenance

### Update Dependencies

```bash
# Check for updates
cargo outdated

# Update all dependencies
cargo update

# Update specific dependency
cargo update -p axum

# Rebuild
cargo build --release

# Test
cargo test --workspace
```

### Database Maintenance

**Neo4j index maintenance:**
```cypher
// Check index status
SHOW INDEXES

// Rebuild specific index
DROP INDEX vehicle_search IF EXISTS;
CREATE FULLTEXT INDEX vehicle_search
  FOR (v:Vehicle)
  ON EACH [v.name, v.model_code];
```

**Query performance analysis:**
```cypher
// Find slow queries
CALL dbms.listQueries()
YIELD query, elapsedTimeMillis
WHERE elapsedTimeMillis > 1000
RETURN query, elapsedTimeMillis;

// Kill slow query
CALL dbms.killQuery('query-id');
```

**Database cleanup:**
```cypher
// Remove orphaned nodes
MATCH (n)
WHERE NOT (n)--()
DELETE n;

// Update statistics
CALL db.stats.retrieve('GRAPH COUNTS');
```

### Redis Maintenance

```bash
# Memory analysis
docker exec ev-redis redis-cli INFO memory

# Key analysis
docker exec ev-redis redis-cli --bigkeys

# Flush expired keys
docker exec ev-redis redis-cli --scan --pattern "ev:*" | \
  xargs docker exec -i ev-redis redis-cli DEL
```

### Log Rotation

```bash
# Configure Docker log rotation
# /etc/docker/daemon.json
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  }
}
```

---

## Troubleshooting

### Common Issues

**1. Neo4j Connection Failed**
```bash
# Check Neo4j status
docker-compose logs neo4j

# Verify Neo4j is listening
docker exec ev-neo4j cypher-shell -u neo4j -p password123 "RETURN 1"

# Check network connectivity
docker-compose exec api nc -zv neo4j 7687
```

**2. Redis Connection Failed**
```bash
# Check Redis status
docker-compose logs redis

# Test Redis connection
docker exec ev-redis redis-cli ping

# Check from API container
docker-compose exec api nc -zv redis 6379
```

**3. API Not Responding**
```bash
# Check API logs
docker-compose logs api

# Check health endpoint
curl -v http://localhost:8080/health

# Check if port is bound
netstat -tuln | grep 8080
```

**4. High Memory Usage**
```bash
# Check container stats
docker stats ev-api ev-neo4j ev-redis

# Reduce Neo4j heap
NEO4J_dbms_memory_heap_max__size=2G

# Reduce Redis maxmemory
maxmemory 1gb
```

**5. Slow Queries**
```bash
# Check API metrics
curl http://localhost:8080/metrics | grep duration

# Profile Neo4j query
PROFILE MATCH (v:Vehicle) RETURN v LIMIT 10

# Check Redis latency
docker exec ev-redis redis-cli --latency
```

For more detailed troubleshooting, see [TROUBLESHOOTING.md](TROUBLESHOOTING.md)

---

## Production Checklist

Before going live, ensure:

- [ ] **Security**
  - [ ] Changed all default passwords
  - [ ] Configured HTTPS with valid certificates
  - [ ] Enabled rate limiting
  - [ ] Configured firewall rules
  - [ ] Set up API key authentication (if required)
  - [ ] Reviewed and hardened CORS settings

- [ ] **Monitoring**
  - [ ] Prometheus scraping configured
  - [ ] Grafana dashboards created
  - [ ] Health check endpoints verified
  - [ ] Log aggregation configured
  - [ ] Alerting rules defined

- [ ] **Backup & Recovery**
  - [ ] Automated backups scheduled
  - [ ] Backup restoration tested
  - [ ] Disaster recovery plan documented
  - [ ] RTO/RPO objectives defined

- [ ] **Performance**
  - [ ] Load testing completed
  - [ ] Cache hit ratios optimized
  - [ ] Database indexes verified
  - [ ] Connection pools tuned
  - [ ] Resource limits configured

- [ ] **High Availability**
  - [ ] Multiple API instances running
  - [ ] Load balancer configured
  - [ ] Database replication enabled
  - [ ] Redis persistence configured
  - [ ] Auto-scaling policies defined

- [ ] **Documentation**
  - [ ] API documentation published
  - [ ] Runbooks created
  - [ ] On-call procedures documented
  - [ ] Architecture diagrams updated

---

## Support

For deployment assistance:

- **Documentation:** [docs/](.)
- **Issues:** [GitHub Issues](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues)
- **Discussions:** [GitHub Discussions](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/discussions)

---

**Last Updated:** 2024-11-18
**Version:** 0.1.0
**Powered by:** Folonite 2026-01
