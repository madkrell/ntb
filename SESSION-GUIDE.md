# Network Topology Builder - Session Continuation Guide

## 🎯 Starting a New Conversation

**Use this prompt to continue from where we left off:**

```
I'm continuing work on the Network Topology Builder project at:
/Users/mattearp/Documents/CodeProjects/ntb/

Please read these files to understand the current state:
1. CLAUDE.md - Complete architecture, all phases, all learnings
2. This file (SESSION-GUIDE.md) - Quick context

Current Status: Phase 6.4.2 COMPLETE! ✅ (Particle Animation System) - 60fps animated traffic flows!

**✅ Phase 6.1-6.3 Complete (2025-01-15):**
Traffic Monitoring with realistic network simulation:

**Traffic Monitoring Features (Phases 6.1-6.3):**
- Mock traffic generator with three intensity levels (Low/Medium/High)
- Color-coded connections based on utilization (green 0-40%, orange 40-70%, red 70-100%)
- Traffic Controls panel with Generate/Clear buttons
- Bandwidth-aware traffic using actual link properties
- Congestion-based latency calculation (increases with utilization)
- Exponential packet loss modeling (realistic network behavior)
- Comprehensive tooltips with 4 metrics (utilization, throughput, latency, packet loss)
- Real-time viewport updates when traffic changes
- Manual color override capability maintained

**✅ Phase 6.4.1 Complete (2025-01-18):**
Traffic Flow Controls - User control over traffic animation:

**Traffic Flow Control Features:**
- Database migration: carries_traffic (BOOLEAN) and flow_direction (TEXT) fields
- Properties Panel checkbox: Enable/disable traffic animation per connection
- Properties Panel radio buttons: Control flow direction (source→target, target→source, bidirectional)
- Swap Source/Target button: Reverse connection direction with one click
- TrafficParticle struct: Foundation for particle animation system
- SQLite validation triggers: Enforce valid flow directions
- All SQL queries updated: Include new traffic flow fields

**Key Technical Implementation:**
1. Server-side traffic generation (generate_mock_traffic server function)
2. Realistic patterns based on connection type (Fiber > Ethernet > Wireless)
3. Congestion penalties for latency at high utilization
4. Exponential packet loss increase at high utilization
5. Color-coded tooltips for instant metric interpretation

**✅ Phase 5.7 Complete (2025-11-14):**
HDR Environment Lighting with studio-quality illumination:

**HDR Lighting Features:**
- HDR environment map loading from `public/environments/` (4 HDR files included)
- Toggle control in View Controls panel (Enabled/Disabled)
- Dropdown selector for different HDR environments (Studio Small, Studio Loft, Photo Studio 4K, Docklands)
- Real-time toggle without viewport reinitialization
- Conditional lighting system: HDR mode uses ambient only, Manual mode uses full 3-point lighting
- Settings persistence across page refreshes
- **Critical Fix:** Removed directional lights when HDR enabled (prevented texture washout)
- **Dynamic signal reading:** Pass `RwSignal<bool>` to render closure for real-time updates
- Perfect Blender color + lighting match when using same HDR environment

**Key Technical Solutions:**
1. Reactive signals in Effects (use `.get()` not `.get_untracked()`)
2. Conditional lighting in render loop (HDR: 1 light | Manual: 4 lights)
3. Dynamic signal reading in closures (pass signal, read each frame)
4. Image textures + HDR = perfect Blender match

**✅ Phase 5.5 Complete (2025-11-08):**
Vendor-based multi-vendor model selection with auto-discovery system:

**Vendor System Features:**
- Device Palette buttons now plural ("Routers", "Switches", "Servers", etc.)
- Added new "Applications" device type (7 types total)
- Click device button → dropdown shows vendors and their models
- Auto-discovers vendors from `public/models/{type}/{vendor}/` folders
- Auto-discovers vendor icons from `public/icons/vendors/{vendor}.svg`
- Generic vendor always shown first as fallback
- Zero configuration - just add folders and models appear automatically!
- Database stores vendor and model_name for each node

**✅ Phase 4 Complete (2025-11-06 + Phase 4.5 on 2025-11-07):**
ALL core 3D features, visual polish, UI optimization, customization, and settings persistence implemented:

**Core 3D Features:**
- 3D node rotation controls (X/Y/Z in degrees)
- Model Selection UI (glTF/GLB for all 6 node types including Cloud)
- 3D Grid and Axes (Blender-style reference)
- Topology Switching Control (dropdown selector)
- Device Palette buttons functional (all 6 types create nodes)
- Grid/Axes visibility toggles (independent controls)
- Connection creation mode (click two nodes)

**Visual Polish:**
- Node Labels/Tooltips (hover to see name)
- Color-Coded Nodes by Type
- Connection rendering (thin cylinders)
- Connection selection (ray-cylinder picking)
- **Improved Lighting** - Three-point lighting (key, fill, rim) with user controls
- **Camera Controls** - 4 presets (Top, Front, Side, Isometric) with smooth animations

**UI/UX Enhancements:**
- **Space Optimization** - Panels narrowed 25% to maximize viewport
- **Settings Persistence** - All View/Lighting controls saved to database
- **Code Quality** - Zero compiler warnings, clippy-clean
- **Fullscreen Toggle** - F11 key and toolbar button for immersive view
- **Camera Pan** - Shift+Drag to pan viewport in all directions

**Export & Customization:**
- **PNG Export** - High-quality image export with transparency support
- **Node Scale Control** - Per-node size adjustment (0.1-5.0 range)
- **Background Color Control** - 6 presets + transparent option for exports
- **Connection Color Control** - 13 presets + full color palette picker
- **Node Color Control** - 13 presets + HTML5 color picker with hex↔RGB conversion

**Phase 4.5 Polish Features (2025-11-07):**
21. ✅ **Fullscreen Toggle** - F11 key and toolbar button for immersive viewport
22. ✅ **Camera Pan Controls** - Shift+Drag to pan (translate) camera in viewport
23. ✅ **Viewport Centering** - Center view on topology bounding box
24. ✅ **Zoom to Fit** - Auto-calculate optimal camera distance (10% margin)
25. ✅ **Node Color Control** - 13 preset colors + full palette picker (hex↔RGB)
26. ✅ **Cloud Type in UI** - Added missing "Cloud" option to Properties Panel dropdown

**✅ Phase 5 Complete (Export/Import functionality):**
27. ✅ **Export as PNG** - High-quality image export with transparency support
28. ✅ **Export as JSON** - Full topology backup (nodes, connections, all properties)
29. ✅ **Import from JSON** - Restore or share topologies with validation

Next: Optional enhancements (Traffic Dashboard, Animation System, etc.)
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

**Phase 4 - Visual Enhancements & 3D Interaction (Git tag: v0.1.0-phase4-complete) ✅ COMPLETE**
**Phase 4.5 - Polish & Refinements (2025-11-07) ✅ COMPLETE**
**Phase 5 - Export & Import (JSON/PNG) ✅ COMPLETE**

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
12. ✅ **Improved Lighting and Materials** (2025-11-06) - Three-point lighting system
    - Key light (warm, from above-front), Fill light (cool, from side), Rim light (subtle, from behind)
    - User-adjustable lighting controls (4 intensity sliders: Ambient, Key, Fill, Rim)
    - PBR materials with metallic/roughness properties varying by device type
    - Metallic nodes (router, firewall) vs matte nodes (server, client)
13. ✅ **Better Camera Controls** (2025-11-06) - Preset views with smooth animations
    - 4 camera presets: Top, Front, Side, Isometric
    - Smooth lerp animation with ease-in-out easing (600ms transitions)
    - Reset button to return to default isometric view
    - Compact viewport overlay controls (2×2 grid, top-right corner)
    - Camera state sync enables dragging from preset positions

✅ **COMPLETED - Additional Polish (2025-11-06):**
14. ✅ **UI Space Optimization** - Maximized viewport space
    - Device Palette narrowed to 75% (256px → 192px)
    - Properties Panel narrowed to 75% (320px → 240px)
    - Position/rotation controls made compact
    - View Controls color-coded (X=red, Y=green, Z=blue)
15. ✅ **Settings Persistence** - UI state survives page refresh/restart
    - Database table: ui_settings
    - Persists View Controls and Lighting Controls
    - Auto-save on change, auto-load on startup
16. ✅ **Code Quality** - Clean, warning-free codebase
    - All compiler warnings fixed
    - Clippy-clean code
17. ✅ **PNG Export Functionality** - High-quality image export
    - Export dropdown menu in toolbar with PNG/JSON options
    - WebGL2 context with preserveDrawingBuffer enabled
    - Transparent background support for clean exports
    - Fixed dropdown z-index for proper overlay visibility
18. ✅ **Node Scale Control** - Per-node size adjustment
    - Database migration: 20250106000003_add_node_scale.sql
    - Properties panel slider (range 0.1-5.0, default 1.0)
    - Real-time viewport rendering with scale transformation
    - Applied to both 3D models and fallback spheres
19. ✅ **Background Color Control** - Customizable viewport background
    - Extended ViewportVisibility struct with background_color field
    - 6 preset buttons: Transparent, White, Light, Gray, Dark, Black
    - Transparent option (None) for PNG exports showing only topology
    - Black default background (rgb(0,0,0))
    - Real-time viewport updates via refetch_trigger
20. ✅ **Connection Color Control** - Customizable link colors
    - Database migration: 20250107000001_add_connection_color.sql
    - Properties panel with 13 preset colors
    - Full color palette picker with HTML5 color input
    - Bidirectional hex↔RGB conversion
    - Real-time color rendering in 3D viewport

✅ **COMPLETED - Phase 4.5 Polish Features (2025-11-07):**
21. ✅ **Fullscreen Toggle** - F11 key and toolbar button
    - RwSignal<bool> for fullscreen state
    - Event handler for F11 keypress
    - Toolbar button with icon/text toggle
    - Document.requestFullscreen() / exitFullscreen()
22. ✅ **Camera Pan Controls** - Shift+Drag to pan viewport
    - Translate camera target without rotating
    - Shift modifier detection in mouse handlers
    - Updates camera_target signal for pan movement
    - Works in all camera modes (presets + manual)
23. ✅ **Viewport Centering** - Center view on topology
    - Calculate bounding box of all nodes
    - Set camera target to bounding box center
    - Smooth transition to centered view
24. ✅ **Zoom to Fit** - Auto-calculate optimal camera distance
    - Bounding box calculation with 10% margin (reduced from 20%)
    - FOV-based distance calculation: `distance = (size / 2) / tan(FOV / 2)`
    - Smooth animation to fitted view
    - Works with all topologies regardless of size
25. ✅ **Node Color Control** - Per-node customization
    - Database migration: 20250107000002_add_node_color.sql (RGB format)
    - 13 preset color buttons in Properties Panel
    - HTML5 color picker with bidirectional hex↔RGB conversion
    - Real-time viewport rendering with custom colors
    - Fallback to type-based colors if parsing fails
26. ✅ **Cloud Type in UI** - Added missing node type
    - "Cloud" option added to Properties Panel dropdown
    - Already existed in model loading, just missing from UI
    - All 6 node types now accessible

✅ **COMPLETED - Phase 5 (Export/Import):**
27. ✅ **PNG Export** (topology_editor.rs:733-838) - High-quality image export
    - Export dropdown menu in toolbar
    - WebGL2 context with preserveDrawingBuffer enabled
    - canvas.toDataURL() for image capture
    - Transparent background support
    - Automatic download with filename timestamp
28. ✅ **JSON Export** (topology_editor.rs:840-931) - Full topology backup
    - Exports complete topology data (nodes, connections, all properties)
    - Pretty-formatted JSON with serde_json
    - Filename: `topology-{name}-{timestamp}.json`
    - Preserves positions, rotations, scales, colors
29. ✅ **JSON Import** (topology_editor.rs:933-1274) - Restore/share topologies
    - File picker UI with drag-and-drop support
    - JSON validation and parsing
    - Creates new topology with "(Imported)" suffix
    - Batch node and connection creation
    - Proper node ID mapping for connections
    - Error handling with user-friendly messages
    - Automatic switch to imported topology

### 🔄 What to Work On Next

**Phases 4, 4.5, 5, 6.1-6.3, & 6.4.1: COMPLETE! ✅** (through 2025-01-18)
All core features, visual polish, export/import, traffic monitoring, and flow controls implemented:
- All core 3D features ✅
- All visual polish features ✅
- UI optimization ✅
- Settings persistence ✅
- Code quality improvements ✅
- PNG/JSON export with import ✅
- Customization (colors, scale, background) ✅
- Fullscreen, camera controls, zoom to fit ✅
- **Traffic monitoring** ✅
- **Mock traffic generator** ✅
- **Color-coded connections by utilization** ✅
- **Comprehensive traffic tooltips** ✅
- **Link metrics impact on traffic** ✅
- **Traffic flow controls** ✅
- **Swap Source/Target button** ✅

**Git Tag:** v0.1.0-phase6.4.2-complete ✅

**Phase 6 Core Features: COMPLETE!** All traffic monitoring and animation implemented.

**Recommended Next Steps:**

**Option 1: Traffic Dashboard** (High Value - Data Visualization)
   - ⏳ **Metrics panel** - Network-wide stats and top connections
   - ⏳ **Historical charts** - Time-series visualization of traffic
   - ⏳ **Alert panel** - Critical connection warnings
   - ⏳ **Export metrics** - CSV download for analysis
   - 🎯 **Why valuable?** Professional monitoring dashboard completes the traffic monitoring experience

**Option 2: Particle Animation System - Phase 6.4.2** (IMMEDIATE NEXT - Visual Impact)
   - 🎯 **RECOMMENDED NEXT STEP** - Foundation already in place!
   - ⏳ **Particle storage** - Vec<TrafficParticle> in Rc<RefCell<>>
   - ⏳ **Spawning logic** - Based on utilization thresholds (1-3, 3-7, 7-12 particles)
   - ⏳ **Animation loop** - 60fps updates with requestAnimationFrame
   - ⏳ **Render particles** - Small glowing spheres using three-d
   - ⏳ **Conditional rendering** - Only when traffic exists and carries_traffic=true
   - ⏳ **Particle flows** - Moving particles along connections
   - ⏳ **Direction indicators** - Particles show data flow direction
   - ⏳ **Speed variation** - Faster particles = higher throughput
   - ⏳ **Density control** - More particles = busier connection
   - 🎯 **Why exciting?** Makes demos much more engaging, immediately shows network activity
   - 🎯 **Why now?** Phase 6.4.1 complete - database, UI controls, TrafficParticle struct ready!

**Option 3: Additional Polish & Features** (Optional UX enhancements)
   - ⏳ Multi-select nodes (Shift+Click to select multiple)
   - ⏳ Multi-select connections (bulk color changes)
   - ⏳ Undo/redo functionality
   - ⏳ Copy/paste nodes
   - ⏳ Keyboard shortcuts (Del to delete, Ctrl+S to save, etc.)
   - ⏳ Node grouping/labeling
   - 🎯 **Why consider?** Nice-to-have UX improvements for power users

**Option 4: Documentation & Deployment** (Can be done anytime)
   - ⏳ User/developer documentation (README with screenshots)
   - ⏳ Deployment guide (Docker, production build)
   - 🎯 **Why later?** Focus on features first, document when stable

---

## 📋 Phase 6 Implementation Details (COMPLETE!)

### ✅ Phases 1-6 - COMPLETE! (Foundation through Traffic Monitoring)

All foundational features, 3D editing, visual polish, export/import, and core traffic monitoring are complete and working!

---

### Phase 6 - Traffic Monitoring ✅ COMPLETE (Phases 6.1-6.3)

**Goal:** Transform the static topology into a live network monitoring tool with real-time traffic visualization

**Why Phase 6 is the Most Impactful Feature:**
This brings the entire application to life! Instead of just showing a static network diagram, Phase 6 will:
- Show live data flowing through the network with animated particles
- Instantly identify network problems with color-coded health indicators
- Provide professional network monitoring capabilities
- Demonstrate the full power of Leptos streaming
- Create a truly impressive, production-ready application

**Core Features:**

1. **Real-time Traffic Visualization**
   - Animated connections showing data flow direction and volume
   - Moving particles along connection paths (bi-directional)
   - Particle speed based on latency (faster = lower latency)
   - Particle density based on throughput (more particles = higher traffic)
   - Smooth 60fps animations using requestAnimationFrame

2. **Live Metrics Display**
   - **Throughput:** Mbps/Gbps shown on each connection
   - **Latency:** Round-trip time in milliseconds
   - **Packet Loss:** Percentage with warning thresholds
   - **Bandwidth Utilization:** Percentage of link capacity
   - **Connection Status:** Up/Down/Degraded
   - All metrics update in real-time (every 100-500ms)

3. **Color-Coded Health Status**
   - **Green (Healthy):** <70% utilization, <50ms latency, <1% packet loss
   - **Yellow (Warning):** 70-90% utilization, 50-100ms latency, 1-5% packet loss
   - **Red (Critical):** >90% utilization, >100ms latency, >5% packet loss
   - Smooth color transitions as metrics change
   - Connection thickness varies with utilization

4. **Traffic Dashboard**
   - New panel showing real-time metrics
   - Top N busiest connections (sorted by throughput)
   - Total network throughput aggregated
   - Historical charts (line graphs for throughput/latency over time)
   - Alert list for connections in warning/critical state
   - Export metrics to CSV

5. **Streaming Data Architecture**
   - Leptos WebSocket server functions: `#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]`
   - Client-side: `Signal::from_stream()` for reactive data binding
   - Efficient binary encoding for minimal overhead
   - Automatic reconnection on connection loss
   - Backpressure handling for slow clients

