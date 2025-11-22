# Native Blender Z-Up Coordinate System - Migration Complete ✅

## What Was Changed

### Code Changes (5 locations)

#### 1. **api.rs:271** - Removed default 90° X-rotation
```rust
// OLD
let rot_x = data.rotation_x.unwrap_or(90.0);  // Forced 90° rotation

// NEW
let rot_x = data.rotation_x.unwrap_or(0.0);   // No default rotation
```

#### 2. **topology_viewport.rs:1261-1263** - Fixed coordinate mapping
```rust
// OLD - Swapped Y and Z
let position = vec3(
    node.position_x as f32,
    node.position_z as f32,  // DB Z → Viewport Y
    node.position_y as f32,  // DB Y → Viewport Z
);

// NEW - Direct mapping (Z-up native)
let position = vec3(
    node.position_x as f32,  // X = X
    node.position_y as f32,  // Y = Y
    node.position_z as f32,  // Z = Z
);
```

#### 3. **topology_viewport.rs:1642** - Connection particle up vector
```rust
// OLD
let world_up = vec3(0.0, 1.0, 0.0);

// NEW
let world_up = vec3(0.0, 0.0, 1.0);
```

#### 4. **topology_viewport.rs:2847** - Connection cylinder up vector
```rust
// OLD
let up = vec3(0.0, 1.0, 0.0);

// NEW
let up = vec3(0.0, 0.0, 1.0);
```

#### 5. **topology_viewport.rs:2917** - Connection box up vector
```rust
// OLD
let up = vec3(0.0, 1.0, 0.0);

// NEW
let up = vec3(0.0, 0.0, 1.0);
```

---

## Database Migration

**File:** `migrations/20250122000001_fix_native_blender_coordinates.sql`

**What it does:**
1. Swaps `position_y` ↔ `position_z` for all 30 existing nodes
2. Changes `rotation_x = 90.0` to `rotation_x = 0.0` for all nodes with default rotation

**When it runs:**
- Automatically when you start the server with `cargo leptos watch`
- SQLx will detect the new migration and apply it

---

## Blender Export Settings - UPDATED! ⚠️

### OLD Settings (Don't use anymore):
```
Transform:
  ☑ +Y Up  ← This was needed before
```

### NEW Settings (Use from now on):
```
Format: glTF Binary (.glb)

Transform:
  ☐ +Y Up  ← UNCHECK THIS! Keep Z-up native

Geometry:
  ☑ Apply Modifiers
  ☑ UVs
  ☑ Normals
  ☑ Tangents

Materials:
  ☑ Materials
  ☑ Images (if using textures)
```

**Critical:** Before exporting, always:
1. Select All (A)
2. Apply All Transforms (Ctrl+A → All Transforms)
3. Export with **+Y Up UNCHECKED**

---

## What to Expect After Changes

### New Nodes (Created After Migration)
✅ **Will appear exactly as in Blender:**
- Position in Blender = Position in viewport
- Rotation in Blender = Rotation in viewport
- Z-up preserved natively

### Existing Nodes (30 nodes in database)
✅ **Will be automatically fixed by migration:**
- Y and Z coordinates swapped back to correct positions
- 90° X-rotation removed
- Should appear in same visual positions (just stored correctly)

### Connections
✅ **Will render correctly:**
- Up vectors updated to Z-up
- Particle flow animations will work correctly
- Connection lines align properly in Z-up space

---

## Testing Checklist

After running `cargo leptos watch`:

### 1. Check Existing Topology
- [ ] Load existing topology with 30 nodes
- [ ] Verify nodes appear in correct positions (visually same as before)
- [ ] Verify connections between nodes look correct
- [ ] Test traffic animation (particles should flow correctly)

### 2. Create New Node in Blender
- [ ] Create simple cube in Blender at origin (0,0,0)
- [ ] Apply All Transforms (Ctrl+A)
- [ ] Export with **+Y Up UNCHECKED**
- [ ] Add to viewport
- [ ] Should appear at origin, sitting on grid floor

### 3. Test Positioning
- [ ] Create node at (5, 0, 2) in Blender
- [ ] Export and add to viewport
- [ ] Should appear at X=5, Y=0, Z=2 (2 units above floor)

### 4. Test Rotation
- [ ] Create cube in Blender
- [ ] Rotate 45° around Z-axis
- [ ] Export and add to viewport
- [ ] Should appear rotated 45° (spinning in place, not tipped)

### 5. Test Rotation Sliders
- [ ] Select a node in viewport
- [ ] Adjust rotation_x slider → should tip forward/back (pitch)
- [ ] Adjust rotation_y slider → should spin left/right (yaw)
- [ ] Adjust rotation_z slider → should roll clockwise/counter (roll)

### 6. Test Dragging
- [ ] Drag node in viewport
- [ ] Should move smoothly on XY plane
- [ ] Check position values in Properties Panel (should match visual position)

---

## Coordinate System Reference

### Before Migration (Confusing!)
```
Database        Viewport        Blender
--------        --------        -------
position_x  →   X (left-right)  X
position_y  →   Z (up-down)     Z ← SWAPPED
position_z  →   Y (front-back)  Y ← SWAPPED
rotation_x  →   90° (forced)    0°
```

