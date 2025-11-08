# Network Topology Visualizer - Claude Development Notes

## Project Status
**Current Phase:** Phase 5.5 COMPLETE! ✅ (Vendor-Based Model Selection)
**Last Updated:** 2025-11-08
**Git Tags:** v0.1.0-phase1-complete, v0.1.0-phase2-complete, v0.1.0-phase3-complete, v0.1.0-phase4-complete, v0.1.0-phase5-complete
**Architecture:** Regular Leptos Components (Islands removed - see notes below)
**Next Phase:** Phase 6 - Traffic Monitoring (Real-time visualization with WebSocket streaming)

### Phase 5.5 - Vendor-Based Model Selection COMPLETE! ✅ (2025-11-08)

**✅ COMPLETED:**
27. ✅ **Vendor-Based Device Palette** (2025-11-08) - Multi-vendor model selection system
   - Database migration: `20250107000003_add_vendor_model.sql`
   - Added `vendor` and `model_name` fields to Node model
   - Device Palette buttons now plural: "Routers", "Switches", "Servers", etc.
   - Added new "Applications" device type
   - Click any device button to show vendor dropdown
   - Auto-discovers vendors from filesystem: `public/models/{type}/{vendor}/*.glb`
   - Auto-discovers vendor icons: `public/icons/vendors/{vendor}.svg`
   - Generic vendor always shown first as fallback
   - Component extraction pattern for clean Leptos closure handling
   - Dynamic z-index layering prevents dropdown overlap issues

28. ✅ **Vendor Auto-Discovery Server Function** (2025-11-08)
   - `get_vendors_for_type()` scans filesystem for vendor folders
   - Returns VendorListResponse with vendors and their models
   - Model display names auto-formatted from filenames (blob-router → Blob Router)
   - Icon detection with fallback to generic.svg
   - Sorted: Generic first, then available vendors, then unavailable (no models)

**File Structure:**
```
public/
├── models/
│   ├── router/
│   │   ├── generic/
│   │   │   └── blob-router.glb
│   │   ├── cisco/
│   │   │   ├── asr9000.glb
│   │   │   └── catalyst.glb
│   │   └── versa/
│   │       └── sd-wan.glb
│   ├── switch/
│   │   ├── generic/
│   │   │   └── blob-switch.glb
│   │   └── cisco/
│   │       └── nexus.glb
│   └── application/
│       ├── generic/
│       │   └── blob-application.glb
│       └── cisco/
│           └── webex.glb
└── icons/
    └── vendors/
        ├── generic.svg
        ├── cisco.svg
        └── versa.svg
```

**Adding New Vendors (Zero Configuration):**
1. Create vendor folder: `mkdir -p public/models/router/cisco`
2. Add models: `cp model.glb public/models/router/cisco/asr9000.glb`
3. Add icon: `cp logo.svg public/icons/vendors/cisco.svg`
4. Refresh browser → Cisco automatically appears in Routers dropdown!

### Phase 4.5 - UI/UX Polish COMPLETE! ✅ (2025-11-07)

**✅ COMPLETED (Latest Session - Critical Fixes):**
21. ✅ **Fullscreen Toggle** (2025-11-07) - Single button to hide both panels
   - Replaced two separate panel toggles with unified fullscreen mode
   - F key toggles fullscreen mode on/off
   - Escape key hierarchy: Exit fullscreen first, then deselect
   - RwSignal<bool> for fullscreen_mode state via context
   - Layout conditionally renders panels based on fullscreen state

22. ✅ **Camera Pan Controls** (2025-11-07) - Pan viewport separately from rotation
   - Added pan_x and pan_y to CameraState struct
   - Middle-mouse button OR Shift+drag to pan
   - Pan speed scales with camera distance for intuitive feel
   - Pan target becomes camera look-at point for centered view
   - Fixes topology shifting when rotating/zooming in fullscreen