6. **Mock Traffic Generator** (for demo/testing)
   - Realistic network traffic simulation
   - Configurable traffic patterns (constant, bursty, periodic)
   - Simulates different scenarios (normal, congestion, failures)
   - Adjustable baseline traffic levels
   - Random events (link failures, traffic spikes)

7. **Performance Optimizations**
   - Render only visible connections (viewport culling)
   - Batch particle updates for efficiency
   - Use instanced rendering for particles
   - Throttle metric updates to avoid overwhelming UI
   - Efficient WebSocket message batching

**Implementation Status:**

**Phase 6.1 - Mock Traffic Generator ✅ COMPLETE**
1. ✅ Traffic metrics table verified (connection_traffic_metrics)
2. ✅ Mock traffic generator server function created
   - Realistic patterns based on connection type
   - Three intensity levels (Low/Medium/High)
   - Stores throughput, latency, packet_loss, utilization
3. ✅ Traffic Controls UI with Generate/Clear buttons
4. ✅ Real-time viewport updates

**Phase 6.2 - Traffic Visualization ✅ COMPLETE**
1. ✅ Color-coded connections (green/orange/red)
2. ✅ Traffic data fetching via get_all_traffic_metrics
3. ✅ Enhanced tooltips with utilization display
4. ✅ Manual color override capability maintained
5. ✅ Proper lighting for color visibility

