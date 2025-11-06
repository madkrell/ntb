use crate::models::{
    Topology, CreateTopology, TopologyFull,
    Node, CreateNode, UpdateNode,
    Connection, CreateConnection, UpdateConnection,
    UISettings, UpdateUISettings,
};
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
            "SELECT id, topology_id, name, node_type, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, metadata, created_at, updated_at
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

// ============================================================================
// Node CRUD Operations
// ============================================================================

/// Get a single node by ID
#[server(GetNode, "/api")]
pub async fn get_node(id: i64) -> Result<Node, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        let node = sqlx::query_as::<_, Node>(
            "SELECT id, topology_id, name, node_type, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, metadata, created_at, updated_at
             FROM nodes WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Node not found: {}", e)))?;

        Ok(node)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Create a new node
#[server(CreateNodeFn, "/api")]
pub async fn create_node(data: CreateNode) -> Result<Node, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        // Use default position if not provided
        let pos_x = data.position_x.unwrap_or(0.0);
        let pos_y = data.position_y.unwrap_or(0.0);
        let pos_z = data.position_z.unwrap_or(0.0);
        // Default rotation X = 90° for Blender glTF models to sit flat on grid floor
        let rot_x = data.rotation_x.unwrap_or(90.0);
        let rot_y = data.rotation_y.unwrap_or(0.0);
        let rot_z = data.rotation_z.unwrap_or(0.0);

        let result = sqlx::query(
            "INSERT INTO nodes (topology_id, name, node_type, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(data.topology_id)
        .bind(&data.name)
        .bind(&data.node_type)
        .bind(&data.ip_address)
        .bind(pos_x)
        .bind(pos_y)
        .bind(pos_z)
        .bind(rot_x)
        .bind(rot_y)
        .bind(rot_z)
        .bind(&data.metadata)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        let id = result.last_insert_rowid();

        // Fetch the created node
        let node = sqlx::query_as::<_, Node>(
            "SELECT id, topology_id, name, node_type, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, metadata, created_at, updated_at
             FROM nodes WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        Ok(node)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Update an existing node
#[server(UpdateNodeFn, "/api")]
pub async fn update_node(id: i64, data: UpdateNode) -> Result<Node, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        // Build dynamic UPDATE query based on which fields are provided
        let mut updates = Vec::new();
        let mut query_str = "UPDATE nodes SET ".to_string();

        if let Some(ref name) = data.name {
            updates.push(("name = ?", name.clone()));
        }
        if let Some(ref node_type) = data.node_type {
            updates.push(("node_type = ?", node_type.clone()));
        }
        if data.position_x.is_some() || data.position_y.is_some() || data.position_z.is_some() {
            if let Some(pos_x) = data.position_x {
                updates.push(("position_x = ?", pos_x.to_string()));
            }
            if let Some(pos_y) = data.position_y {
                updates.push(("position_y = ?", pos_y.to_string()));
            }
            if let Some(pos_z) = data.position_z {
                updates.push(("position_z = ?", pos_z.to_string()));
            }
        }

        if updates.is_empty() && data.ip_address.is_none() && data.metadata.is_none() {
            return Err(ServerFnError::new("No fields to update"));
        }

        // Simpler approach: just update all fields that are provided
        query_str.push_str("updated_at = CURRENT_TIMESTAMP");

        if data.name.is_some() {
            query_str.push_str(", name = ?");
        }
        if data.node_type.is_some() {
            query_str.push_str(", node_type = ?");
        }
        if data.ip_address.is_some() {
            query_str.push_str(", ip_address = ?");
        }
        if data.position_x.is_some() {
            query_str.push_str(", position_x = ?");
        }
        if data.position_y.is_some() {
            query_str.push_str(", position_y = ?");
        }
        if data.position_z.is_some() {
            query_str.push_str(", position_z = ?");
        }
        if data.rotation_x.is_some() {
            query_str.push_str(", rotation_x = ?");
        }
        if data.rotation_y.is_some() {
            query_str.push_str(", rotation_y = ?");
        }
        if data.rotation_z.is_some() {
            query_str.push_str(", rotation_z = ?");
        }
        if data.metadata.is_some() {
            query_str.push_str(", metadata = ?");
        }

        query_str.push_str(" WHERE id = ?");

        let mut query = sqlx::query(&query_str);

        // Bind values in the same order
        if let Some(ref name) = data.name {
            query = query.bind(name);
        }
        if let Some(ref node_type) = data.node_type {
            query = query.bind(node_type);
        }
        if let Some(ref ip) = data.ip_address {
            query = query.bind(ip);
        }
        if let Some(pos_x) = data.position_x {
            query = query.bind(pos_x);
        }
        if let Some(pos_y) = data.position_y {
            query = query.bind(pos_y);
        }
        if let Some(pos_z) = data.position_z {
            query = query.bind(pos_z);
        }
        if let Some(rot_x) = data.rotation_x {
            query = query.bind(rot_x);
        }
        if let Some(rot_y) = data.rotation_y {
            query = query.bind(rot_y);
        }
        if let Some(rot_z) = data.rotation_z {
            query = query.bind(rot_z);
        }
        if let Some(ref metadata) = data.metadata {
            query = query.bind(metadata);
        }

        query = query.bind(id);

        query.execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        // Fetch the updated node
        let node = sqlx::query_as::<_, Node>(
            "SELECT id, topology_id, name, node_type, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, metadata, created_at, updated_at
             FROM nodes WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Node not found: {}", e)))?;

        Ok(node)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Delete a node
#[server(DeleteNode, "/api")]
pub async fn delete_node(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
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

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

// ============================================================================
// Connection CRUD Operations
// ============================================================================

/// Get a single connection by ID
#[server(GetConnection, "/api")]
pub async fn get_connection(id: i64) -> Result<Connection, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        let connection = sqlx::query_as::<_, Connection>(
            "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, metadata, created_at, updated_at
             FROM connections WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Connection not found: {}", e)))?;

        Ok(connection)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Create a new connection
#[server(CreateConnectionFn, "/api")]
pub async fn create_connection(data: CreateConnection) -> Result<Connection, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        // Use defaults if not provided
        let conn_type = data.connection_type.unwrap_or_else(|| "ethernet".to_string());
        let status = data.status.unwrap_or_else(|| "active".to_string());

        let result = sqlx::query(
            "INSERT INTO connections (topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(data.topology_id)
        .bind(data.source_node_id)
        .bind(data.target_node_id)
        .bind(&conn_type)
        .bind(&data.bandwidth_mbps)
        .bind(&data.latency_ms)
        .bind(&status)
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

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Update an existing connection
#[server(UpdateConnectionFn, "/api")]
pub async fn update_connection(id: i64, data: UpdateConnection) -> Result<Connection, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        // Build dynamic UPDATE query
        let mut query_str = "UPDATE connections SET updated_at = CURRENT_TIMESTAMP".to_string();

        if let Some(_) = data.connection_type {
            query_str.push_str(", connection_type = ?");
        }
        if let Some(_) = data.bandwidth_mbps {
            query_str.push_str(", bandwidth_mbps = ?");
        }
        if let Some(_) = data.latency_ms {
            query_str.push_str(", latency_ms = ?");
        }
        if let Some(_) = data.status {
            query_str.push_str(", status = ?");
        }
        if let Some(_) = data.metadata {
            query_str.push_str(", metadata = ?");
        }

        query_str.push_str(" WHERE id = ?");

        let mut query = sqlx::query(&query_str);

        // Bind values in the same order
        if let Some(ref conn_type) = data.connection_type {
            query = query.bind(conn_type);
        }
        if let Some(bw) = data.bandwidth_mbps {
            query = query.bind(bw);
        }
        if let Some(latency) = data.latency_ms {
            query = query.bind(latency);
        }
        if let Some(ref status) = data.status {
            query = query.bind(status);
        }
        if let Some(ref metadata) = data.metadata {
            query = query.bind(metadata);
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
        .map_err(|e| ServerFnError::new(format!("Connection not found: {}", e)))?;

        Ok(connection)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Delete a connection
#[server(DeleteConnection, "/api")]
pub async fn delete_connection(id: i64) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
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

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

// ============================================================================
// UI Settings Functions
// ============================================================================

/// Get UI settings (single row, id=1)
#[server(GetUISettings, "/api")]
pub async fn get_ui_settings() -> Result<UISettings, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        let settings = sqlx::query_as::<_, UISettings>(
            "SELECT id, show_grid, show_x_axis, show_y_axis, show_z_axis,
                    ambient_intensity, key_light_intensity, fill_light_intensity, rim_light_intensity,
                    created_at, updated_at
             FROM ui_settings WHERE id = 1"
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        Ok(settings)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Update UI settings (only updates provided fields)
#[server(UpdateUISettingsFn, "/api")]
pub async fn update_ui_settings(data: UpdateUISettings) -> Result<UISettings, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        // Build dynamic UPDATE query
        let mut updates = Vec::new();
        let mut values: Vec<String> = Vec::new();

        if let Some(show_grid) = data.show_grid {
            updates.push("show_grid = ?");
            values.push(if show_grid { "1" } else { "0" }.to_string());
        }

        if let Some(show_x_axis) = data.show_x_axis {
            updates.push("show_x_axis = ?");
            values.push(if show_x_axis { "1" } else { "0" }.to_string());
        }

        if let Some(show_y_axis) = data.show_y_axis {
            updates.push("show_y_axis = ?");
            values.push(if show_y_axis { "1" } else { "0" }.to_string());
        }

        if let Some(show_z_axis) = data.show_z_axis {
            updates.push("show_z_axis = ?");
            values.push(if show_z_axis { "1" } else { "0" }.to_string());
        }

        if let Some(ambient_intensity) = data.ambient_intensity {
            updates.push("ambient_intensity = ?");
            values.push(ambient_intensity.to_string());
        }

        if let Some(key_light_intensity) = data.key_light_intensity {
            updates.push("key_light_intensity = ?");
            values.push(key_light_intensity.to_string());
        }

        if let Some(fill_light_intensity) = data.fill_light_intensity {
            updates.push("fill_light_intensity = ?");
            values.push(fill_light_intensity.to_string());
        }

        if let Some(rim_light_intensity) = data.rim_light_intensity {
            updates.push("rim_light_intensity = ?");
            values.push(rim_light_intensity.to_string());
        }

        if updates.is_empty() {
            return Err(ServerFnError::new("No fields to update"));
        }

        let query = format!("UPDATE ui_settings SET {} WHERE id = 1", updates.join(", "));

        let mut query_builder = sqlx::query(&query);
        for value in &values {
            query_builder = query_builder.bind(value);
        }

        query_builder
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        // Fetch and return updated settings
        get_ui_settings().await
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}
