use crate::models::{
    Topology, CreateTopology, UpdateTopology, TopologyFull,
    Node, CreateNode, UpdateNode,
    Connection, CreateConnection, UpdateConnection,
    UISettings, UpdateUISettings,
    VendorListResponse, VendorInfo, ModelInfo,
    ConnectionTrafficMetric,
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

/// Update topology (name and/or description)
#[server(UpdateTopologyFn, "/api")]
pub async fn update_topology(id: i64, data: UpdateTopology) -> Result<Topology, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        // Build dynamic UPDATE query
        let mut query_str = "UPDATE topologies SET updated_at = CURRENT_TIMESTAMP".to_string();

        if data.name.is_some() {
            query_str.push_str(", name = ?");
        }
        if data.description.is_some() {
            query_str.push_str(", description = ?");
        }

        query_str.push_str(" WHERE id = ?");

        let mut query = sqlx::query(&query_str);

        if let Some(ref name) = data.name {
            query = query.bind(name);
        }
        if let Some(ref description) = data.description {
            query = query.bind(description);
        }

        query = query.bind(id);

        query.execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        // Fetch and return updated topology
        let topology = sqlx::query_as::<_, Topology>(
            "SELECT id, name, description, created_at, updated_at FROM topologies WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Topology not found: {}", e)))?;

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
            "SELECT id, topology_id, name, node_type, vendor, model_name, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, scale, color, metadata, created_at, updated_at
             FROM nodes WHERE topology_id = ? ORDER BY created_at"
        )
        .bind(id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        // Fetch all connections for this topology
        let connections = sqlx::query_as::<_, Connection>(
            "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, color, metadata, created_at, updated_at
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
            "SELECT id, topology_id, name, node_type, vendor, model_name, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, scale, color, metadata, created_at, updated_at
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
        let scale = data.scale.unwrap_or(1.0);
        let color = data.color.unwrap_or_else(|| "100,150,255".to_string()); // Default blue
        let vendor = data.vendor.unwrap_or_else(|| "generic".to_string());
        let model_name = data.model_name.unwrap_or_else(|| format!("blob-{}", data.node_type));

        let result = sqlx::query(
            "INSERT INTO nodes (topology_id, name, node_type, vendor, model_name, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, scale, color, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(data.topology_id)
        .bind(&data.name)
        .bind(&data.node_type)
        .bind(&vendor)
        .bind(&model_name)
        .bind(&data.ip_address)
        .bind(pos_x)
        .bind(pos_y)
        .bind(pos_z)
        .bind(rot_x)
        .bind(rot_y)
        .bind(rot_z)
        .bind(scale)
        .bind(&color)
        .bind(&data.metadata)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        let id = result.last_insert_rowid();

        // Fetch the created node
        let node = sqlx::query_as::<_, Node>(
            "SELECT id, topology_id, name, node_type, vendor, model_name, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, scale, color, metadata, created_at, updated_at
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
        if data.vendor.is_some() {
            query_str.push_str(", vendor = ?");
        }
        if data.model_name.is_some() {
            query_str.push_str(", model_name = ?");
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
        if data.scale.is_some() {
            query_str.push_str(", scale = ?");
        }
        if data.color.is_some() {
            query_str.push_str(", color = ?");
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
        if let Some(ref vendor) = data.vendor {
            query = query.bind(vendor);
        }
        if let Some(ref model_name) = data.model_name {
            query = query.bind(model_name);
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
        if let Some(scale) = data.scale {
            query = query.bind(scale);
        }
        if let Some(ref color) = data.color {
            query = query.bind(color);
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
            "SELECT id, topology_id, name, node_type, vendor, model_name, ip_address, position_x, position_y, position_z, rotation_x, rotation_y, rotation_z, scale, color, metadata, created_at, updated_at
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
            "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, color, metadata, created_at, updated_at
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
        let color = data.color.unwrap_or_else(|| "128,128,128".to_string());

        let result = sqlx::query(
            "INSERT INTO connections (topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, color, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(data.topology_id)
        .bind(data.source_node_id)
        .bind(data.target_node_id)
        .bind(&conn_type)
        .bind(&data.bandwidth_mbps)
        .bind(&data.latency_ms)
        .bind(&status)
        .bind(&color)
        .bind(&data.metadata)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

        let id = result.last_insert_rowid();

        // Fetch the created connection
        let connection = sqlx::query_as::<_, Connection>(
            "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, color, metadata, created_at, updated_at
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
        if let Some(_) = data.color {
            query_str.push_str(", color = ?");
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
        if let Some(ref color) = data.color {
            query = query.bind(color);
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
            "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, color, metadata, created_at, updated_at
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
                    use_environment_lighting, environment_map,
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

        if let Some(use_environment_lighting) = data.use_environment_lighting {
            updates.push("use_environment_lighting = ?");
            values.push(if use_environment_lighting { "1" } else { "0" }.to_string());
        }

        if let Some(environment_map) = data.environment_map {
            updates.push("environment_map = ?");
            values.push(environment_map);
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

/// Get available vendors and models for a specific node type
/// Scans the public/models/{node_type}/ directory for vendor folders
#[server(GetVendorsForType, "/api")]
pub async fn get_vendors_for_type(node_type: String) -> Result<VendorListResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use std::fs;
        use std::path::Path;

        let models_path = format!("public/models/{}", node_type);
        let models_dir = Path::new(&models_path);

        if !models_dir.exists() {
            return Ok(VendorListResponse {
                node_type: node_type.clone(),
                vendors: vec![],
            });
        }

        let mut vendors = Vec::new();

        // Read vendor directories
        let entries = fs::read_dir(models_dir)
            .map_err(|e| ServerFnError::new(format!("Failed to read models directory: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| ServerFnError::new(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                let vendor_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Scan for .glb files in vendor directory
                let mut models = Vec::new();

                if let Ok(model_entries) = fs::read_dir(&path) {
                    for model_entry in model_entries {
                        if let Ok(model_entry) = model_entry {
                            let model_path = model_entry.path();
                            if let Some(ext) = model_path.extension() {
                                if ext == "glb" {
                                    if let Some(file_name) = model_path.file_name().and_then(|n| n.to_str()) {
                                        // Remove .glb extension for storage
                                        let file_name_no_ext = file_name.trim_end_matches(".glb");
                                        let display_name = file_name_no_ext
                                            .replace("-", " ")
                                            .replace("_", " ");

                                        models.push(ModelInfo {
                                            file_name: file_name_no_ext.to_string(),
                                            display_name: capitalize_words(&display_name),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                // Check if vendor icon exists
                let icon_path = format!("public/icons/vendors/{}.svg", vendor_name);
                let has_icon = Path::new(&icon_path).exists();

                let display_name = capitalize_words(&vendor_name.replace("-", " ").replace("_", " "));

                vendors.push(VendorInfo {
                    name: vendor_name,
                    display_name,
                    has_icon,
                    is_available: !models.is_empty(),
                    models,
                });
            }
        }

        // Sort vendors: generic first, then available vendors, then unavailable
        vendors.sort_by(|a, b| {
            if a.name == "generic" {
                std::cmp::Ordering::Less
            } else if b.name == "generic" {
                std::cmp::Ordering::Greater
            } else if a.is_available && !b.is_available {
                std::cmp::Ordering::Less
            } else if !a.is_available && b.is_available {
                std::cmp::Ordering::Greater
            } else {
                a.display_name.cmp(&b.display_name)
            }
        });

        Ok(VendorListResponse {
            node_type,
            vendors,
        })
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

#[cfg(feature = "ssr")]
fn capitalize_words(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ============================================================================
// Traffic Monitoring (Phase 6)
// ============================================================================

/// Generate mock traffic data for a specific topology
/// This simulates realistic network traffic patterns for demonstration purposes
#[server(GenerateMockTraffic, "/api")]
pub async fn generate_mock_traffic(topology_id: i64, traffic_level: String) -> Result<usize, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        // Get all connections for this topology
        let connections = sqlx::query_as::<_, Connection>(
            "SELECT id, topology_id, source_node_id, target_node_id, connection_type, bandwidth_mbps, latency_ms, status, color, metadata, created_at, updated_at
             FROM connections WHERE topology_id = ?"
        )
        .bind(topology_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch connections: {}", e)))?;

        if connections.is_empty() {
            return Ok(0);
        }

        // Traffic level ranges for predictable color visualization
        // Each level maps to a specific utilization range
        let (min_util, max_util) = match traffic_level.as_str() {
            "low" => (10.0, 35.0),      // Green (< 40%)
            "medium" => (45.0, 65.0),   // Orange (40-70%)
            "high" => (75.0, 95.0),     // Red (> 70%)
            _ => (45.0, 65.0),          // default to medium
        };

        use rand::SeedableRng;
        use rand::Rng;
        // Use StdRng which is Send-safe for async contexts
        let mut rng = rand::rngs::StdRng::from_entropy();
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut metrics_created = 0;

        for connection in connections {
            // Skip inactive connections
            if connection.status != "active" {
                continue;
            }

            // Get bandwidth capacity (default to 1000 Mbps if not set)
            let bandwidth_capacity = connection.bandwidth_mbps.unwrap_or(1000) as f64;

            // Generate utilization within the specified range for this traffic level
            let utilization_pct = rng.gen_range(min_util..max_util);

            // Calculate throughput based on utilization and bandwidth capacity
            let throughput_mbps = (bandwidth_capacity * utilization_pct / 100.0).max(0.1);

            // Packets per second (roughly 1000 packets per Mbps for standard Ethernet)
            let packets_per_sec = (throughput_mbps * 1000.0) as i64;

            // Latency: base latency + congestion penalty (higher utilization = higher latency)
            let base_latency = connection.latency_ms.unwrap_or(10.0);
            let congestion_penalty = if utilization_pct > 70.0 {
                // Heavy congestion: significant latency increase
                (utilization_pct - 70.0) * rng.gen_range(0.5..1.5)
            } else if utilization_pct > 40.0 {
                // Moderate congestion: slight latency increase
                (utilization_pct - 40.0) * rng.gen_range(0.1..0.3)
            } else {
                0.0
            };
            let jitter = rng.gen_range(-2.0..3.0); // Natural network jitter
            let latency_ms = (base_latency + congestion_penalty + jitter).max(0.1);

            // Packet loss: increases exponentially with utilization
            // Real networks experience packet loss when buffers overflow at high utilization
            let packet_loss_base: f64 = if utilization_pct > 90.0 {
                // Critical: severe packet loss
                rng.gen_range(2.0..5.0)
            } else if utilization_pct > 80.0 {
                // High: noticeable packet loss
                rng.gen_range(0.5..2.0)
            } else if utilization_pct > 60.0 {
                // Moderate: occasional packet loss
                rng.gen_range(0.1..0.5)
            } else {
                // Low: minimal packet loss
                rng.gen_range(0.0..0.1)
            };
            let packet_loss_pct = packet_loss_base.min(10.0);

            // Calculate bytes and packets transferred (for 1 second interval)
            let bytes_transferred = (throughput_mbps * 125000.0) as i64; // Convert Mbps to bytes/sec
            let packets_transferred = packets_per_sec;

            // Insert metric into database
            let _result = sqlx::query(
                "INSERT INTO connection_traffic_metrics
                 (connection_id, timestamp, throughput_mbps, packets_per_sec, latency_ms,
                  packet_loss_pct, utilization_pct, bytes_transferred, packets_transferred)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(connection.id)
            .bind(current_timestamp)
            .bind(throughput_mbps)
            .bind(packets_per_sec)
            .bind(latency_ms)
            .bind(packet_loss_pct)
            .bind(utilization_pct)
            .bind(bytes_transferred)
            .bind(packets_transferred)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to insert traffic metric: {}", e)))?;

            metrics_created += 1;
        }

        Ok(metrics_created)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Get latest traffic metrics for all connections in a topology
#[server(GetConnectionTrafficMetrics, "/api")]
pub async fn get_connection_traffic_metrics(topology_id: i64) -> Result<Vec<ConnectionTrafficMetric>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;
        use crate::models::ConnectionTrafficMetric;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        // Get latest metric for each connection (using subquery for max timestamp per connection)
        let metrics = sqlx::query_as::<_, ConnectionTrafficMetric>(
            "SELECT ctm.*
             FROM connection_traffic_metrics ctm
             INNER JOIN connections c ON ctm.connection_id = c.id
             INNER JOIN (
                 SELECT connection_id, MAX(timestamp) as max_timestamp
                 FROM connection_traffic_metrics
                 GROUP BY connection_id
             ) latest ON ctm.connection_id = latest.connection_id
                     AND ctm.timestamp = latest.max_timestamp
             WHERE c.topology_id = ?
             ORDER BY ctm.utilization_pct DESC"
        )
        .bind(topology_id)
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch traffic metrics: {}", e)))?;

        Ok(metrics)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Get latest traffic metric for each connection (for real-time visualization)
/// Returns a map of connection_id -> latest metric
#[server(GetLatestTrafficMetrics, "/api")]
pub async fn get_latest_traffic_metrics(topology_id: i64) -> Result<std::collections::HashMap<i64, ConnectionTrafficMetric>, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;
        use std::collections::HashMap;

        let Extension(pool): Extension<SqlitePool> = extract().await?;

        // Get latest metric for each connection in this topology
        // Uses a subquery to find the max timestamp per connection, then joins to get full row
        let metrics = sqlx::query_as::<_, ConnectionTrafficMetric>(
            r#"
            SELECT ctm.*
            FROM connection_traffic_metrics ctm
            INNER JOIN connections c ON c.id = ctm.connection_id
            INNER JOIN (
                SELECT connection_id, MAX(timestamp) as max_timestamp
                FROM connection_traffic_metrics
                WHERE connection_id IN (
                    SELECT id FROM connections WHERE topology_id = ?
                )
                GROUP BY connection_id
            ) latest ON latest.connection_id = ctm.connection_id
                    AND latest.max_timestamp = ctm.timestamp
            WHERE c.topology_id = ?
            "#
        )
        .bind(topology_id)
        .bind(topology_id)
        .fetch_all(&pool)
        .await?;

        // Convert to HashMap for easy lookup
        let mut map = HashMap::new();
        for metric in metrics {
            map.insert(metric.connection_id, metric);
        }

        Ok(map)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Clear all traffic data for a specific topology (restore manual colors)
#[server(ClearTrafficData, "/api")]
pub async fn clear_traffic_data(topology_id: i64) -> Result<usize, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool): Extension<SqlitePool> = extract().await?;

        // Delete all traffic metrics for connections in this topology
        let result = sqlx::query(
            r#"
            DELETE FROM connection_traffic_metrics
            WHERE connection_id IN (
                SELECT id FROM connections WHERE topology_id = ?
            )
            "#
        )
        .bind(topology_id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to clear traffic data: {}", e)))?;

        Ok(result.rows_affected() as usize)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}

/// Clean up old traffic metrics (keep only last 1 hour of data)
#[server(CleanOldTrafficMetrics, "/api")]
pub async fn clean_old_traffic_metrics() -> Result<usize, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract database pool: {}", e)))?;

        let one_hour_ago = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 - 3600;

        let result = sqlx::query(
            "DELETE FROM connection_traffic_metrics WHERE timestamp < ?"
        )
        .bind(one_hour_ago)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to clean old metrics: {}", e)))?;

        Ok(result.rows_affected() as usize)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}
