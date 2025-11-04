# Network Topology Visualizer - Session Continuation Guide

## 🎯 Starting a New Conversation

**Use this prompt to continue from where we left off:**

```
I'm continuing work on the Network Topology Visualizer project at:
/Users/mattearp/Documents/CodeProjects/ntv/

Please read these files to understand the current state:
1. CLAUDE.md - Complete architecture, Phases 1-3 status, all learnings
2. This file (SESSION-GUIDE.md) - Quick context

Current Status: Phase 4 IN PROGRESS - 3D Model Rotation Complete! ✅
- ✅ Professional 3-panel UI layout working perfectly
- ✅ Node selection with visual feedback (yellow highlight)
- ✅ Click empty space to deselect
- ✅ Full CRUD server functions for nodes and connections
- ✅ Properties panel loads and saves real data
- ✅ Real-time viewport updates (no refresh needed!)
- ✅ Suspense components eliminating hydration warnings
- ✅ **NEW: 3D node rotation controls (X/Y/Z in degrees)**
- ✅ **NEW: Default rotation_x=90° for Blender glTF models**

Next: [specify what you want to work on - see Phase 4 remaining options below]
```

## 📊 Current Project State

### ✅ Completed Phases

**Phase 1 - Foundation (Git tag: v0.1.0-phase1-complete)**
- Leptos 0.8 with SQLite database and migrations
- Server functions in `src/api.rs` (non-feature-gated)
- Database schema: topologies, nodes, connections, traffic_metrics
- **Note:** Originally used islands, removed in Phase 3

**Phase 2 - 3D Viewport (Git tag: v0.1.0-phase2-complete)**
- TopologyViewport component with WebGL2 + three-d
- Interactive orbit camera controls (drag to rotate, scroll to zoom)
- Nodes rendered as 3D spheres at database positions
- Connections rendered as properly rotated cylinders
- Sample topology with 7 nodes and 7 connections

**Phase 3 - UI Layout & 3D Editing (Git tag: v0.1.0-phase3-complete)**
- ✅ Architecture change: Removed islands, using regular Leptos components
- ✅ Context-based state sharing (`provide_context` / `use_context`)
- ✅ Professional 3-panel layout (device palette, viewport, properties)
- ✅ Top toolbar with action buttons
- ✅ Node selection via ray-sphere intersection with visual feedback (yellow highlight)
- ✅ Click empty space to deselect
- ✅ Complete CRUD server functions (8 total: 4 for nodes, 4 for connections)
- ✅ Properties panel loads real data via Resources with Suspense
- ✅ Save changes from properties panel with instant viewport updates
- ✅ Refetch mechanism using context-shared trigger signal

**Phase 4 - Visual Enhancements & 3D Interaction (IN PROGRESS)**

✅ **COMPLETED: 3D Model Rotation Controls (2025-11-04)**
- ✅ Database migration: Added rotation_x/y/z columns to nodes table
- ✅ Updated Node model with rotation fields (stored in degrees)
- ✅ Full CRUD API support for rotation values
- ✅ Properties panel UI with X/Y/Z rotation sliders (-180° to +180°)
- ✅ Viewport rendering applies rotations using cgmath `degrees()` function
- ✅ Default rotation_x=90° for Blender glTF models (correct orientation on Z-up grid)
- ✅ **Key lesson:** cgmath `degrees()` converts from degrees to radians, `radians()` just wraps radian values
- ✅ **Troubleshooting:** Clean rebuild (`cargo clean`) + hard browser refresh fixed rendering issues

Files modified:
- `migrations/20250102000002_add_node_rotations.sql`
- `src/models/node.rs` - Added rotation fields to Node, CreateNode, UpdateNode
- `src/api.rs` - Updated all CRUD functions, set default rotation_x=90.0
- `src/islands/topology_editor.rs` - Added rotation UI controls
- `src/islands/topology_viewport.rs` - Applied rotations with degrees() fix

### 🔄 What to Work On Next

