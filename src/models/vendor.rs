use serde::{Deserialize, Serialize};

/// Represents a vendor with available models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorInfo {
    pub name: String,
    pub display_name: String,
    pub has_icon: bool,
    pub models: Vec<ModelInfo>,
    pub is_available: bool, // False if no models found
}

/// Represents a 3D model file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub file_name: String, // e.g., "blob-router.glb"
    pub display_name: String, // e.g., "Blob Router" or "ASR 9000"
}

/// Response containing all vendors for a specific node type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorListResponse {
    pub node_type: String,
    pub vendors: Vec<VendorInfo>,
}
