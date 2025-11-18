# Network Topology Builder - Claude Development Notes

## Project Status
**Current Phase:** Phase 6.4.1 COMPLETE! ✅ (Traffic Flow Controls)
**Last Updated:** 2025-01-18
**Git Tags:** v0.1.0-phase1-complete, v0.1.0-phase2-complete, v0.1.0-phase3-complete, v0.1.0-phase4-complete, v0.1.0-phase5-complete, v0.1.0-phase5.7-complete, v0.1.0-phase6-complete
**Architecture:** Regular Leptos Components (Islands removed - see notes below)
**Next Phase:** Phase 6.4.2 - Particle Animation System (3D traffic animation)

### three-d API Audit (2025-11-14) ✅

**Status:** COMPLETE - See `docs/THREE-D-AUDIT-RESULTS.md` for full report

**Key Findings:**
- ✅ NTB uses latest three-d 0.18.x (current stable)
- ✅ Full PBR texture support implemented (albedo, metallic/roughness, normal, occlusion, emissive, alpha)
- ✅ HDR environment lighting fully compatible with textured materials
- ✅ No conflicts between image textures and HDR lighting
- ✅ Color matching solution: Textured materials + HDR environment = perfect Blender match

**Recommendations:**
1. **Textured workflow** (already supported) - Solves color space issues
2. **HDR environment lighting** (not yet implemented) - Solves lighting appearance
3. **Combined approach** - Matches Blender renders exactly

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

### Phase 5.6 - Full glTF/GLB Material Support COMPLETE! ✅ (2025-11-12)

**✅ COMPLETED:**
29. ✅ **Full glTF Material Support** (2025-11-12) - Complete PBR material pipeline with texture support
   - Fixed material rendering to properly support ALL glTF features
   - Two-path material system:
     - **Textured materials**: Use `PhysicalMaterial::new()` for full glTF support (no color conversion)
     - **Color-only materials**: Apply linear→sRGB conversion to fix three-d color space bug
   - Now properly supports:
     - ✅ Albedo/base color textures (image textures)
     - ✅ Metallic/roughness packed textures
     - ✅ Normal maps (surface detail)
     - ✅ Occlusion maps (ambient shadows)
     - ✅ Emissive textures (glowing LEDs, screens)
     - ✅ Alpha transparency (glass, transparent parts)
   - Console logging shows which features are active per material
   - Texture workflow = perfect Blender color match (no conversion issues!)

**Technical Implementation:**
```rust
// topology_viewport.rs:940-975
if has_textures {
    // Full glTF material with all texture support
    PhysicalMaterial::new(&context, gltf_mat)
} else {
    // Color-only: apply sRGB conversion
    let corrected_albedo = convert_linear_color_to_srgba(&gltf_mat.albedo);
    PhysicalMaterial::new_opaque(&context, &CpuMaterial {
        albedo: corrected_albedo,
        metallic: gltf_mat.metallic,
        roughness: gltf_mat.roughness,
        ..Default::default()
    })
}
```

**Color Space Handling:**
- **glTF Spec**: Stores `baseColorFactor` in linear RGB space
- **Textures**: Already in sRGB color space (no conversion needed)
- **three-d Bug**: Library treats linear values as sRGB without conversion
- **Solution**:
  - Textured materials → Use as-is (textures correct)
  - Color-only → Apply exact sRGB transfer function (gamma 2.4 piecewise)

**sRGB Conversion Formula (lines 633-657):**
```rust
fn linear_to_srgb(linear: f32) -> f32 {
    if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}
```

**Benefits:**
- Image textures match Blender exactly (recommended workflow)
- Full PBR material capabilities (industry standard)
- Normal maps add detail without geometry
- Emissive materials for glowing LEDs
- Alpha transparency for glass/screens
- Proper color space handling

### Phase 5.7 - HDR Environment Lighting COMPLETE! ✅ (2025-11-14)

