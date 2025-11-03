# 3D Models Directory

This directory contains 3D device models exported from Blender.

## Supported Formats

- **GLB** (Recommended): Binary glTF format, single file
- **glTF**: JSON glTF format with separate binary and texture files

## Exporting from Blender

1. In Blender, select your device model
2. File → Export → glTF 2.0 (.glb/.gltf)
3. Export settings:
   - Format: **GLB** (Binary .glb) - recommended for web
   - Include: Selected Objects
   - Transform: +Y Up (important for three-d compatibility)
   - Geometry: Apply Modifiers, UVs, Normals, Tangents
   - Materials: Export
4. Save to this directory with descriptive name:
   - router_cisco.glb
   - switch_generic.glb
   - server_rack.glb
   - firewall.glb

## Usage in Application

Models will be loaded in Phase 4 using three-d's asset loading:

```rust
use three_d::*;

// Load GLB model
let mut loaded = Loader::new()
    .load(&["models/router_cisco.glb"])?;
let model = loaded.model("router_cisco.glb")?;

// Use in scene
model.set_transformation(Mat4::from_translation(position) * Mat4::from_scale(scale));
```

## Current Status

Phase 2: Using simple sphere primitives for nodes
Phase 4: Will implement custom model loading from this directory