23. ✅ **Viewport Centering Fix** (2025-11-07) - Topology stays centered on resize
   - Root cause: Canvas resize didn't update viewport/projection matrix
   - Solution: Always query canvas dimensions and update resolution on every render
   - ```rust
     let width = canvas.client_width() as u32;
     let height = canvas.client_height() as u32;
     canvas.set_width(width);
     canvas.set_height(height);
     ```
   - Fixes: Fullscreen toggle, window resize, panel visibility changes

24. ✅ **Zoom to Fit with Bounding Box** (2025-11-07) - Proper topology fitting
   - Replaced fixed distance (20.0) with dynamic bounding box calculation
   - Algorithm:
     - Iterate all nodes to find min/max X/Y/Z coordinates
     - Calculate bounding box dimensions
     - Determine camera distance using FOV math: `distance = (size / 2) / tan(FOV / 2)`
     - Center camera on bounding box center (pan offset)
     - 10% margin factor for visual padding
   - Special handling in camera preset Effect (accesses node data storage)

25. ✅ **Node Color Customization** (2025-11-07) - Full color control per node
   - Database migration: `20250107000002_add_node_color.sql`
   - Added `color: String` field to Node model ("R,G,B" format, default "100,150,255")
   - Properties panel color picker UI:
     - 13 preset color buttons (Blue, Orange, Green, Red, Purple, Gray, Light Blue, Bright Orange, etc.)
     - HTML5 color picker with bidirectional hex↔RGB conversion
     - Current color displayed as RGB text
   - Viewport rendering updated to parse and apply custom node colors
   - Fallback to type-based colors if parse fails

26. ✅ **Cloud Node Type** (2025-11-07) - Added missing device type
   - Added "Cloud" option to Properties Panel node type dropdown
   - Positioned between "Load Balancer" and "Database"
   - Matches glTF/GLB model loading (blob-cloud.glb)

### Phase 4 - COMPLETE! ✅

**✅ COMPLETED (Priority 1 - Core 3D Features):**
1. ✅ **3D node rotation controls** - Full X/Y/Z rotation with database storage, UI sliders, and viewport rendering
2. ✅ **Model Selection UI** - Loads correct glTF/GLB model for each node type (router, switch, server, firewall, load_balancer, cloud)
3. ✅ **3D Grid and Axes** - Blender-style reference grid with X/Y/Z axis lines and grid floor plane
4. ✅ **Topology switching control** - Multiple topologies with dropdown selector in UI
5. ✅ **Enable Device Palette buttons** - All 6 device types ('Router', 'Switch', 'Server', 'Firewall', 'Load Balancer', 'Cloud') create nodes with grid positioning
6. ✅ **Grid/Axes visibility controls** (2025-11-05) - Toggle buttons to show/hide grid and individual axes
   - ViewportVisibility struct pattern prevents context collision for same-typed signals
   - Independent toggles for Grid Floor, X Axis (Red), Y Axis (Green), Z Axis (Blue)
   - Z-axis extremely transparent (alpha=25), all axes thinned to 0.006
7. ✅ **Connection creation mode** (2025-11-05) - Click two nodes to create connection between them
   - "Connect Nodes" button with visual feedback (button color changes)
   - Three-state mode: Disabled → SelectingFirstNode → SelectingSecondNode
   - Creates connections via create_connection() server function
   - Deselects on second node click to trigger viewport refresh

**✅ COMPLETED (Priority 2 - Visual Polish):**
8. ✅ **Node Labels/Tooltips** - Show node name on hover in 3D viewport
9. ✅ **Color-Coded Nodes by Type** - Router=blue, Switch=green, Server=orange, etc. (now overridden by custom colors)
10. ✅ **Connection rendering improvements** (2025-11-05) - Thin cylindrical lines (0.012 thickness) using ColorMaterial
11. ✅ **Connection selection** (2025-11-05) - Click to select connections in viewport
    - Ray-cylinder intersection algorithm for accurate 3D picking
    - Visual feedback with yellow/orange highlighting for selected connections
    - Properties panel shows connection details (type, bandwidth, status)
    - Critical fix: Mutable storage pattern for event handlers to access fresh data
