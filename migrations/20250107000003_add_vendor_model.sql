-- Add vendor and model_name columns to nodes table
-- These support the new vendor-based model selection system

ALTER TABLE nodes ADD COLUMN vendor TEXT NOT NULL DEFAULT 'generic';
ALTER TABLE nodes ADD COLUMN model_name TEXT NOT NULL DEFAULT 'blob-router';

-- Update existing nodes to have proper model names based on their type
UPDATE nodes SET model_name = 'blob-' || node_type WHERE model_name = 'blob-router';
