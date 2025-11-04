# Network Topology Visualizer - Claude Development Notes

## Project Status
**Current Phase:** Phase 4 IN PROGRESS - 3D Model Rotation Complete ✅
**Last Updated:** 2025-11-04
**Git Tags:** v0.1.0-phase1-complete, v0.1.0-phase2-complete, v0.1.0-phase3-complete
**Architecture:** Regular Leptos Components (Islands removed - see notes below)

### Phase 4 - IN PROGRESS

**✅ COMPLETED (Priority 1 - Core 3D Features):**
1. ✅ **3D node rotation controls** - Full X/Y/Z rotation with database storage, UI sliders, and viewport rendering
2. ✅ **Model Selection UI (Partial)** - Can load glTF/GLB models; need to add different node types (switch, firewall, server, etc.)
3. ✅ **3D Grid and Axes** - Blender-style reference grid with X/Y/Z axis lines and grid floor plane

**✅ COMPLETED (Priority 2 - Visual Polish):**
5. ✅ **Node Labels/Tooltips** - Show node name on hover in 3D viewport
6. ✅ **Color-Coded Nodes by Type** - Router=blue, Switch=green, Server=orange, etc.

**⏳ REMAINING (Priority 1 - Core 3D Features):**
3. ⏳ **Enable Device Palette buttons** - Make 'Router', 'Switch', etc. 'Click to Add' buttons functional
4. ⏳ **Topology switching control** - Add UI to switch/load different topologies (add another mock topology in database)

**⏳ REMAINING (Priority 2 - Visual Polish):**
7. ⏳ **Improved Lighting and Materials** - Better 3D scene lighting
8. ⏳ **Better Camera Controls** - Presets, bookmarks, reset view

### Phase 3 - COMPLETE ✅
- ✅ Professional 3-panel layout (device palette, viewport, properties)
- ✅ Node selection via 3D raycasting with visual feedback (yellow highlight)
- ✅ Click empty space to deselect
- ✅ Properties panel loads and displays actual node/connection data
- ✅ Full CRUD server functions for nodes and connections
- ✅ Save changes from properties panel with real-time viewport updates
- ✅ Suspense components for proper loading states (no hydration warnings)
- ✅ Context-based state sharing across components

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

## ~~Islands vs Components vs Code Splitting~~ (NO LONGER APPLICABLE)

**NOTE:** This section is kept for reference only. We removed islands architecture on 2025-11-03.

**Previous approach - Islands ≠ Automatic Code Splitting!**

- `#[component]` - Server-rendered HTML only, no client JS
- `#[island]` - Interactive WASM component with full Leptos reactivity (hydrates on-demand)
- `#[lazy]` - Code-splits island into separate WASM bundle (ONLY works with simple async functions)

**Reality Check:**
```rust
// ❌ Does NOT work - complex reactive logic with Effects/signals
#[island]
#[lazy]
async fn ComplexIsland() -> impl IntoView { /* Effects, Resources, etc. */ }

// ✅ Works - simple async data fetching
#[island]
#[lazy]
async fn SimpleIsland() -> impl IntoView {
    let data = fetch_data().await;
    view! { <div>{data}</div> }
}
```

**Our Architecture:**
- Islands provide on-demand hydration (not loaded until component renders)
- All islands in single WASM bundle (~2.3MB with three-d)
- `#[lazy]` only works for simple async components without reactive primitives

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

```rust
// src/api.rs - Server functions accessible from client AND server
use crate::models::{Topology, Node, Connection};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use sqlx::SqlitePool;

#[server(GetTopologyFull, "/api")]
pub async fn get_topology_full(id: i64) -> Result<TopologyFull, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use axum::Extension;
        use leptos_axum::extract;

        let Extension(pool) = extract::<Extension<SqlitePool>>()
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to extract: {}", e)))?;

        // Database operations...
        Ok(data)
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!("Server function called on client")
    }
}
```

