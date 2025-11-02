use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;

/// Represents a connection (link) between two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Connection {
    pub id: i64,
    pub topology_id: i64,
    pub source_node_id: i64,
    pub target_node_id: i64,
    pub connection_type: String,
    pub bandwidth_mbps: Option<i64>,
    pub latency_ms: Option<f64>,
    pub status: String,
    pub metadata: Option<String>, // JSON string
    pub created_at: i64,
    pub updated_at: i64,
}

/// Data transfer object for creating a new connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnection {
    pub topology_id: i64,
    pub source_node_id: i64,
    pub target_node_id: i64,
    pub connection_type: Option<String>,
    pub bandwidth_mbps: Option<i64>,
    pub latency_ms: Option<f64>,
    pub status: Option<String>,
    pub metadata: Option<String>,
}

/// Data transfer object for updating a connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConnection {
    pub connection_type: Option<String>,
    pub bandwidth_mbps: Option<i64>,
    pub latency_ms: Option<f64>,
    pub status: Option<String>,
    pub metadata: Option<String>,
}

/// Common connection types as constants
pub mod connection_types {
    pub const ETHERNET: &str = "ethernet";
    pub const FIBER: &str = "fiber";
    pub const WIRELESS: &str = "wireless";
    pub const VPN: &str = "vpn";
    pub const WAN: &str = "wan";
}

/// Connection status constants
pub mod connection_status {
    pub const ACTIVE: &str = "active";
    pub const INACTIVE: &str = "inactive";
    pub const DEGRADED: &str = "degraded";
}
