# Model Validation & Troubleshooting Guide

## Quick Start

### Validate All Models
```bash
./validate_models.py
```

### Validate Single Model
```bash
./validate_models.py public/models/server/generic/server_base.glb
```

### Verbose Output (with gltf-transform inspection)
```bash
./validate_models.py -v
```

---

## What Was Fixed

### 1. ✅ Installed Validation Tools
- **gltf-transform** (v4.2.1) - Model inspection and analysis
- **gltf-validator** - glTF specification compliance checking

### 2. ✅ Created Validation Script (`validate_models.py`)

**Features:**
- Scans all models in `public/models/` directory
- Checks for missing materials, colors, and textures
- Calculates bounding boxes and validates against selection radius
- Provides color-coded output (✓ green, ⚠ yellow, ✗ red)
- Gives specific Blender fixes for each issue
- Shows summary of all models

**Script Output:**
```
📦 Materials - Lists all materials with color/texture status
📏 Bounding Boxes - Shows model dimensions and selection compatibility
❌ Errors - Critical issues preventing proper use
⚠️  Warnings - Non-critical issues (recommendations)
```

### 3. ✅ Updated Code for Auto-Calculated Bounding Boxes

**Changes in `src/islands/topology_viewport.rs`:**

1. **Added function `calculate_model_bounding_radius()`** (lines 1036-1082)
   - Calculates actual bounding box from loaded glTF model geometry
   - Accounts for primitive transformations
   - Scales by node's scale factor
   - Adds 20% margin for easier clicking (1.2x multiplier)

2. **Updated NodeData creation** (lines 1276-1293)
   - Now calculates radius dynamically based on model geometry
   - Falls back to fixed radius (0.6 units) for sphere primitives
   - Each model gets appropriate selection radius based on its actual size

**Before:**
```rust
// Fixed radius for all nodes
radius: node_radius * 2.0  // Always 0.6 units
```

**After:**
```rust
// Dynamic radius based on model
let selection_radius = if has_model {
    calculate_model_bounding_radius(cpu_model, node.scale as f32)
} else {
    node_radius * 2.0  // Fallback for spheres
};
```

---

## Common Issues Found

### Issue #1: Models Too Large for Selection
**Problem:** Model dimensions exceed the selection radius, making them unclickable

**Example:**
```
firewall_base.glb: 6.43 units (10x too big!)
server_base.glb: 4.45 units (7x too big!)
router_base.glb: 4.64 units (7x too big!)
```

**Solution:** Scale models in Blender
```
1. Open .blend file
2. Select All (A key)
3. Scale (S key) → type the recommended factor (e.g., 0.156)
4. Apply Scale: Ctrl+A → Scale
5. Re-export as .glb
```

**Validation script provides exact scale factors:**
```
Fix: In Blender: Select All (A) → Scale (S) → type 0.156 → Apply Scale (Ctrl+A)
```

### Issue #2: Materials Without Colors
**Problem:** Materials exported without `baseColorFactor` in glTF file

**Affected Models:**
- `server_base.glb` - Materials M1, M3, M8, M10, M11, M12, M15
- `firewall_base.glb` - Materials Black, Orange, F1, F2, F3, F4
- `router_base.glb` - Material M2

**Solution:** Set Base Color in Blender
```
1. Switch to Shading workspace
2. Select object with missing material
3. In Shader Editor, find Principled BSDF node
4. Click Base Color swatch → Choose a color
5. Re-export as .glb
```

**Verification:**
```bash
./validate_models.py your_model.glb
# Should show: ✓ Base Color: RGBA(...)
```

---

## Recommended Workflow

### 1. Before Exporting from Blender

**Check Materials:**
- All objects have materials assigned
- Principled BSDF has Base Color set (or texture connected)
- Material preview looks correct

**Check Scale:**
- Model should be ~0.5-1.5 units in largest dimension
- Use rulers or dimensions panel to verify

**Export Settings:**
```
Format: glTF Binary (.glb)
Include:
  ☑ Materials
  ☑ Images (if using textures)
Transform:
  ☑ +Y Up
Geometry:
  ☑ Apply Modifiers
  ☑ UVs
  ☑ Normals
  ☑ Tangents
```

