-- Add last_topology_id to ui_settings to remember which topology was last viewed
ALTER TABLE ui_settings ADD COLUMN last_topology_id INTEGER DEFAULT NULL;
