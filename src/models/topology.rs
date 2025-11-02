use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;

/// Represents a network topology
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Topology {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Data transfer object for creating a new topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTopology {
    pub name: String,
    pub description: Option<String>,
}

/// Data transfer object for updating a topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTopology {
    pub name: Option<String>,
    pub description: Option<String>,
}
