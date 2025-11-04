-- Add rotation columns to nodes table
ALTER TABLE nodes ADD COLUMN rotation_x REAL NOT NULL DEFAULT 0.0;
ALTER TABLE nodes ADD COLUMN rotation_y REAL NOT NULL DEFAULT 0.0;
ALTER TABLE nodes ADD COLUMN rotation_z REAL NOT NULL DEFAULT 0.0;