12. ✅ **Improved Lighting and Materials** (2025-11-06) - Professional three-point lighting system with PBR materials
    - Key light (warm, from above-front), Fill light (cool, from side), Rim light (subtle, from behind)
    - User-adjustable lighting controls with 4 intensity sliders (Ambient, Key, Fill, Rim)
    - PBR materials with metallic/roughness properties varying by device type
    - Metallic nodes (router, firewall) vs matte nodes (server, client)
13. ✅ **Better Camera Controls** (2025-11-06) - Preset views with smooth animations
    - 4 camera presets: Top, Front, Side, Isometric
    - Smooth lerp animation with ease-in-out easing (600ms transitions)
    - Reset button to return to default isometric view
    - Compact viewport overlay controls (2×2 grid, top-right corner)
    - Camera state sync enables dragging from preset positions

**✅ COMPLETED (Phase 4 Additions - UI/UX Polish):**
14. ✅ **UI Space Optimization** (2025-11-06) - Maximized viewport space
    - Device Palette narrowed to 75% (256px → 192px)
    - Properties Panel narrowed to 75% (320px → 240px)
    - Position/rotation controls made compact (smaller text, reduced padding)
    - View Controls color-coded (X=red, Y=green, Z=blue)
    - Camera controls moved to viewport overlay
15. ✅ **Settings Persistence** (2025-11-06) - UI state survives page refresh/restart
    - Database table: ui_settings (single row, id=1)
    - Persists all View Controls (show_grid, show_x/y/z_axis)
    - Persists all Lighting Controls (ambient, key, fill, rim intensities)
    - Auto-save on any control change
    - Auto-load on application startup
16. ✅ **Code Quality** (2025-11-06) - Clean, warning-free codebase
    - All compiler warnings fixed
    - Clippy-clean code
    - Proper #[allow(unused_variables)] for false positives in reactive closures
17. ✅ **PNG Export Functionality** (2025-11-06) - High-quality image export with transparency
    - Export dropdown menu in toolbar with PNG/JSON options
    - WebGL2 context with preserveDrawingBuffer enabled for frame capture
    - Transparent background support for clean exports
    - Fixed dropdown z-index for proper overlay visibility
18. ✅ **Node Scale Control** (2025-11-06) - Per-node size adjustment
    - Database migration: `20250106000003_add_node_scale.sql`
    - Added `scale: f64` field to Node model (default 1.0, range 0.1-5.0)
    - Properties panel slider for scale adjustment
    - Real-time viewport rendering with scale transformation
    - Scale applied to both 3D models and fallback spheres
19. ✅ **Background Color Control** (2025-11-06) - Customizable viewport background
    - Extended ViewportVisibility struct with background_color field
    - 6 preset buttons: Transparent, White, Light, Gray, Dark, Black
    - Transparent option (None) for PNG exports showing only topology
    - Black default background (rgb(0,0,0))
    - Real-time viewport updates via refetch_trigger
    - ClearState implementation with alpha channel support
20. ✅ **Connection Color Control** (2025-11-06) - Customizable link colors
    - Database migration: `20250107000001_add_connection_color.sql`
    - Added `color: String` field to Connection model ("R,G,B" format)
    - Properties panel with 13 preset colors (Gray, Black, White, Blue, Green, Yellow, Red, Purple, Pink, Orange, Cyan, Lime, Amber)
    - Full color palette picker with HTML5 color input
    - Bidirectional hex↔RGB conversion for user-friendly color selection
    - Real-time color rendering in 3D viewport
    - Current color displayed as RGB text (e.g., "128,128,128")

### Phase 3 - COMPLETE ✅
- ✅ Professional 3-panel layout (device palette, viewport, properties)
- ✅ Node selection via 3D raycasting with visual feedback (yellow highlight)
- ✅ Click empty space to deselect
- ✅ Properties panel loads and displays actual node/connection data
- ✅ Full CRUD server functions for nodes and connections
- ✅ Save changes from properties panel with real-time viewport updates
- ✅ Suspense components for proper loading states (no hydration warnings)
- ✅ Context-based state sharing across components

