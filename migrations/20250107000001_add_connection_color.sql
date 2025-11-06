-- Add color column to connections table (stored as "R,G,B" format)
ALTER TABLE connections ADD COLUMN color TEXT NOT NULL DEFAULT '128,128,128';
