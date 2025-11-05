# Network Topology Visualizer - Session Continuation Guide

## 🎯 Starting a New Conversation

**Use this prompt to continue from where we left off:**

```
I'm continuing work on the Network Topology Visualizer project at:
/Users/mattearp/Documents/CodeProjects/ntv/

Please read these files to understand the current state:
1. CLAUDE.md - Complete architecture, Phases 1-3 status, all learnings
2. This file (SESSION-GUIDE.md) - Quick context

Current Status: Phase 4 NEARLY COMPLETE - All Priority 1 Features Done! ✅

**✅ Phase 3 Complete:**
- Professional 3-panel UI layout working perfectly
- Node selection with visual feedback (yellow highlight)
- Click empty space to deselect
- Full CRUD server functions for nodes and connections
- Properties panel loads and saves real data
- Real-time viewport updates (no refresh needed!)
- Suspense components eliminating hydration warnings

**✅ Phase 4 Priority 1 - ALL COMPLETE:**
- 3D node rotation controls (X/Y/Z in degrees with default rotation_x=90°)
- Model Selection UI (loads correct glTF/GLB for each node type: router, switch, server, firewall, load_balancer, cloud)
- 3D Grid and Axes (Blender-style reference grid)
- Topology Switching Control (dropdown selector with 2 sample topologies)
- **Device Palette buttons functional** (all 6 device types create nodes with grid positioning) ✅
- **Grid/Axes visibility toggles** - Show/hide grid and individual axes independently ✅
- **Connection creation mode** (2025-11-05) - Click two nodes to create connection ✅
  - Three-state mode: Disabled → SelectingFirstNode → SelectingSecondNode
  - Visual button feedback showing current mode
  - Creates connections via server function with real-time updates

**✅ Phase 4 Priority 2 - MOSTLY COMPLETE:**
- Node Labels/Tooltips (show node name on hover)
- Color-Coded Nodes by Type (router=blue, switch=green, server=orange, etc.)
- **Connection rendering improved** (thin cylindrical lines, 0.012 thickness) ✅
- **Connection selection** (2025-11-05) - Click to select connections in viewport ✅
  - Ray-cylinder intersection algorithm for accurate 3D picking
  - Visual feedback with yellow/orange highlighting
  - Properties panel shows connection details
  - **Critical fix:** Mutable storage pattern for event handlers to access fresh data

**⏳ Remaining Phase 4 Tasks (Priority 2):**
- Improved Lighting and Materials
- Better Camera Controls (presets, bookmarks, reset view)

Next: Priority 2 polish OR Phase 5 Export features OR Phase 6 Traffic Monitoring
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

✅ **COMPLETED - Priority 1 (Core 3D Features):**
1. ✅ **3D node rotation controls** (2025-11-04)
   - Database migration: rotation_x/y/z columns (stored in degrees)
   - Properties panel: X/Y/Z sliders (-180° to +180°)
   - Viewport: Applied using cgmath `degrees()` function
   - Default rotation_x=90° for Blender glTF models
   - Key lesson: `degrees()` converts to radians, `radians()` just wraps values
2. ✅ **Model Selection UI** (2025-11-04) - Loads correct glTF/GLB model for each node type
   - All 6 models: router, switch, server, firewall, load_balancer, cloud
   - Dynamic model loading based on node.node_type
   - Each model colored according to node type
3. ✅ **3D Grid and Axes** - Blender-style reference grid with X/Y/Z axis lines and grid floor plane
4. ✅ **Topology switching control** (2025-11-04)
   - Dropdown selector in top toolbar
   - 2 sample topologies in database
   - Dynamic loading on selection change
   - Critical fix: Disposed signal access in event handlers using non-reactive snapshot pattern

✅ **COMPLETED - Priority 1 (Core 3D Features):**
5. ✅ **Enable Device Palette buttons** (2025-11-05) - All 6 device types functional
   - Create nodes via create_node() server function
   - Grid positioning to avoid overlap (5-column layout)
   - Real-time viewport updates via refetch trigger
6. ✅ **Grid/Axes visibility controls** (2025-11-05) - Toggle buttons to show/hide elements
   - ViewportVisibility struct pattern prevents context collision
   - Independent toggles for Grid Floor, X Axis (Red), Y Axis (Green), Z Axis (Blue)
   - Z-axis extremely transparent (alpha=25), axes thinned to 0.006
7. ✅ **Connection creation mode** (2025-11-05) - Click two nodes to create connection
   - Three-state mode with visual button feedback
   - Creates connections via create_connection() server function
   - Real-time viewport updates after creation

✅ **COMPLETED - Priority 2 (Visual Polish):**
8. ✅ **Node Labels/Tooltips** - Show node name on hover in 3D viewport
9. ✅ **Color-Coded Nodes by Type** - Router=blue, Switch=green, Server=orange, etc.
10. ✅ **Connection rendering improvements** (2025-11-05) - Thin cylindrical lines (0.012 thickness) using ColorMaterial
11. ✅ **Connection selection** (2025-11-05) - Click to select connections in viewport
    - Ray-cylinder intersection algorithm for 3D picking
    - Visual feedback with yellow/orange highlighting
    - Properties panel shows connection details (type, bandwidth, status)
    - Critical mutable storage pattern fix for event handler data access

⏳ **REMAINING - Priority 2 (Visual Polish):**
12. ⏳ **Improved Lighting and Materials** - Better 3D scene lighting
13. ⏳ **Better Camera Controls** - Presets, bookmarks, reset view

### 🔄 What to Work On Next

**Phase 4 - Priority 1: ALL COMPLETE! ✅**
All core 3D features are now implemented and working:
- Connection creation mode ✅
- Device palette buttons ✅
- Grid/axes visibility controls ✅
- All other Priority 1 features ✅

**Phase 4 - Priority 2: MOSTLY COMPLETE! ✅**
Visual polish features implemented:
- Connection selection ✅
- Node labels/tooltips ✅
- Color-coded nodes ✅
- Connection rendering improvements ✅

**REMAINING: Phase 4 - Priority 2**
```
Two remaining polish items:
- Task #12: Improved lighting and materials
- Task #13: Better camera controls (presets, bookmarks, reset view)
```

**NEXT MAJOR WORK:**
Choose your direction:
- Complete Phase 4 polish (lighting & camera controls)
- OR Phase 5: Export & Finalization (PNG export, JSON import/export)
- OR Phase 6: Traffic Monitoring (real-time traffic visualization)

**LATER: Phase 5 - Export & Finalization**
```
- Export topology as PNG image
- Export/Import topology as JSON
- UI polish and optimizations
- Documentation and deployment
```

**FUTURE: Phase 6 - Traffic Monitoring**
```
- Real-time traffic visualization using Leptos streaming
- Use #[server(protocol = Websocket<...>)]
- Animate connections based on traffic load
- Traffic metrics dashboard
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