## Key Lessons Learned (Phase 4.5 Session)

### 1. Canvas Resize and Viewport Updates
**Issue:** Topology shifts when toggling fullscreen or resizing window
**Root Cause:** Canvas element resizes but WebGL viewport and projection matrix don't update
**Solution:** Always update canvas resolution on every render
```rust
// In render function - always get current dimensions
let width = canvas.client_width() as u32;
let height = canvas.client_height() as u32;
canvas.set_width(width);
canvas.set_height(height);
let viewport = Viewport::new_at_origo(width, height);
```
**Why:** Ensures viewport and projection matrix always match actual canvas size, preventing distortion and off-center rendering

### 2. Bounding Box Calculation for Smart Zoom
**Pattern:** Dynamic camera positioning based on scene contents
**Implementation:**
```rust
// Iterate nodes to find bounds
let mut min_x/max_x/min_y/max_y/min_z/max_z = ...;
for node in nodes { /* update bounds */ }

// Calculate dimensions and center
let width = max_x - min_x;
let center_x = (min_x + max_x) / 2.0;

// Apply margin and calculate distance
let margin_factor = 1.1;  // 10% margin
let max_dimension = width.max(height).max(depth) * margin_factor;
let distance = (max_dimension / 2.0) / (fov_radians / 2.0).tan();
```
**Result:** Camera automatically positions to fit entire topology with consistent margin

### 3. Fullscreen Toggle Pattern
**Anti-pattern:** Two separate signals for left/right panel visibility
- Leads to state inconsistency
- Multiple signals of same type in context collide
- Complex to manage

**Better pattern:** Single fullscreen mode signal
```rust
let fullscreen_mode = RwSignal::new(false);
provide_context(fullscreen_mode);

// Layout conditionally renders both panels
{move || {
    if !fullscreen_mode.get() {
        Some(view! { <DevicePalette /> })
    } else {
        None
    }
}}
```
**Benefits:** Simpler state, single source of truth, keyboard-friendly (F to toggle, Esc to exit)

### 4. RGB Color Storage Format
**Design Decision:** Store colors as "R,G,B" text in database
**Rationale:**
- Human-readable in database queries
- Easy to parse and validate
- Clear separation between components
- Flexible for different color spaces

**Conversion Pattern:**
```rust
// Parse from database
let parts: Vec<&str> = node.color.split(',').collect();
if parts.len() == 3 {
    if let (Ok(r), Ok(g), Ok(b)) = (
        parts[0].parse::<u8>(),
        parts[1].parse::<u8>(),
        parts[2].parse::<u8>(),
    ) {
        Srgba::new(r, g, b, 255)
    }
}

// Hex for HTML5 color picker
format!("#{:02x}{:02x}{:02x}", r, g, b)

// Hex back to RGB
let r = u8::from_str_radix(&hex[1..3], 16)?;
```

### 5. Camera Pan State Management
**Pattern:** Pan offset changes camera look-at target
```rust
struct CameraState {
    distance: f32,
    azimuth: f32,
    elevation: f32,
    pan_x: f32,    // NEW: horizontal pan
    pan_y: f32,    // NEW: vertical pan
}

// Camera calculation
let target = vec3(state.pan_x, state.pan_y, 0.0);
let eye = target + vec3(/* orbit offset from target */);
let camera = Camera::new_perspective(viewport, eye, target, up, ...);
```
**Result:** Natural camera behavior - pan moves the view center, rotation/zoom orbit around that center

## ✅ VERIFIED Configuration (from Leptos 0.7/0.8 docs)

### Important: NO Leptos.toml Required!
Modern Leptos projects use `cargo-leptos` and configure everything in `Cargo.toml`.
The original plan referenced Leptos.toml which is NOT standard.

### ⚠️ ARCHITECTURE CHANGE: Islands Removed (2025-11-03)

**Decision:** Removed Leptos islands architecture in favor of regular components.

