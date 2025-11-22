# Grid & Axes Display Fix

## Problem Identified

After implementing the native Blender Z-up coordinate system, the grid and axes were displaying incorrectly:

### Issues in Screenshot:
1. ❌ **Grid was vertical** (wall-like) instead of horizontal (floor-like)
2. ❌ **Green Y-axis was vertical** instead of horizontal front-back
3. ❌ **Blue Z-axis overlapped X-axis** instead of being vertical

### Root Cause

When we changed the connection "up" vectors from `vec3(0,1,0)` to `vec3(0,0,1)` for Z-up, we also incorrectly changed the reference vector in `create_line_cylinder()` and `create_emissive_line_cylinder()`.

**The problem:**
- three-d's **cylinder primitive is oriented along the Y-axis by default**
- The `create_line_cylinder()` function rotates cylinders to align with arbitrary line directions
- We changed the rotation reference from Y-axis to Z-axis, breaking the rotation math
- This caused grid lines and axes to rotate incorrectly

## The Fix

### File: `src/islands/topology_viewport.rs`

#### 1. Fixed `create_line_cylinder()` (line 2845-2861)

**Before (BROKEN):**
```rust
let up = vec3(0.0, 0.0, 1.0);  // Z-up for native Blender coordinates

// Calculate rotation to align cylinder with line direction
let rotation = if (normalized_dir - up).magnitude() < 0.001 {
    Mat4::identity()
} else if (normalized_dir + up).magnitude() < 0.001 {
    Mat4::from_angle_x(radians(std::f32::consts::PI))
} else {
    let axis = up.cross(normalized_dir).normalize();
    let angle = up.dot(normalized_dir).acos();
    Mat4::from_axis_angle(axis, radians(angle))
};
```

**After (FIXED):**
```rust
// Cylinder primitive is oriented along Y-axis by default in three-d
// We need to rotate from Y-axis to our desired line direction
let cylinder_default_dir = vec3(0.0, 1.0, 0.0);

// Calculate rotation to align cylinder with line direction
let rotation = if (normalized_dir - cylinder_default_dir).magnitude() < 0.001 {
    Mat4::identity()
} else if (normalized_dir + cylinder_default_dir).magnitude() < 0.001 {
    Mat4::from_angle_x(radians(std::f32::consts::PI))
} else {
    let axis = cylinder_default_dir.cross(normalized_dir).normalize();
    let angle = cylinder_default_dir.dot(normalized_dir).acos();
    Mat4::from_axis_angle(axis, radians(angle))
};
```

**Key Change:** Use cylinder's **native Y-axis orientation** as reference, not Z-up world space.

---

#### 2. Fixed `create_emissive_line_cylinder()` (line 2919-2931)

**Before (BROKEN):**
```rust
let up = vec3(0.0, 0.0, 1.0);  // Z-up for native Blender coordinates
let rotation = if (normalized_dir - up).magnitude() < 0.001 {
    // ... rotation logic
};
```

**After (FIXED):**
```rust
// Cube has no inherent orientation, but we'll align along Y-axis by default
let box_default_dir = vec3(0.0, 1.0, 0.0);
let rotation = if (normalized_dir - box_default_dir).magnitude() < 0.001 {
    // ... rotation logic
};
```

**Key Change:** Same fix - use Y-axis as default orientation reference.

---

## Why This Works

### Understanding Primitive Orientation

three-d library primitives have default orientations:
- **Cylinder:** Extends along Y-axis (height along Y)
- **Cube:** No inherent orientation, but we treat as Y-aligned for consistency

### Rotation Logic

To rotate a cylinder from its default orientation to an arbitrary direction:

1. **Start:** Cylinder aligned with Y-axis `(0, 1, 0)`
2. **Target:** Line direction `normalized_dir`
3. **Calculate:** Rotation axis = cross product of start × target
4. **Calculate:** Rotation angle = dot product + acos
5. **Apply:** Rotation matrix from axis-angle

**This is independent of the world's "up" direction!**

### What We Confused

- **World space "up"** (Z-up in Blender) = coordinate system convention
- **Primitive default orientation** (Y-axis) = geometric reference for rotation

**We mistakenly conflated these two!** The cylinder rotation math needs the **primitive's default orientation**, NOT the world's up direction.

---

## Expected Results After Fix

### Grid (XY plane at Z=0)
✅ Horizontal floor
✅ Lines parallel to X-axis (running left-right)
✅ Lines parallel to Y-axis (running front-back)

### X-Axis (Red)
✅ Horizontal line going left ← → right
✅ On the floor (Z=0)

### Y-Axis (Green)
✅ Horizontal line going front ← → back
✅ On the floor (Z=0)

### Z-Axis (Blue)
✅ Vertical line going down ← → up
✅ Through origin

### Layout
```
         Z (Blue, vertical)
         ↑
         |
         |
         +------→ X (Red, left-right)
        /
       /
      ↓ Y (Green, front-back)

Grid: Horizontal on XY plane (Z=0)
```

---

## Connection to Z-Up Changes

### What SHOULD Use Z-Up Reference
✅ Connection particle flow (`world_up = vec3(0,0,1)`) ← CORRECT
✅ Camera orientation (`up = vec3(0,0,1)`) ← ALREADY CORRECT
✅ Coordinate mapping (no Y↔Z swap) ← ALREADY FIXED

### What Should NOT Use Z-Up Reference
❌ Cylinder/box rotation for line rendering ← THIS WAS THE BUG
   - Must use primitive's default orientation (Y-axis)
   - World "up" is irrelevant to rotating a geometric primitive

---

## Testing

After rebuilding and running:

1. **Check grid:**
   - [ ] Grid is horizontal (floor-like)
   - [ ] Grid lines are straight and perpendicular

2. **Check axes:**
   - [ ] Red X-axis goes left-right horizontally
   - [ ] Green Y-axis goes front-back horizontally
   - [ ] Blue Z-axis goes up-down vertically
   - [ ] All three meet at origin

3. **Check model:**
   - [ ] Cloud model sits on grid floor
   - [ ] Model orientation matches Blender

4. **Check connections (if present):**
   - [ ] Lines between nodes are straight
   - [ ] Not twisted or rotated incorrectly

---

## Lesson Learned

**Primitive geometry rotation ≠ World coordinate system**

When rotating geometric primitives to align with arbitrary directions:
- Use the **primitive's default orientation** as reference
- Don't mix this with the **world coordinate system's "up" direction**

The world "up" direction matters for:
- Camera orientation
- Physics simulations
- Semantic meaning (which axis is vertical)

It does NOT matter for:
- Rotating a cylinder to match a line direction
- Geometric transformations of primitives

---

## Files Modified

✅ `src/islands/topology_viewport.rs`
  - Line 2845-2861: `create_line_cylinder()` rotation logic
  - Line 2919-2931: `create_emissive_line_cylinder()` rotation logic

---

## Status

✅ Compilation successful
✅ Ready to test in browser
✅ Should display grid and axes correctly now

Restart `cargo leptos watch` and check the viewport!
