-- Add visibility column to nodes table
-- This allows hiding nodes in the 3D viewport (Blender-style outliner)
-- Hidden nodes and their connections won't render, but data persists

ALTER TABLE nodes ADD COLUMN visible BOOLEAN NOT NULL DEFAULT TRUE;

-- Create index for efficient visibility filtering
CREATE INDEX idx_nodes_visible ON nodes(visible);