```rust
// src/islands/my_island.rs - Now this works!
use crate::api::get_topology_full;  // ✅ Can import!

#[island]
pub fn MyIsland() -> impl IntoView {
    let data = Resource::new(
        || (),
        |_| async move {
            get_topology_full(1).await  // ✅ Works!
        }
    );
    // ...
}
```

**Why this works:**
- `#[server]` macro generates client-side stub when `ssr` feature is off
- Non-feature-gated module makes function signature visible to client
- SSR-specific implementation is conditionally compiled
- Leptos handles the HTTP request/response serialization

## Server Functions & Streaming (VERIFIED)
```rust
// Regular server function
#[server(FunctionName)]
pub async fn function_name(...) -> Result<T, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        let pool = extract::<Extension<SqlitePool>>().await?.0;
        // Database operations...
    }
}

// ✅ NATIVE SSE/STREAMING (no Axum SSE needed!)
#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
async fn stream_data(
    input: BoxedStream<Message, ServerFnError>
) -> Result<BoxedStream<Message, ServerFnError>, ServerFnError> {
    Ok(input.into())
}

// Client side: create signal from stream
let signal = Signal::from_stream(my_stream);
```

## Browser Console Logging from WASM

**Issue:** `tracing` logs don't appear in browser console from WASM

**Solution:** Use `web_sys::console` directly:
```rust
// Add to Cargo.toml web-sys features
web-sys = { version = "0.3", features = ["console", "HtmlCanvasElement", ...] }

// In your code
web_sys::console::log_1(&"Hello from WASM!".into());
web_sys::console::log_1(&format!("Value: {}", x).into());
web_sys::console::error_1(&format!("Error: {}", e).into());
```

## Project Initialization (VERIFIED)
```bash
# Install cargo-leptos
cargo install --locked cargo-leptos

# Create project from template
cargo leptos new --git leptos-rs/start-axum

# OR manual setup with correct structure
cargo new --lib my-project
# Configure Cargo.toml as shown above
```

## Verified Working Dependencies
- leptos = "0.8.0" (with "ssr" and "islands" features)
- leptos_axum = "0.8.0"
- cargo-leptos = latest
- sqlx = "0.7" (with sqlite, macros, migrate)
- wasm-bindgen = "0.2.101" (matching installed CLI)
- web-sys = "0.3" (WebGL2, console features)
- **three-d = "0.17.1" ✅ VERIFIED** - Works with custom WebGL2 context

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

## Styling with Tailwind CSS v4
- **No Node.js required** - Using standalone Tailwind CLI
- **CSS-first configuration** - No `tailwind.config.js` file needed
- **Auto content detection** - Scans `src/**/*.rs` for classes
- **Apply classes directly** in Leptos `view!` macros: `<div class="text-blue-600 font-bold">`
- **See [TAILWIND.md](TAILWIND.md)** for complete setup guide

