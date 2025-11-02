-- Rollback initial schema

DROP TRIGGER IF EXISTS update_connection_timestamp;
DROP TRIGGER IF EXISTS update_node_timestamp;
DROP TRIGGER IF EXISTS update_topology_timestamp;

DROP INDEX IF EXISTS idx_traffic_timestamp;
DROP INDEX IF EXISTS idx_traffic_node;
DROP INDEX IF EXISTS idx_connections_target;
DROP INDEX IF EXISTS idx_connections_source;
DROP INDEX IF EXISTS idx_connections_topology;
DROP INDEX IF EXISTS idx_nodes_topology;

DROP TABLE IF EXISTS traffic_metrics;
DROP TABLE IF EXISTS connections;
DROP TABLE IF EXISTS nodes;
DROP TABLE IF EXISTS topologies;
