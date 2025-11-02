use crate::models::{Connection, CreateConnection, UpdateConnection};
use leptos::prelude::*;
use sqlx::SqlitePool;

/// Get all connections for a topology
#[server(GetConnections, "/api")]
pub async fn get_connections(topology_id: i64) -> Result<Vec<Connection>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    let connections = sqlx::query_as::<_, Connection>(
        "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, metadata, created_at, updated_at
         FROM connections WHERE topology_id = ? ORDER BY created_at DESC"
    )
    .bind(topology_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(connections)
}

/// Create a new connection
#[server(CreateConnectionFn, "/api")]
pub async fn create_connection(data: CreateConnection) -> Result<Connection, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    // Validate that source and target are different
    if data.source_node_id == data.target_node_id {
        return Err(ServerFnError::new("Source and target nodes must be different"));
    }

    let result = sqlx::query(
        "INSERT INTO connections (topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, metadata)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(data.topology_id)
    .bind(data.source_node_id)
    .bind(data.target_node_id)
    .bind(data.connection_type.as_deref().unwrap_or("ethernet"))
    .bind(data.bandwidth_mbps)
    .bind(data.latency_ms)
    .bind(data.status.as_deref().unwrap_or("active"))
    .bind(&data.metadata)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    let id = result.last_insert_rowid();

    // Fetch the created connection
    let connection = sqlx::query_as::<_, Connection>(
        "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, metadata, created_at, updated_at
         FROM connections WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(connection)
}

/// Update a connection
#[server(UpdateConnectionFn, "/api")]
pub async fn update_connection(id: i64, data: UpdateConnection) -> Result<Connection, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();

    if let Some(connection_type) = &data.connection_type {
        updates.push("connection_type = ?");
        values.push(connection_type.clone());
    }
    if let Some(bandwidth_mbps) = data.bandwidth_mbps {
        updates.push("bandwidth_mbps = ?");
        values.push(bandwidth_mbps.to_string());
    }
    if let Some(latency_ms) = data.latency_ms {
        updates.push("latency_ms = ?");
        values.push(latency_ms.to_string());
    }
    if let Some(status) = &data.status {
        updates.push("status = ?");
        values.push(status.clone());
    }
    if let Some(metadata) = &data.metadata {
        updates.push("metadata = ?");
        values.push(metadata.clone());
    }

    if updates.is_empty() {
        return Err(ServerFnError::new("No fields to update"));
    }

    let query_str = format!("UPDATE connections SET {} WHERE id = ?", updates.join(", "));

    let mut query = sqlx::query(&query_str);
    for value in values {
        query = query.bind(value);
    }
    query = query.bind(id);

    query.execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    // Fetch the updated connection
    let connection = sqlx::query_as::<_, Connection>(
        "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, metadata, created_at, updated_at
         FROM connections WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(connection)
}

/// Delete a connection
#[server(DeleteConnection, "/api")]
pub async fn delete_connection(id: i64) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    sqlx::query("DELETE FROM connections WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(())
}
