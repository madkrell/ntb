-- Add scale column to nodes table
ALTER TABLE nodes ADD COLUMN scale REAL NOT NULL DEFAULT 1.0;
