# Network Topology Visualizer - Implementation Plan
## Pure Rust/WASM with Leptos Islands Architecture

---

## 📊 PROJECT STATUS

**Current Phase:** Phase 2 Complete ✅
**Git Tag:** `v0.1.0-phase2-complete`
**Last Updated:** 2025-11-03

### Completed Phases

#### ✅ Phase 1 - Foundation (Git tag: v0.1.0-phase1-complete)
- Leptos 0.8 Islands architecture configured
- SQLite database with migrations
- Server functions in `src/api.rs` (non-feature-gated pattern)
- Database schema: topologies, nodes, connections, traffic_metrics
- Sample topology data with 7 nodes and 7 connections

#### ✅ Phase 2 - 3D Viewport (Git tag: v0.1.0-phase2-complete)
- TopologyViewport island with WebGL2 + three-d
- Interactive orbit camera controls (drag to rotate, scroll to zoom)
- Nodes rendered as 3D spheres at database positions
- Connections rendered as properly rotated cylinders between nodes
- Browser console logging for debugging
- Working with sample topology (7 nodes, 7 connections)

### Remaining Phases

- ⏳ Phase 3 - UI Layout & 3D Editing Interface
- ⏳ Phase 4 - Visual Enhancements & 3D Models (Blender glTF/GLB)
- ⏳ Phase 5 - Traffic Monitoring (Real-time with Leptos streaming)
- ⏳ Phase 6 - Export & Finalization (PNG, JSON)

**See [SESSION-GUIDE.md](SESSION-GUIDE.md) for continuation guidance.**

---

## ⚠️ CRITICAL CORRECTIONS - READ FIRST!

**This plan was written before verifying Leptos 0.8 documentation. Several patterns are INCORRECT.**

**✅ Always refer to [CLAUDE.md](CLAUDE.md) for verified, working configurations!**

### Key Errors in This Document:

1. **❌ NO Leptos.toml File**
   - This document references `Leptos.toml` throughout
   - Modern Leptos does NOT use this file
   - ✅ All configuration goes in `Cargo.toml` with feature flags

2. **❌ Wrong Hydration Function**
   - Document shows: `leptos::leptos_dom::HydrationCtx::stop_hydrating()`
   - ✅ Correct: `leptos::mount::hydrate_islands()`

3. **❌ Manual Axum SSE/EventSource**
   - Document suggests manual Axum SSE endpoints with EventSource API
   - ✅ Leptos has NATIVE streaming via `#[server(protocol = Websocket<>)]`
   - ✅ Client side: `Signal::from_stream(stream)` - fully reactive!

4. **❌ Server Function Syntax**
   - Some examples use outdated syntax
   - ✅ See CLAUDE.md for correct `#[server]` patterns

### How to Use This Plan:

1. Read the overall architecture and phase breakdown for strategy
2. **For actual implementation code**, always check CLAUDE.md first
3. When you see Leptos.toml → ignore it, use Cargo.toml
4. When you see SSE/EventSource setup → use native Leptos streaming
5. When unsure → check CLAUDE.md for verified patterns

**The plan's STRATEGY is sound. The SPECIFIC CODE needs corrections per CLAUDE.md.**

---

## INITIAL INSTRUCTIONS

You are building a web-based network topology visualization tool for network architects and engineers. The application must be built entirely in **Rust** using **Leptos 0.8.11 with islands architecture and code splitting**. No JavaScript libraries or frameworks are permitted - all interactive functionality must be implemented in Rust and compiled to WASM.

The user has existing 3D network device models created in Blender that will be exported as glTF/GLB files and loaded into the application. The application must support both 3D visualization and a 2D canvas-based editor for creating and editing network topologies.

---

## APPLICATION REQUIREMENTS

### Core Requirements

**R1: Topology Viewing**
- Display network topologies in an interactive 3D viewport
- Load 3D device models from glTF/GLB files created in Blender
- Support camera controls (rotate, pan, zoom)
- Render connections between devices as lines
- Provide device labels and metadata on hover/click

**R2: Topology Editing**
- Provide a canvas-based editor for creating new topologies
- Support drag-and-drop device placement from a palette
- Enable drawing connections between devices
- Allow editing device properties (name, type, configuration)
- Support saving topologies to database

**R3: Data Persistence**
- Store topologies, devices, and connections in SQLite database
- Support CRUD operations (Create, Read, Update, Delete)
- Maintain relationships between topologies, devices, and connections
- Track creation and modification timestamps

**R4: Real-Time Monitoring** (Optional for v1)
- Display simulated traffic data using Server-Sent Events (SSE)
- Show traffic throughput for connections
- Update traffic visualization in real-time

**R5: Export Functionality**
- Export topologies as PNG images
- Export topologies as JSON data
- Provide download mechanism for exported files

### Technical Requirements

**TR1: Leptos Islands Architecture**
- Use Leptos 0.8.11 with islands feature enabled (`islands = true` in Leptos.toml)
- Mark only interactive components as islands using `#[island]` macro
- Keep static content as server-rendered components using `#[component]`
- Minimize WASM bundle sizes through selective hydration
- Configure hydration with `HydrationScripts` component with `islands=true`

**TR2: Code Splitting**
- Each island must compile to a separate WASM bundle
- Bundles load on-demand when island is rendered
- No WASM should load on initial page if no islands are present
- Verify build output contains multiple .wasm files (one per island)
- Total combined WASM size target: < 1 MB across all islands

**TR3: Leptos Server Functions**
- All backend logic must use `#[server]` macro (no plain Axum routes)
- Server functions must handle database operations
- Use `ServerFnError` for error handling
- Provide database pool via context for server functions
- Integrate with Leptos resource system for data loading

**TR4: Pure Rust Implementation**
- All client-side code must be written in Rust
- Use `three-d` crate for 3D rendering (compiles to WASM)
- Use `web-sys` for browser APIs (Canvas, DOM manipulation)
- No JavaScript libraries or dependencies

**TR5: Performance Targets**
- Initial page load: < 2 seconds
- 3D viewport initialization: < 1 second
- Support topologies with up to 100 devices
- Maintain 60 FPS in 3D viewport
- Individual island WASM sizes: TopologyViewport ~300KB, TopologyEditor ~150KB

**TR6: Browser Compatibility**
- Support modern browsers with WebGL2 support
- Chrome, Firefox, Safari, Edge (latest versions)
- Responsive design for desktop viewports (1280x720 minimum)

### Expected Outcomes

Upon completion, the application must deliver:

1. **Working 3D Viewer**: Users can load existing topologies and view them in 3D with device models from Blender
2. **Functional Editor**: Users can create new topologies by placing devices and drawing connections
3. **Data Management**: All topologies persist to database and can be loaded/edited/deleted
4. **Export Capability**: Users can export their topologies as PNG or JSON files
5. **Optimized Performance**: Application uses islands architecture to minimize WASM bundle sizes

### Out of Scope (for initial version)

- Multi-user collaboration
- Authentication and authorization
- Import from network discovery tools
- Mobile/tablet support
- Animated traffic flows (beyond basic SSE display)
- Template library
- AI-powered features

---

## PROJECT OVERVIEW

**Tech Stack**:
- Frontend: Leptos 0.8.11 (islands mode) + three-d (3D rendering)
- Backend: **Leptos SSR with server functions** (integrated with Axum)
- Database: SQLite with sqlx
- Build Tool: cargo-leptos
- 3D Models: Blender → glTF/GLB format

