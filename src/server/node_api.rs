use crate::models::{Node, CreateNode, UpdateNode};
use leptos::prelude::*;
use sqlx::SqlitePool;

/// Get all nodes for a topology
#[server(GetNodes, "/api")]
pub async fn get_nodes(topology_id: i64) -> Result<Vec<Node>, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    let nodes = sqlx::query_as::<_, Node>(
        "SELECT id, topology_id, name, node_type, ip_address, position_x, position_y, position_z, metadata, created_at, updated_at
         FROM nodes WHERE topology_id = ? ORDER BY created_at DESC"
    )
    .bind(topology_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(nodes)
}

/// Create a new node
#[server(CreateNodeFn, "/api")]
pub async fn create_node(data: CreateNode) -> Result<Node, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    let result = sqlx::query(
        "INSERT INTO nodes (topology_id, name, node_type, ip_address, position_x, position_y, position_z, metadata)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(data.topology_id)
    .bind(&data.name)
    .bind(&data.node_type)
    .bind(&data.ip_address)
    .bind(data.position_x.unwrap_or(0.0))
    .bind(data.position_y.unwrap_or(0.0))
    .bind(data.position_z.unwrap_or(0.0))
    .bind(&data.metadata)
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    let id = result.last_insert_rowid();

    // Fetch the created node
    let node = sqlx::query_as::<_, Node>(
        "SELECT id, topology_id, name, node_type, ip_address, position_x, position_y, position_z, metadata, created_at, updated_at
         FROM nodes WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(node)
}

/// Update a node
#[server(UpdateNodeFn, "/api")]
pub async fn update_node(id: i64, data: UpdateNode) -> Result<Node, ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    // Build dynamic update query based on provided fields
    let mut updates = Vec::new();
    let mut values: Vec<String> = Vec::new();

    if let Some(name) = &data.name {
        updates.push("name = ?");
        values.push(name.clone());
    }
    if let Some(node_type) = &data.node_type {
        updates.push("node_type = ?");
        values.push(node_type.clone());
    }
    if let Some(ip_address) = &data.ip_address {
        updates.push("ip_address = ?");
        values.push(ip_address.clone());
    }
    if let Some(position_x) = data.position_x {
        updates.push("position_x = ?");
        values.push(position_x.to_string());
    }
    if let Some(position_y) = data.position_y {
        updates.push("position_y = ?");
        values.push(position_y.to_string());
    }
    if let Some(position_z) = data.position_z {
        updates.push("position_z = ?");
        values.push(position_z.to_string());
    }
    if let Some(metadata) = &data.metadata {
        updates.push("metadata = ?");
        values.push(metadata.clone());
    }

    if updates.is_empty() {
        return Err(ServerFnError::new("No fields to update"));
    }

    let query_str = format!("UPDATE nodes SET {} WHERE id = ?", updates.join(", "));

    let mut query = sqlx::query(&query_str);
    for value in values {
        query = query.bind(value);
    }
    query = query.bind(id);

    query.execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    // Fetch the updated node
    let node = sqlx::query_as::<_, Node>(
        "SELECT id, topology_id, name, node_type, ip_address, position_x, position_y, position_z, metadata, created_at, updated_at
         FROM nodes WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(node)
}

/// Delete a node
#[server(DeleteNode, "/api")]
pub async fn delete_node(id: i64) -> Result<(), ServerFnError> {
    use axum::Extension;
    use leptos_axum::extract;

    let Extension(pool) = extract::<Extension<SqlitePool>>()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

    sqlx::query("DELETE FROM nodes WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    Ok(())
}
