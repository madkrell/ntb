-- Fix coordinate system to native Blender Z-up (no Y-Z swapping)
-- This migration swaps position_y and position_z for existing nodes
-- and removes the 90° default X-rotation

-- Swap Y and Z coordinates for all existing nodes
-- (Since old code swapped them, we need to swap them back)
UPDATE nodes
SET
    position_y = position_z,
    position_z = position_y
WHERE 1=1;  -- Update all rows

-- Remove 90° X-rotation from nodes that have the default value
-- (Only affects nodes with exactly 90° rotation, preserving user customizations)
UPDATE nodes
SET rotation_x = 0.0
WHERE rotation_x = 90.0;

-- Note: After this migration:
-- - position_x = left-right (unchanged)
-- - position_y = front-back (was position_z)
-- - position_z = up-down/vertical (was position_y)
-- - rotation_x defaults to 0° (was 90°)
-- - Models exported with Blender Z-up will appear natively
