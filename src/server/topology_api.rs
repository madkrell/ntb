use crate::models::{Topology, CreateTopology};
use leptos::prelude::*;
use sqlx::SqlitePool;

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