**Phase 6.3 - Link Metrics Impact ✅ COMPLETE**
1. ✅ Bandwidth-aware traffic generation
2. ✅ Congestion-based latency calculation
3. ✅ Exponential packet loss modeling
4. ✅ Comprehensive tooltips (4 metrics with color coding)
5. ✅ All metrics update in real-time

**Phase 6.4.1 - Traffic Flow Controls ✅ COMPLETE (2025-01-18)**
1. ✅ Database migration with carries_traffic and flow_direction fields
2. ✅ Connection model updated (src/models/connection.rs:19-20)
3. ✅ All SQL queries updated (5 locations in api.rs)
4. ✅ Properties Panel UI with checkbox and radio buttons
5. ✅ Swap Source/Target button (reverse connection direction)
6. ✅ TrafficParticle struct defined (topology_viewport.rs:40-49)

**Phase 6.4.2 - Particle Animation System ✅ COMPLETE (2025-01-18)**
1. ✅ Global particle storage (static GLOBAL_PARTICLES: Mutex<Vec<TrafficParticle>>)
   - Ensures animation loop and render function access SAME data
   - ANIMATION_RUNNING flag controls animation state
   - Prevents stale data issues when viewport Effect reruns
2. ✅ Particle spawning logic:
   - Reads traffic metrics from database
   - Checks carries_traffic flag (only spawns if true)
   - Particle count based on utilization:
     - <40%: 1-3 particles (green)
     - 40-70%: 3-7 particles (orange)
     - >70%: 7-12 particles (red)
   - Speed varies: 0.15-0.25 (low), 0.25-0.40 (medium), 0.40-0.60 (high)
