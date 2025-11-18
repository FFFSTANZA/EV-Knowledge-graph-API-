use neo4rs::{Graph, ConfigBuilder};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphDbError {
    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Query error: {0}")]
    Query(String),

    #[error("Transaction error: {0}")]
    Transaction(String),
}

pub type Result<T> = std::result::Result<T, GraphDbError>;

/// Neo4j Graph Database connection
#[derive(Clone)]
pub struct GraphDb {
    graph: Arc<Graph>,
}

impl GraphDb {
    /// Create a new GraphDb connection
    pub async fn new(uri: &str, user: &str, password: &str) -> Result<Self> {
        let config = ConfigBuilder::default()
            .uri(uri)
            .user(user)
            .password(password)
            .db("neo4j")
            .fetch_size(500)
            .max_connections(100)
            .build()
            .map_err(|e| GraphDbError::Connection(e.to_string()))?;

        let graph = Graph::connect(config)
            .await
            .map_err(|e| GraphDbError::Connection(e.to_string()))?;

        Ok(Self {
            graph: Arc::new(graph),
        })
    }

    /// Get reference to the graph
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// Initialize the graph schema
    pub async fn init_schema(&self) -> Result<()> {
        use crate::schema::INIT_SCHEMA;

        for query in INIT_SCHEMA {
            self.graph
                .run(neo4rs::query(query))
                .await
                .map_err(|e| GraphDbError::Query(format!("Schema init failed: {}", e)))?;
        }

        tracing::info!("Graph schema initialized successfully");
        Ok(())
    }

    /// Health check
    pub async fn health_check(&self) -> Result<()> {
        self.graph
            .run(neo4rs::query("RETURN 1"))
            .await
            .map_err(|e| GraphDbError::Connection(format!("Health check failed: {}", e)))?;
        Ok(())
    }

    /// Get database statistics
    pub async fn get_stats(&self) -> Result<GraphStats> {
        let mut result = self
            .graph
            .execute(neo4rs::query(
                "MATCH (n) RETURN labels(n) as label, count(*) as count"
            ))
            .await
            .map_err(|e| GraphDbError::Query(e.to_string()))?;

        let mut stats = GraphStats::default();

        while let Ok(Some(row)) = result.next().await {
            if let (Ok(labels), Ok(count)) = (
                row.get::<Vec<String>>("label"),
                row.get::<i64>("count"),
            ) {
                if let Some(label) = labels.first() {
                    stats.node_counts.push((label.clone(), count));
                }
            }
        }

        Ok(stats)
    }
}

#[derive(Debug, Default)]
pub struct GraphStats {
    pub node_counts: Vec<(String, i64)>,
}
