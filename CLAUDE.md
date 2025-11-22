# Network Topology Builder - Development Guide

## Project Status
**Phase:** Scene Objects Outliner ✅ | **Updated:** 2025-01-22 | **Tag:** v0.1.0-scene-objects
**Architecture:** Leptos 0.8 (Regular Components) | **Database:** ntv.db (SQLite)
**Coordinate System:** Native Blender Z-up (no Y↔Z swapping, no default rotations)

## Core Features ✅
- 3D visualization with glTF/GLB models (full PBR: textures, normals, emissive, alpha)
- CRUD operations (nodes, connections, topologies)
- Camera controls (pan, zoom, presets, fullscreen) with bounding box zoom-to-fit
- Customization (per-node colors, scale, rotation, HDR lighting environments)
- Export/Import (PNG with transparency, JSON backup/restore)
- **Traffic monitoring** (mock generator, color-coded utilization, 4-metric tooltips)
- **60fps particle animation** (flow direction, enable/disable per connection)
- **Simplified connection creation** (dropdown-based, single-click)
- Multi-vendor device models (auto-discovery from filesystem)
- **Auto-save** (all node & connection properties save automatically on change)
- **Undo** (reverses last 5 changes to nodes/connections per topology)
- **Scene Objects panel** (Blender-style outliner with visibility toggle, node selection)

## Optional Next Steps
1. **Traffic Dashboard** (2-3h): Metrics panel, top-N connections, historical charts, CSV export
2. **WebSocket Streaming** (2-3h): Live updates via Leptos WebSocket server functions
3. **UX Polish** (1-2h): Multi-select, keyboard shortcuts, undo/redo, grouping
4. **Real Integration** (3-5h): SNMP/API connectors for production monitoring
5. **Documentation**: User guide, developer docs, Docker deployment

## Critical Architecture Patterns

### Native Blender Z-Up Coordinate System (2025-01-22)
**Direct Blender-to-Viewport workflow** - No transformations, no confusion:
- **Coordinate mapping**: Direct 1:1 mapping (X→X, Y→Y, Z→Z) - no Y↔Z swapping
- **Default rotation**: None (0°, 0°, 0°) - models appear exactly as modeled in Blender
- **Scale handling**: Native Blender scale (glTF models use user scale value directly, no 0.3x multiplier)
- **World "up"**: Z-axis is vertical (matches Blender convention)
- **Grid floor**: XY plane at Z=0 (horizontal floor)
- **Blender export**: UNCHECK "+Y Up" to preserve native Z-up orientation
- **Benefits**:
  - Models exported from Blender appear identically in viewport
  - Position values match visual positions (no mental math needed)
  - Rotation controls work intuitively (X=pitch, Y=yaw, Z=roll)
  - Scale in viewport matches Blender scale (what you see is what you get)
  - Simplified workflow: Model → Export → Add → Works!

**Database migration** (migrations/20250122000001_fix_native_blender_coordinates.sql):
- Swapped existing Y↔Z coordinates for 30 nodes
- Removed 90° default X-rotation from all nodes
- Automatic on server startup

**Key files modified**:
- `src/api.rs:271` - Removed 90° default rotation
- `src/islands/topology_viewport.rs:1261-1263` - Removed Y↔Z coordinate swap
- `src/islands/topology_viewport.rs:1398` - Native scale (removed 0.3x multiplier for glTF models)
- `src/islands/topology_viewport.rs:1642,2847,2917` - Fixed connection "up" vectors to Z-up
- `src/islands/topology_viewport.rs:2848-2860` - Fixed grid/axes cylinder rotation (use Y-axis primitive default, not world Z-up)
- `src/islands/topology_editor.rs:1608-1611` - New nodes default to origin (0,0,0)
- `src/islands/topology_editor.rs:2199-2201` - Removed Y↔Z swap in node updates

**Primitive rotation vs world coordinates**:
- three-d cylinders/boxes default to Y-axis orientation
- Grid/axes rotation uses primitive's native Y-axis, NOT world Z-up
- This is independent of the world coordinate system

**Model scale recommendations**:
- **Ideal range**: 0.3-1.5 units (bounding box max dimension)
- **Maximum recommended**: 2.0 units
- **Selection**: Auto-calculated from model geometry (any size works!)
- **Validation tool**: `./validate_models.py` checks materials, bounds, provides scaling recommendations
- Models can be scaled in Blender (Apply Transforms before export) for optimal viewport appearance

