use crate::config::AppConfig;
use ev_cache::Cache;
use ev_graph::GraphDb;
use anyhow::Result;

#[derive(Clone)]
pub struct AppState {
    config: AppConfig,
    graph_db: GraphDb,
    cache: Cache,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self> {
        // Initialize Neo4j connection
        let graph_db = GraphDb::new(
            &config.neo4j.uri,
            &config.neo4j.user,
            &config.neo4j.password,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to Neo4j: {}", e))?;

        tracing::info!("✓ Connected to Neo4j at {}", config.neo4j.uri);

        // Initialize Redis cache
        let cache = Cache::new(&config.redis.url, config.cache_ttl_secs)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to Redis: {}", e))?;

        tracing::info!("✓ Connected to Redis at {}", config.redis.url);

        Ok(Self {
            config,
            graph_db,
            cache,
        })
    }

    pub fn graph_db(&self) -> &GraphDb {
        &self.graph_db
    }

    pub fn cache(&self) -> Cache {
        self.cache.clone()
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }
}
