-- Add HDR environment lighting settings to ui_settings table
-- Phase 5.7: HDR Environment Lighting (2025-01-14)

ALTER TABLE ui_settings ADD COLUMN use_environment_lighting BOOLEAN DEFAULT 0;
ALTER TABLE ui_settings ADD COLUMN environment_map TEXT DEFAULT 'studio_small_09_2k.hdr';
