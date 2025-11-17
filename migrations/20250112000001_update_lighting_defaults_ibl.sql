-- Update lighting defaults for Image-Based Lighting (IBL)
-- Changes from three-point lighting to HDRI environment lighting
-- Ambient now controls environment intensity (default 1.0 matches Blender)
-- Key light provides optional shadows (default 1.0)
-- Fill and rim lights no longer used with IBL

-- Update existing ui_settings row to new IBL defaults
UPDATE ui_settings
SET
    ambient_intensity = 1.0,      -- Environment intensity (matches Blender Material Preview)
    key_light_intensity = 1.0,    -- Shadow light (subtle directional shadows)
    fill_light_intensity = 0.0,   -- No longer used (kept for compatibility)
    rim_light_intensity = 0.0     -- No longer used (kept for compatibility)
WHERE id = 1;