### Material Rendering (Phase 5.6)
**Two-path system** (topology_viewport.rs:940-975):
- **Textured materials**: `PhysicalMaterial::new(&context, gltf_mat)` - no color conversion
- **Color-only**: Apply sRGB conversion (`linear_to_srgb()` at lines 633-657) to fix three-d bug
- **Result**: Blender color match + full PBR support (albedo, metallic/roughness, normal, occlusion, emissive, alpha)

### HDR Lighting (Phase 5.7)
**Conditional lighting with dynamic signal reading**:
- **HDR mode**: Use ONLY ambient light (contains environment map from `public/environments/*.hdr`)
- **Manual mode**: Use all 4 lights (ambient + key + fill + rim)
- **Why**: HDR + directional lights = overexposure/washout
- **Settings**: Persist to database, signals passed to render closure for real-time updates
- **Dynamic updates**: Lights recreated every frame based on current signal values
- **Environment map changes**: Trigger full reinit to load new HDR file

### Traffic System (Phase 6)
**Realistic simulation** (api.rs:1011-1124):
- **Traffic generation**: 3 intensity levels (Low 10-30%, Medium 30-70%, High 70-95%)
- **Utilization coloring**: Green (0-40%), Orange (40-70%), Red (70-100%)
- **Congestion modeling**:
  - Latency: Base + penalties (0.1-0.3x per % >40%, 0.5-1.5x per % >70%)
  - Packet loss: Exponential (0-0.1% healthy, 2-5% >90% util)
  - Utilization: `base + latency_penalty + packet_loss_penalty` (degraded links show red)
- **Tooltips**: Utilization, throughput, latency, packet loss (all color-coded)

### Particle Animation (Phase 6.4.2)
**Global state synchronization** (topology_viewport.rs:51-66):
```rust
static GLOBAL_PARTICLES: Mutex<Vec<TrafficParticle>> = Mutex::new(Vec::new());
static ANIMATION_RUNNING: Mutex<bool> = Mutex::new(false);
static ANIMATION_LOOP_ID: Mutex<u32> = Mutex::new(0);  // Prevents multiple loops
```
- **Loop ID pattern**: Increment on start, old loops check and exit if ID changed
- **60fps**: `requestAnimationFrame()` with delta time for frame-independent movement
- **Rendering**: Small spheres (radius 0.08) with emissive glow, position interpolated (0.0-1.0)
- **Density**: Utilization-based (1-3 low, 3-7 med, 7-12 high)

### Connection Creation (Phase 6.4.3)
**Simplified dropdown-based approach** (topology_editor.rs:2132-2553):
- **Old approach (removed)**: Three-state mode (Disabled/SelectingFirst/SelectingSecond) with click-based selection
- **New approach**: Dropdown in Node Properties Panel showing all available target nodes
- **Benefits**: Simpler, more reliable, not dependent on 3D viewport event handlers
- **How it works**:
  1. Select source node in viewport
  2. Properties Panel shows "Create Connection" section
  3. Dropdown lists all other nodes (excludes current node)
  4. Select target → Click "Create Connection" button
  5. Connection created with default properties (ethernet, 1000 Mbps, 1.0ms latency)
- **Preserved features**: All connection properties, traffic visualization, particle animation

## Key Lessons Learned

### Canvas & WebGL
- **Canvas resize**: Always update dimensions on EVERY render (not just init)
  ```rust
  let width = canvas.client_width() as u32;
  canvas.set_width(width);
  canvas.set_height(height);
  ```
- **Bounding box zoom**: Dynamic calculation from node positions (not fixed distance)
  - `distance = (max_dimension * 1.1) / (fov_radians / 2.0).tan()`

### Leptos Patterns
- **Server functions**: Module NOT behind `#[cfg(feature = "ssr")]` for visibility
- **Reactive signals**: Use `.get()` in Effects for reactivity, `.get_untracked()` in render loops
- **Dynamic render updates**: Pass signals to render closures, read with `.get_untracked()` each frame
- **Visual settings reactivity**: Capture signals (not values) for real-time lighting/background changes
- **Fullscreen toggle**: Single `RwSignal<bool>` better than separate panel signals (avoids collision)
- **Event handlers**: Use `Arc<Mutex<>>` snapshot with `.forget()` to prevent disposed signal panics
- **Context collision**: Wrap same-typed signals in unique struct

### three-d Library
- **Color space**: glTF linear RGB → sRGB conversion needed for color-only materials
- **Emissive glow**: Render with empty lights array `target.render(&camera, mesh, &[])`
- **Solid meshes**: Use `CpuMesh::cube()`, not cylinder (hollow tubes without end caps)
- **Lighting conflict**: HDR environment provides full illumination (disable directional lights)