**Reason:** Islands are designed for content-heavy sites with sparse interactivity. Our Network Topology Visualizer is a fully interactive application where most of the interface needs to respond to user input. Regular Leptos components with standard hydration provide better context sharing and are more appropriate for highly interactive apps.

### Current Leptos Configuration (Cargo.toml)
```toml
[dependencies]
leptos = { version = "0.8" }  # NO islands feature
leptos_meta = { version = "0.8" }
leptos_router = { version = "0.8" }
leptos_axum = { version = "0.8", optional = true }

[features]
hydrate = [
    "leptos/hydrate",
    # NO "leptos/islands"
    "dep:console_error_panic_hook",
    "dep:wasm-bindgen",
    "dep:web-sys",
    "dep:three-d",
]
ssr = [
    "leptos/ssr",
    # NO "leptos/islands"
    "leptos_meta/ssr",
    "leptos_router/ssr",
    "dep:axum",
    "dep:leptos_axum",
    # ... other deps
]

[lib]
crate-type = ["cdylib", "rlib"]  # cdylib required for WASM
```

### Hydration Setup (Current)
```rust
// In shell function (app.rs)
<HydrationScripts options/>  // NO islands=true

// In lib.rs hydrate entry point
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(app::App);  // Standard hydration
}
```

## Server Functions Architecture (CRITICAL DISCOVERY)

**Issue:** Server functions need to be accessible from both client and server, but `#[cfg(feature = "ssr")]` gates module visibility.

**Solution:** Create a **non-feature-gated module** for server functions:

```rust
// src/lib.rs
pub mod app;
pub mod islands;
pub mod models;
pub mod api;  // ✅ NOT behind #[cfg(feature = "ssr")]

#[cfg(feature = "ssr")]
pub mod server;  // Old implementation-specific code
```

**Why this works:**
- `#[server]` macro generates client-side stub when `ssr` feature is off
- Non-feature-gated module makes function signature visible to client
- SSR-specific implementation is conditionally compiled
- Leptos handles the HTTP request/response serialization

## Database Migrations

### Phase 4 Migrations
- `20250102000002_add_node_rotations.sql` - rotation_x/y/z (REAL, default 0.0, stored in degrees)
- `20250106000003_add_node_scale.sql` - scale (REAL, default 1.0, range 0.1-5.0)
- `20250107000001_add_connection_color.sql` - color (TEXT, default '128,128,128', format "R,G,B")
- `20250107000002_add_node_color.sql` - color (TEXT, default '100,150,255', format "R,G,B")

### Phase 5.5 Migrations
- `20250107000003_add_vendor_model.sql` - vendor (TEXT, default 'generic') and model_name (TEXT, default 'blob-{type}') for multi-vendor support

## Phase 5 - Export & JSON Import/Export ✅ COMPLETE!

### ✅ Completed Features
1. ✅ **Export topology as PNG image** - COMPLETE! (Phase 4, item 17)
   - Export dropdown menu in toolbar with PNG/JSON options
   - WebGL2 context with preserveDrawingBuffer enabled
   - Transparent background support for clean exports
   - canvas.toDataURL() for high-quality image capture

2. ✅ **Export topology as JSON** - COMPLETE! (topology_editor.rs:840-931)
   - Full topology data export (nodes, connections, all properties)
   - Pretty-formatted JSON with serde_json
   - Automatic file download with timestamp: `topology-{name}-{timestamp}.json`
   - Blob API for client-side file generation
   - Preserves all node properties (position, rotation, scale, color)
   - Preserves all connection properties (type, bandwidth, color, status)

3. ✅ **Import topology from JSON** - COMPLETE! (topology_editor.rs:933-1274)
   - File picker UI with drag-and-drop support
   - JSON validation and parsing
   - Creates new topology with imported data
   - Batch node creation via server function
   - Batch connection creation with proper node ID mapping
   - Error handling with user-friendly messages
   - Success notification with new topology name
   - Automatic switch to newly imported topology