3. ✅ 60fps animation loop:
   - Uses requestAnimationFrame for browser-synchronized updates
   - Delta time calculation for frame-rate independence
   - Updates particle positions (position += speed * delta_time)
   - Recycles particles at destination (reset to 0.0)
   - Stops cleanly when "Clear Traffic" clicked
4. ✅ Particle rendering:
   - Small glowing spheres (radius 0.08) with emissive materials
   - Colors match utilization (green/orange/red)
   - Interpolates position along connection path
   - Respects direction_forward flag for bidirectional flows
5. ✅ Animation control:
   - start_particle_animation() / stop_particle_animation() functions
   - Called from Generate/Clear Traffic buttons
   - Works on first load (no manual refresh needed)
   - Animation setup moved outside skip_event_handlers block

**Phase 6.3 - Animated Connections (3-4 hours)**
1. Particle system for connection animations
   - Create particle struct (position, velocity, color)
   - Spawn particles at source node
   - Move particles along connection path
   - Remove particles at destination
2. Render particles as small spheres or sprites
3. Update particle positions every frame (60fps)
4. Color particles based on connection health status

**Phase 6.4 - Live Metrics Display (2-3 hours)**
1. Add metric overlays to connections in viewport
   - Throughput label
   - Latency badge
   - Packet loss indicator
