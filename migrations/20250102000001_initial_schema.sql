-- Network Topology Visualizer - Initial Schema

-- Topologies table: stores topology metadata
CREATE TABLE IF NOT EXISTS topologies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Nodes table: network devices/hosts in the topology
CREATE TABLE IF NOT EXISTS nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topology_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    node_type TEXT NOT NULL DEFAULT 'host', -- host, router, switch, firewall, etc.
    ip_address TEXT,
    position_x REAL DEFAULT 0.0,
    position_y REAL DEFAULT 0.0,
    position_z REAL DEFAULT 0.0,
    metadata TEXT, -- JSON for additional properties
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (topology_id) REFERENCES topologies(id) ON DELETE CASCADE
);

-- Connections table: links between nodes
CREATE TABLE IF NOT EXISTS connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topology_id INTEGER NOT NULL,
    source_node_id INTEGER NOT NULL,
    target_node_id INTEGER NOT NULL,
    connection_type TEXT DEFAULT 'ethernet', -- ethernet, fiber, wireless, etc.
    bandwidth_mbps INTEGER, -- bandwidth in Mbps
    latency_ms REAL, -- latency in milliseconds
    status TEXT DEFAULT 'active', -- active, inactive, degraded
    metadata TEXT, -- JSON for additional properties
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (topology_id) REFERENCES topologies(id) ON DELETE CASCADE,
    FOREIGN KEY (source_node_id) REFERENCES nodes(id) ON DELETE CASCADE,
    FOREIGN KEY (target_node_id) REFERENCES nodes(id) ON DELETE CASCADE,
    CHECK (source_node_id != target_node_id) -- prevent self-connections
);

-- Traffic metrics table: real-time monitoring data
CREATE TABLE IF NOT EXISTS traffic_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    node_id INTEGER NOT NULL,
    timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    bytes_in INTEGER DEFAULT 0,
    bytes_out INTEGER DEFAULT 0,
    packets_in INTEGER DEFAULT 0,
    packets_out INTEGER DEFAULT 0,
    packet_loss_percent REAL DEFAULT 0.0,
    cpu_usage_percent REAL,
    memory_usage_percent REAL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

-- Indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_nodes_topology ON nodes(topology_id);
CREATE INDEX IF NOT EXISTS idx_connections_topology ON connections(topology_id);
CREATE INDEX IF NOT EXISTS idx_connections_source ON connections(source_node_id);
CREATE INDEX IF NOT EXISTS idx_connections_target ON connections(target_node_id);
CREATE INDEX IF NOT EXISTS idx_traffic_node ON traffic_metrics(node_id);
CREATE INDEX IF NOT EXISTS idx_traffic_timestamp ON traffic_metrics(timestamp);

-- Trigger to update updated_at timestamp on topologies
CREATE TRIGGER IF NOT EXISTS update_topology_timestamp
AFTER UPDATE ON topologies
BEGIN
    UPDATE topologies SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- Trigger to update updated_at timestamp on nodes
CREATE TRIGGER IF NOT EXISTS update_node_timestamp
AFTER UPDATE ON nodes
BEGIN
    UPDATE nodes SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;

-- Trigger to update updated_at timestamp on connections
CREATE TRIGGER IF NOT EXISTS update_connection_timestamp
AFTER UPDATE ON connections
BEGIN
    UPDATE connections SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;