**✅ COMPLETED:**
30. ✅ **HDR Environment Lighting System** (2025-11-14) - Realistic studio lighting
   - Database migration: `20250114000001_add_environment_lighting.sql`
   - Added `use_environment_lighting` (BOOLEAN) and `environment_map` (TEXT) to ui_settings
   - HDR file loading with `three-d-asset` hdr feature enabled (Cargo.toml)
   - Skybox creation from equirectangular HDR maps
   - `AmbientLight::new_with_environment()` for realistic ambient illumination
   - Fallback to manual three-point lighting when disabled

31. ✅ **HDR UI Controls** (2025-11-14) - User control over environment lighting
   - Toggle button in View Controls panel (Enabled/Disabled)
   - Dropdown selector for HDR environment maps:
     - Studio Small (2K) - `studio_small_09_2k.hdr`
     - Studio Loft (2K) - `photo_studio_loft_hall_2k.hdr`
     - Photo Studio (4K) - `photo_studio_01_4k.hdr`
     - Docklands (2K) - `docklands_02_2k.hdr`
   - Real-time viewport updates when settings change
   - Reactive signal tracking via `.get()` instead of `.get_untracked()`

32. ✅ **Settings Persistence** (2025-11-14) - All UI settings saved to database
   - Auto-load on page mount via Effect with spawn_local
   - Auto-save when any setting changes (viewport visibility, lighting, HDR)
   - `settings_loaded` flag prevents save during initial load
   - Updated `get_ui_settings()` SQL query to include new columns
   - Updated `update_ui_settings()` to save HDR preferences

33. ✅ **Conditional Lighting System** (2025-11-14) - Critical fix for HDR + texture compatibility
   - **Problem:** HDR environment + directional lights caused texture washout
   - **Root Cause:** All 4 lights (ambient + key + fill + rim) rendered simultaneously with HDR
   - **Solution:** Conditional lighting in render loop:
     - HDR mode: Use ONLY ambient light (contains HDR environment)
     - Manual mode: Use all 4 lights (ambient + key + fill + rim)
   - **Implementation:** Dynamic signal reading in render closure (topology_viewport.rs:1284-1290)
   - Pass `use_environment_lighting_signal` to render function for real-time updates
   - Result: Textured materials now display correctly with HDR lighting

**Technical Implementation:**
```rust
// 1. HDR loading (topology_viewport.rs:811-844)
let skybox_option: Option<Skybox> = if use_environment_lighting {
    let hdr_url = format!("{}/environments/{}", origin, environment_map);
    match load_async(&[hdr_url.as_str()]).await {
        Ok(mut loaded) => {
            match loaded.deserialize::<three_d_asset::Texture2D>(&environment_map) {
                Ok(hdr_texture) => {
                    Some(Skybox::new_from_equirectangular(&context, &hdr_texture))
                }
                Err(e) => None,
            }
        }
        Err(e) => None,
    }
} else {
    None
};

// 2. Ambient light creation (topology_viewport.rs:1212-1224)
let ambient = if use_environment_lighting && skybox_option.is_some() {
    let skybox = skybox_option.as_ref().unwrap();
    Rc::new(AmbientLight::new_with_environment(
        &context, ambient_intensity, Srgba::WHITE, skybox.texture()
    ))
} else {
    Rc::new(AmbientLight::new(&context, ambient_intensity, Srgba::WHITE))
};

// 3. Dynamic signal reading in render closure (topology_viewport.rs:1287-1290)
move |state: CameraState| {
    // Read HDR environment signal dynamically each frame
    let use_env_lighting = use_env_lighting_signal
        .map(|sig| sig.get_untracked())
        .unwrap_or(use_environment_lighting);

    // ... render code ...

    // 4. Conditional lighting (topology_viewport.rs:1359-1366)
    if use_env_lighting {
        // HDR mode: Use ONLY ambient (contains HDR environment)
        target.render(&camera, mesh_to_render, &[&*ambient]);
    } else {
        // Manual mode: Use full three-point lighting
        target.render(&camera, mesh_to_render, &[&*ambient, &*key_light, &*fill_light, &*rim_light]);
    }
}
```

