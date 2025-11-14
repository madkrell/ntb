use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;

/// Represents global UI settings for view and lighting controls
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct UISettings {
    pub id: i64,
    pub show_grid: bool,
    pub show_x_axis: bool,
    pub show_y_axis: bool,
    pub show_z_axis: bool,
    pub ambient_intensity: f64,
    pub key_light_intensity: f64,
    pub fill_light_intensity: f64,
    pub rim_light_intensity: f64,
    pub use_environment_lighting: bool,
    pub environment_map: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Data transfer object for updating UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUISettings {
    pub show_grid: Option<bool>,
    pub show_x_axis: Option<bool>,
    pub show_y_axis: Option<bool>,
    pub show_z_axis: Option<bool>,
    pub ambient_intensity: Option<f64>,
    pub key_light_intensity: Option<f64>,
    pub fill_light_intensity: Option<f64>,
    pub rim_light_intensity: Option<f64>,
    pub use_environment_lighting: Option<bool>,
    pub environment_map: Option<String>,
}