**Implementation Details:**
```rust
// Export: Fetches topology data and creates downloadable JSON file
async fn export_topology_json(topology_id: i64) {
    let topology_data = get_topology_full(topology_id).await?;
    let json_string = serde_json::to_string_pretty(&topology_data)?;
    // Create blob and trigger download...
}

// Import: Parses JSON file and recreates topology
async fn import_topology_json(json_content: String) -> Result<ImportResult, String> {
    let imported: TopologyFull = serde_json::from_str(&json_content)?;
    let new_topology_id = create_topology(CreateTopology {
        name: format!("{} (Imported)", imported.topology.name),
        description: imported.topology.description
    }).await?;
    // Batch create nodes and connections...
}
```

**UI Integration:**
- Export dropdown in toolbar: "Export PNG" and "Export JSON" options
- Import button in toolbar: Opens file picker dialog
- File validation: Checks JSON structure before import
- Progress indication during import process
- Error messages displayed to user if import fails

**Remaining Phase 5 Items (Optional enhancements):**
4. ⏳ UI polish and optimizations - Loading states, error handling (mostly done)
5. ⏳ Documentation - User guide with screenshots (can be done anytime)

## Phase 6 - Traffic Monitoring (MOST IMPACTFUL FEATURE)

### Overview
This is probably the most exciting next phase! Real-time traffic visualization transforms the static 3D network diagram into a live monitoring tool.

### Planned Features

#### 1. Real-Time Traffic Visualization
- **Animated connections:** Flowing particles/pulses moving along connection paths
- **Direction indicators:** Particles move from source to target showing data flow direction
- **Speed variation:** Faster particles = higher throughput
- **Particle density:** More particles = more active connection

#### 2. Live Traffic Metrics
- **Per-connection metrics:**
  - Throughput (Mbps) - Current data rate
  - Packet count - Packets per second
  - Latency (ms) - Round-trip time
  - Packet loss (%) - Dropped packets
- **Display options:**
  - Hover tooltip shows current metrics
  - Always-on labels for selected connections
  - Dashboard panel with detailed stats

#### 3. Color-Coded Status
- **Traffic load visualization:**
  - Green (0-30% utilization) - Healthy, light load
  - Yellow (30-70% utilization) - Moderate, warning threshold
  - Orange (70-90% utilization) - Heavy load approaching capacity
  - Red (90-100% utilization) - Critical, at or over capacity
- **Status indicators:**
  - Connection color changes based on current load
  - Pulsing/flashing for alerts
  - Thickness variation based on bandwidth utilization

#### 4. Traffic Dashboard
- **Metrics panel (right sidebar or bottom panel):**
  - Top connections by traffic volume
  - Total network throughput
  - Average latency across all connections
  - Packet loss summary
- **Historical data:**
  - Time-series charts showing traffic over time
  - Sparklines for quick trend visualization
  - Configurable time windows (1min, 5min, 15min, 1hour)
- **Alerts panel:**
  - List of current warnings/errors
  - Connection health scores
  - Anomaly detection (sudden spikes/drops)

#### 5. Streaming Data Architecture
- **Leptos Native WebSocket:**
  ```rust
  #[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
  async fn stream_traffic_data(
      input: BoxedStream<TrafficRequest, ServerFnError>
  ) -> Result<BoxedStream<TrafficUpdate, ServerFnError>, ServerFnError> {
      // Server streams traffic updates every 100-500ms
      Ok(traffic_stream)
  }

  // Client side
  let traffic_signal = Signal::from_stream(traffic_stream);
  ```
- **Data structure:**
  ```rust
  struct TrafficUpdate {
      connection_id: i64,
      timestamp: i64,
      throughput_mbps: f64,
      packets_per_sec: u64,
      latency_ms: f64,
      packet_loss_pct: f64,
      utilization_pct: f64,  // 0-100
  }
  ```

#### 6. Mock Traffic Generator (for Demo)
- **Server-side generator:**
  - Simulates realistic network traffic patterns
  - Varies by time of day (higher during business hours)
  - Random bursts and quiet periods
  - Connection-specific patterns (servers busier than switches)