### 2. After Exporting

**Run validation:**
```bash
./validate_models.py public/models/your_type/your_vendor/your_model.glb
```

**Check for:**
- ✓ Green checkmarks for all materials
- ✓ Dimensions within 0.5-1.5 units
- ✓ No errors in summary

### 3. If Issues Found

**Follow the fix instructions** provided by the script, then re-export and re-validate.

---

## Understanding Selection Radius

### How It Works

The app uses **ray-sphere intersection** for selecting nodes in the 3D viewport:

1. Mouse click → Cast ray from camera into 3D scene
2. Test ray against sphere around each node's position
3. Select closest node within intersection distance

### Previous Behavior (Fixed)
- **Fixed radius:** 0.6 units for ALL models
- Large models extended far outside this sphere → unclickable

### Current Behavior (After Fix)
- **Dynamic radius:** Calculated from actual model geometry
- Accounts for model scale factor
- 20% margin for easier clicking
- Fallback to 0.6 units for sphere primitives (backward compatible)

### Visual Representation
```
Small model (0.5 units):
   Selection sphere: ~0.3 units radius ✓

Large model (4.0 units) - OLD:
   Selection sphere: 0.6 units radius ✗ (too small!)

Large model (4.0 units) - NEW:
   Selection sphere: ~2.4 units radius ✓ (auto-calculated!)
```

---

## Validation Script Options

```bash
# Scan all models in public/models
./validate_models.py

# Validate specific file(s)
./validate_models.py path/to/model.glb

# Verbose mode (show gltf-transform output)
./validate_models.py -v

# Summary only (no per-model details)
./validate_models.py --summary-only

# Skip gltf-transform output (faster)
./validate_models.py --no-gltf-transform

# Help
./validate_models.py --help
```

---

## Current Status (Nov 22, 2024)

### Models Requiring Fixes

**Material Issues:**
- `firewall_base.glb` - 6 materials missing colors
- `server_base.glb` - 7 materials missing colors
- `router_base.glb` - 1 material missing color

**Scale Issues:**
- Most blob-* models need 0.5x scale
- `firewall_base.glb` needs 0.156x scale
- `server_base.glb` needs 0.225x scale
- `router_base.glb` needs 0.216x scale
- `Untitled.glb` (switch) needs 0.062x scale (16 units!)

**Working Models:**
- `cloud_base.glb` ✓ (0.974 units, has color)
- `cisco_router.glb` ✓ (textured, but needs scaling to 0.5x)

---

## Next Steps

1. **Fix materials in Blender** - Set Base Colors for all materials
2. **Scale models appropriately** - Use validation script's recommended factors
3. **Re-validate** - Ensure all models show ✓ green checkmarks
4. **Test in app** - Models should now be selectable and show colors

**Note:** With the code changes, even large models are now selectable, but scaling them down will improve visual consistency and performance.

---

## Troubleshooting

### Validation script not running?
```bash
chmod +x validate_models.py
python3 validate_models.py
```

### Models still not showing colors after fixing materials?
- Check that you re-exported from Blender
- Verify file was replaced in `public/models/`
- Rebuild and restart: `cargo leptos watch`
- Clear browser cache

### Models still not clickable after code changes?
- Rebuild: `cargo leptos build`
- Check browser console for errors
- Verify model loaded successfully (check Network tab)

---

## Technical Details

### Code Changes Summary

**File:** `src/islands/topology_viewport.rs`

**Function added:** `calculate_model_bounding_radius()` (line 1036)
- Iterates through all geometry primitives
- Applies transformation matrices
- Finds min/max XYZ coordinates
- Returns bounding sphere radius

**Logic updated:** Node data creation (line 1276)
- Checks if model is loaded
- Calculates dynamic radius or uses fallback
- Stores per-node radius in `NodeData`

**Result:**
- Automatic adaptation to any model size
- No hardcoded limits
- Backward compatible with sphere primitives
- Better UX with 20% selection margin