### After Migration (Native!)
```
Database        Viewport        Blender
--------        --------        -------
position_x  →   X (left-right)  X ← MATCH!
position_y  →   Y (front-back)  Y ← MATCH!
position_z  →   Z (up-down)     Z ← MATCH!
rotation_x  →   0° (natural)    0° ← MATCH!
```

---

## Axes Reference (Unchanged - Already Correct!)

The viewport axes were already correct for Z-up:

- **X-axis (Red):** Left ← → Right
- **Y-axis (Green):** Back ← → Front
- **Z-axis (Blue):** Down ← → Up (Vertical!)

Grid floor: XY plane at Z=0

---

## Validation Script Update

Your `validate_models.py` script still works! But now you should see:
- Models close to 1.0 units are "OPTIMAL SIZE"
- No need for 90° rotation compensation
- Bounding box calculations work natively

---

## Common Questions

### Q: Do I need to re-export all my existing .glb models?
**A:** Only if they were exported with **+Y Up checked**. If you had it unchecked before (preserving Blender's Z-up), they'll work perfectly now!

### Q: Will my existing topologies break?
**A:** No! The migration fixes the database so existing nodes will appear in the same visual positions.

### Q: What about the blob-* placeholder models?
**A:** They might appear rotated now (they were created assuming 90° X-rotation). You can either:
- Re-export them with correct settings
- Manually set their `rotation_x = 90.0` in the database (reverse fix)

### Q: Can I undo this migration?
**A:** Yes, but you'd need to:
1. Revert the code changes
2. Create a reverse migration (swap Y↔Z back, set rotation_x = 90.0)
3. Re-export all models with +Y Up checked

---

## Known Issues & Solutions

### Issue: Old placeholder models (blob-*) appear rotated

**Cause:** They were created for the old Y-up system.

**Solution 1 - Quick Fix (SQL):**
```sql
-- Set 90° X-rotation for blob models
UPDATE nodes
SET rotation_x = 90.0
WHERE model_name LIKE 'blob-%';
```

**Solution 2 - Proper Fix:**
- Re-export blob models from Blender with +Y Up unchecked
- Replace files in `public/models/`

### Issue: Custom models appear upside-down

**Cause:** Models were exported with +Y Up checked.

**Solution:**
- Re-export from Blender with +Y Up **unchecked**
- OR: Manually set `rotation_x = 90.0` for those specific nodes

### Issue: Connections look wrong

**Cause:** Unlikely, but possible if nodes at very specific angles.

**Solution:**
- Check node positions are correct (should match visual)
- Verify connection source/target are correct
- Traffic animation should flow along connection lines

---

## Files Modified

✅ `src/api.rs` - Line 271
✅ `src/islands/topology_viewport.rs` - Lines 1261-1263, 1642, 2847, 2917
✅ `migrations/20250122000001_fix_native_blender_coordinates.sql` - New migration

---

## Next Steps

1. **Start the server:**
   ```bash
   cargo leptos watch
   ```
   - Migration will run automatically
   - Check console for "Running database migrations..." message

2. **Test existing topology:**
   - Load your topology
   - Verify everything looks correct

3. **Create test model in Blender:**
   - Simple cube
   - Export with +Y Up **UNCHECKED**
   - Add to viewport
   - Verify it appears correctly

4. **Update your Blender export template:**
   - Save export settings in Blender
   - Document the +Y Up = unchecked requirement

---

## Success Criteria

✅ Existing nodes appear correctly after migration
✅ New models from Blender appear exactly as modeled
✅ Rotation sliders work intuitively (X=pitch, Y=yaw, Z=roll)
✅ Position values match visual positions
✅ Connections render correctly
✅ Traffic animations flow properly
✅ No coordinate mental gymnastics needed!

---

## Migration Status

- [x] Code changes applied (5 locations)
- [x] Database migration created
- [x] Compilation verified (no errors)
- [x] Ready to test!

**Current Node Count:** 30 nodes will be migrated automatically
**Migration File:** `migrations/20250122000001_fix_native_blender_coordinates.sql`

---

## Support

If you encounter issues:

1. **Check migration ran:**
   ```bash
   sqlite3 ntv.db "SELECT MAX(version) FROM _sqlx_migrations;"
   ```
   Should show: 20250122000001

2. **Verify coordinates swapped:**
   ```bash
   sqlite3 ntv.db "SELECT id, name, position_y, position_z, rotation_x FROM nodes LIMIT 5;"
   ```
   - rotation_x should be 0.0 (not 90.0)
   - Y and Z should be swapped from before

3. **Check console logs:**
   - Look for migration success messages
   - Check for any WebGL errors

---

## Congratulations! 🎉

You now have a **native Blender Z-up coordinate system** with:
- ✅ No coordinate swapping
- ✅ No forced rotations
- ✅ Direct Blender → Viewport workflow
- ✅ Intuitive rotation controls
- ✅ Position values that make sense

**Happy modeling!** 🎨
