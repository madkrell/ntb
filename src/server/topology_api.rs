use crate::models::{Topology, CreateTopology, Node, Connection};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// Complete topology data with nodes and connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyFull {
    pub topology: Topology,
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

/// Get all topologies from the database
#[server(GetTopologies, "/api")]
pub async fn get_topologies() -> Result<Vec<Topology>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    let topologies = sqlx::query_as::<_, Topology>(
        "SELECT id, name, description, created_at, updated_at FROM topologies ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(topologies)
}

/// Create a new topology
#[server(CreateTopologyFn, "/api")]
pub async fn create_topology(data: CreateTopology) -> Result<Topology, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    let result = sqlx::query(
        "INSERT INTO topologies (name, description) VALUES (?, ?)"
    )
    .bind(&data.name)
    .bind(&data.description)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    let id = result.last_insert_rowid();

    // Fetch the created topology
    let topology = sqlx::query_as::<_, Topology>(
        "SELECT id, name, description, created_at, updated_at FROM topologies WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(topology)
}

/// Delete a topology
#[server(DeleteTopology, "/api")]
pub async fn delete_topology(id: i64) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    sqlx::query("DELETE FROM topologies WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(())
}

/// Get complete topology with all nodes and connections
#[server(GetTopologyFull, "/api")]
pub async fn get_topology_full(id: i64) -> Result<TopologyFull, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    // Fetch topology
    let topology = sqlx::query_as::<_, Topology>(
        "SELECT id, name, description, created_at, updated_at FROM topologies WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Topology not found: {}", e)))?;

    // Fetch all nodes for this topology
    let nodes = sqlx::query_as::<_, Node>(
        "SELECT id, topology_id, name, node_type, ip_address, position_x, position_y, position_z, metadata, created_at, updated_at
         FROM nodes WHERE topology_id = ? ORDER BY created_at"
    )
    .bind(id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    // Fetch all connections for this topology
    let connections = sqlx::query_as::<_, Connection>(
        "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, metadata, created_at, updated_at
         FROM connections WHERE topology_id = ? ORDER BY created_at"
    )
    .bind(id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(TopologyFull {
        topology,
        nodes,
        connections,
    })
}
