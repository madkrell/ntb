use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;

/// Represents traffic metrics for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct TrafficMetric {
    pub id: i64,
    pub node_id: i64,
    pub timestamp: i64,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub packets_in: i64,
    pub packets_out: i64,
    pub packet_loss_percent: f64,
    pub cpu_usage_percent: Option<f64>,
    pub memory_usage_percent: Option<f64>,
}

/// Data transfer object for creating a new traffic metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTrafficMetric {
    pub node_id: i64,
    pub bytes_in: i64,
    pub bytes_out: i64,
    pub packets_in: i64,
    pub packets_out: i64,
    pub packet_loss_percent: f64,
    pub cpu_usage_percent: Option<f64>,
    pub memory_usage_percent: Option<f64>,
}

/// Represents traffic metrics for a connection (Phase 6)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct ConnectionTrafficMetric {
    pub id: i64,
    pub connection_id: i64,
    pub timestamp: i64,
    pub throughput_mbps: f64,
    pub packets_per_sec: i64,
    pub latency_ms: f64,
    pub packet_loss_pct: f64,
    pub utilization_pct: f64,
    pub bytes_transferred: i64,
    pub packets_transferred: i64,
}

/// Data transfer object for creating connection traffic metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConnectionTrafficMetric {
    pub connection_id: i64,
    pub throughput_mbps: f64,
    pub packets_per_sec: i64,
    pub latency_ms: f64,
    pub packet_loss_pct: f64,
    pub utilization_pct: f64,
    pub bytes_transferred: i64,
    pub packets_transferred: i64,
}