2. Update connection colors based on health status
3. Vary connection thickness based on utilization
4. Smooth transitions for all visual changes

**Phase 6.5 - Traffic Dashboard (2-3 hours)**
1. Create new TrafficDashboard component
2. Display aggregated metrics
3. Top connections list (sortable)
4. Historical charts using lightweight charting library
5. Alert panel for critical connections
6. Export to CSV functionality

**Expected Outcomes:**
- ✅ Live, animated network topology showing real-time traffic
- ✅ Instant identification of network bottlenecks and issues
- ✅ Professional-grade network monitoring capabilities
- ✅ Impressive demo showcasing Leptos streaming
- ✅ Production-ready network visualization tool

**Technical Challenges & Solutions:**
- **Challenge:** 60fps particle animation performance
  - **Solution:** Use instanced rendering, batch updates, viewport culling
- **Challenge:** WebSocket message volume
  - **Solution:** Batch metrics, delta encoding, client-side interpolation
- **Challenge:** Synchronizing animations with data
  - **Solution:** Timestamp-based interpolation, buffering for smooth playback
- **Challenge:** Handling topology changes during streaming
  - **Solution:** Graceful reconnection, state reconciliation on reconnect

---

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

## 🎓 Key Lessons Learned (Phase 4.5 Session - 2025-11-07)