### Continue with Next Phase 4 Task
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Current Status: Phase 4 IN PROGRESS
- ✅ Rotation controls, grid/axes, labels, color-coding complete
- ⏳ Next: Enable Device Palette buttons

Let's implement Phase 4 Priority 1, Task #3:
Make the Device Palette buttons functional so clicking 'Router', 'Switch', etc.
creates new nodes in the topology via the create_node() server function.

Ready to start!
```

### Work on Topology Switching
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Let's implement Phase 4 Priority 1, Task #4:
- Add another mock topology to the database
- Create topology selector UI in top toolbar
- Enable switching between topologies

Ready to start!
```

### Jump to Specific Feature
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md for complete context.

I want to work on [specific feature from Phase 4/5/6]:
[describe what you want]

How should we approach this?
```

## 💡 Pro Tips

1. **Always read CLAUDE.md first** - Contains all architectural discoveries and solutions
2. **Use Context7 MCP** - When unsure about Leptos/three-d patterns, check `/websites/book_leptos_dev`
3. **Check git tags** - `git tag` shows v0.1.0-phase1-complete, v0.1.0-phase2-complete, v0.1.0-phase3-complete
4. **Test in browser** - http://127.0.0.1:3000 to see current state
5. **Console logs** - Browser console shows WASM logs from `web_sys::console`
6. **Real-time updates work!** - Save node positions in properties panel, viewport updates instantly

## 🚀 You're Ready!

Phase 4 is partially complete! Next up:
1. **Enable Device Palette buttons** (Priority 1, Task #3)
2. **Topology switching control** (Priority 1, Task #4)

All architectural patterns are working and documented in CLAUDE.md.