## Bundle Sizes (Actual)
- **Current WASM bundle:** 2.3MB (dev build, includes three-d + all islands)
- Release build with optimizations will be significantly smaller
- Islands hydrate on-demand but share single WASM bundle (no #[lazy] splitting)

## Known Issues & Solutions

### 1. Server Functions Database Access
**Issue:** `use_context::<SqlitePool>()` returns None in server functions
**Solution:** Use `leptos_axum::extract()` instead
```rust
use leptos_axum::extract;
use axum::Extension;

let Extension(pool) = extract::<Extension<SqlitePool>>()
    .await
    .map_err(|e| ServerFnError::new(format!("Failed to extract: {}", e)))?;
```

### 2. Server Functions Not Accessible from Islands
**Issue:** `use crate::server::my_function` fails with "unresolved import" from client code
**Solution:** Create non-feature-gated `api.rs` module (see "Server Functions Architecture" above)

### 3. SQLite Database Creation
**Issue:** "unable to open database file" on first run
**Solution:** Use `SqliteConnectOptions` with `create_if_missing(true)`
```rust
use sqlx::sqlite::SqliteConnectOptions;
use std::str::FromStr;

let options = SqliteConnectOptions::from_str(&database_url)?
    .create_if_missing(true);
let pool = SqlitePoolOptions::new()
    .connect_with(options)
    .await?;
```

### 4. Islands Code Splitting with #[lazy]
**Issue:** `#[lazy]` attribute fails with "trait bounds not satisfied" for reactive islands
**Root Cause:** `#[lazy]` requires simple async functions; doesn't work with Effects, Resources, or complex reactive logic
**Solution:** Accept single WASM bundle or simplify island to pure async data fetching

### 5. wasm-bindgen Version Mismatch
**Issue:** "Wasm file schema version: 0.2.105, binary schema version: 0.2.101"
**Solution:** Pin wasm-bindgen to match installed CLI version in Cargo.toml:
```toml
wasm-bindgen = { version = "=0.2.101", optional = true }
```

### 6. JsCast Import Not Found
**Issue:** `use leptos::wasm_bindgen::JsCast` fails or `unchecked_ref()` not available
**Solution:** Import directly from wasm_bindgen crate:
```rust
#[cfg(feature = "hydrate")]
use wasm_bindgen::JsCast;
```

## Database
- SQLite with sqlx
- Migrations in /migrations/
- Pool provided via Axum Extension
- Sample data: 7 nodes (Router, Switches, Servers, Firewall), 7 connections

## Key Corrections to Original Plan

### ❌ INCORRECT in plan:
1. **Leptos.toml file** - Does NOT exist in modern Leptos
2. **Islands architecture for fully interactive apps** - Wrong! Islands are for sparse interactivity. Use regular components.
3. **leptos::leptos_dom::HydrationCtx::stop_hydrating()** - Wrong! Use `hydrate_body()`
4. **Axum SSE endpoints** - NOT needed! Leptos has native streaming via server functions
5. **Manual EventSource setup** - NOT needed! Use `Signal::from_stream()`
6. **Server functions in `#[cfg(feature = "ssr")]` module** - Wrong! Create non-gated `api.rs`

### ✅ CORRECT approach:
1. Use `cargo leptos new --git leptos-rs/start-axum` for project template
2. Configure in `Cargo.toml` WITHOUT "islands" feature (for fully interactive apps)
3. Use `leptos::mount::hydrate_body(app::App)` in lib.rs
4. Use `#[server(protocol = Websocket<>)]` for streaming
5. Use `Signal::from_stream()` for reactive SSE/streaming data
6. Put server functions in non-feature-gated module (api.rs)
7. Use `web_sys::console` for browser console logging from WASM
8. Use `provide_context()` and `use_context()` for sharing state across components

## IDE Configuration
All editors should enable all Cargo features for rust-analyzer:
```json
// VSCode settings.json
{
  "rust-analyzer.cargo.features": "all"
}
```

## ✅ Phase 1 COMPLETE - Foundation

**Stack:** Leptos 0.8 + SQLite + Server Functions
**Repo:** https://github.com/madkrell/ntv.git

**Key Architecture (Updated 2025-11-03):**
- `#[component]` = Regular Leptos component with full reactivity and hydration
- `#[server]` = Backend API via leptos_axum::extract()
- Database pool via Axum Extension layer
- Context-based state sharing with `provide_context()` / `use_context()`

**Database Schema:** topologies, nodes (3D x/y/z), connections, traffic_metrics
**Git Tag:** v0.1.0-phase1-complete

**Note:** Originally used islands architecture, but this was removed in Phase 3 for better interactivity.

## ✅ Phase 2 COMPLETE - 3D Viewport & Rendering

### Server Functions (Moved to api.rs)
Created `src/api.rs` module (NOT feature-gated) with all server functions:
- `get_topologies()` - List all topologies
- `create_topology()` - Create new topology
- `delete_topology()` - Delete topology
- `get_topology_full()` - Get topology with all nodes and connections

### 3D Viewport Implementation
**Approach:** three-d with custom WebGL2 context (NOT three-d's Window module)

**Key Discovery:** three-d can work WITHOUT event loop control!
- three-d's Window module requires event loop (conflicts with Leptos islands)
- **Solution:** Use `three_d::Context::from_gl_context()` with web-sys WebGL2 context
- Leptos island controls DOM/canvas, three-d just renders to it

**Implementation Pattern:**
```rust
#[island]
pub fn TopologyViewport(#[prop(optional)] topology_id: Option<i64>) -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let camera_state = RwSignal::new(CameraState::default());

    // Fetch topology data
    let topology_data = Resource::new(
        move || topology_id,
        |id| async move {
            match id {
                Some(id) => get_topology_full(id).await.ok(),
                None => None,
            }
        }
    );

    Effect::new(move || {
        if let Some(canvas) = canvas_ref.get() {
            #[cfg(feature = "hydrate")]
            {
                // Get WebGL2 context
                let gl = canvas.get_context("webgl2")?.dyn_into::<WebGl2RenderingContext>()?;

                // Wrap in glow (three-d uses glow internally)
                let gl_context = three_d::context::Context::from_webgl2_context(gl);

                // Create three-d Context
                let context = Context::from_gl_context(Arc::new(gl_context))?;

                // Render nodes and connections...
            }
        }
    });

    view! { <canvas node_ref=canvas_ref width="800" height="600" /> }
}
```

### Camera Controls (Orbit Camera)
**Implementation:** Spherical coordinate system
- **Drag to rotate:** Updates azimuth (horizontal) and elevation (vertical) angles
- **Scroll to zoom:** Adjusts camera distance
- **Camera state:** RwSignal with distance, azimuth, elevation

```rust
#[derive(Clone, Copy)]
struct CameraState {
    distance: f32,     // 18.0 default (zoomed out to show all nodes)
    azimuth: f32,      // horizontal rotation in radians
    elevation: f32,    // vertical rotation in radians
}

// Camera position calculation
let eye = vec3(
    state.distance * state.elevation.cos() * state.azimuth.sin(),
    state.distance * state.elevation.sin(),
    state.distance * state.elevation.cos() * state.azimuth.cos(),
);
```

### Node Rendering
- **Nodes as spheres:** `CpuMesh::sphere(16)` with PhysicalMaterial
- **Scale:** 0.3 (tested, prevents overlap)
- **Color:** Blue (Srgba::new(50, 150, 255, 255))
- **Positioning:** Uses node.position_x/y/z from database

### Connection Rendering
- **Connections as cylinders:** Rotated to align between nodes
- **Challenge:** three-d's default cylinder is along Y-axis, needs rotation
- **Solution:** Axis-angle rotation from direction vector

```rust
// Calculate rotation to align cylinder with connection direction
let direction = end_pos - start_pos;
let length = direction.magnitude();
let normalized_dir = direction.normalize();
let up = vec3(0.0, 1.0, 0.0);

// Calculate rotation axis and angle
if (normalized_dir - up).magnitude() < 0.001 {
    // Already aligned
    Mat4::identity()
} else if (normalized_dir + up).magnitude() < 0.001 {
    // Opposite direction (180 degrees)
    Mat4::from_angle_x(radians(std::f32::consts::PI))
} else {
    // General case: axis-angle rotation
    let axis = up.cross(normalized_dir).normalize();
    let angle = up.dot(normalized_dir).acos();
    Mat4::from_axis_angle(axis, radians(angle))
}
```

### Sample Data
Created test topology with 7 nodes and 7 connections:
- Router-Core at origin (0,0,0)
- Switch-A and Switch-B at (-3,2,0) and (3,2,0)
- 3 Servers at y=4, z=-2
- Firewall at (0,-3,0)
- Connections between them (fiber, ethernet)

### Achievements
1. ✅ WebGL2 context initialized
2. ✅ three-d Context created from WebGL2
3. ✅ Test cube rendered and verified
4. ✅ Interactive camera controls (drag + scroll)
5. ✅ Topology data loaded from database via server function
6. ✅ Nodes rendered as 3D spheres at correct positions
7. ✅ Connections rendered as properly rotated cylinders
8. ✅ Camera zoomed out to show full topology (distance 18.0)
9. ✅ Browser console logging working

### Current Status
**Working features:**
- Interactive 3D viewport with orbit controls
- Topology data fetched from server and rendered
- 7 nodes displayed as blue spheres
- 7 connections displayed as gray cylinders
- Proper camera positioning to view entire network

**Git Tag:** v0.1.0-phase2-complete (to be created)

## ✅ Phase 3 COMPLETE - UI Layout & 3D Editing Interface

**Git Tag:** v0.1.0-phase3-complete (to be created)

### Architecture Change
- ✅ Removed islands architecture (2025-11-03)
- ✅ Converted to regular Leptos components for better interactivity
- ✅ Context-based state sharing with `provide_context()` / `use_context()`
- ✅ Fixed hydration to use `hydrate_body()` instead of `hydrate_islands()`

### UI Layout
1. ✅ Professional 3-panel layout implemented (src/islands/topology_editor.rs)
   - Left: Device Palette with 6 device types (Router, Switch, Server, Firewall, LoadBalancer, Database)
   - Center: 3D Viewport (TopologyViewport component)
   - Right: Properties Panel (updates on selection)
2. ✅ Top toolbar with action buttons (Add Node, Connect, Delete, Save, Export)
3. ✅ Responsive layout with Tailwind CSS styling

### 3D Selection & Interaction
1. ✅ Click detection in 3D viewport (differentiates drag vs click)
2. ✅ Ray-sphere intersection for node selection
3. ✅ **Visual feedback:** Selected nodes render in yellow/orange color
4. ✅ **Click empty space to deselect:** Clicking non-node areas clears selection
5. ✅ Properties panel updates reactively when selection changes
6. ✅ Selection signals shared via Leptos context:
   - `RwSignal<Option<i64>>` for selected_node_id
   - `RwSignal<Option<SelectedItem>>` for selected_item (Node or Connection enum)

### Backend & Data Management
1. ✅ **Complete Node CRUD server functions** (src/api.rs):
   - `get_node(id)` - Fetch single node
   - `create_node(data)` - Create new node
   - `update_node(id, data)` - Update node with dynamic fields
   - `delete_node(id)` - Delete node
2. ✅ **Complete Connection CRUD server functions** (src/api.rs):
   - `get_connection(id)` - Fetch single connection
   - `create_connection(data)` - Create new connection
   - `update_connection(id, data)` - Update connection
   - `delete_connection(id)` - Delete connection

### Properties Panel Integration
1. ✅ **NodeProperties component** loads real data via `get_node()` Resource
2. ✅ **ConnectionProperties component** loads real data via `get_connection()` Resource
3. ✅ **Suspense wrappers** for proper loading states (eliminates hydration warnings)
4. ✅ **Save functionality** updates database via `update_node()`/`update_connection()`
5. ✅ **Real-time viewport updates:** Viewport automatically refetches and re-renders when data changes
   - Implemented via `refetch_trigger` signal shared through context
   - No page refresh needed - updates are instant!

### Implementation Highlights
- **Ray-sphere intersection:** Accurate 3D picking with 45° FOV perspective camera
- **Dual material rendering:** Normal (blue) and selected (yellow) materials for each node
- **Dynamic UPDATE queries:** Server functions only update provided fields (all fields optional)
- **Action-based saves:** Leptos Actions provide pending states and error handling
- **Effect-based refetch:** Save success triggers viewport reload via signal increment

### Key Files Modified
- `src/islands/topology_editor.rs` - Main editor with 3-panel layout, properties components
- `src/islands/topology_viewport.rs` - 3D rendering, selection, refetch mechanism
- `src/api.rs` - All 8 CRUD server functions (4 nodes + 4 connections)

## Phase 4 - Visual Enhancements & 3D Interaction (IN PROGRESS)

### ✅ COMPLETED: 3D Model Rotation Controls (2025-11-04)

**Implementation:**
1. ✅ Database migration for rotation_x/y/z columns
2. ✅ Updated Node model structs and DTOs with rotation fields
3. ✅ Full CRUD API support for rotation values
4. ✅ Properties panel UI with X/Y/Z rotation sliders
5. ✅ Viewport rendering with rotation transformations
6. ✅ Default rotation_x=90° for Blender glTF models

**Database Changes:**
- Migration: `20250102000002_add_node_rotations.sql`
- Added columns: `rotation_x`, `rotation_y`, `rotation_z` (REAL, default 0.0)
- All rotation values stored in **degrees** for user clarity

**Key Files Modified:**
- `src/models/node.rs` - Added rotation fields to Node, CreateNode, UpdateNode
- `src/api.rs` - Updated all CRUD functions, set default rotation_x=90.0
- `src/islands/topology_editor.rs` - Added rotation UI controls (-180° to +180° range)
- `src/islands/topology_viewport.rs` - Applied rotations using cgmath `degrees()` function

**Critical Bug Fix:**
Initial implementation used `radians()` function incorrectly. The cgmath library has two angle wrapper functions:
- `radians(value)` - Wraps a value that's ALREADY in radians (no conversion)
- `degrees(value)` - Wraps a value in degrees, auto-converts to radians via Angle trait

**WRONG (initial implementation):**
```rust
let x_rotation = Mat4::from_angle_x(radians(node.rotation_x as f32));
// User enters 90° → treated as 90 radians → ~5156° rotation!
```

**CORRECT (fixed implementation):**
```rust
let x_rotation = Mat4::from_angle_x(degrees(node.rotation_x as f32));
// User enters 90° → converted to radians properly → quarter turn
```

**Blender Coordinate System:**
Blender uses Y-up coordinate system, while our viewport uses Z-up. To make glTF/GLB models from Blender sit flat on the grid floor, they need a 90° rotation around X-axis:
- Default rotation for new nodes: `rotation_x = 90.0`
- This ensures router models (and all Blender exports) are oriented correctly by default

**Lessons Learned:**
1. **Always verify unit conversions** - cgmath's function naming is subtle (radians vs degrees)
2. **Clean rebuilds are essential** - Browser caches WASM modules; use `cargo clean` + hard refresh
3. **Kill old processes** - Multiple cargo-leptos instances can conflict
4. **Coordinate system defaults** - Set sensible rotation defaults based on model source
5. **Store degrees in database** - More intuitive for users than radians

**Troubleshooting Process:**
- Issue: Topology stopped rendering after rotation implementation
- Root cause: Browser caching old WASM + conflicting cargo-leptos processes
- Solution: `cargo clean`, kill all processes, fresh rebuild, hard browser refresh

### Remaining Phase 4 Features

**Priority 1 - Core 3D Features:**
3. ⏳ **Enable Device Palette buttons** - Make left panel buttons functional ('Router', 'Switch', etc. 'Click to Add')
4. ⏳ **Topology switching control** - Add UI to switch/load different topologies
   - Add another mock topology to database for testing
   - Dropdown or selector to switch between topologies

**Priority 2 - Visual Polish:**
7. ⏳ **Improved Lighting and Materials** - Better 3D scene lighting
8. ⏳ **Better Camera Controls** - Presets, bookmarks, reset view

## Phase 5 - Export & Finalization (FUTURE)

### Planned Features
1. Export topology as PNG image
2. Export topology as JSON data
3. Import topology from JSON
4. UI polish and optimizations
5. Documentation and deployment

## Phase 6 - Traffic Monitoring (FUTURE)

### Planned Features
1. Real-time traffic data visualization using Leptos native streaming
2. Use `#[server(protocol = Websocket<...>)]` for streaming data
3. Display traffic throughput on connections
4. Color/animate connections based on traffic load
5. Traffic metrics dashboard