**Architecture Approach**:
- **Islands Architecture**: Selective hydration - only `#[island]` components become WASM
- **Code Splitting**: Each island compiles to a separate WASM bundle that loads on-demand
- **Server Functions**: Backend logic written in Leptos using `#[server]` macro
- Axum serves as the HTTP framework that Leptos integrates with, but all server-side logic uses Leptos server functions

**Deployment**:
- Single server binary with embedded SQLite database
- Static assets served from /public directory
- WASM bundles in /pkg directory (one per island)
- Can run on any Linux/Mac/Windows server with Rust runtime

---

## Understanding Leptos Islands Architecture

### What Islands Are (and Aren't)

**Islands Architecture** means:
- Most of your page is **static HTML** rendered on the server (no WASM)
- Only components marked with `#[island]` compile to WASM and hydrate on the client
- This dramatically reduces WASM bundle size (from ~355KB to ~166KB in examples)
- Each island is a separate entrypoint and hydrates independently

**NOT about:**
- Using JavaScript libraries (we use pure Rust)
- Traditional component splitting (that's just code organization)

### Key Differences from Full SSR

```rust
// Traditional SSR - ENTIRE app becomes WASM
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <h1>"Title"</h1>
        <button on:click=move |_| set_count.update(|n| *n + 1)>
            {count}
        </button>
    }
}
// Result: All code ships as WASM, hydrates everything

// Islands Mode - Only interactive parts become WASM
#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Title"</h1>  // Static HTML, no WASM
        <Counter/>        // This is an island, becomes WASM
    }
}

#[island]
fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| set_count.update(|n| *n + 1)>
            {count}
        </button>
    }
}
// Result: Only Counter compiles to WASM, h1 stays static
```

### Our Strategy

- **Static SSR Components**: Navbar, footer, topology list, documentation, static cards
- **Islands (WASM)**: 3D viewport, device palette, node editor, traffic monitor, property panels
- Each island is **lazy-loaded** on demand (code splitting happens automatically)
- No WASM loads until user interacts with an island

---

## Code Splitting with Islands

### How Code Splitting Works

Code splitting in Leptos islands means each `#[island]` component compiles to its **own separate WASM file**. This is automatic when using islands architecture.

**Example**:
```rust
// src/islands/topology_viewport.rs
#[island]
pub fn TopologyViewport() -> impl IntoView {
    // 3D rendering code
}

// src/islands/topology_editor.rs
#[island]
pub fn TopologyEditor() -> impl IntoView {
    // Editor code
}
```

**Results in**:
```
target/site/pkg/
  ├── TopologyViewport.wasm     (~300 KB with three-d)
  ├── TopologyViewport.js       (WASM loader)
  ├── TopologyEditor.wasm       (~150 KB)
  ├── TopologyEditor.js         (WASM loader)
  └── ...
```

### Loading Behavior

When a user visits a page:

1. **Initial Load**: Only HTML, CSS, and minimal JS loader (~20 KB)
2. **Page with Island**: Fetches only that island's WASM bundle
3. **Different Page**: Fetches different island (previous one may be cached)

**Example User Journey**:
- User lands on homepage → **0 KB WASM** loaded (static HTML only)
- User clicks "View Topology" → **300 KB WASM** loads (TopologyViewport island)
- User clicks "Edit" → **150 KB WASM** loads (TopologyEditor island)
- Total downloaded: ~450 KB (not 1.2 MB if everything was bundled)

### Configuration

**Leptos.toml**:
```toml
[package]
name = "network-topology-visualizer"

[server]
port = 3000
output-name = "server"

[client]
bin-features = ["hydrate"]

# Enable islands mode - this automatically enables code splitting
islands = true

# Optional: Optimize WASM output
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
```

**Cargo.toml features**:
```toml
[features]
default = []
hydrate = ["leptos/hydrate"]
ssr = [
    "leptos/ssr",
    "dep:axum",
    "dep:leptos_axum",
]

[lib]
crate-type = ["cdylib", "rlib"]  # cdylib required for WASM
```

### Verification

After building, verify code splitting worked:

```bash
# Build release
cargo leptos build --release

# Check output
ls -lh target/site/pkg/

# You should see multiple .wasm files, one per island
# Example output:
# TopologyViewport-a1b2c3d4.wasm    300K
# TopologyEditor-e5f6g7h8.wasm     150K
# TrafficMonitor-i9j0k1l2.wasm      80K
```

### Bundle Size Targets

| Island | Estimated Size | When Loaded |
|--------|---------------|-------------|
| TopologyViewport | ~300 KB | Viewer page |
| TopologyEditor | ~150 KB | Editor page |
| TrafficMonitor | ~80 KB | When monitoring enabled |
| ExportDialog | ~60 KB | When export clicked |
| PropertiesPanel | ~50 KB | Editor page |
| **Total (if all loaded)** | **~640 KB** | - |

**Without islands/code splitting**: ~1.2 MB single bundle loaded upfront

**Optimization Strategies**:
1. Keep islands small and focused
2. Move non-interactive code to regular components
3. Use conditional rendering to defer island loading
4. Share common dependencies between islands (Leptos does this automatically)

---

## Leptos Server Functions

### Backend Architecture

We use **Leptos server functions** for all backend logic, not plain Axum routes. Server functions are the standard way to write server-side code in Leptos SSR applications.

**Why Server Functions?**
- Type-safe API between client and server (same Rust types)
- Automatic serialization/deserialization
- Integrated with Leptos's reactive system
- Built-in error handling
- Works seamlessly with islands

### How Server Functions Work

```rust
// Define server function with #[server] macro
#[server(LoadTopology, "/api")]
pub async fn load_topology(topology_id: String) -> Result<TopologyData, ServerFnError> {
    // This code ONLY runs on the server
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::ServerError("No DB pool".to_string()))?;

    // Query database
    let topology = sqlx::query_as::<_, Topology>(
        "SELECT * FROM topologies WHERE id = ?"
    )
    .bind(&topology_id)
    .fetch_one(&pool)
    .await?;

    Ok(topology)
}

// Call from client (in an island or component)
let topology_data = create_resource(
    move || topology_id.get(),
    |id| async move { load_topology(id).await }
);
```

### Server Function Features

**Automatic Client Generation**:
When you use `#[server]`, Leptos automatically generates:
- Server-side implementation that runs the function
- Client-side stub that makes HTTP request
- Serialization/deserialization code

**Usage in Islands**:
```rust
#[island]
pub fn TopologyViewport(topology_id: String) -> impl IntoView {
    // Call server function from island
    let topology = create_resource(
        move || topology_id.clone(),
        |id| async move {
            load_topology(id).await  // Automatically makes HTTP request
        }
    );

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move || {
                topology.get().map(|data| {
                    // Render topology data
                })
            }}
        </Suspense>
    }
}
```

### Server Context

Server functions can access server-specific context like database pools:

```rust
// In main.rs, provide context
let app = Router::new()
    .leptos_routes(&leptos_options, routes, App)
    .with_state(leptos_options)
    .layer(Extension(pool.clone()));  // Make DB pool available

// In server function, access context
#[server]
pub async fn save_topology(...) -> Result<(), ServerFnError> {
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::ServerError("No pool".to_string()))?;

    // Use pool for database operations
}
```

### Error Handling

```rust
#[server]
pub async fn risky_operation() -> Result<Data, ServerFnError> {
    // Database errors automatically convert to ServerFnError
    let result = sqlx::query("...")
        .fetch_one(&pool)
        .await?;  // ? operator works with ServerFnError

    // Custom errors
    if result.is_empty() {
        return Err(ServerFnError::ServerError("Not found".to_string()));
    }

    Ok(result)
}
```

### All Backend Operations Use Server Functions

In this application, every backend operation is a server function:
- `load_topology()` - Fetch topology from database
- `save_topology()` - Save new/updated topology
- `delete_topology()` - Remove topology
- `list_topologies()` - Get all topologies
- `export_topology()` - Generate export files

**No plain Axum routes** - Axum is just the HTTP server that Leptos integrates with. All business logic goes through `#[server]` functions.

---

## Blender → Rust WASM Workflow

### Complete Pipeline

```
Blender 3D Model
    ↓ (Export as glTF 2.0)
.glb binary file
    ↓ (Place in /public/models/)
Static asset served by web server
    ↓ (Load in Rust island)
three-d crate loads and renders in WASM
    ↓
WebGL2 rendering in browser
```

### Blender Model Creation & Export

**Step 1: Model in Blender**
- Create low-poly network device models (routers, switches, firewalls)
- Target: <5,000 polygons per model for web performance
- Use simple materials (ideally single material per model)
- Add proper origin points (for easy positioning)

**Step 2: Export to glTF**
- File → Export → glTF 2.0 (.glb)
- Settings:
  - Format: **glTF Binary (.glb)** (single file, easier)
  - Include: Selected Objects only
  - Transform: +Y Up (important for web)
  - Geometry: Apply Modifiers ✓
  - Compression: Draco (if available)
  - Export to `/public/models/devices/`

**File Organization**:
```
/public/models/
  /devices/
    router-generic.glb       # ~50KB
    switch-generic.glb       # ~40KB
    firewall-generic.glb     # ~45KB
    server-generic.glb       # ~35KB
    cloud-icon.glb          # ~20KB
  /vendor-specific/
    router-cisco.glb
    switch-cisco.glb
    firewall-fortinet.glb
    gateway-zscaler.glb
```

### Alternative: Start with 2D SVG

If 3D is initially too complex, start with:
- Simple SVG icons for devices
- Canvas API via `web-sys` for connections
- Upgrade to 3D later without changing architecture

**2D Rendering Stack**:
- `web-sys` crate for Canvas API
- Draw nodes as circles/rectangles
- Draw connections as lines
- Handle mouse events for interaction
- All in Rust/WASM, no JavaScript

---

## Application Architecture

### Pure Rust Stack

```
network-topology-visualizer/
├── src/
│   ├── app.rs                    # Main app component (SSR shell)
│   ├── lib.rs                    # Hydration entry point
│   │
│   ├── islands/                  # Interactive WASM islands
│   │   ├── mod.rs
│   │   ├── topology_viewport.rs  # 3D viewer (three-d rendering)
│   │   ├── device_palette.rs     # Drag-drop device picker
│   │   ├── node_editor.rs        # Canvas-based editor
│   │   ├── traffic_monitor.rs    # Real-time SSE display
│   │   ├── properties_panel.rs   # Device configuration
│   │   └── export_dialog.rs      # Export UI
│   │
│   ├── components/               # Server-only components (no WASM)
│   │   ├── mod.rs
│   │   ├── navbar.rs             # Static navigation
│   │   ├── topology_card.rs      # Topology preview card
│   │   ├── documentation.rs      # Static docs
│   │   └── footer.rs
│   │
│   ├── pages/                    # Route pages (SSR)
│   │   ├── mod.rs
│   │   ├── home.rs               # Landing page
│   │   ├── viewer.rs             # View topology (3D island)
│   │   ├── editor.rs             # Edit topology (editor island)
│   │   ├── list.rs               # Browse topologies
│   │   └── admin.rs              # Admin panel
│   │
│   ├── models/                   # Shared data structures
│   │   ├── mod.rs
│   │   ├── topology.rs           # Topology, Device, Connection
│   │   ├── device_types.rs       # Enums for device types
│   │   └── config.rs             # Device configurations
│   │
│   ├── server_fns/               # Server functions (Leptos)
│   │   ├── mod.rs
│   │   ├── topology.rs           # CRUD operations
│   │   ├── export.rs             # Export to various formats
│   │   └── traffic.rs            # Traffic data streaming
│   │
│   └── rendering/                # 3D rendering helpers
│       ├── mod.rs
│       ├── model_loader.rs       # Load glTF via three-d
│       ├── scene_setup.rs        # Camera, lights, controls
│       └── materials.rs          # Material definitions
│
├── public/
│   ├── models/                   # glTF/GLB files from Blender
│   │   ├── devices/
│   │   └── icons/                # SVG fallbacks for 2D mode
│   └── styles/
│       └── main.css
│
├── migrations/                   # SQLite migrations
│   └── 001_initial.sql
│
├── Cargo.toml
├── Leptos.toml                   # Leptos configuration
└── README.md
```

### Technology Choices Explained

**Why three-d?**
- Pure Rust, no JavaScript
- Designed for WASM from the ground up
- Much simpler API than wgpu or Bevy
- Loads glTF/GLB natively
- Good for visualization (not a full game engine)
- Active development, good community

**Why SQLite?**
- Simple to start with (file-based)
- No separate database server needed
- Easy migration to PostgreSQL later if needed
- sqlx for compile-time checked queries
- Can scale to thousands of topologies

**Why Not Cloudflare Workers?**
- Workers adds deployment complexity
- D1 has limitations and is still evolving
- SQLite + regular server is simpler to develop
- Can always migrate later if needed

---

## Phase 1: Foundation (Week 1-2)

### 1.1 Project Setup with Islands Mode

**Initialize Project**:
```bash
# Create new Leptos project
cargo leptos new network-topology-visualizer
cd network-topology-visualizer

# Or manually create with the right structure
cargo new --bin network-topology-visualizer
```

**Cargo.toml Configuration**:
```toml
[package]
name = "network-topology-visualizer"
version = "0.1.0"
edition = "2021"

[dependencies]
# Leptos with islands feature
leptos = { version = "0.8", features = ["ssr", "islands"] }
leptos_meta = { version = "0.8", features = ["ssr"] }
leptos_router = { version = "0.8", features = ["ssr"] }
leptos_axum = { version = "0.8", optional = true }

# 3D Rendering (only for islands that need it)
three-d = { version = "0.17", features = ["egui-gui"] }
three-d-asset = "0.7"

# Database
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }

# Server
axum = { version = "0.7", optional = true }
tokio = { version = "1", features = ["full"], optional = true }
tower = { version = "0.4", optional = true }
tower-http = { version = "0.5", features = ["fs"], optional = true }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
tracing = "0.1"
console_error_panic_hook = "0.1"  # Better WASM error messages
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [
    "HtmlCanvasElement",
    "WebGl2RenderingContext",
    "CanvasRenderingContext2d",
    "MouseEvent",
    "KeyboardEvent",
] }

[features]
default = []
hydrate = [
    "leptos/hydrate",
    "leptos_meta/hydrate",
    "leptos_router/hydrate",
]
ssr = [
    "dep:axum",
    "dep:tokio",
    "dep:tower",
    "dep:tower-http",
    "dep:leptos_axum",
    "leptos/ssr",
    "leptos_meta/ssr",
    "leptos_router/ssr",
]

[[bin]]
name = "server"
path = "src/main.rs"
required-features = ["ssr"]

[lib]
crate-type = ["cdylib", "rlib"]

[profile.release]
opt-level = 'z'     # Optimize for size in WASM
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
panic = 'abort'     # Smaller WASM
strip = true        # Remove debug info
```

**Leptos.toml Configuration**:
```toml
[package]
name = "network-topology-visualizer"
version = "0.1.0"

[server]
port = 3000
output-name = "server"
site-root = "target/site"
site-pkg-dir = "pkg"
bin-features = ["ssr"]

[client]
bin-features = ["hydrate"]

# Islands mode configuration
islands = true
```

### 1.2 Hydration Setup for Islands

**src/lib.rs** (Client-side entry point):
```rust
use leptos::*;

pub mod app;
pub mod components;
pub mod islands;
pub mod models;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    // Better panic messages in WASM
    console_error_panic_hook::set_once();

    // In islands mode, this tells Leptos to stop looking for hydration
    // Each island hydrates itself independently
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
```

**src/main.rs** (Server entry point):
```rust
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use network_topology_visualizer::app::*;
    use tower_http::services::ServeDir;

    // Setup database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:topologies.db".to_string());

    let pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    // Leptos config
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Build application
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .nest_service("/public", ServeDir::new("public"))
        .with_state(leptos_options);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```

### 1.3 Data Models

**src/models/topology.rs**:
```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Topology {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyData {
    pub topology: Topology,
    pub devices: Vec<Device>,
    pub connections: Vec<Connection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: String,
    pub topology_id: String,
    pub device_type: DeviceType,
    pub vendor: Vendor,
    pub name: String,
    pub position: Position3D,
    pub config: Option<String>, // JSON string
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DeviceType {
    Router,
    Switch,
    Firewall,
    LoadBalancer,
    Server,
    Gateway,
    CloudService,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum Vendor {
    Cisco,
    Fortinet,
    Zscaler,
    PaloAlto,
    Meraki,
    AWS,
    Azure,
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Connection {
    pub id: String,
    pub topology_id: String,
    pub source_device_id: String,
    pub target_device_id: String,
    pub connection_type: ConnectionType,
    pub bandwidth: Option<u32>, // Mbps
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConnectionType {
    Ethernet,
    Fiber,
    VPN,
    Internet,
    MPLS,
    SDWAN,
}
```

### 1.4 Database Schema

**migrations/001_initial.sql**:
```sql
-- Topologies table
CREATE TABLE IF NOT EXISTS topologies (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Devices table
CREATE TABLE IF NOT EXISTS devices (
    id TEXT PRIMARY KEY,
    topology_id TEXT NOT NULL,
    device_type TEXT NOT NULL,
    vendor TEXT NOT NULL,
    name TEXT NOT NULL,
    position_x REAL NOT NULL,
    position_y REAL NOT NULL,
    position_z REAL NOT NULL,
    config TEXT,
    FOREIGN KEY (topology_id) REFERENCES topologies(id) ON DELETE CASCADE
);

-- Connections table
CREATE TABLE IF NOT EXISTS connections (
    id TEXT PRIMARY KEY,
    topology_id TEXT NOT NULL,
    source_device_id TEXT NOT NULL,
    target_device_id TEXT NOT NULL,
    connection_type TEXT NOT NULL,
    bandwidth INTEGER,
    FOREIGN KEY (topology_id) REFERENCES topologies(id) ON DELETE CASCADE,
    FOREIGN KEY (source_device_id) REFERENCES devices(id) ON DELETE CASCADE,
    FOREIGN KEY (target_device_id) REFERENCES devices(id) ON DELETE CASCADE
);

-- Indexes for common queries
CREATE INDEX idx_devices_topology ON devices(topology_id);
CREATE INDEX idx_connections_topology ON connections(topology_id);
CREATE INDEX idx_connections_source ON connections(source_device_id);
CREATE INDEX idx_connections_target ON connections(target_device_id);
```

### 1.5 Basic SSR Shell (No Islands Yet)

**src/app.rs**:
```rust
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::pages::*;
use crate::components::*;

// This is the shell function for islands mode
#[component]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                // CRITICAL: islands=true tells Leptos to use islands mode
                <HydrationScripts options=options islands=true/>
                <link rel="stylesheet" href="/public/styles/main.css"/>
                <title>"Network Topology Visualizer"</title>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

// Main app - mostly static SSR
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="description" content="Professional network topology visualization"/>
        <Router>
            <Navbar />  // Static component, no WASM
            <main class="main-content">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/topologies" view=TopologyListPage/>
                    <Route path="/topology/:id" view=ViewerPage/>
                    <Route path="/editor" view=EditorPage/>
                    <Route path="/editor/:id" view=EditorPage/>
                </Routes>
            </main>
            <Footer />  // Static component, no WASM
        </Router>
    }
}
```

**src/components/navbar.rs** (Regular component, NOT an island):
```rust
use leptos::*;
use leptos_router::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="nav-container">
                <A href="/" class="nav-brand">
                    "Network Topology Visualizer"
                </A>
                <div class="nav-links">
                    <A href="/">"Home"</A>
                    <A href="/topologies">"Browse"</A>
                    <A href="/editor">"Create New"</A>
                </div>
            </div>
        </nav>
    }
}
```

**src/components/footer.rs**:
```rust
use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer-content">
                <p>"Built with Rust + Leptos + three-d"</p>
                <p>"© 2024 Network Topology Visualizer"</p>
            </div>
        </footer>
    }
}
```

**src/pages/home.rs** (Static page, no islands yet):
```rust
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <section class="hero">
                <h1>"Network Topology Visualizer"</h1>
                <p>"Professional network diagram creation and visualization"</p>
                <div class="cta-buttons">
                    <a href="/editor" class="btn btn-primary">"Create Topology"</a>
                    <a href="/topologies" class="btn btn-secondary">"Browse Examples"</a>
                </div>
            </section>

            <section class="features">
                <h2>"Features"</h2>
                <div class="feature-grid">
                    <FeatureCard
                        title="3D Visualization"
                        description="Interactive 3D rendering of network topologies"
                    />
                    <FeatureCard
                        title="Drag & Drop"
                        description="Easy device placement and connection drawing"
                    />
                    <FeatureCard
                        title="Real-time Traffic"
                        description="Monitor traffic flows across your network"
                    />
                    <FeatureCard
                        title="Export"
                        description="Export to PNG, SVG, PDF, or JSON"
                    />
                </div>
            </section>
        </div>
    }
}

#[component]
fn FeatureCard(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="feature-card">
            <h3>{title}</h3>
            <p>{description}</p>
        </div>
    }
}
```

### 1.6 Development Commands

```bash
# Install cargo-leptos if not already installed
cargo install cargo-leptos

# Create the database
touch topologies.db

# Run migrations
sqlx migrate run

# Start development server (watches for changes)
cargo leptos watch

# Build for production
cargo leptos build --release

# The output will be in target/site/
# - pkg/ contains WASM files for each island
# - server binary to run
```

---

## Phase 2: 3D Viewer Island with three-d (Week 3-4)

### 2.1 Understanding the three-d Rendering Approach

**three-d** is a Rust 3D rendering library that:
- Compiles to WASM for web
- Uses WebGL2 under the hood
- Loads glTF/GLB models natively
- Has a simple, high-level API
- No JavaScript required

**Architecture**:
```
Leptos Island Component
    ↓
three-d Window/Context
    ↓
WebGL2 Canvas
    ↓
Browser Rendering
```

### 2.2 Create the 3D Viewport Island

**src/islands/topology_viewport.rs**:
```rust
use leptos::*;
use three_d::*;
use three_d_asset::io::load;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use crate::models::{TopologyData, Device, Position3D};

// This is an ISLAND - only this compiles to WASM
#[island]
pub fn TopologyViewport(
    topology_id: String,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    // Fetch topology data (this runs on mount)
    create_effect(move |_| {
        let topology_id = topology_id.clone();

        spawn_local(async move {
            match fetch_topology_data(&topology_id).await {
                Ok(data) => {
                    set_loading(false);

                    // Initialize 3D scene after data loads
                    if let Some(canvas) = canvas_ref.get() {
                        match init_3d_scene(canvas, data).await {
                            Ok(_) => {},
                            Err(e) => set_error(Some(format!("3D Error: {}", e))),
                        }
                    }
                },
                Err(e) => {
                    set_loading(false);
                    set_error(Some(format!("Load Error: {}", e)));
                }
            }
        });
    });

    view! {
        <div class="viewport-container">
            <Show
                when=move || !loading.get() && error.get().is_none()
                fallback=move || view! {
                    <div class="viewport-message">
                        {move || if loading.get() {
                            "Loading 3D scene...".to_string()
                        } else {
                            error.get().unwrap_or_else(|| "Error".to_string())
                        }}
                    </div>
                }
            >
                <canvas
                    _ref=canvas_ref
                    class="topology-canvas"
                    width="1200"
                    height="800"
                />
                <div class="viewport-controls">
                    <button class="btn-control" on:click=|_| reset_camera_view()>
                        "Reset View"
                    </button>
                    <button class="btn-control" on:click=|_| toggle_grid()>
                        "Toggle Grid"
                    </button>
                </div>
            </Show>
        </div>
    }
}

// Initialize three-d scene
async fn init_3d_scene(
    canvas: HtmlCanvasElement,
    topology: TopologyData
) -> Result<(), String> {
    // Create WebGL2 context
    let context = canvas
        .get_context("webgl2")
        .map_err(|_| "Failed to get WebGL2 context")?
        .ok_or("WebGL2 not supported")?
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .map_err(|_| "Failed to cast to WebGL2 context")?;

    // Create three-d context from WebGL2 context
    let three_d_context = Context::from_gl_context(
        std::sync::Arc::new(context)
    ).map_err(|e| format!("Failed to create three-d context: {:?}", e))?;

    // Setup camera
    let target = vec3(0.0, 0.0, 0.0);
    let scene_center = target;
    let scene_radius = 50.0;
    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(canvas.width(), canvas.height()),
        scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(),
        scene_center,
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );

    // Setup lighting
    let ambient_light = AmbientLight::new(&three_d_context, 0.4, Srgba::WHITE);
    let directional_light = DirectionalLight::new(
        &three_d_context,
        2.0,
        Srgba::WHITE,
        &vec3(0.0, -1.0, -1.0),
    );

    // Load device models and create scene
    let mut meshes = Vec::new();

    for device in &topology.devices {
        // Load the appropriate model for this device type
        let model_path = get_model_path(&device.device_type, &device.vendor);

        match load_device_model(&three_d_context, &model_path).await {
            Ok(mut model) => {
                // Position the model
                model.set_transformation(
                    Mat4::from_translation(vec3(
                        device.position.x,
                        device.position.y,
                        device.position.z,
                    ))
                );
                meshes.push(model);
            },
            Err(e) => {
                // Fallback to simple cube if model fails to load
                log::warn!("Failed to load model for {}: {}", device.name, e);
                let fallback = create_fallback_mesh(&three_d_context, &device.position);
                meshes.push(fallback);
            }
        }
    }

    // Draw connections as lines
    let connections = create_connection_lines(&three_d_context, &topology);

    // Render loop
    spawn_local(async move {
        loop {
            // Handle camera controls
            // In a real app, you'd handle mouse/keyboard input here

            // Clear and render
            Screen::write(
                &three_d_context,
                ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0),
                || {
                    // Render all meshes
                    for mesh in &meshes {
                        mesh.render(&camera, &[&ambient_light, &directional_light])?;
                    }

                    // Render connections
                    connections.render(&camera)?;

                    Ok(())
                },
            ).map_err(|e| format!("Render error: {:?}", e))?;

            // Simple animation frame
            gloo_timers::future::TimeoutFuture::new(16).await; // ~60fps
        }
    });

    Ok(())
}

// Load a glTF/GLB model
async fn load_device_model(
    context: &Context,
    model_path: &str,
) -> Result<Gm<Mesh, PhysicalMaterial>, String> {
    // Fetch the model file
    let response = gloo_net::http::Request::get(model_path)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch model: {}", e))?;

    let bytes = response
        .binary()
        .await
        .map_err(|e| format!("Failed to read model bytes: {}", e))?;

    // Load with three-d-asset
    let loaded = three_d_asset::io::load_from_memory(&bytes)
        .map_err(|e| format!("Failed to parse model: {:?}", e))?;

    // Get the first mesh from the model
    if let Some(model_data) = loaded.models.first() {
        let cpu_mesh = model_data.to_mesh()
            .map_err(|e| format!("Failed to convert to mesh: {:?}", e))?;

        let material = PhysicalMaterial::new(
            context,
            &CpuMaterial {
                albedo: Srgba::new(100, 150, 200, 255),
                roughness: 0.5,
                metallic: 0.2,
                ..Default::default()
            },
        );

        let mesh = Gm::new(
            Mesh::new(context, &cpu_mesh),
            material,
        );

        Ok(mesh)
    } else {
        Err("No mesh found in model".to_string())
    }
}

// Create fallback cube for missing models
fn create_fallback_mesh(
    context: &Context,
    position: &Position3D,
) -> Gm<Mesh, PhysicalMaterial> {
    let mut cpu_mesh = CpuMesh::cube();
    cpu_mesh.transform(&Mat4::from_translation(vec3(
        position.x,
        position.y,
        position.z,
    ))).unwrap();

    let material = PhysicalMaterial::new(
        context,
        &CpuMaterial {
            albedo: Srgba::new(200, 100, 100, 255),
            ..Default::default()
        },
    );

    Gm::new(Mesh::new(context, &cpu_mesh), material)
}

// Helper to get model path based on device type/vendor
fn get_model_path(device_type: &DeviceType, vendor: &Vendor) -> String {
    use crate::models::{DeviceType, Vendor};

    match (device_type, vendor) {
        (DeviceType::Router, Vendor::Cisco) => "/public/models/devices/router-cisco.glb",
        (DeviceType::Switch, Vendor::Cisco) => "/public/models/devices/switch-cisco.glb",
        (DeviceType::Firewall, Vendor::Fortinet) => "/public/models/devices/firewall-fortinet.glb",
        (DeviceType::Gateway, Vendor::Zscaler) => "/public/models/devices/gateway-zscaler.glb",
        _ => "/public/models/devices/generic.glb",
    }.to_string()
}

// Create lines for connections
fn create_connection_lines(
    context: &Context,
    topology: &TopologyData,
) -> Lines {
    let mut positions = Vec::new();

    // Create a map of device IDs to positions for quick lookup
    let device_positions: std::collections::HashMap<_, _> = topology
        .devices
        .iter()
        .map(|d| (d.id.clone(), &d.position))
        .collect();

    // For each connection, add line vertices
    for conn in &topology.connections {
        if let (Some(source_pos), Some(target_pos)) = (
            device_positions.get(&conn.source_device_id),
            device_positions.get(&conn.target_device_id),
        ) {
            positions.push(vec3(source_pos.x, source_pos.y, source_pos.z));
            positions.push(vec3(target_pos.x, target_pos.y, target_pos.z));
        }
    }

    Lines::new(
        context,
        &CpuMesh {
            positions: Positions::F32(positions),
            ..Default::default()
        },
        1.0, // line width
        Srgba::new(50, 100, 200, 255), // blue lines
    )
}

// Fetch topology data from server
async fn fetch_topology_data(topology_id: &str) -> Result<TopologyData, String> {
    // This would use a Leptos server function
    // For now, placeholder
    todo!("Implement server function to fetch topology")
}

// Camera control functions (called from UI buttons)
#[wasm_bindgen]
pub fn reset_camera_view() {
    // Implementation to reset camera
    log::info!("Resetting camera view");
}

#[wasm_bindgen]
pub fn toggle_grid() {
    // Implementation to show/hide grid
    log::info!("Toggling grid");
}
```

### 2.3 Server Function for Loading Topology

**src/server_fns/topology.rs**:
```rust
use leptos::*;
use crate::models::*;

#[server(LoadTopology, "/api")]
pub async fn load_topology(topology_id: String) -> Result<TopologyData, ServerFnError> {
    use sqlx::SqlitePool;

    // Get database pool from context (set up in main.rs)
    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::ServerError("No database pool".to_string()))?;

    // Load topology metadata
    let topology = sqlx::query_as::<_, Topology>(
        "SELECT * FROM topologies WHERE id = ?"
    )
    .bind(&topology_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::ServerError(format!("Database error: {}", e)))?;

    // Load devices
    let devices = sqlx::query_as::<_, Device>(
        "SELECT id, topology_id, device_type, vendor, name,
                position_x, position_y, position_z, config
         FROM devices WHERE topology_id = ?"
    )
    .bind(&topology_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::ServerError(format!("Database error: {}", e)))?;

    // Load connections
    let connections = sqlx::query_as::<_, Connection>(
        "SELECT * FROM connections WHERE topology_id = ?"
    )
    .bind(&topology_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::ServerError(format!("Database error: {}", e)))?;

    Ok(TopologyData {
        topology,
        devices,
        connections,
    })
}
```

### 2.4 Using the Island in a Page

**src/pages/viewer.rs**:
```rust
use leptos::*;
use leptos_router::*;
use crate::islands::TopologyViewport;

#[component]
pub fn ViewerPage() -> impl IntoView {
    let params = use_params_map();
    let topology_id = move || {
        params.with(|p| p.get("id").cloned().unwrap_or_default())
    };

    view! {
        <div class="viewer-page">
            // This header is static SSR
            <div class="viewer-header">
                <h1>"Network Topology Viewer"</h1>
                <a href="/topologies" class="btn-back">"← Back to List"</a>
            </div>

            // This is the island - only this becomes WASM
            <TopologyViewport topology_id=topology_id()/>

            // This is also static SSR
            <div class="viewer-info">
                <p>"Use mouse to rotate, scroll to zoom"</p>
            </div>
        </div>
    }
}
```

### 2.5 Alternative: Simple 2D Canvas Approach

If 3D is too complex initially, start with 2D using web-sys Canvas:

**src/islands/topology_canvas_2d.rs**:
```rust
use leptos::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsCast;

#[island]
pub fn TopologyCanvas2D(topology_id: String) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            if let Ok(context) = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
            {
                draw_topology_2d(&context, &topology_id);
            }
        }
    });

    view! {
        <canvas
            _ref=canvas_ref
            width="1200"
            height="800"
            class="topology-canvas-2d"
        />
    }
}

fn draw_topology_2d(ctx: &CanvasRenderingContext2d, _topology_id: &str) {
    // Clear canvas
    ctx.clear_rect(0.0, 0.0, 1200.0, 800.0);

    // Draw a simple example
    // In real app, load data and draw devices as circles, connections as lines

    // Draw device
    ctx.set_fill_style(&"#4A90E2".into());
    ctx.begin_path();
    ctx.arc(400.0, 300.0, 30.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    ctx.fill();

    // Draw another device
    ctx.begin_path();
    ctx.arc(800.0, 300.0, 30.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    ctx.fill();

    // Draw connection
    ctx.set_stroke_style(&"#333".into());
    ctx.set_line_width(2.0);
    ctx.begin_path();
    ctx.move_to(400.0, 300.0);
    ctx.line_to(800.0, 300.0);
    ctx.stroke();

    // Draw labels
    ctx.set_fill_style(&"#000".into());
    ctx.set_font("14px sans-serif");
    ctx.fill_text("Router A", 380.0, 350.0).unwrap();
    ctx.fill_text("Router B", 780.0, 350.0).unwrap();
}
```

This 2D approach is much simpler and still uses pure Rust!

---

## Phase 3: UI Layout & 3D Editing Interface (Week 5-6)

> ⚠️ **OUTDATED SECTION BELOW**
> This section describes a 2D canvas-based editor approach which is NO LONGER USED.
>
> **Current Approach:** 3D viewport-based editing with UI panels
> - Professional layout: 3D viewport (center), device palette (left/top), properties panel (right), toolbar (top)
> - All editing happens directly in the 3D viewport (click to select, drag to connect, etc.)
> - No separate 2D canvas editor needed
>
> **See [CLAUDE.md Phase 3](CLAUDE.md#phase-3---ui-layout--3d-editing-interface-next) for current requirements.**

---

### 3.1 Canvas-Based Editor Island (OUTDATED - 2D approach)

**src/islands/topology_editor.rs**:
```rust
use leptos::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use wasm_bindgen::JsCast;
use crate::models::*;

#[island]
pub fn TopologyEditor(topology_id: Option<String>) -> impl IntoView {
    let canvas_ref = create_node_ref::<html::Canvas>();

    let (devices, set_devices) = create_signal(Vec::<Device>::new());
    let (connections, set_connections) = create_signal(Vec::<Connection>::new());
    let (selected_device_type, set_selected_device_type) = create_signal(None::<DeviceType>);
    let (selected_device_id, set_selected_device_id) = create_signal(None::<String>);

    // Handle canvas click to place devices
    let handle_canvas_click = move |e: MouseEvent| {
        if let (Some(canvas), Some(device_type)) = (
            canvas_ref.get(),
            selected_device_type.get()
        ) {
            let rect = canvas.get_bounding_client_rect();
            let x = e.client_x() as f32 - rect.left() as f32;
            let y = e.client_y() as f32 - rect.top() as f32;

            // Create new device
            let device = Device {
                id: uuid::Uuid::new_v4().to_string(),
                topology_id: topology_id.clone().unwrap_or_default(),
                device_type,
                vendor: Vendor::Generic,
                name: format!("Device {}", devices.get().len() + 1),
                position: Position3D { x, y, z: 0.0 },
                config: None,
            };

            set_devices.update(|d| d.push(device));
            redraw_canvas(&canvas, &devices.get(), &connections.get());
        }
    };

    // Render the canvas whenever devices/connections change
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            redraw_canvas(&canvas, &devices.get(), &connections.get());
        }
    });

    view! {
        <div class="editor-container">
            <div class="device-palette">
                <h3>"Device Library"</h3>
                <DevicePaletteGrid on_select=set_selected_device_type/>
            </div>

            <div class="editor-canvas-container">
                <canvas
                    _ref=canvas_ref
                    width="1200"
                    height="800"
                    class="editor-canvas"
                    on:click=handle_canvas_click
                />
            </div>

            <div class="properties-panel">
                <h3>"Properties"</h3>
                <Show
                    when=move || selected_device_id.get().is_some()
                    fallback=|| view! { <p>"Select a device to edit"</p> }
                >
                    <DevicePropertiesForm device_id=selected_device_id.get().unwrap()/>
                </Show>
            </div>

            <div class="editor-actions">
                <button class="btn-primary" on:click=move |_| save_topology(
                    topology_id.clone(),
                    devices.get(),
                    connections.get()
                )>
                    "Save Topology"
                </button>
                <button class="btn-secondary" on:click=move |_| clear_canvas()>
                    "Clear All"
                </button>
            </div>
        </div>
    }
}

#[component]
fn DevicePaletteGrid(on_select: WriteSignal<Option<DeviceType>>) -> impl IntoView {
    view! {
        <div class="device-grid">
            <button
                class="device-item"
                on:click=move |_| on_select.set(Some(DeviceType::Router))
            >
                "Router"
            </button>
            <button
                class="device-item"
                on:click=move |_| on_select.set(Some(DeviceType::Switch))
            >
                "Switch"
            </button>
            <button
                class="device-item"
                on:click=move |_| on_select.set(Some(DeviceType::Firewall))
            >
                "Firewall"
            </button>
            <button
                class="device-item"
                on:click=move |_| on_select.set(Some(DeviceType::Server))
            >
                "Server"
            </button>
        </div>
    }
}

fn redraw_canvas(
    canvas: &HtmlCanvasElement,
    devices: &[Device],
    connections: &[Connection],
) {
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Clear canvas
    ctx.clear_rect(0.0, 0.0, 1200.0, 800.0);

    // Draw grid
    ctx.set_stroke_style(&"#e0e0e0".into());
    ctx.set_line_width(0.5);
    for i in 0..24 {
        let x = (i * 50) as f64;
        ctx.begin_path();
        ctx.move_to(x, 0.0);
        ctx.line_to(x, 800.0);
        ctx.stroke();
    }
    for i in 0..16 {
        let y = (i * 50) as f64;
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(1200.0, y);
        ctx.stroke();
    }

    // Draw connections first (under devices)
    ctx.set_stroke_style(&"#4A90E2".into());
    ctx.set_line_width(2.0);

    let device_map: std::collections::HashMap<_, _> =
        devices.iter().map(|d| (&d.id, &d.position)).collect();

    for conn in connections {
        if let (Some(source), Some(target)) = (
            device_map.get(&conn.source_device_id),
            device_map.get(&conn.target_device_id),
        ) {
            ctx.begin_path();
            ctx.move_to(source.x as f64, source.y as f64);
            ctx.line_to(target.x as f64, target.y as f64);
            ctx.stroke();
        }
    }

    // Draw devices
    for device in devices {
        // Draw device as circle
        ctx.set_fill_style(&get_device_color(&device.device_type).into());
        ctx.begin_path();
        ctx.arc(
            device.position.x as f64,
            device.position.y as f64,
            25.0,
            0.0,
            2.0 * std::f64::consts::PI,
        ).unwrap();
        ctx.fill();

        // Draw label
        ctx.set_fill_style(&"#000".into());
        ctx.set_font("12px sans-serif");
        ctx.fill_text(
            &device.name,
            (device.position.x - 20.0) as f64,
            (device.position.y + 40.0) as f64,
        ).unwrap();
    }
}

fn get_device_color(device_type: &DeviceType) -> &'static str {
    match device_type {
        DeviceType::Router => "#4A90E2",
        DeviceType::Switch => "#50C878",
        DeviceType::Firewall => "#E24A4A",
        DeviceType::Server => "#9B59B6",
        DeviceType::LoadBalancer => "#F39C12",
        DeviceType::Gateway => "#1ABC9C",
        DeviceType::CloudService => "#95A5A6",
    }
}
```

### 3.2 Save Topology Server Function

**src/server_fns/topology.rs** (add to existing file):
```rust
#[server(SaveTopology, "/api")]
pub async fn save_topology(
    topology_id: Option<String>,
    name: String,
    devices: Vec<Device>,
    connections: Vec<Connection>,
) -> Result<String, ServerFnError> {
    use sqlx::SqlitePool;

    let pool = use_context::<SqlitePool>()
        .ok_or_else(|| ServerFnError::ServerError("No database pool".to_string()))?;

    let id = topology_id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    // Begin transaction
    let mut tx = pool.begin().await?;

    // Insert or update topology
    sqlx::query(
        "INSERT INTO topologies (id, name, description, created_at, updated_at)
         VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
         ON CONFLICT(id) DO UPDATE SET
            name = excluded.name,
            updated_at = CURRENT_TIMESTAMP"
    )
    .bind(&id)
    .bind(&name)
    .bind(Option::<String>::None)
    .execute(&mut *tx)
    .await?;

    // Delete old devices and connections
    sqlx::query("DELETE FROM devices WHERE topology_id = ?")
        .bind(&id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM connections WHERE topology_id = ?")
        .bind(&id)
        .execute(&mut *tx)
        .await?;

    // Insert new devices
    for device in devices {
        sqlx::query(
            "INSERT INTO devices (id, topology_id, device_type, vendor, name,
                                  position_x, position_y, position_z, config)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&device.id)
        .bind(&id)
        .bind(&device.device_type)
        .bind(&device.vendor)
        .bind(&device.name)
        .bind(device.position.x)
        .bind(device.position.y)
        .bind(device.position.z)
        .bind(&device.config)
        .execute(&mut *tx)
        .await?;
    }

    // Insert new connections
    for conn in connections {
        sqlx::query(
            "INSERT INTO connections (id, topology_id, source_device_id,
                                     target_device_id, connection_type, bandwidth)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&conn.id)
        .bind(&id)
        .bind(&conn.source_device_id)
        .bind(&conn.target_device_id)
        .bind(&conn.connection_type)
        .bind(conn.bandwidth)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(id)
}
```

---

## Phase 4: Real-Time Traffic Monitoring (Week 7)

### 4.1 SSE-Based Traffic Island

**src/islands/traffic_monitor.rs**:
```rust
use leptos::*;
use web_sys::{EventSource, MessageEvent};
use wasm_bindgen::prelude::*;

#[island]
pub fn TrafficMonitor(topology_id: String) -> impl IntoView {
    let (traffic_data, set_traffic_data) = create_signal(Vec::<TrafficData>::new());
    let (is_monitoring, set_is_monitoring) = create_signal(false);
    let (event_source, set_event_source) = create_signal(None::<EventSource>);

    let start_monitoring = move |_| {
        let url = format!("/api/traffic/{}", topology_id);

        match EventSource::new(&url) {
            Ok(es) => {
                // Setup message handler
                let set_traffic_clone = set_traffic_data.clone();
                let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Some(data) = e.data().as_string() {
                        if let Ok(traffic) = serde_json::from_str::<TrafficData>(&data) {
                            set_traffic_clone.update(|td| {
                                td.push(traffic);
                                // Keep only last 100 data points
                                if td.len() > 100 {
                                    td.drain(0..td.len()-100);
                                }
                            });
                        }
                    }
                }) as Box<dyn FnMut(_)>);

                es.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                onmessage.forget();

                set_event_source(Some(es));
                set_is_monitoring(true);
            },
            Err(e) => {
                log::error!("Failed to create EventSource: {:?}", e);
            }
        }
    };

    let stop_monitoring = move |_| {
        if let Some(es) = event_source.get() {
            es.close();
        }
        set_event_source(None);
        set_is_monitoring(false);
    };

    view! {
        <div class="traffic-monitor">
            <div class="monitor-controls">
                <button
                    class="btn-primary"
                    on:click=start_monitoring
                    disabled=move || is_monitoring.get()
                >
                    "Start Monitoring"
                </button>
                <button
                    class="btn-secondary"
                    on:click=stop_monitoring
                    disabled=move || !is_monitoring.get()
                >
                    "Stop"
                </button>
            </div>

            <div class="traffic-stats">
                <For
                    each=move || traffic_data.get()
                    key=|td| td.timestamp
                    children=|td| {
                        view! {
                            <div class="stat-row">
                                <span>{&td.connection_name}</span>
                                <span class="stat-value">{td.throughput}" Mbps"</span>
                                <span class="stat-time">{&td.timestamp}</span>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TrafficData {
    connection_name: String,
    throughput: f64,
    timestamp: String,
}
```

### 4.2 SSE Server Endpoint

**src/main.rs** (add to router):
```rust
use axum::{
    response::Sse,
    response::sse::{Event, KeepAlive},
};
use futures::stream::{self, Stream};
use std::time::Duration;

async fn traffic_stream(
    Path(topology_id): Path<String>
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // Generate fake traffic data for demo
    let stream = stream::repeat_with(move || {
        // In real app, query actual traffic metrics
        let data = TrafficData {
            connection_name: "Router A → Router B".to_string(),
            throughput: (rand::random::<f64>() * 100.0),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        Event::default().json_data(data)
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// Add to router:
// .route("/api/traffic/:id", get(traffic_stream))
```

---

## Phase 5: Export & Polish (Week 8)

### 5.1 Export Island

**src/islands/export_dialog.rs**:
```rust
use leptos::*;
use web_sys::HtmlCanvasElement;

#[island]
pub fn ExportDialog(topology_id: String) -> impl IntoView {
    let (format, set_format) = create_signal("png".to_string());
    let (exporting, set_exporting) = create_signal(false);

    let handle_export = move |_| {
        set_exporting(true);
        let format_val = format.get();

        spawn_local(async move {
            match format_val.as_str() {
                "png" => export_as_png(&topology_id).await,
                "svg" => export_as_svg(&topology_id).await,
                "json" => export_as_json(&topology_id).await,
                _ => {}
            }
            set_exporting(false);
        });
    };

    view! {
        <div class="export-dialog">
            <h3>"Export Topology"</h3>
            <select on:change=move |e| set_format(event_target_value(&e))>
                <option value="png">"PNG Image"</option>
                <option value="svg">"SVG Vector"</option>
                <option value="json">"JSON Data"</option>
            </select>

            <button
                class="btn-primary"
                on:click=handle_export
                disabled=move || exporting.get()
            >
                {move || if exporting.get() { "Exporting..." } else { "Export" }}
            </button>
        </div>
    }
}

async fn export_as_png(topology_id: &str) {
    // Get canvas, convert to blob, trigger download
    // Implementation uses web-sys APIs
}

async fn export_as_svg(topology_id: &str) {
    // Generate SVG representation, trigger download
}

async fn export_as_json(topology_id: &str) {
    // Export topology data as JSON
}
```

---

## Islands Architecture Summary

### What Gets Compiled to WASM

**Islands (WASM bundles)**:
- `TopologyViewport` → ~300KB (includes three-d)
- `TopologyEditor` → ~150KB (canvas operations)
- `TrafficMonitor` → ~80KB (SSE handling)
- `ExportDialog` → ~60KB (export logic)
- `DevicePropertiesForm` → ~50KB (forms)

**Static SSR (no WASM)**:
- Navbar, Footer, HomePage
- Topology list page
- Documentation pages
- All static text/images

### Bundle Size Optimization

With islands, total WASM across ALL islands: ~640KB
Without islands (full hydration): ~1.2MB

**Code splitting means**:
- Initial page load: 0 KB WASM (just HTML)
- View page loads: Only TopologyViewport island (~300KB)
- Edit page loads: Only TopologyEditor island (~150KB)
- User never downloads unused code

---

## Deployment Instructions

### Build for Production

```bash
# Build optimized release version
cargo leptos build --release

# Output structure:
target/site/
  ├── server              # Server binary
  ├── pkg/               # WASM files (one per island)
  │   ├── TopologyViewport.wasm
  │   ├── TopologyEditor.wasm
  │   └── TrafficMonitor.wasm
  └── public/            # Static assets
      └── models/        # glTF/GLB files
```

### Run the Server

```bash
# Set database path
export DATABASE_URL="sqlite:topologies.db"

# Create database and run migrations
sqlx migrate run

# Start the server
./target/site/server

# Server will run on http://localhost:3000 by default
```

### Optional: Nginx Reverse Proxy

If deploying behind Nginx for SSL/domain:

```nginx
server {
    listen 80;
    server_name topology.example.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
    }

    location /public/ {
        alias /path/to/target/site/public/;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

---

## Implementation Notes

### Islands Architecture Reminders

- Only components marked with `#[island]` compile to WASM
- Each island creates a separate WASM bundle that loads on-demand
- Server-rendered components use `#[component]` and stay as static HTML
- Islands can communicate via context and signals
- Keep islands small and focused for optimal bundle sizes

### Code Splitting Verification

After each build, verify code splitting is working:

```bash
# Build and check output
cargo leptos build --release
ls -lh target/site/pkg/*.wasm

# Expected output: Multiple WASM files
# TopologyViewport-[hash].wasm
# TopologyEditor-[hash].wasm
# TrafficMonitor-[hash].wasm
```

If you only see one large WASM file, islands mode is not configured correctly.

### Server Functions Best Practices

- Use `#[server]` for ALL backend operations
- Access database pool via `use_context::<SqlitePool>()`
- Return `Result<T, ServerFnError>` from all server functions
- Call server functions from islands using `create_resource()`
- Handle loading states with `<Suspense>` component

**Example Pattern**:
```rust
// Define server function
#[server(GetData, "/api")]
pub async fn get_data(id: String) -> Result<Data, ServerFnError> {
    let pool = use_context::<SqlitePool>().unwrap();
    // Query database...
}

// Call from island
#[island]
fn MyIsland(id: String) -> impl IntoView {
    let data = create_resource(
        move || id.clone(),
        |id| async move { get_data(id).await }
    );

    view! {
        <Suspense fallback=|| "Loading...">
            {move || data.get().map(|d| /* render */)}
        </Suspense>
    }
}
```

### Development Workflow

1. Start with static SSR components for layout and navigation
2. Identify interactive features that need client-side state
3. Convert those specific components to islands
4. Write server functions for all backend operations
5. Test that non-interactive parts stay as static HTML
6. Verify code splitting by checking multiple WASM files in pkg/
7. Monitor individual island bundle sizes

### Blender Model Requirements

- Export format: glTF 2.0 Binary (.glb)
- Polygon count: < 5,000 per model
- Single material per model preferred
- Origin point centered for easy positioning
- Scale: 1 Blender unit = 1 meter (consistent sizing)

---

## END OF IMPLEMENTATION PLAN

This plan provides the complete technical specification for building the Network Topology Visualizer. Implement each phase sequentially, testing thoroughly before moving to the next phase. All code must be written in Rust with no JavaScript dependencies.
