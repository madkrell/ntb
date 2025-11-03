# Network Topology Visualizer - Session Continuation Guide

## 🎯 Starting a New Conversation

**Use this prompt to continue from where we left off:**

```
I'm continuing work on the Network Topology Visualizer project at:
/Users/mattearp/Documents/CodeProjects/ntv/

Please read these files to understand the current state:
1. CLAUDE.md - Complete architecture, Phase 1 & 2 status, all learnings
2. This file (SESSION-GUIDE.md) - Quick context

Current Status: Phase 2 COMPLETE ✅
- Working 3D viewport with orbit controls
- Topology data rendering (nodes as spheres, connections as cylinders)
- Sample data: 7 nodes, 7 connections

Next: [specify what you want to work on - see options below]
```

## 📊 Current Project State

### ✅ Completed Phases

**Phase 1 - Foundation (Git tag: v0.1.0-phase1-complete)**
- Leptos 0.8 Islands architecture
- SQLite database with migrations
- Server functions in `src/api.rs` (non-feature-gated)
- Database schema: topologies, nodes, connections, traffic_metrics

**Phase 2 - 3D Viewport (Git tag: v0.1.0-phase2-complete)**
- TopologyViewport island with WebGL2 + three-d
- Interactive orbit camera controls (drag to rotate, scroll to zoom)
- Nodes rendered as 3D spheres at database positions
- Connections rendered as properly rotated cylinders
- Sample topology with 7 nodes and 7 connections

### 🔄 What to Work On Next

**Option 1: Phase 3 - UI Layout & 3D Editing**
```
Let's build the editing UI and interface. I want to:
- Create professional UI layout (3D viewport center, panels around edges)
- Add device palette/toolbar for selecting device types
- Enable node selection/editing directly in 3D viewport
- Build properties panel for editing selected node/connection details
- Add toolbar with common actions (Add, Delete, Connect, Save)
```

**Option 2: Phase 4 - Visual Enhancements**
```
Let's enhance the 3D visualization. I want to:
- Show node labels/tooltips on hover
- Color-code nodes by type (router=blue, switch=green, etc.)
- Load custom 3D models from Blender (glTF/GLB files)
- Improve lighting and materials
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

```bash
# Development with hot reload
cargo leptos watch

# Production build
cargo leptos build --release

# Check WASM output
ls -lh target/site/pkg/*.wasm

# Run server manually
./target/site/server

# Database operations (if needed)
sqlite3 ntv.db
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
# Should see 3D viewport with 7 nodes and connections
# Console logs should show initialization messages
```

## 🎬 Example Session Start Prompts

### Continue with Phase 3 (UI Layout & 3D Editing)
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md for complete context. Phase 2 is complete (3D viewport working).

Let's build Phase 3: UI Layout & 3D Editing Interface
- Professional UI layout (3D viewport center, panels around edges)
- Device palette/toolbar for selecting device types
- Node selection and editing directly in 3D viewport
- Properties panel for editing selected nodes/connections
- Toolbar with common actions (Add, Delete, Connect, Save)

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

Read CLAUDE.md for complete context. Phase 2 is complete.

I'm seeing [issue description] or want to improve [feature name].

Can you help me [what you want to do]?
```

## 💡 Pro Tips

1. **Always read CLAUDE.md first** - Contains all architectural discoveries and solutions
2. **Use Context7 MCP** - When unsure about Leptos patterns, check `/websites/book_leptos_dev`
3. **Check git tags** - `git tag` shows v0.1.0-phase1-complete and v0.1.0-phase2-complete
4. **Test in browser** - http://127.0.0.1:3000 to see current state
5. **Console logs** - Browser console shows WASM logs from `web_sys::console`

## 🚀 You're Ready!

Pick an option above and start coding. All the architectural patterns are working and documented in CLAUDE.md.