### 1. Bounding Box Margin Calculations
**Pattern:** FOV-based distance calculation with configurable margin
```rust
let margin_factor = 1.1;  // 10% margin (user preference - was 20% before)
let max_dimension = (width.max(height).max(depth)) * margin_factor;
let distance = (max_dimension / 2.0) / (fov_radians / 2.0).tan();
```
**Lesson:** Small margin changes (20% → 10%) have significant visual impact on "Zoom to Fit" tightness. User preference matters!

### 2. Bidirectional Color Conversion (Hex ↔ RGB)
**Pattern:** Database stores RGB text, UI uses hex color picker
```rust
// RGB → Hex (for display)
let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);

// Hex → RGB (for storage)
let r = u8::from_str_radix(&hex[1..3], 16)?;
let g = u8::from_str_radix(&hex[3..5], 16)?;
let b = u8::from_str_radix(&hex[5..7], 16)?;
let rgb = format!("{},{},{}", r, g, b);
```
**Lesson:** Text-based RGB storage is human-readable and database-friendly. Hex format provides best UI experience with HTML5 color picker. Convert at the boundaries!

### 3. Database Color Format Consistency
**Pattern:** All color fields use "R,G,B" text format
- Nodes: `color: String` (e.g., "100,150,255")
- Connections: `color: String` (e.g., "128,128,128")
- Background: `Option<(u8, u8, u8)>` in ViewportVisibility (not persisted)

**Lesson:** Consistent format across all color fields simplifies parsing and reduces bugs. Text format in database makes debugging and SQL queries easier.

