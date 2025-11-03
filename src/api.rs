use crate::models::{Topology, CreateTopology, TopologyFull, Node, Connection};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use sqlx::SqlitePool;

/// Get all topologies from the database
#[server(GetTopologies, "/api")]
pub async fn get_topologies() -> Result<Vec<Topology>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
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

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Create a new topology
#[server(CreateTopologyFn, "/api")]
pub async fn create_topology(data: CreateTopology) -> Result<Topology, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
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

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Delete a topology
#[server(DeleteTopology, "/api")]
pub async fn delete_topology(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
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

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Get complete topology with all nodes and connections
#[server(GetTopologyFull, "/api")]
pub async fn get_topology_full(id: i64) -> Result<TopologyFull, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
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

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}