**HDR Files Location:**
- **Directory:** `public/environments/`
- **Source:** Poly Haven (https://polyhaven.com/hdris)
- **Format:** Equirectangular .hdr files
- **Sizes:** 2K (6MB) and 4K (24MB) available

**Benefits Achieved:**
- ✅ Perfect color + lighting match with Blender (when using same HDR)
- ✅ Realistic reflections on metallic device surfaces
- ✅ Professional studio appearance
- ✅ No manual light tuning needed
- ✅ Settings persist across page refreshes
- ✅ Textured materials display correctly with HDR (no washout)
- ✅ Real-time HDR toggle without viewport reinitialization

**Key Lessons Learned (Phase 5.7):**

1. **Reactive Signals in Effects** - Use `.get()` not `.get_untracked()` for reactivity
   - Changed viewport Effect to track HDR settings with `.get()`
   - Enables automatic viewport refresh when settings change

2. **HDR + Directional Lights = Overexposure**
   - HDR environment maps provide comprehensive lighting
   - Adding directional lights on top causes massive overexposure
   - Solution: Conditional lighting based on mode
   - HDR mode: ambient only | Manual mode: full 3-point lighting

3. **Dynamic Signal Reading in Closures**
   - Pass signals (not values) to render closures for real-time updates
   - Use `RwSignal<bool>` parameter in function signature
   - Read signal with `.get_untracked()` each frame in render loop
   - Enables toggle without reinitializing entire viewport

4. **Blender Texture Workflow**
   - Image textures must be properly UV-mapped in Blender
   - Export glTF with embedded textures
   - Texture colors appear correct in web app (no color space conversion needed)
   - Albedo texture + HDR environment = perfect Blender match

### Phase 6 - Traffic Monitoring COMPLETE! ✅ (2025-01-15)

**Overview:** Real-time traffic visualization transforms the static 3D network diagram into a live monitoring tool. Mock traffic generator simulates realistic network patterns with color-coded connections showing utilization levels.

**✅ COMPLETED:**

#### Phase 6.1 - Mock Traffic Generator (2025-01-15)
34. ✅ **Server-Side Traffic Generation** - Realistic network traffic simulation
   - Server function `generate_mock_traffic()` creates traffic for all active connections
   - Three intensity levels: Low (10-30%), Medium (30-70%), High (70-95%)
   - Traffic patterns vary by connection type (Fiber > Ethernet > Wireless)
   - Respects connection status (only "Active" connections generate traffic)
   - Uses actual bandwidth from connection properties
   - Database storage: traffic_metrics table (connection_id, throughput, latency, packet_loss, utilization)
   - `clear_traffic()` server function removes all traffic data

35. ✅ **Traffic Control UI** - User interface for traffic management
   - Traffic Controls panel in Properties section (bottom of panel)
   - "Generate Traffic" button with dropdown intensity selector (Low/Medium/High)
   - "Clear Traffic" button to reset all traffic data
   - Real-time viewport updates when traffic is generated/cleared
   - Button styling with Tailwind CSS (green for generate, red for clear)

#### Phase 6.2 - Traffic Visualization (2025-01-15)
36. ✅ **Color-Coded Connections** - Visual traffic load indication
   - Dynamic connection colors based on utilization percentage:
     - Green: 0-40% utilization (healthy)
     - Orange: 40-70% utilization (moderate)
     - Red: 70-100% utilization (heavy/critical)
   - Color updates in real-time when traffic changes
   - Manual color override capability (custom colors take precedence)
   - Traffic data fetched via `get_all_traffic_metrics()` server function
   - Connection rendering with proper lighting (ambient + directional)

37. ✅ **Traffic Tooltips** - Detailed metrics on hover
   - Enhanced connection tooltips show utilization percentage
   - Color-coded utilization display (green/orange/red)
   - Displays source → target connection name
   - Tooltip data structure supports both Node and Connection types
   - Ray-cylinder intersection for accurate hover detection

#### Phase 6.3 - Link Metrics Impact (2025-01-15)
38. ✅ **Bandwidth-Aware Traffic** - Realistic throughput calculations
   - Traffic generation uses actual link bandwidth from database
   - Throughput calculated as: `bandwidth_mbps * utilization_pct / 100.0`
   - Intensity level determines utilization range, not absolute values
   - Different connection types have different throughput patterns

39. ✅ **Congestion-Based Latency** - Dynamic latency calculation
   - Base latency from connection properties (default 10ms)
   - Congestion penalties based on utilization:
     - < 40%: Base latency + jitter only
     - 40-70%: Base + slight congestion penalty (0.1-0.3x per % over 40)
     - > 70%: Base + significant congestion penalty (0.5-1.5x per % over 70)
   - Random jitter (-2ms to +3ms) for realistic variation
   - Higher utilization = higher latency (simulates real network behavior)

40. ✅ **Exponential Packet Loss** - Realistic packet loss patterns
   - Packet loss increases exponentially with utilization:
     - < 60%: 0-0.1% (minimal, healthy network)
     - 60-80%: 0.1-0.5% (occasional drops)
     - 80-90%: 0.5-2.0% (noticeable degradation)
     - > 90%: 2-5% (severe congestion)
   - Random variation within ranges for realism
   - Models real-world network congestion behavior

41. ✅ **Comprehensive Tooltips** - All metrics displayed
   - Connection hover tooltips now show 4 metrics:
     - Utilization (color-coded: green/orange/red)
     - Throughput in Mbps (blue)
     - Latency in ms (color-coded: green < 20ms, yellow 20-50ms, orange > 50ms)
     - Packet Loss in % (color-coded: green < 0.5%, yellow 0.5-2%, red > 2%)
   - All metrics update in real-time with traffic changes
   - Clean, readable tooltip design with color coding

**Technical Implementation:**

```rust
// Traffic generation algorithm (api.rs:1011-1052)
let utilization_pct = rng.gen_range(utilization_range.clone());
let throughput_mbps = bandwidth_mbps * utilization_pct / 100.0;

// Congestion-based latency
let base_latency = connection.latency_ms.unwrap_or(10.0);
let congestion_penalty = if utilization_pct > 70.0 {
    (utilization_pct - 70.0) * rng.gen_range(0.5..1.5)
} else if utilization_pct > 40.0 {
    (utilization_pct - 40.0) * rng.gen_range(0.1..0.3)
} else {
    0.0
};
let latency_ms = (base_latency + congestion_penalty + jitter).max(0.1);

// Exponential packet loss
let packet_loss_pct = if utilization_pct > 90.0 {
    rng.gen_range(2.0..5.0)
} else if utilization_pct > 80.0 {
    rng.gen_range(0.5..2.0)
} else if utilization_pct > 60.0 {
    rng.gen_range(0.1..0.5)
} else {
    rng.gen_range(0.0..0.1)
};
```

**Key Features:**
- ✅ Realistic traffic simulation using link properties
- ✅ Three-tier intensity control (Low/Medium/High)
- ✅ Color-coded connections for instant visual feedback
- ✅ Comprehensive tooltips with 4 metrics
- ✅ Congestion-based latency modeling
- ✅ Exponential packet loss at high utilization
- ✅ Manual color override capability
- ✅ Real-time viewport updates

**Database Schema:**
```sql
CREATE TABLE IF NOT EXISTS connection_traffic_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    connection_id INTEGER NOT NULL,
    throughput_mbps REAL NOT NULL,
    latency_ms REAL NOT NULL,
    packet_loss_pct REAL NOT NULL,
    utilization_pct REAL NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE CASCADE
);
```

#### Phase 6.4.1 - Traffic Flow Controls (2025-01-18) ✅ COMPLETE

42. ✅ **Traffic Flow Configuration** - User control over traffic animation
   - Database migration: `20250118000001_add_traffic_flow_controls.sql`
   - Added `carries_traffic` (BOOLEAN, default TRUE) - Enable/disable traffic animation per connection
   - Added `flow_direction` (TEXT, default 'source_to_target') - Control particle direction
   - SQLite triggers enforce valid directions: 'source_to_target', 'target_to_source', 'bidirectional'
   - Properties Panel UI: Checkbox to enable/disable traffic + Radio buttons for direction
   - Updated Connection model with new fields (src/models/connection.rs:19-20)
   - Updated all SQL queries in api.rs (5 locations: lines 200, 525-526, 580-581, 675-676, 985-986)

43. ✅ **Swap Source/Target Button** - Reverse connection direction
   - Server function: `swap_connection_direction(id: i64)` (api.rs:1190-1226)
   - SQL UPDATE swaps source_node_id and target_node_id
   - UI button in Properties Panel with loading state
   - Real-time viewport refresh after swap
   - Use case: Correct connection direction without recreating

44. ✅ **TrafficParticle Struct** - Data structure for animation system
   - Defined in topology_viewport.rs:40-49
   - Fields: connection_id, position (0.0-1.0), speed, color, direction_forward
   - Ready for particle system implementation in Phase 6.4.2

**Database Migration:**
```sql
-- migrations/20250118000001_add_traffic_flow_controls.sql
ALTER TABLE connections ADD COLUMN carries_traffic BOOLEAN NOT NULL DEFAULT TRUE;
ALTER TABLE connections ADD COLUMN flow_direction TEXT NOT NULL DEFAULT 'source_to_target';

-- Validation triggers
CREATE TRIGGER check_flow_direction_insert
BEFORE INSERT ON connections
FOR EACH ROW
WHEN NEW.flow_direction NOT IN ('source_to_target', 'target_to_source', 'bidirectional')
BEGIN
    SELECT RAISE(ABORT, 'flow_direction must be one of: source_to_target, target_to_source, bidirectional');
END;
```

**Properties Panel UI (topology_editor.rs:2708-2775):**
```rust
// Checkbox: Enable/Disable Traffic
<input
    type="checkbox"
    checked=move || carries_traffic.get()
    on:change=move |ev| carries_traffic.set(event_target_checked(&ev))
/>

// Radio Buttons: Flow Direction
<input type="radio" value="source_to_target" checked=move || flow_direction.get() == "source_to_target" />
<input type="radio" value="target_to_source" checked=move || flow_direction.get() == "target_to_source" />
<input type="radio" value="bidirectional" checked=move || flow_direction.get() == "bidirectional" />

// Swap Button (topology_editor.rs:2591-2605)
<button on:click=move |_| { swap_action.dispatch(()); }>
    "🔄 Swap Source ↔ Target"
</button>
```

**TrafficParticle Struct (topology_viewport.rs:40-49):**
```rust
#[cfg(feature = "hydrate")]
#[derive(Clone, Debug)]
struct TrafficParticle {
    connection_id: i64,
    position: f32,  // 0.0 to 1.0 along connection path
    speed: f32,     // Movement speed per frame (units per second)
    color: three_d::Srgba,  // Particle color based on utilization
    direction_forward: bool, // true = source->target, false = target->source
}
```

**Key Benefits:**
- ✅ Users can selectively enable/disable traffic on specific connections
- ✅ Control traffic flow direction (source→target, target→source, bidirectional)
- ✅ Quickly reverse connection direction without recreating it
- ✅ Foundation ready for particle animation system
- ✅ All new fields stored in database with validation

**Next Steps (Phase 6.4.2 - Particle Animation):**
1. ⏳ Particle storage (Vec<TrafficParticle> in Rc<RefCell<>>)
2. ⏳ Spawning logic based on traffic metrics and carries_traffic flag
3. ⏳ Utilization-based density (1-3 for <40%, 3-7 for 40-70%, 7-12 for >70%)
4. ⏳ Animation loop with requestAnimationFrame (60fps updates)
5. ⏳ Render particles as small glowing spheres using three-d
6. ⏳ Conditional rendering (only when traffic exists)
7. ⏳ Particle interpolation along connection path
8. ⏳ Particle recycling at destination (respawn at source)

**Key Lessons Learned (Phase 6):**

1. **Realistic Traffic Modeling**
   - Use actual link properties (bandwidth, latency, status)
   - Model congestion effects (latency increases, packet loss)
   - Random variation within realistic ranges
   - Different patterns for different connection types

2. **Color-Coded Visualization**
   - Instant visual feedback with traffic load colors
   - Manual color override for specific use cases
   - Proper lighting required for color visibility
   - Three-tier thresholds (0-40%, 40-70%, 70-100%)

3. **Traffic Data Management**
   - Separate table for traffic metrics (not in connections table)
   - Cascade delete when connection removed
   - HashMap lookup for efficient access in viewport
   - Real-time updates via refetch trigger

4. **Tooltip Enhancement Pattern**
   - Enum-based tooltip data (Node vs Connection)
   - Color-coded metrics for quick interpretation
   - Display all relevant metrics without clutter
   - Update tooltip data in mousemove handler

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

## Phase 6 - Traffic Monitoring (Future Enhancements - OPTIONAL)

### Status
**Core traffic monitoring complete!** Phases 6.1-6.3 are fully implemented with realistic traffic generation, color-coded connections, and comprehensive tooltips. Remaining items are optional enhancements for advanced features.

### ✅ Completed Features (Phases 6.1-6.3)
1. ✅ Mock traffic generator with realistic patterns
2. ✅ Color-coded connections (green/orange/red by utilization)
3. ✅ Traffic metrics tooltips (utilization, throughput, latency, packet loss)
4. ✅ Link properties impact (bandwidth, latency, congestion modeling)
5. ✅ Three-tier intensity control (Low/Medium/High)
6. ✅ Manual color override capability

### Optional Future Enhancements

#### 1. Real-Time Traffic Animation (Phase 6.4 - Optional)
- **Animated connections:** Flowing particles/pulses moving along connection paths
- **Direction indicators:** Particles move from source to target showing data flow direction
- **Speed variation:** Faster particles = higher throughput
- **Particle density:** More particles = more active connection
- **Implementation:** three-d particle system with instanced rendering

#### 2. Traffic Dashboard (Phase 6.5 - Optional)
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

#### 3. WebSocket Streaming (Phase 6.6 - Optional)
- **Real-time updates:** Live traffic data streaming to viewport
- **Leptos Native WebSocket:**
  ```rust
  #[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
  async fn stream_traffic_data(
      input: BoxedStream<TrafficRequest, ServerFnError>
  ) -> Result<BoxedStream<TrafficUpdate, ServerFnError>, ServerFnError> {
      // Server streams traffic updates every 100-500ms
      Ok(traffic_stream)
  }
  ```
- **Benefits:** Automatic updates without manual refresh, lower latency
- **Implementation:** Signal::from_stream() for reactive updates

### Recommended Next Steps
While Phase 6 core features are complete, here are some high-value next steps to consider:

1. **User Experience Polish**
   - Add loading states for traffic generation
   - Toast notifications for successful operations
   - Better error messaging

2. **Traffic Dashboard (High Value)**
   - Simple metrics panel showing network-wide stats
   - Top 5 busiest connections list
   - Total throughput counter
   - Average latency display

3. **Animation System (Visual Impact)**
   - Particle flows along connections
   - Direction indicators
   - Speed based on throughput
   - Makes demos more engaging

4. **Real Integration**
   - Replace mock generator with real network data sources
   - SNMP integration for network devices
   - API connectors for cloud platforms
   - Transforms from demo tool to production monitoring

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
**Repo:** https://github.com/madkrell/ntb.git
**Tags:** v0.1.0-phase1-complete, v0.1.0-phase2-complete, v0.1.0-phase3-complete, v0.1.0-phase4-complete, v0.1.0-phase5-complete, v0.1.0-phase5.7-complete, v0.1.0-phase6-complete
**Current Branch:** phase-6.2-traffic-visualization
**Next Tag:** v0.1.0-phase7 (if continuing with optional features)

## All Known Issues & Solutions

See original CLAUDE.md for complete list. Key patterns to remember:
1. **Server Functions** - Use leptos_axum::extract() for database access
2. **Event Handlers** - Use mutable storage (Rc<RefCell<>>) for data access
3. **Disposed Signals** - Use Arc<Mutex<>> snapshot for event handlers with .forget()
4. **Context Collision** - Wrap same-typed signals in unique struct
5. **Canvas Resize** - Always update dimensions on every render
6. **Bounding Box** - Calculate from actual node positions, not fixed values
