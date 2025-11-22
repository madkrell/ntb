# Blender Material Fix Guide - Preserving Multi-Color Models

## Understanding the Problem

Your firewall model has **6 materials** (Black, Orange, F1, F2, F3, F4), and each material is assigned to **different faces** of the mesh. This is what creates the multi-colored appearance:

- **Black** material → Black outline/edges
- **Orange** material → Orange bricks
- **F1** material → Yellow part of flame
- **F2** material → Orange part of flame
- **F3** material → Another flame color
- **F4** material → Another part of the model

The current shader setup (Diffuse BSDF + Glossy BSDF) shows colors in Blender's viewport, but the **glTF exporter doesn't understand these old nodes**. We need to convert each material to use **Principled BSDF** while preserving the exact colors.

---

## Step-by-Step Fix (Do this for EACH of the 6 materials)

### Step 1: Identify Current Colors

Before changing anything, note down the current color of each material:

1. Click on material slot "Black" in the material properties
2. Switch to Shading workspace (top menu)
3. Look at the shader nodes - note the color values
4. **Take a screenshot** or write down RGB values for safety
5. Repeat for all 6 materials (Black, Orange, F1, F2, F3, F4)

### Step 2: Fix Material "Black"

1. **Select your firewall object** in 3D viewport
2. Go to **Shading** workspace (top of Blender window)
3. In the material slots (right side), click **"Black"**
4. In the shader editor (bottom), you'll see:
   ```
   [Image Texture] → [Diffuse BSDF] → [Mix Shader] → [Material Output]
                     [Glossy BSDF] ↗
   ```

5. **Note the current color**:
   - Look at the Glossy BSDF "Color" field
   - Write down or screenshot the color value