**Option 1: Phase 4 - 3D Interaction Enhancements**
```
Let's add interactive 3D features. I want to:
- Implement drag-to-move nodes in 3D viewport
  * Click and drag nodes to reposition them
  * Update position in real-time during drag
  * Save to database on drag end
  * Differentiate node-drag from camera-drag
- Add 3D grid and axes (Blender-style)
  * Faint X, Y, Z axis lines (red, green, blue)
  * Grid floor plane at Y=0 with subtle lines
  * Helps with spatial orientation
```

**Option 2: Phase 4 - Visual Enhancements**
```
Let's enhance the 3D visualization. I want to:
- Show node labels/tooltips on hover
- Color-code nodes by type (router=blue, switch=green, etc.)
- Load custom 3D models from Blender (glTF/GLB files)
- Improve lighting and materials
- Better camera controls (presets, bookmarks)
```

**Option 3: Traffic Monitoring (Leptos Streaming)**
```
Let's add real-time traffic monitoring using Leptos native streaming:
- Use #[server(protocol = Websocket<...>)] for streaming
- Display traffic data on connections in real-time
- No manual Axum SSE needed!
```

**Option 4: Export Functionality**
```
Let's add export features:
- Export topology as PNG image
- Export topology as JSON data
- Provide download mechanism
```

## 📁 Key Files to Reference

### Primary Documentation
- **CLAUDE.md** (490 lines) - Complete architecture reference, all phases, all learnings
- **network-topology-visualizer-plan.md** (2326 lines) - Original detailed plan with corrections

### Code Structure
```
src/
├── app.rs              # Main SSR shell, routing
├── lib.rs              # Hydration entry point
├── main.rs             # Server entry point
│
├── api.rs              # ✅ Server functions (accessible from client & server)
│
├── islands/
│   ├── mod.rs
│   ├── counter.rs      # Test island
│   ├── simple_button.rs # Test island
│   └── topology_viewport.rs  # ✅ 3D viewport with three-d
│
├── models/
│   ├── mod.rs
│   ├── topology.rs     # Topology, TopologyFull
│   ├── node.rs         # Node model
│   ├── connection.rs   # Connection model
│   └── traffic.rs      # Traffic metrics
│
└── server/             # ⚠️ Old implementation (feature-gated)
    └── topology_api.rs # Moved to api.rs

migrations/
└── 001_init.sql        # Database schema

public/
└── models/             # Future: glTF/GLB 3D models from Blender
```

### Database Sample Data
```sql
-- 1 topology
INSERT INTO topologies (name, description) VALUES ('Test Network', 'Sample 3D network');

-- 7 nodes with 3D positions
Router-Core (0,0,0), Switch-A (-3,2,0), Switch-B (3,2,0),
Server-1/2/3 (varying x, y=4, z=-2), Firewall (0,-3,0)

-- 7 connections
Router connects to switches and firewall
Switches connect to servers
```

## 🎓 Critical Architecture Patterns

### 1. Server Functions (MUST be in non-feature-gated module!)
```rust
// src/api.rs - NOT behind #[cfg(feature = "ssr")]
#[server(MyFunction, "/api")]
pub async fn my_function(id: i64) -> Result<Data, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::extract;
        use axum::Extension;

        let Extension(pool) = extract::<Extension<SqlitePool>>().await?.0;
        // Database operations...
    }
}
```

### 2. Islands (NOT #[lazy] for reactive components!)
```rust
// src/islands/my_island.rs
use crate::api::my_function;  // ✅ Works because api.rs not feature-gated

#[island]  // ✅ NOT #[lazy] - doesn't work with Effects/Resources
pub fn MyIsland() -> impl IntoView {
    let data = Resource::new(
        || (),
        |_| async move { my_function(1).await }
    );

    view! { /* reactive UI */ }
}
```

### 3. Browser Console Logging from WASM
```rust
// Add to Cargo.toml
web-sys = { version = "0.3", features = ["console", ...] }

// In code
web_sys::console::log_1(&format!("Value: {}", x).into());
```

### 4. three-d WITHOUT Window Module
```rust
// Get WebGL2, wrap in glow, create three-d Context
let gl = canvas.get_context("webgl2")?.dyn_into::<WebGl2RenderingContext>()?;
let gl_context = three_d::context::Context::from_webgl2_context(gl);
let context = Context::from_gl_context(Arc::new(gl_context))?;

// Now use three-d core API for rendering
```

