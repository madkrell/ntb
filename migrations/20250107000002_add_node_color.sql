-- Add color column to nodes table (RGB format: "R,G,B")
-- Default: "100,150,255" (blue color)
ALTER TABLE nodes ADD COLUMN color TEXT NOT NULL DEFAULT '100,150,255';