- **Configuration options:**
  - Enable/disable mock data
  - Adjust traffic intensity (low/medium/high)
  - Trigger specific scenarios (DDoS, link failure, congestion)

#### 7. Animation Implementation
- **Particle system in three-d:**
  - Small sphere instances moving along connection paths
  - Interpolate position from source to target
  - Recycle particles at target (spawn new at source)
- **Performance optimization:**
  - Limit particle count per connection (max 5-10)
  - Update positions in requestAnimationFrame
  - Use instanced rendering for efficiency
- **Visual effects:**
  - Motion blur for speed impression
  - Glow effect around particles
  - Trail effect showing path history

### Implementation Plan

**Phase 6.1 - Backend & Data Streaming:**
1. Create mock traffic generator (server function)
2. Set up WebSocket streaming server function
3. Test data flow client→server→client
4. Database schema for traffic_metrics (already exists)

**Phase 6.2 - Basic Visualization:**
1. Color connections based on traffic load
2. Update connection colors in real-time from stream
3. Add traffic metrics to connection tooltips
4. Test with mock data

**Phase 6.3 - Animation System:**
1. Implement particle system in three-d
2. Spawn particles at source node
3. Animate particles along connection path
4. Vary speed/density based on throughput

**Phase 6.4 - Dashboard & Metrics:**
1. Create traffic metrics panel component
2. Display current stats for all connections
3. Add time-series charts (optional)
4. Top connections list sorted by traffic

**Phase 6.5 - Polish & Configuration:**
1. Enable/disable traffic visualization
2. Adjust animation speed/density
3. Configure alert thresholds
4. Save preferences to database

### Expected Outcomes
- **Visual impact:** Instantly see network activity and hotspots
- **Monitoring:** Identify bottlenecks and congestion in real-time
- **Professional tool:** Transforms prototype into production-ready network monitoring solution
- **Demo appeal:** Animated traffic makes presentations much more engaging

### Technical Challenges
1. **Performance:** Animating many particles while maintaining 60fps
   - Solution: Use three-d instanced rendering, limit particle count
2. **Data rate:** WebSocket bandwidth for many connections
   - Solution: Aggregate updates, send diffs only, configurable refresh rate
3. **State management:** Keeping traffic data in sync with topology
   - Solution: Use same mutable storage pattern as event handlers
4. **Visual clutter:** Too many particles/metrics overwhelming
   - Solution: Progressive disclosure, filters, focus mode

## Build Commands (VERIFIED)

### Development Mode
Run **both** commands in separate terminals:
```bash
# Terminal 1: Tailwind CSS watch mode (v4.1.16)
./tailwindcss -i style/input.css -o style/output.css --watch

# Terminal 2: Leptos development server with hot reload
cargo leptos watch
```

### Production Build
```bash
# Build optimized CSS first
./tailwindcss -i style/input.css -o style/output.css --minify

# Then build Leptos application
cargo leptos build --release

# Verify code splitting (look for multiple .wasm files)
ls -lh target/site/pkg/*.wasm
```

## Git Repository
**Repo:** https://github.com/madkrell/ntv.git
**Tags:** v0.1.0-phase1-complete, v0.1.0-phase2-complete, v0.1.0-phase3-complete, v0.1.0-phase4-complete
**Next Tag:** v0.1.0-phase5-complete (after JSON export/import)

## All Known Issues & Solutions

See original CLAUDE.md for complete list. Key patterns to remember:
1. **Server Functions** - Use leptos_axum::extract() for database access
2. **Event Handlers** - Use mutable storage (Rc<RefCell<>>) for data access
3. **Disposed Signals** - Use Arc<Mutex<>> snapshot for event handlers with .forget()
4. **Context Collision** - Wrap same-typed signals in unique struct
5. **Canvas Resize** - Always update dimensions on every render
6. **Bounding Box** - Calculate from actual node positions, not fixed values