### 4. Type-Based Fallback Colors
**Pattern:** Custom colors with graceful fallback to type-based defaults
```rust
let color = if let Ok(custom_color) = parse_rgb(&node.color) {
    custom_color
} else {
    get_node_color(&node.node_type)  // Fallback to type default
};
```
**Lesson:** Always have a fallback! Parsing can fail (corrupted data, old records). Type-based colors ensure the viewport always renders correctly.

### 5. Incremental Feature Implementation
**Pattern:** Build on existing infrastructure rather than rewriting
- Node color control built on connection color pattern
- Zoom to Fit reused bounding box calculation from viewport centering
- Cloud type was already in model loading, just needed UI dropdown update

**Lesson:** Before implementing a new feature, search for similar patterns in the codebase. Often 70% of the work is already done!

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
# ⚠️ IMPORTANT: Single database file: ntv.db (NOT ntb.db!)
# This was intentionally kept during project rename to preserve data

# Open database
sqlite3 ntv.db

# View tables
sqlite3 ntv.db ".tables"

# Check data
sqlite3 ntv.db "SELECT COUNT(*) FROM nodes;"

# Run migrations (normally auto-run on startup)
sqlx migrate run

# Configuration
# .env file: DATABASE_URL=sqlite:ntv.db
# src/main.rs: Falls back to "sqlite:ntv.db" if .env not found
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

### Option 1: Multi-Select Nodes (Polish Feature)
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Current Status: Phase 5 COMPLETE! ✅

Let's add multi-select functionality:
1. Shift+Click to select multiple nodes
2. Visual feedback (all selected nodes highlighted)
3. Properties panel shows "Multiple nodes selected (N)"
4. Bulk operations: delete, change type, move together

How should we approach this?
```

### Option 2: Keyboard Shortcuts (UX Enhancement)
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Current Status: Phase 5 COMPLETE! ✅

Let's add keyboard shortcuts for better UX:
- Del/Backspace: Delete selected node/connection
- Ctrl/Cmd+S: Save changes in properties panel
- Ctrl/Cmd+E: Export PNG
- Esc: Deselect/cancel current mode
- Space: Toggle grid visibility
- 1-6: Quick add device types

Ready to implement!
```

### Option 3: Traffic Monitoring - Phase 6.1 (Database & Mock Generator)
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Current Status: Phase 4 & 4.5 COMPLETE! ✅

Let's start Phase 6 - Traffic Monitoring with Phase 6.1:

Goals (Database & Mock Generator):
1. Verify traffic_metrics table schema has all required fields
2. Add database indexes for efficient time-range queries
3. Create mock traffic generator server function
   - Random but realistic traffic patterns (throughput, latency, packet loss)
   - Configurable traffic level (low/medium/high)
   - Store metrics in traffic_metrics table with timestamps
4. Add UI controls to enable/disable generator and set traffic level

This sets the foundation for real-time streaming in Phase 6.2!
```

### Option 4: Traffic Animation - Phase 6.4.2 (Particle System) 🎯 RECOMMENDED NEXT
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntb/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Current Status: Phase 6.4.1 COMPLETE! ✅ (Traffic flow controls implemented)

Let's implement Phase 6.4.2 - Particle Animation System:

Goals (3D Traffic Animation):
1. Add particle storage (Vec<TrafficParticle> in Rc<RefCell<>>)
2. Implement spawning logic based on traffic metrics:
   - Check carries_traffic flag (only spawn if true)
   - Determine particle count based on utilization:
     - <40%: 1-3 particles
     - 40-70%: 3-7 particles
     - >70%: 7-12 particles
3. Create animation loop with requestAnimationFrame (60fps):
   - Update particle positions (position += speed * delta_time)
   - Recycle particles at destination (reset to 0.0)
4. Render particles as small glowing spheres using three-d:
   - Color based on utilization (green/orange/red)
   - Scale: 0.05-0.1 units for visibility
   - Interpolate position along connection path
5. Conditional rendering:
   - Only render when traffic metrics exist
   - Respect carries_traffic flag
   - Respect flow_direction (forward/backward/bidirectional)

Foundation already in place:
✅ TrafficParticle struct defined (topology_viewport.rs:40-49)
✅ Traffic flow controls in UI
✅ Database fields for carries_traffic and flow_direction
✅ Traffic metrics available from database

Ready to bring the network to life with animated traffic flows!
```

