# Vendor-Based Model Selection - Implementation Status

## ✅ COMPLETED (95% Done!)

### 1. Database Schema & Models
- ✅ Added `vendor` and `model_name` fields to Node model
- ✅ Created migration: `migrations/20250107000003_add_vendor_model.sql`
- ✅ Updated all DTOs (CreateNode, UpdateNode)
- ✅ Added `APPLICATION` node type constant

### 2. File Structure
- ✅ Created vendor folder hierarchy:
  ```
  public/models/{type}/{vendor}/*.glb
  public/icons/vendors/*.svg
  ```
- ✅ Moved all existing models to `generic/` subfolders
- ✅ Created placeholder `generic.svg` icon

### 3. Server Functions (API)
- ✅ `get_vendors_for_type()` - Auto-discovers vendors/models from filesystem
- ✅ Updated `create_node()` to accept vendor/model_name
- ✅ Updated `update_node()` to handle vendor/model_name
- ✅ All SQL queries updated to include new fields

### 4. Viewport Rendering
- ✅ Updated model loading to use vendor/model paths:
  ```rust
  let model_url = format!("{}/models/{}/{}/{}", origin, node_type, vendor, model_name);
  ```
- ✅ Model caching by vendor:model combination

### 5. UI Components
- ✅ All Device Palette buttons now plural ("Routers", "Switches", "Applications")
- ✅ Properties Panel shows vendor and model_name fields
- ✅ Created VendorSection component for rendering individual vendors
- ✅ Created VendorDropdown component structure

## 🔧 REMAINING ISSUES

### Leptos Closure Compilation Errors

**Problem:** Nested closures in Suspense components causing type mismatch errors.

**Affected Files:**
1. `src/islands/topology_editor.rs:541` - Topology selector Suspense
2. VendorDropdown component closure captures

**Root Cause:**
- `Suspense` children closure returns `Option<View>` but needs consistent type
- Nested `.map()` capturing moved values multiple times

**Solution Options:**

#### Option A: Use Show Component (Recommended)
```rust
<Suspense fallback=|| view! { <div>"Loading..."</div> }>
    <Show
        when=move || topologies.get().is_some()
        fallback=|| view! { <div>"No topologies"</div> }
    >
        {move || {
            let topos = topologies.get().unwrap();
            // ... rest of view
        }}
    </Show>
</Suspense>
```

#### Option B: Match Instead of Map
```rust
<Suspense fallback=|| view! { <div>"Loading..."</div> }>
    {move || {
        match topologies.get() {
            Some(topos) => view! { /* content */ }.into_any(),
            None => view! { <div>"No data"</div> }.into_any(),
        }
    }}
</Suspense>
```

#### Option C: Unwrap with Default
```rust
<Suspense fallback=|| view! { <div>"Loading..."</div> }>
    {move || {
        let topos = topologies.get().unwrap_or_default();
        view! { /* content */ }
    }}
</Suspense>
```

## 📋 QUICK FIX CHECKLIST

1. **Fix Topology Selector (line 541):**
   - Replace `.map()` with `match` or `Show` component
   - Ensure consistent return type

2. **Verify VendorDropdown:**
   - Component extraction already done ✅
   - Should compile once topology selector fixed

3. **Test Application:**
   ```bash
   cargo build
   cargo leptos watch
   ```

4. **Verify Features:**
   - Create node with vendor dropdown
   - Check model loads from correct vendor path
   - Edit node vendor/model in Properties Panel

## 🎯 EXPECTED BEHAVIOR

### Creating Nodes:
1. Click device type button (e.g., "Routers")
2. Dropdown shows vendors with icons
3. Each vendor shows available models
4. Click model → node created with vendor/model_name

### Model Loading:
- Path: `/models/{type}/{vendor}/{model_name}.glb`
- Example: `/models/router/cisco/asr9000.glb`
- Fallback: `/models/router/generic/blob-router.glb`

### Properties Panel:
- Shows current vendor and model_name
- Editable text fields
- Changes saved to database

## 📚 HELPFUL RESOURCES

- **Leptos Book - Suspense:** https://book.leptos.dev/async/10_resources.html#suspense
- **Leptos Book - Show Component:** https://book.leptos.dev/view/04_iteration.html#show
- **File Location:** `src/islands/topology_editor.rs`
- **Vendor Models:** `src/models/vendor.rs`
- **API Functions:** `src/api.rs` (lines 807-923)

## 💡 ARCHITECTURE NOTES

### Why Component Extraction?
- Avoids nested closure ownership issues
- Cleaner separation of concerns
- Easier to test and maintain
- Leptos best practice

### VendorSection Component
Each vendor rendered as separate component:
- Receives vendor data as props
- Handles own model list rendering
- No nested closures = no ownership conflicts

### Auto-Discovery System
Server scans filesystem on each request:
- Lists folders in `public/models/{type}/`
- Each folder = one vendor
- Lists `.glb` files in vendor folder
- Checks for matching SVG icon

Add new vendor by:
1. Create folder: `public/models/router/fortinet/`
2. Add model: `fortigate.glb`
3. Add icon: `public/icons/vendors/fortinet.svg`
4. ✅ Auto-appears in dropdown!

## 🚀 NEXT STEPS AFTER FIX

1. Run database migration (happens automatically on first run)
2. Create vendor folders and add models
3. Add vendor SVG icons
4. Test creating nodes with different vendors
5. Verify 3D viewport loads correct models

## ⚡ QUICK TEST

```bash
# Create test vendor structure
mkdir -p public/models/router/cisco
cp public/models/router/generic/blob-router.glb public/models/router/cisco/catalyst.glb

# Create Cisco icon
cat > public/icons/vendors/cisco.svg << 'EOF'
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
  <rect x="2" y="10" width="4" height="8" rx="1"/>
  <rect x="10" y="6" width="4" height="12" rx="1"/>
  <rect x="18" y="8" width="4" height="10" rx="1"/>
</svg>
EOF

# Build and run
cargo leptos watch
```

## 📞 SUPPORT

If stuck on Leptos closures:
1. Check Leptos Discord: https://discord.gg/leptos
2. Review Leptos Book section on closures and reactivity
3. Use `leptos-expert` subagent (already tried, gave good advice!)

---

**Status:** Implementation 95% complete. Just need to fix Suspense return types!