## ⚙️ Development Commands

### First Time Setup
```bash
# Download Tailwind CSS standalone CLI (macOS ARM64)
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-macos-arm64
chmod +x tailwindcss-macos-arm64
mv tailwindcss-macos-arm64 tailwindcss

# Verify installation
./tailwindcss --help
```

### Development (Dual Terminal)
```bash
# Terminal 1: Tailwind CSS watch mode
./tailwindcss -i style/input.css -o style/output.css --watch

# Terminal 2: Leptos development server with hot reload
cargo leptos watch
```

### Production Build
```bash
# Build CSS first
./tailwindcss -i style/input.css -o style/output.css --minify

# Build application
cargo leptos build --release

# Check WASM output
ls -lh target/site/pkg/*.wasm

# Run server manually
./target/site/server
```

### Database Operations
```bash
# Open database
sqlite3 ntv.db

# Run migrations
sqlx migrate run
```

## 🐛 Common Issues & Solutions

See CLAUDE.md "Known Issues & Solutions" section for:
1. Server function database access (use leptos_axum::extract)
2. Server functions not accessible from islands (use api.rs)
3. Islands code splitting with #[lazy] (doesn't work with reactive code)
4. wasm-bindgen version mismatch (pin to =0.2.101)
5. JsCast import (use `wasm_bindgen::JsCast`)
6. SQLite database creation (use create_if_missing(true))

## 🔗 Useful References

**Leptos Documentation:**
- Use Context7 MCP: `mcp__context7__get-library-docs` with `/websites/book_leptos_dev`
- Islands architecture: https://book.leptos.dev/islands.html
- Server functions: https://book.leptos.dev/server/25_server_functions.html

**three-d Documentation:**
- Repository: https://github.com/asny/three-d
- Examples: https://github.com/asny/three-d/tree/master/examples

**Project Repository:**
- GitHub: https://github.com/madkrell/ntv.git

## 📋 Quick Status Check

Before starting work, verify:
```bash
cd /Users/mattearp/Documents/CodeProjects/ntv
cargo leptos watch  # Should compile without errors
# Visit http://127.0.0.1:3000
# Should see:
# - 3-panel UI: Device palette (left), 3D viewport (center), Properties (right)
# - 7 nodes and connections in 3D viewport
# - Click a node to select it (turns yellow)
# - Properties panel loads node data
# - Edit properties and click Save - viewport updates instantly!
```

## 🎬 Example Session Start Prompts

### Continue with Phase 4 (3D Interaction & Visual Enhancements)
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md for complete context. Phase 3 is complete!

Let's build Phase 4: 3D Interaction Enhancements
- Add drag-to-move functionality for nodes in 3D viewport
- Add 3D grid and axis lines (Blender-style) for spatial reference
- (Optional) Node labels/tooltips, color-coding by type, custom 3D models

Where should we start?
```

### Add Specific Feature
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md for complete context. Phase 2 is complete.

I want to add [specific feature]:
[describe what you want]

How should we approach this?
```

### Fix or Improve Something
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md for complete context. Phase 3 is complete.

I'm seeing [issue description] or want to improve [feature name].

Can you help me [what you want to do]?
```

## 💡 Pro Tips

1. **Always read CLAUDE.md first** - Contains all architectural discoveries and solutions
2. **Use Context7 MCP** - When unsure about Leptos/three-d patterns, check `/websites/book_leptos_dev`
3. **Check git tags** - `git tag` shows v0.1.0-phase1-complete, v0.1.0-phase2-complete, v0.1.0-phase3-complete
4. **Test in browser** - http://127.0.0.1:3000 to see current state
5. **Console logs** - Browser console shows WASM logs from `web_sys::console`
6. **Real-time updates work!** - Save node positions in properties panel, viewport updates instantly

## 🚀 You're Ready!

Phase 3 is complete! Pick a Phase 4 task above and start coding. All the architectural patterns are working and documented in CLAUDE.md.