### Option 5: Traffic Monitoring - Full Phase 6 (All Sub-Phases)
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Current Status: Phase 4 & 4.5 COMPLETE! ✅

Let's implement the FULL Phase 6 - Traffic Monitoring:

This is the most exciting feature! It includes:
1. Real-time traffic visualization with animated particles
2. Live metrics (throughput, latency, packet loss) on connections
3. Color-coded health status (green/yellow/red)
4. Traffic dashboard with charts and alerts
5. WebSocket streaming using Leptos native functions

Implementation order:
- Phase 6.1: Database & Mock Generator (1-2 hours)
- Phase 6.2: WebSocket Streaming (2-3 hours)
- Phase 6.3: Animated Connections (3-4 hours)
- Phase 6.4: Live Metrics Display (2-3 hours)
- Phase 6.5: Traffic Dashboard (2-3 hours)

Total estimated time: 10-14 hours for complete implementation.

Ready to bring this topology to life!
```

### Option 6: Create User Documentation
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Current Status: Phase 5 COMPLETE! ✅

Let's create comprehensive user documentation:
1. Update README.md with screenshots
2. Feature overview with examples
3. How to use the application (getting started guide)
4. Keyboard shortcuts reference
5. Tips and tricks

Take screenshots from http://127.0.0.1:3000 and write clear, user-friendly docs.

Ready to document!
```

### General: Ask for Guidance
```
I'm continuing the Network Topology Visualizer at /Users/mattearp/Documents/CodeProjects/ntv/

Read CLAUDE.md and SESSION-GUIDE.md for complete context.

Current Status: Phase 5 COMPLETE! ✅
All core features, export/import, and customization complete. Ready for Phase 6!

What should I work on next? Please review the options in SESSION-GUIDE.md and recommend the best next step based on:
1. User value
2. Completeness (rounding out existing features)
3. Ease of implementation
```

## 💡 Pro Tips

1. **Always read CLAUDE.md first** - Contains all architectural discoveries and solutions
2. **Use Context7 MCP** - When unsure about Leptos/three-d patterns, check `/websites/book_leptos_dev`
3. **Check git tags** - `git tag` shows v0.1.0-phase1-complete through v0.1.0-phase4-complete
4. **Test in browser** - http://127.0.0.1:3000 to see current state
5. **Console logs** - Browser console shows WASM logs from `web_sys::console`
6. **Real-time updates work!** - Save node properties in properties panel, viewport updates instantly
7. **Search before implementing** - Many patterns already exist (color pickers, bounding boxes, etc.)
8. **User preferences matter** - Small UX tweaks (10% vs 20% margin) can significantly improve feel

## 🚀 You're Ready!

**Phases 1-6 are COMPLETE!** ✅ All core features implemented:
- ✅ All core 3D features (rotation, models, grid, axes, topology switching, device palette, connections)
- ✅ All visual polish (labels, colors, rendering, selection, lighting, camera controls)
- ✅ All UI optimization (space, persistence, code quality, fullscreen, camera pan)
- ✅ Export & customization (PNG/JSON export, JSON import, node scale, background/node/connection colors)
- ✅ Polish features (viewport centering, zoom to fit with 10% margin, cloud type in UI)
- ✅ **Traffic monitoring (mock generator, color-coded connections, comprehensive tooltips)**
- ✅ **Link metrics impact (bandwidth, latency, congestion, packet loss)**

**Recommended next steps (all optional enhancements):**
1. **Traffic Dashboard** (High Value) - Metrics panel with historical charts
   - Network-wide stats and top connections
   - Time-series visualization
   - Export to CSV

2. **Traffic Animation System** (Visual Impact) - Particle flows along connections
   - Moving particles showing data flow
   - Speed/density based on metrics
   - Engaging demo presentation

3. **Additional UX Polish** - Multi-select, keyboard shortcuts, undo/redo
   - Power user features
   - Bulk operations

4. **Documentation** - User guide, deployment instructions
   - Help users learn the application
   - Production deployment guide

All architectural patterns are working and documented in CLAUDE.md.
Use the example prompts above to start your next session!
