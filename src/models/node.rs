use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;

/// Represents a node (network device) in the topology
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Node {
    pub id: i64,
    pub topology_id: i64,
    pub name: String,
    pub node_type: String,
    pub ip_address: Option<String>,
    pub position_x: f64,
    pub position_y: f64,
    pub position_z: f64,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub metadata: Option<String>, // JSON string
    pub created_at: i64,
    pub updated_at: i64,
}

/// Data transfer object for creating a new node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNode {
    pub topology_id: i64,
    pub name: String,
    pub node_type: String,
    pub ip_address: Option<String>,
    pub position_x: Option<f64>,
    pub position_y: Option<f64>,
    pub position_z: Option<f64>,
    pub rotation_x: Option<f64>,
    pub rotation_y: Option<f64>,
    pub rotation_z: Option<f64>,
    pub metadata: Option<String>,
}

/// Data transfer object for updating a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNode {
    pub name: Option<String>,
    pub node_type: Option<String>,
    pub ip_address: Option<String>,
    pub position_x: Option<f64>,
    pub position_y: Option<f64>,
    pub position_z: Option<f64>,
    pub rotation_x: Option<f64>,
    pub rotation_y: Option<f64>,
    pub rotation_z: Option<f64>,
    pub metadata: Option<String>,
}

/// Common node types as constants
pub mod node_types {
    pub const HOST: &str = "host";
    pub const ROUTER: &str = "router";
    pub const SWITCH: &str = "switch";
    pub const FIREWALL: &str = "firewall";
    pub const LOAD_BALANCER: &str = "load_balancer";
    pub const SERVER: &str = "server";
    pub const CLOUD: &str = "cloud";
}