### Animation & State
- **Global vs local**: `static Mutex<T>` for cross-closure sync, not `Rc<RefCell<>>`
- **Loop ID counter**: Prevents multiple `requestAnimationFrame` loops running simultaneously
- **Timing**: Animation setup outside `skip_event_handlers` to run on EVERY Effect execution

### Traffic Semantics
- **Throughput vs Utilization**: Separate calculations (throughput ↓ with degradation, utilization ↑)
- **Realistic modeling**: Base latency + congestion penalties, exponential packet loss
- **Database storage**: Separate `connection_traffic_metrics` table (not in connections)

### Visual Controls Reactivity (Phase 6.4.3)
- **Problem**: Visual settings (lighting, background, grid) captured as static values at init
- **Solution**: Pass signals to render closure, read dynamically with `.get_untracked()` each frame
- **Pattern**: Capture `RwSignal<T>` (not `T`), recreate objects (lights, colors) on every frame
- **Grid/Axes**: Always create all meshes at init, control visibility purely at render time
- **HDR changes**: Environment map changes trigger reinit (load new HDR), toggle just re-renders
- **Result**: All visual controls update in real-time without viewport reinitialization

### Auto-Save & Undo System (Phase 6.4.4)
**Auto-save pattern** (topology_editor.rs:2118-2139, 2700-2716):
- **Loading flag**: `node_loaded` / `connection_loaded` signal prevents auto-save during initial data load
- **Effect pattern**: Track ALL editable fields with `.get()` in Effect, trigger save when any changes
- **Benefits**: Removes explicit Save button requirement, more intuitive UX like modern apps
- **Example**:
  ```rust
  let node_loaded = RwSignal::new(false);
  Effect::new(move || {
      // Track all fields
      let _name = name.get();
      let _color = color.get();
      // ... all other fields

      if node_loaded.get() {  // Only after initial load
          save_action.dispatch(());
      }
  });
  ```

**Undo system** (api.rs:1488-1696, migrations/20250120000001_add_undo_history.sql):
- **Database table**: `undo_history` stores JSON snapshots of entity state before updates
- **Trigger**: SQLite trigger auto-maintains last 5 entries per topology (prevents bloat)
- **Helpers**: `save_node_undo_history()` / `save_connection_undo_history()` called before updates
- **Restore**: `undo_last_change()` deserializes JSON and restores previous state, then deletes entry
- **UI**: Undo button in toolbar, disabled when no history available, triggers viewport refresh
- **Scope**: Per-topology (undo only affects current topology)

### Scene Objects Panel (2025-01-22)
**Blender-style outliner** (topology_editor.rs:1734-1809):
- **Location**: Left sidebar, above Traffic Monitoring section
- **Features**:
  - Lists all nodes in current topology
  - Eye icon (👁) to show / filled circle (⚫) to hide nodes
  - Click node name to select → highlights in viewport → shows properties panel
  - Selected node highlighted in blue
  - Scrollable list (max-h-48) for many nodes
- **Visibility toggle** (topology_editor.rs:1776-1800):
  - Persisted to database (`nodes.visible` column)
  - Hidden nodes don't render in viewport
  - Connections to/from hidden nodes auto-hide
  - Uses `refetch_trigger.update()` for reactive UI updates
- **Selection integration**:
  - Sets `selected_item` signal when node clicked
  - Viewport checks both `selected_node_id` and `selected_item` for highlighting
  - Compatible with both outliner and viewport selection methods
- **Database**: `visible BOOLEAN NOT NULL DEFAULT TRUE` (migrations/20250122000002_add_node_visibility.sql)
- **Viewport filtering** (topology_viewport.rs:1258-1261):
  ```rust
  // Skip invisible nodes (Blender-style outliner)
  if !node.visible {
      continue;
  }
  ```
- **Connection filtering**: Automatic via `node_positions` HashMap (only contains visible nodes)

## Database Configuration

**Active Database:** `ntv.db` (SQLite) - AUTO-CREATED on first run
**Configuration:** `DATABASE_URL=sqlite:ntv.db` (.env file)
**Tables:** topologies, nodes, connections, connection_traffic_metrics, ui_settings, traffic_metrics, undo_history

