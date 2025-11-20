-- Undo history table for tracking the last 5 changes to nodes and connections
-- This enables a simple undo feature without bloating the database

CREATE TABLE IF NOT EXISTS undo_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topology_id INTEGER NOT NULL,
    entity_type TEXT NOT NULL,  -- 'node' or 'connection'
    entity_id INTEGER NOT NULL,
    action_type TEXT NOT NULL,  -- 'update' or 'delete'
    previous_state TEXT NOT NULL,  -- JSON snapshot of the entity before the change
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (topology_id) REFERENCES topologies(id) ON DELETE CASCADE
);

-- Index for fast retrieval by topology
CREATE INDEX IF NOT EXISTS idx_undo_history_topology ON undo_history(topology_id, timestamp DESC);

-- Trigger to maintain only the last 5 entries per topology
CREATE TRIGGER IF NOT EXISTS trim_undo_history
AFTER INSERT ON undo_history
BEGIN
    DELETE FROM undo_history
    WHERE topology_id = NEW.topology_id
    AND id NOT IN (
        SELECT id FROM undo_history
        WHERE topology_id = NEW.topology_id
        ORDER BY timestamp DESC
        LIMIT 5
    );
END;
