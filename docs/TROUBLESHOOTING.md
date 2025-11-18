# Troubleshooting Guide

**Version:** 0.1.0
**Last Updated:** 2024-11-18

This guide helps you diagnose and resolve common issues with the EV Knowledge Graph API.

## Table of Contents

- [Quick Diagnostics](#quick-diagnostics)
- [Installation Issues](#installation-issues)
- [Connection Problems](#connection-problems)
- [Performance Issues](#performance-issues)
- [API Errors](#api-errors)
- [Database Issues](#database-issues)
- [Cache Issues](#cache-issues)
- [Docker Issues](#docker-issues)
- [Build Errors](#build-errors)
- [Runtime Errors](#runtime-errors)
- [Data Seeding Issues](#data-seeding-issues)
- [Network Issues](#network-issues)
- [Getting Help](#getting-help)

---

## Quick Diagnostics

Start with these commands to quickly assess system health:

```bash
# 1. Check if all services are running
docker-compose ps

# 2. Check API health
curl http://localhost:8080/health

# 3. Check detailed component health
curl http://localhost:8080/admin/health/deep

# 4. View recent logs
docker-compose logs --tail=50 api
docker-compose logs --tail=50 neo4j
docker-compose logs --tail=50 redis

# 5. Check resource usage
docker stats ev-api ev-neo4j ev-redis

# 6. Check database stats
curl http://localhost:8080/stats
```

**Interpreting results:**
- All services should show "Up" status
- Health endpoints should return `"status": "healthy"`
- Logs should not show repeated errors
- Memory/CPU usage should be reasonable

---

## Installation Issues

### Issue: Docker Compose fails to start

**Symptoms:**
```
ERROR: Couldn't connect to Docker daemon
```

**Solutions:**

1. **Check Docker is running:**
```bash
docker info
```

2. **Start Docker service:**
```bash
# Linux
sudo systemctl start docker

# macOS
open -a Docker

# Windows
# Start Docker Desktop
```

3. **Check Docker Compose version:**
```bash
docker-compose --version
# Should be 2.0+
```

4. **Reinstall Docker Compose:**
```bash
# Linux
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### Issue: Ports already in use

**Symptoms:**
```
ERROR: for neo4j  Cannot start service neo4j: driver failed programming external connectivity on endpoint ev-neo4j: Bind for 0.0.0.0:7687 failed: port is already allocated
```

**Solutions:**

1. **Check which process is using the port:**
```bash
# Linux/macOS
lsof -i :7687
lsof -i :7474
lsof -i :6379
lsof -i :8080

# Windows
netstat -ano | findstr :7687
```

2. **Kill the conflicting process:**
```bash
# Linux/macOS
kill -9 <PID>

# Windows
taskkill /PID <PID> /F
```

3. **Change ports in docker-compose.yml:**
```yaml
services:
  neo4j:
    ports:
      - "17474:7474"  # Changed from 7474
      - "17687:7687"  # Changed from 7687
```

4. **Update your .env file:**
```bash
NEO4J_URI=bolt://localhost:17687
```

### Issue: Permission denied errors

**Symptoms:**
```
Permission denied while trying to connect to the Docker daemon socket
```

**Solutions:**

1. **Add user to docker group (Linux):**
```bash
sudo usermod -aG docker $USER
newgrp docker
```

2. **Run with sudo (temporary):**
```bash
sudo docker-compose up -d
```

3. **Fix Docker socket permissions:**
```bash
sudo chmod 666 /var/run/docker.sock
```

---

## Connection Problems

### Issue: Cannot connect to Neo4j

**Symptoms:**
```
Failed to establish connection to bolt://neo4j:7687
Connection refused
```

**Diagnostics:**

1. **Check Neo4j is running:**
```bash
docker-compose ps neo4j
# Should show "Up"
```

2. **Check Neo4j logs:**
```bash
docker-compose logs neo4j
# Look for startup errors
```

3. **Test Neo4j connection:**
```bash
# From host
docker exec ev-neo4j cypher-shell -u neo4j -p password123 "RETURN 1"

# From API container
docker-compose exec api nc -zv neo4j 7687
```

**Solutions:**

1. **Wait for Neo4j to fully start:**
```bash
# Neo4j can take 30-60 seconds to start
docker-compose logs -f neo4j
# Wait for "Bolt enabled on localhost:7687"
```

2. **Restart Neo4j:**
```bash
docker-compose restart neo4j
```

3. **Check credentials:**
```bash
# Verify .env file
cat .env | grep NEO4J
# Should match docker-compose.yml
```

4. **Check network connectivity:**
```bash
docker network ls
docker network inspect ev-knowledge-graph-api-_default
```

5. **Rebuild and restart:**
```bash
docker-compose down
docker-compose up -d
```

### Issue: Cannot connect to Redis

**Symptoms:**
```
Failed to connect to Redis
Connection refused (os error 111)
```

**Diagnostics:**

1. **Check Redis is running:**
```bash
docker-compose ps redis
```

2. **Test Redis connection:**
```bash
# From host
docker exec ev-redis redis-cli ping
# Should return "PONG"

# From API container
docker-compose exec api nc -zv redis 6379
```

3. **Check Redis logs:**
```bash
docker-compose logs redis
```

**Solutions:**

1. **Restart Redis:**
```bash
docker-compose restart redis
```

2. **Check Redis is accepting connections:**
```bash
docker exec ev-redis redis-cli INFO server
```

3. **Verify Redis URL:**
```bash
# In .env
REDIS_URL=redis://localhost:6379

# Or with password
REDIS_URL=redis://:password@localhost:6379
```

4. **Test from API container:**
```bash
docker-compose exec api sh
# Inside container:
nc -zv redis 6379
```

### Issue: API not responding

**Symptoms:**
- `curl http://localhost:8080/health` times out or connection refused

**Diagnostics:**

1. **Check API container status:**
```bash
docker-compose ps api
docker-compose logs api
```

2. **Check if API is listening:**
```bash
# From host
netstat -tuln | grep 8080

# Inside container
docker-compose exec api netstat -tuln | grep 8080
```

3. **Check API logs for errors:**
```bash
docker-compose logs --tail=100 api
```

**Solutions:**

1. **Check dependencies are ready:**
```bash
# Ensure Neo4j and Redis are healthy first
curl http://localhost:8080/admin/health/deep
```

2. **Restart API:**
```bash
docker-compose restart api
```

3. **Check environment variables:**
```bash
docker-compose exec api env | grep -E '(NEO4J|REDIS|SERVER)'
```

4. **Rebuild API container:**
```bash
docker-compose up -d --build api
```

---

## Performance Issues

### Issue: Slow API responses

**Symptoms:**
- Requests take > 1 second
- Timeout errors

**Diagnostics:**

1. **Check metrics:**
```bash
curl http://localhost:8080/metrics | grep duration
```

2. **Check resource usage:**
```bash
docker stats ev-api ev-neo4j ev-redis
```

3. **Check cache hit rate:**
```bash
curl http://localhost:8080/admin/cache/stats
```

4. **Profile Neo4j queries:**
```bash
docker exec ev-neo4j cypher-shell -u neo4j -p password123
# In cypher-shell:
PROFILE MATCH (v:Vehicle) RETURN v LIMIT 10;
```

**Solutions:**

1. **Check if cache is working:**
```bash
# First request (should be slower)
time curl http://localhost:8080/api/v1/vehicles

# Second request (should be faster)
time curl http://localhost:8080/api/v1/vehicles
```

2. **Increase cache TTL:**
```bash
# In .env
CACHE_TTL_SECS=7200  # 2 hours
```

3. **Optimize Neo4j memory:**
```yaml
# In docker-compose.yml
environment:
  NEO4J_dbms_memory_heap_max__size: 4G
  NEO4J_dbms_memory_pagecache_size: 2G
```

4. **Check for missing indexes:**
```cypher
SHOW INDEXES
// Ensure indexes exist for frequently queried fields
```

5. **Use batch operations:**
```bash
# Instead of multiple single requests, use batch endpoint
curl -X POST http://localhost:8080/api/v1/batch/compatibility \
  -H "Content-Type: application/json" \
  -d '{"items": [...]}'
```

### Issue: High memory usage

**Symptoms:**
- Container keeps restarting
- Out of memory errors

**Diagnostics:**

```bash
# Check memory usage
docker stats --no-stream

# Check Neo4j memory
docker exec ev-neo4j sh -c "free -m"

# Check Redis memory
docker exec ev-redis redis-cli INFO memory
```

**Solutions:**

1. **Reduce Neo4j heap size:**
```yaml
environment:
  NEO4J_dbms_memory_heap_initial__size: 1G
  NEO4J_dbms_memory_heap_max__size: 2G
  NEO4J_dbms_memory_pagecache_size: 1G
```

2. **Set Redis maxmemory:**
```bash
docker exec ev-redis redis-cli CONFIG SET maxmemory 1gb
docker exec ev-redis redis-cli CONFIG SET maxmemory-policy allkeys-lru
```

3. **Limit Docker container memory:**
```yaml
services:
  api:
    mem_limit: 2g
  neo4j:
    mem_limit: 4g
  redis:
    mem_limit: 1g
```

4. **Clear caches:**
```bash
curl -X POST http://localhost:8080/admin/cache/clear
```

### Issue: High CPU usage

**Diagnostics:**

```bash
# Check CPU usage per container
docker stats --no-stream

# Check running queries in Neo4j
docker exec ev-neo4j cypher-shell -u neo4j -p password123
CALL dbms.listQueries();
```

**Solutions:**

1. **Kill slow queries:**
```cypher
CALL dbms.listQueries()
YIELD queryId, query, elapsedTimeMillis
WHERE elapsedTimeMillis > 10000
RETURN queryId, query;

// Kill specific query
CALL dbms.killQuery('query-123');
```

2. **Add query timeout:**
```bash
NEO4J_db_transaction_timeout=30s
```

3. **Optimize queries:**
```cypher
// Use EXPLAIN to check query plan
EXPLAIN MATCH (v:Vehicle)-[:COMPATIBLE_WITH]->(c:Charger)
WHERE v.id = $vehicle_id
RETURN c;
```

---

## API Errors

### Error: Rate limit exceeded (429)

**Response:**
```json
{
  "success": false,
  "error": "Rate limit exceeded",
  "error_code": "RATE_LIMIT_EXCEEDED"
}
```

**Solutions:**

1. **Wait for rate limit reset:**
```bash
# Check headers
curl -I http://localhost:8080/api/v1/vehicles
# Look for X-RateLimit-Reset
```

2. **Increase rate limits:**
```bash
# In .env
RATE_LIMIT_PER_MINUTE=100
RATE_LIMIT_PER_HOUR=5000
```

3. **Use batch operations:**
```bash
# Instead of 10 individual requests, batch them
curl -X POST http://localhost:8080/api/v1/batch/compatibility
```

4. **Implement client-side rate limiting:**
```python
import time
import requests

for i in range(100):
    response = requests.get('http://localhost:8080/api/v1/vehicles')
    if response.status_code == 429:
        retry_after = int(response.headers.get('X-RateLimit-Reset', 60))
        time.sleep(retry_after)
```

### Error: Not found (404)

**Response:**
```json
{
  "success": false,
  "error": "Vehicle not found",
  "error_code": "NOT_FOUND"
}
```

**Solutions:**

1. **Verify the resource exists:**
```bash
# List all vehicles
curl http://localhost:8080/api/v1/vehicles

# Search for the resource
curl "http://localhost:8080/api/v1/vehicles/search?q=nexon"
```

2. **Check if database is seeded:**
```bash
curl http://localhost:8080/stats
# Should show node counts
```

3. **Seed the database:**
```bash
docker-compose exec api ev-data seed
```

### Error: Internal server error (500)

**Response:**
```json
{
  "success": false,
  "error": "Internal server error",
  "error_code": "INTERNAL_ERROR"
}
```

**Diagnostics:**

1. **Check API logs:**
```bash
docker-compose logs --tail=50 api
```

2. **Check request ID:**
```bash
# Use X-Folonite-Request-ID from response headers
docker-compose logs api | grep "550e8400-e29b-41d4-a716-446655440000"
```

3. **Check component health:**
```bash
curl http://localhost:8080/admin/health/deep
```

**Solutions:**

1. **Restart affected service:**
```bash
docker-compose restart api
```

2. **Check database connection:**
```bash
docker exec ev-neo4j cypher-shell -u neo4j -p password123 "RETURN 1"
```

3. **Check Redis:**
```bash
docker exec ev-redis redis-cli ping
```

---

## Database Issues

### Issue: Neo4j won't start

**Symptoms:**
```
Neo4j container keeps restarting
Neo4j logs show "Failed to start database"
```

**Diagnostics:**

```bash
# Check Neo4j logs
docker-compose logs neo4j

# Check disk space
df -h

# Check Neo4j data directory
docker exec ev-neo4j ls -lh /data
```

**Solutions:**

1. **Check disk space:**
```bash
# Need at least 1GB free
df -h
```

2. **Remove corrupted data:**
```bash
docker-compose down
docker volume rm ev-knowledge-graph-api-_neo4j_data
docker-compose up -d neo4j
```

3. **Restore from backup:**
```bash
# See DEPLOYMENT.md for backup/restore procedures
docker-compose exec neo4j neo4j-admin database load neo4j \
  --from=/backups/neo4j-backup.dump
```

### Issue: Neo4j index creation failed

**Symptoms:**
```
Failed to create index
Index already exists
```

**Solutions:**

1. **Drop and recreate indexes:**
```cypher
// Drop existing index
DROP INDEX vehicle_search IF EXISTS;

// Recreate
CREATE FULLTEXT INDEX vehicle_search
FOR (v:Vehicle)
ON EACH [v.name, v.model_code];
```

2. **Check index status:**
```cypher
SHOW INDEXES;
```

3. **Rebuild all indexes:**
```bash
curl -X POST http://localhost:8080/admin/graph/rebuild-indexes
```

### Issue: Data not persisting

**Symptoms:**
- Data disappears after restart
- Fresh database on every startup

**Solutions:**

1. **Check volumes are mounted:**
```bash
docker inspect ev-neo4j | grep Mounts -A 10
docker volume ls
```

2. **Verify docker-compose.yml has volumes:**
```yaml
services:
  neo4j:
    volumes:
      - neo4j_data:/data

volumes:
  neo4j_data:
```

3. **Don't use `-v` flag with `docker-compose down`:**
```bash
# Bad - deletes volumes
docker-compose down -v

# Good - preserves data
docker-compose down
```

---

## Cache Issues

### Issue: Cache not working

**Symptoms:**
- All requests are slow
- No improvement on repeated requests

**Diagnostics:**

```bash
# Check cache stats
curl http://localhost:8080/admin/cache/stats

# Check Redis is accessible
docker exec ev-redis redis-cli ping

# Check cache keys
docker exec ev-redis redis-cli KEYS "ev:*"
```

**Solutions:**

1. **Verify Redis connection:**
```bash
# Check .env
echo $REDIS_URL

# Test from API container
docker-compose exec api nc -zv redis 6379
```

2. **Check cache TTL:**
```bash
# In .env
CACHE_TTL_SECS=3600
```

3. **Clear and rebuild cache:**
```bash
curl -X POST http://localhost:8080/admin/cache/clear

# Make a few requests to populate cache
curl http://localhost:8080/api/v1/vehicles
curl http://localhost:8080/api/v1/chargers
```

### Issue: Stale cache data

**Symptoms:**
- Updated data not reflecting in API responses

**Solutions:**

1. **Clear specific cache pattern:**
```bash
docker exec ev-redis redis-cli --scan --pattern "ev:vehicle:*" | \
  xargs docker exec -i ev-redis redis-cli DEL
```

2. **Clear all cache:**
```bash
curl -X POST http://localhost:8080/admin/cache/clear
```

3. **Reduce cache TTL:**
```bash
# In .env
CACHE_TTL_SECS=300  # 5 minutes instead of 1 hour
```

---

## Docker Issues

### Issue: Container keeps restarting

**Diagnostics:**

```bash
# Check container status
docker-compose ps

# Check exit code
docker inspect ev-api | grep -A 5 State

# Check logs
docker-compose logs api
```

**Common causes:**

1. **Application crash:**
```bash
# Look for panic or error in logs
docker-compose logs api | grep -i "error\|panic"
```

2. **Health check failing:**
```bash
# Test health check manually
curl http://localhost:8080/health
```

3. **Port conflict:**
```bash
lsof -i :8080
```

**Solutions:**

1. **Check dependencies:**
```bash
# Ensure Neo4j and Redis are healthy
docker-compose ps neo4j redis
```

2. **Increase health check timeout:**
```yaml
services:
  api:
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 5
      start_period: 60s  # Increased
```

3. **Disable restart temporarily:**
```yaml
services:
  api:
    restart: "no"  # Temporarily disable to see error
```

### Issue: Out of disk space

**Symptoms:**
```
no space left on device
docker: Error response from daemon: write /var/lib/docker: no space left on device
```

**Solutions:**

1. **Check disk usage:**
```bash
df -h
docker system df
```

2. **Clean up Docker:**
```bash
# Remove unused containers
docker container prune

# Remove unused images
docker image prune -a

# Remove unused volumes
docker volume prune

# Nuclear option - clean everything
docker system prune -a --volumes
```

3. **Clean logs:**
```bash
# Find large log files
find /var/lib/docker/containers -name "*.log" -exec ls -lh {} \;

# Truncate logs
truncate -s 0 /var/lib/docker/containers/**/*-json.log
```

4. **Configure log rotation:**
```json
// /etc/docker/daemon.json
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  }
}
```

---

## Build Errors

### Issue: Cargo build fails

**Symptoms:**
```
error: failed to compile ev-api v0.1.0
could not compile `ev-api`
```

**Solutions:**

1. **Clean and rebuild:**
```bash
cargo clean
cargo build
```

2. **Update Rust:**
```bash
rustup update
rustc --version  # Should be 1.75+
```

3. **Update dependencies:**
```bash
cargo update
```

4. **Check for specific errors:**
```bash
# Look for missing dependencies
cargo build 2>&1 | grep "not found"

# Check for version conflicts
cargo tree
```

### Issue: Missing dependencies

**Symptoms:**
```
error: linker `cc` not found
error: failed to run custom build command for `openssl-sys`
```

**Solutions:**

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

**macOS:**
```bash
xcode-select --install
brew install openssl pkg-config
```

**RHEL/CentOS:**
```bash
sudo yum install gcc openssl-devel
```

### Issue: Docker build fails

**Symptoms:**
```
ERROR [builder 4/4] RUN cargo build --release --workspace
failed to solve: executor failed
```

**Solutions:**

1. **Increase Docker memory:**
```bash
# Docker Desktop > Settings > Resources
# Increase memory to 4GB+
```

2. **Build without cache:**
```bash
docker-compose build --no-cache api
```

3. **Build incrementally:**
```bash
docker build --target builder -t ev-api-builder .
docker build --target production -t ev-api .
```

---

## Runtime Errors

### Error: panic at 'connection pool exhausted'

**Solution:**

Increase connection pool size:
```rust
// In config.rs or .env
NEO4J_MAX_CONNECTIONS=200
REDIS_MAX_CONNECTIONS=100
```

### Error: Tokio runtime panic

**Symptoms:**
```
thread 'tokio-runtime-worker' panicked at 'Cannot drop a runtime in a context where blocking is not allowed'
```

**Solution:**

This is usually a code issue. Check logs for stack trace:
```bash
docker-compose logs api | grep -A 20 "panicked"
```

### Error: UUID parse error

**Symptoms:**
```
error_code: "BAD_REQUEST"
error: "Invalid UUID format"
```

**Solution:**

Ensure you're passing valid UUIDs:
```bash
# Good
curl http://localhost:8080/api/v1/vehicles/550e8400-e29b-41d4-a716-446655440000

# Bad
curl http://localhost:8080/api/v1/vehicles/invalid-uuid
```

---

## Data Seeding Issues

### Issue: Seed command fails

**Symptoms:**
```
Error: Failed to seed data
Database connection failed
```

**Solutions:**

1. **Ensure database is ready:**
```bash
# Wait for Neo4j to fully start
docker-compose logs -f neo4j
# Look for "Bolt enabled on localhost:7687"
```

2. **Check credentials:**
```bash
# In .env
NEO4J_USER=neo4j
NEO4J_PASSWORD=password123
```

3. **Run seed with verbose logging:**
```bash
RUST_LOG=debug cargo run --bin ev-data -- seed
```

### Issue: Duplicate data

**Symptoms:**
- Constraint violation errors
- Multiple entries for same entity

**Solutions:**

1. **Clear database before seeding:**
```cypher
docker exec ev-neo4j cypher-shell -u neo4j -p password123
MATCH (n) DETACH DELETE n;
```

2. **Or drop and recreate:**
```bash
docker-compose down -v
docker-compose up -d
docker-compose exec api ev-data seed
```

---

## Network Issues

### Issue: Cannot access API from outside Docker

**Symptoms:**
- Works inside container but not from host
- Works on localhost but not from other machines

**Solutions:**

1. **Check port binding:**
```bash
docker-compose ps
# Should show 0.0.0.0:8080->8080/tcp
```

2. **Verify in docker-compose.yml:**
```yaml
services:
  api:
    ports:
      - "8080:8080"  # Not "127.0.0.1:8080:8080"
```

3. **Check firewall:**
```bash
# Linux
sudo ufw allow 8080/tcp

# Check if port is accessible
telnet your-server-ip 8080
```

4. **Check Docker network:**
```bash
docker network inspect bridge
```

### Issue: Containers cannot communicate

**Symptoms:**
- API cannot reach Neo4j/Redis
- "connection refused" errors

**Solutions:**

1. **Use service names, not localhost:**
```yaml
# Good
NEO4J_URI=bolt://neo4j:7687
REDIS_URL=redis://redis:6379

# Bad
NEO4J_URI=bolt://localhost:7687
```

2. **Verify network:**
```bash
docker network ls
docker network inspect ev-knowledge-graph-api-_default
```

3. **Test connectivity:**
```bash
docker-compose exec api ping neo4j
docker-compose exec api nc -zv neo4j 7687
```

---

## Getting Help

### Before asking for help

1. **Collect diagnostic information:**
```bash
# System info
uname -a
docker --version
docker-compose --version

# Service status
docker-compose ps

# Recent logs
docker-compose logs --tail=100 > logs.txt

# Health check
curl http://localhost:8080/admin/health/deep > health.json
```

2. **Try in order:**
- Check this troubleshooting guide
- Search [GitHub Issues](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues)
- Check [documentation](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/tree/main/docs)

### Getting Support

**GitHub Issues:**
- [Report a bug](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues/new?template=bug_report.md)
- [Request a feature](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/issues/new?template=feature_request.md)

**Include in bug reports:**
1. Docker and system versions
2. Complete error messages
3. Relevant logs (use pastebin for long logs)
4. Steps to reproduce
5. What you've already tried

**Community:**
- [GitHub Discussions](https://github.com/FFFSTANZA/EV-Knowledge-graph-API-/discussions)

---

## Common Command Reference

```bash
# Health checks
curl http://localhost:8080/health
curl http://localhost:8080/admin/health/deep
curl http://localhost:8080/stats

# View logs
docker-compose logs -f api
docker-compose logs --tail=100 neo4j
docker-compose logs --since=1h redis

# Restart services
docker-compose restart
docker-compose restart api
docker-compose restart neo4j redis

# Clean restart
docker-compose down
docker-compose up -d

# Nuclear option (deletes all data)
docker-compose down -v
docker volume prune
docker-compose up -d

# Check resources
docker stats
docker system df
df -h

# Database operations
docker exec ev-neo4j cypher-shell -u neo4j -p password123
docker exec ev-redis redis-cli

# Cache management
curl -X POST http://localhost:8080/admin/cache/clear
curl http://localhost:8080/admin/cache/stats

# Rebuild
cargo clean && cargo build --release
docker-compose build --no-cache
```

---

## Advanced Debugging

### Enable debug logging

```bash
# In .env
RUST_LOG=debug,ev_api=trace,ev_graph=debug

# Restart
docker-compose restart api
```

### Attach debugger

```bash
# Add to Dockerfile
RUN cargo install cargo-watch

# Run with debugging
docker-compose run --service-ports api cargo watch -x run
```

### Profile performance

```bash
# Install flamegraph
cargo install flamegraph

# Profile
sudo cargo flamegraph --bin ev-api

# View flamegraph.svg
```

### Network debugging

```bash
# Install tcpdump in container
docker-compose exec api apk add tcpdump

# Capture traffic
docker-compose exec api tcpdump -i any -w /tmp/capture.pcap

# Analyze with wireshark
```

---

**Last Updated:** 2024-11-18
**Version:** 0.1.0

For more information:
- [Getting Started Guide](GETTING_STARTED.md)
- [Architecture Documentation](ARCHITECTURE.md)
- [Deployment Guide](DEPLOYMENT.md)
- [API Documentation](API.md)