**Key Migrations:**
- `20250102000002_add_node_rotations.sql` - rotation_x/y/z (degrees)
- `20250106000003_add_node_scale.sql` - scale (0.1-5.0)
- `20250107000001_add_connection_color.sql` - color "R,G,B" format
- `20250107000002_add_node_color.sql` - node color "R,G,B" format
- `20250107000003_add_vendor_model.sql` - vendor/model_name for multi-vendor
- `20250114000001_add_environment_lighting.sql` - HDR lighting settings
- `20250118000001_add_traffic_flow_controls.sql` - carries_traffic, flow_direction
- `20250119000002_add_baseline_packet_loss.sql` - baseline_packet_loss_pct
- `20250120000001_add_undo_history.sql` - undo_history table with auto-trim trigger (last 5)
- `20250122000001_fix_native_blender_coordinates.sql` - **CRITICAL** Native Z-up coordinates (swaps Y↔Z, removes 90° rotation)
- `20250122000002_add_node_visibility.sql` - nodes.visible column for Scene Objects outliner

**⚠️ Historical Note:** Database kept as `ntv.db` during "ntv→ntb" rename to preserve data.

## Vendor Model System (Phase 5.5)

**Auto-discovery** from filesystem:
```
public/
├── models/{type}/{vendor}/*.glb  → Auto-discovered models
└── icons/vendors/{vendor}.svg     → Auto-discovered icons
```

**Adding new vendor:**
1. `mkdir -p public/models/router/cisco`
2. `cp model.glb public/models/router/cisco/asr9000.glb`
3. `cp logo.svg public/icons/vendors/cisco.svg`
4. Refresh browser → Cisco appears in Routers dropdown

**Server function:** `get_vendors_for_type()` scans filesystem, formats display names (blob-router → Blob Router)

**Blender Export Settings (CRITICAL):**
```
Format: glTF Binary (.glb)

Transform:
  ☐ +Y Up  ← MUST BE UNCHECKED! (Preserves native Z-up)

Geometry:
  ☑ Apply Modifiers
  ☑ UVs, Normals, Tangents

Materials:
  ☑ Materials
  ☑ Images (if using textures)
```

**Before exporting:**
1. Select All (A)
2. Apply All Transforms (Ctrl+A → All Transforms) - **CRITICAL!**
3. Export with +Y Up **UNCHECKED**

## Build Commands

### Development (Run BOTH in separate terminals):
```bash
./tailwindcss -i style/input.css -o style/output.css --watch
cargo leptos watch
```

### Production:
```bash
./tailwindcss -i style/input.css -o style/output.css --minify
cargo leptos build --release
```

## Git Repository
**URL:** https://github.com/madkrell/ntb.git
**Branch:** main
**Latest:** v0.1.0-phase6.4.2-complete ✅

## Architecture Notes

### Leptos Configuration
- **Version:** 0.8 (NO islands feature - fully interactive app)
- **Hydration:** Standard `leptos::mount::hydrate_body(app::App)`
- **Crate types:** `["cdylib", "rlib"]` for WASM support
- **Config location:** `Cargo.toml` (NO Leptos.toml required)

### Color Storage
- **Format:** "R,G,B" text in database (human-readable, easy parsing)
- **HTML5 picker:** Hex ↔ RGB conversion
  ```rust
  // DB → Display
  let parts: Vec<&str> = color.split(',').collect();
  Srgba::new(r, g, b, 255)

  // Picker → DB
  format!("#{:02x}{:02x}{:02x}", r, g, b)
  ```

### Camera State
```rust
struct CameraState {
    distance: f32,    // Zoom level
    azimuth: f32,     // Horizontal rotation
    elevation: f32,   // Vertical rotation
    pan_x: f32,       // Horizontal pan offset
    pan_y: f32,       // Vertical pan offset
}
```
**Controls:** Middle-click or Shift+drag to pan, pan target becomes camera look-at point

## Common Issues & Solutions

1. **Server Functions** → Use `leptos_axum::extract()` for database access
2. **Event Handlers** → Use mutable storage `Rc<RefCell<>>` for data access
3. **Disposed Signals** → Use `Arc<Mutex<>>` snapshot with `.forget()`
4. **Context Collision** → Wrap same-typed signals in unique struct
5. **Canvas Resize** → Always update dimensions on every render
6. **Bounding Box** → Calculate from actual node positions, not fixed values
7. **Latency Persistence** → Use `.filter(|&v| v >= 0.0)` not `> 0.0` to allow 0ms
8. **Animation Speed** → Loop ID counter prevents multiple animation loops
9. **Visual Controls Not Working** → Pass signals to render closure, read dynamically each frame
10. **Grid/Axes Visibility** → Always create all meshes, control visibility at render time
11. **HDR Environment Changes** → Trigger reinit when environment map changes (load new HDR)
12. **Node Properties Y↔Z Swap** → Must use direct 1:1 mapping (no coordinate swapping in load/save)
13. **Reactive Tracking in Async** → Use `.update()` instead of `.set(signal.get())` in spawn_local blocks
