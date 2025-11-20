# Network Topology Builder - Development Guide

## Project Status
**Phase:** 6.4.3 COMPLETE ✅ | **Updated:** 2025-01-20 | **Tag:** v0.1.0-phase6.4.3-complete
**Architecture:** Leptos 0.8 (Regular Components) | **Database:** ntv.db (SQLite)

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

## Optional Next Steps
1. **Traffic Dashboard** (2-3h): Metrics panel, top-N connections, historical charts, CSV export
2. **WebSocket Streaming** (2-3h): Live updates via Leptos WebSocket server functions
3. **UX Polish** (1-2h): Multi-select, keyboard shortcuts, undo/redo, grouping
4. **Real Integration** (3-5h): SNMP/API connectors for production monitoring
5. **Documentation**: User guide, developer docs, Docker deployment

## Critical Architecture Patterns

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

## Database Configuration

**Active Database:** `ntv.db` (SQLite) - AUTO-CREATED on first run
**Configuration:** `DATABASE_URL=sqlite:ntv.db` (.env file)
**Tables:** topologies, nodes, connections, connection_traffic_metrics, ui_settings, traffic_metrics

**Key Migrations:**
- `20250102000002_add_node_rotations.sql` - rotation_x/y/z (degrees)
- `20250106000003_add_node_scale.sql` - scale (0.1-5.0)
- `20250107000001_add_connection_color.sql` - color "R,G,B" format
- `20250107000002_add_node_color.sql` - node color "R,G,B" format
- `20250107000003_add_vendor_model.sql` - vendor/model_name for multi-vendor
- `20250114000001_add_environment_lighting.sql` - HDR lighting settings
- `20250118000001_add_traffic_flow_controls.sql` - carries_traffic, flow_direction
- `20250119000002_add_baseline_packet_loss.sql` - baseline_packet_loss_pct

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