6. **Delete old nodes**:
   - Select the following nodes (click, then Shift+click for multiple):
     - Diffuse BSDF
     - Glossy BSDF
     - Mix Shader
   - Press `X` to delete them
   - **Keep** the Image Texture node (we'll handle this in Step 7)

7. **Add Principled BSDF**:
   - Press `Shift+A` → Shader → Principled BSDF
   - Place it between the Image Texture and Material Output

8. **Set the Base Color**:
   - Click on the **"Base Color"** field in Principled BSDF
   - Set it to black (RGB: 0, 0, 0) or your noted color
   - You can use the color picker or type exact RGB values

9. **Connect nodes**:
   ```
   [Principled BSDF] → [Material Output]
        BSDF output  →  Surface input
   ```

10. **Adjust Roughness** (optional):
    - Set "Roughness" to match the old Glossy BSDF roughness
    - 0.0 = very shiny (like old Glossy)
    - 1.0 = completely matte (like old Diffuse)
    - Start with 0.316 (the value I see in your screenshot)

### Step 3: Fix Material "Orange"

Repeat Step 2, but:
- Select material slot **"Orange"**
- Set Base Color to orange (probably something like RGB: 0.8, 0.4, 0.1)
- Use the **exact color** from the original Glossy BSDF or Diffuse BSDF node

### Step 4: Fix Materials F1, F2, F3, F4

Repeat Step 2 for each flame material:
- These are probably different shades of yellow/orange for the flame
- Note each color carefully before deleting nodes
- F1 might be bright yellow: RGB(1.0, 0.9, 0.2)
- F2 might be orange: RGB(1.0, 0.5, 0.0)
- F3, F4 are other flame colors

---

## What About the Image Texture?

Looking at your screenshot, I see an **"Image Texture"** node. This node has two options:

### Option A: If the texture contains your colors
If the Image Texture is what provides the colors (like a UV-mapped texture image):

```
[Image Texture] → [Principled BSDF] → [Material Output]
  Color output  →  Base Color input
  Alpha output  →  Alpha input
```

**Important**: If using textures, you might NOT need to set Base Color manually - the texture provides it!

### Option B: If you just want solid colors (recommended for now)

1. **Disconnect** the Image Texture node
2. **Don't delete it** - just leave it unconnected
3. Set Base Color manually in Principled BSDF
4. Export and test
5. If it works, you can delete the Image Texture later

---

## Complete Node Setup Examples

### For Solid Color Materials (Black, Orange, Flame colors):

```
┌─────────────────────┐      ┌──────────────────┐
│ Principled BSDF     │      │ Material Output  │
│                     │      │                  │
│ Base Color: [color]│─────→│ Surface          │
│ Roughness: 0.316   │ BSDF │                  │
└─────────────────────┘      └──────────────────┘
```

### If You Want to Use the Image Texture:

```
┌────────────────┐     ┌─────────────────────┐      ┌──────────────────┐
│ Image Texture  │     │ Principled BSDF     │      │ Material Output  │
│                │     │                     │      │                  │
│ Color ─────────┼────→│ Base Color         │      │                  │
│                │     │                     │─────→│ Surface          │
│ Alpha ─────────┼────→│ Alpha              │ BSDF │                  │
└────────────────┘     └─────────────────────┘      └──────────────────┘
```

---

## How to Find the Current Colors

If you're not sure what color values to use:

### Method 1: Check Viewport Color
1. Select material in material properties
2. Look at the 3D viewport - what color is it showing?
3. Use color picker to match that color

### Method 2: Check Node Values
1. Click on Glossy BSDF or Diffuse BSDF node
2. Look at the "Color" input field
3. Click the color to see RGB values
4. Write down these values

### Method 3: Sample from Viewport
1. With Principled BSDF selected
2. Click the Base Color picker
3. Use the eyedropper tool (looks like pipette icon)
4. Click on the model in 3D viewport to sample color

---

## Example Color Values (You'll need to verify these):

Based on typical firewall icons:

- **Black**: RGB(0.02, 0.02, 0.02) - Not pure black, slightly gray
- **Orange** (bricks): RGB(0.8, 0.4, 0.1) - Burnt orange
- **F1** (bright flame): RGB(1.0, 0.9, 0.3) - Bright yellow
- **F2** (mid flame): RGB(1.0, 0.6, 0.1) - Orange-yellow
- **F3** (dark flame): RGB(0.9, 0.4, 0.0) - Dark orange
- **F4**: RGB(0.7, 0.3, 0.0) - Reddish orange

**Don't use these blindly!** Sample your actual colors from the current setup.

---

## Verification Checklist

After fixing all 6 materials:

- [ ] Each material has a Principled BSDF node
- [ ] Each Principled BSDF is connected to Material Output
- [ ] Base Color is set for each material
- [ ] Model still looks correct in Blender viewport (Shift+Z for rendered view)
- [ ] All 6 materials are in the material slots
- [ ] No red error nodes in shader editor

---

## Export Settings Reminder

After fixing materials:

1. **Select All** objects (A)
2. **Apply All Transforms**: Ctrl+A → All Transforms
3. **File → Export → glTF 2.0 (.glb)**
4. **Transform** tab:
   - ☐ +Y Up (MUST BE UNCHECKED!)
5. **Geometry** tab:
   - ☑ Apply Modifiers
   - ☑ UVs
   - ☑ Normals
   - ☑ Tangents
6. **Material** tab:
   - ☑ Materials
7. **Export glTF Binary (.glb)**

---

## Testing

After export:

```bash
./validate_models.py public/models/firewall/paloalto/firewall_base.glb
```

You should see:
```
✓ [0] Black
✓ [1] Orange
✓ [2] F1
✓ [3] F2
✓ [4] F3
✓ [5] F4
```

If you still see ✗, the material doesn't have Base Color set properly.

---

## Common Issues

### Issue: Model is all white in viewport
**Cause**: Base Color not set in Principled BSDF
**Fix**: Click Base Color field and choose your color

### Issue: Some parts are correct color, others are white
**Cause**: You fixed some materials but not all 6
**Fix**: Go through each material slot and verify Principled BSDF setup

### Issue: Colors look different than before
**Cause**: Base Color doesn't match original color values
**Fix**: Sample colors from original setup using eyedropper

### Issue: Model is black in web app but colored in Blender
**Cause**: Principled BSDF not connected to Material Output
**Fix**: Connect BSDF output → Surface input

---

## Quick Reference: Node Connection

Always use this pattern:

```
Principled BSDF.BSDF → Material Output.Surface
```

Set these Principled BSDF inputs:
- **Base Color**: The actual color you want (RGB)
- **Roughness**: 0.0-1.0 (0=shiny, 1=matte)
- **Metallic**: Usually 0.0 for non-metal objects
- **Alpha**: 1.0 for solid, <1.0 for transparency

---

## Why This Works

- **Blender viewport**: Shows colors from any shader node (Diffuse, Glossy, etc.)
- **glTF exporter**: Only understands Principled BSDF
- **Three-d renderer**: Reads glTF materials which need Principled BSDF data

The materials are still assigned to the same mesh faces - we're just changing HOW the color is defined in a way that the exporter understands.

---

## Pro Tip: Batch Processing

If you have many models to fix:

1. Fix one material completely
2. Make sure it works (export + validate)
3. Use the same process for all other materials
4. You can copy/paste Principled BSDF nodes between materials (Ctrl+C, Ctrl+V)
5. Just change the Base Color for each one

---

## Need Help?

If colors don't export correctly:
1. Check shader editor - is Principled BSDF connected?
2. Check Base Color - is it set (not pure black by accident)?
3. Check validation tool - what does it say?
4. Compare Blender viewport (Material Preview mode) with exported result

Remember: The mesh face assignments haven't changed - we're just converting the material definitions to a format that glTF understands!
