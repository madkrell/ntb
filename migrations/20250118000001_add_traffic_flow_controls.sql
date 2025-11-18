-- Add traffic flow control fields to connections table
-- This enables users to control which connections carry traffic and in which direction

-- carries_traffic: Boolean flag to enable/disable traffic animation on this connection
-- Default TRUE so existing connections automatically show traffic when generated
ALTER TABLE connections ADD COLUMN carries_traffic BOOLEAN NOT NULL DEFAULT TRUE;

-- flow_direction: Controls the direction of traffic flow for particle animations
-- Options: 'source_to_target', 'target_to_source', 'bidirectional'
-- Default 'source_to_target' for standard unidirectional flow
ALTER TABLE connections ADD COLUMN flow_direction TEXT NOT NULL DEFAULT 'source_to_target';

-- Add check constraint to ensure only valid flow directions
-- SQLite doesn't support CHECK constraints in ALTER TABLE, so we create a trigger instead
CREATE TRIGGER check_flow_direction_insert
BEFORE INSERT ON connections
FOR EACH ROW
WHEN NEW.flow_direction NOT IN ('source_to_target', 'target_to_source', 'bidirectional')
BEGIN
    SELECT RAISE(ABORT, 'flow_direction must be one of: source_to_target, target_to_source, bidirectional');
END;

CREATE TRIGGER check_flow_direction_update
BEFORE UPDATE ON connections
FOR EACH ROW
WHEN NEW.flow_direction NOT IN ('source_to_target', 'target_to_source', 'bidirectional')
BEGIN
    SELECT RAISE(ABORT, 'flow_direction must be one of: source_to_target, target_to_source, bidirectional');
END;
