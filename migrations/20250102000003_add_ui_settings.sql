-- Add UI settings table for persisting view and lighting controls
CREATE TABLE IF NOT EXISTS ui_settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Single row table
    show_grid INTEGER NOT NULL DEFAULT 1,   -- Boolean: 1 = true, 0 = false
    show_x_axis INTEGER NOT NULL DEFAULT 1,
    show_y_axis INTEGER NOT NULL DEFAULT 1,
    show_z_axis INTEGER NOT NULL DEFAULT 1,
    ambient_intensity REAL NOT NULL DEFAULT 0.4,
    key_light_intensity REAL NOT NULL DEFAULT 1.5,
    fill_light_intensity REAL NOT NULL DEFAULT 0.6,
    rim_light_intensity REAL NOT NULL DEFAULT 0.3,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert default settings row (only if it doesn't exist)
INSERT OR IGNORE INTO ui_settings (id) VALUES (1);

-- Create trigger to update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_ui_settings_timestamp
AFTER UPDATE ON ui_settings
FOR EACH ROW
BEGIN
    UPDATE ui_settings SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;
