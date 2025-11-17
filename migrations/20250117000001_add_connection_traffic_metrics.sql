-- Add connection-based traffic metrics for Phase 6
-- Phase 6.1: Traffic Monitoring (2025-01-17)

-- Connection traffic metrics table: real-time monitoring data per connection
CREATE TABLE IF NOT EXISTS connection_traffic_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    connection_id INTEGER NOT NULL,
    timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),

    -- Throughput metrics
    throughput_mbps REAL DEFAULT 0.0,          -- Current data rate in Mbps
    packets_per_sec INTEGER DEFAULT 0,         -- Packets per second

    -- Performance metrics
    latency_ms REAL DEFAULT 0.0,               -- Round-trip time in milliseconds
    packet_loss_pct REAL DEFAULT 0.0,          -- Packet loss percentage (0-100)

    -- Utilization
    utilization_pct REAL DEFAULT 0.0,          -- Bandwidth utilization (0-100)

    -- Raw data counters
    bytes_transferred INTEGER DEFAULT 0,       -- Total bytes in this interval
    packets_transferred INTEGER DEFAULT 0,     -- Total packets in this interval

    FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE CASCADE
);

-- Indexes for efficient time-range queries
CREATE INDEX IF NOT EXISTS idx_conn_traffic_connection ON connection_traffic_metrics(connection_id);
CREATE INDEX IF NOT EXISTS idx_conn_traffic_timestamp ON connection_traffic_metrics(timestamp);
CREATE INDEX IF NOT EXISTS idx_conn_traffic_conn_time ON connection_traffic_metrics(connection_id, timestamp);

-- Composite index for dashboard queries (top connections by traffic)
CREATE INDEX IF NOT EXISTS idx_conn_traffic_utilization ON connection_traffic_metrics(utilization_pct DESC, timestamp DESC);
