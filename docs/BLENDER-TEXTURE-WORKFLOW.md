# Blender Textured Material Workflow for NTB

**Goal:** Create glTF/GLB models with image textures that display perfectly in the web app without color space conversion issues.

---

## Why Use Textures?

**Benefits:**
- ✅ **Perfect color matching** - Textures already in sRGB color space (no conversion needed)
- ✅ **Fine detail** - Add logos, labels, port numbers, text
- ✅ **Visual variety** - Different materials on different parts
- ✅ **Professional appearance** - Industry-standard PBR workflow
- ✅ **No color space bugs** - Avoids three-d linear RGB interpretation issues

---

## Method 1: Simple Texture from Color (Recommended for Start)

This method converts your existing Principled BSDF base colors into image textures.

### Step 1: UV Unwrap Your Model

1. **Select your model** in Object Mode
2. **Tab** into Edit Mode
3. **Select all** faces (A key)
4. **U** → Select unwrap method:
   - **Smart UV Project** - Good for complex objects (recommended)
   - **Unwrap** - Good if you have seams marked
   - **Cube Projection** - Good for boxy shapes

**Settings for Smart UV Project:**
- Angle Limit: 66°
- Island Margin: 0.02
- Check "Scale to Bounds" for maximum texture space usage

### Step 2: Create Color Texture for Each Material

For each material that needs a texture:

1. **Open Shader Editor** (Shading workspace)
2. **Select the material** you want to texture
3. **Shift+A** → Search: "Image Texture"
4. **Click "New"** button in the Image Texture node
5. **Settings:**
   - Name: `router_body_color` (descriptive name)
   - Width: 512 or 1024 (higher = more detail, larger file)
   - Height: Same as width
   - **Color**: Set to your current Base Color
   - **Generate Type**: Blank
   - Check **Alpha** if you need transparency
   - **32-bit Float**: Unchecked (8-bit is fine)

6. **Connect Image Texture → Base Color** of Principled BSDF
7. **Keep Metallic and Roughness** as direct values (or add their own textures)

### Step 3: Bake the Color to Texture (Optional but Recommended)

This "locks in" the color into the texture:

1. **Switch to Cycles** render engine (top right)
2. **UV Editing** workspace
3. **Select your object**
4. **Edit Mode** → Select all (A)
5. **In Shader Editor**: Select the **Image Texture node** you want to bake to
6. **Rendering** → **Render Properties** (camera icon) → **Bake**
7. **Bake Settings:**
   - Bake Type: **Diffuse**
   - Influence: Uncheck everything except **Color**
   - Selected to Active: **Unchecked**
   - Margin: 2px
8. **Click "Bake"** button
9. **Image Editor**: **Image** → **Save As...** → Save the texture

Repeat for each material.

### Step 4: Export as glTF

1. **File** → **Export** → **glTF 2.0** (.glb)
2. **Settings:**
   - Format: **glTF Binary** (.glb) ← Single file
   - Include:
     - ☑ Selected Objects (if only exporting one object)
     - ☐ Cameras
     - ☐ Lights
   - Transform:
     - ☑ +Y Up (coordinate system)
   - Geometry:
     - ☑ Apply Modifiers
     - ☑ UVs
     - ☑ Normals
     - ☑ Tangents (needed for normal maps)
     - ☐ Vertex Colors (unless you use them)
   - Materials:
     - ☑ Materials
     - ☑ Images
   - Animation: ☐ (not needed for static models)
   - Compression: ☐ (keep unchecked for debugging)
3. **Export glTF 2.0**

4. **Place in:** `public/models/{type}/{vendor}/{model-name}.glb`
   - Example: `public/models/router/cisco/cisco_router.glb`

---

## Method 2: Detailed Texture with UV Painting

For more control and adding details like logos, labels, and port numbers.

### Step 1: UV Unwrap (Same as Method 1)

### Step 2: Create Base Texture Image

1. **Shading** workspace → **Shader Editor**
2. **Shift+A** → **Image Texture**
3. **New** → Settings:
   - Name: `router_detail_texture`
   - Width/Height: 1024x1024 or 2048x2048
   - Color: Base color for material
   - Alpha: Check if needed
4. **Connect** to Principled BSDF Base Color

### Step 3: Texture Paint Mode

1. **Switch to Texture Paint** workspace
2. **3D Viewport** → **Texture Paint** mode (top dropdown)
3. **In Shader Editor**: Select the **Image Texture** node to paint on
4. **Brush settings** (left sidebar):
   - Radius: Adjust brush size
   - Strength: Paint opacity
   - Blend: Mix (normal painting)
5. **Paint directly on the 3D model** - colors appear where you paint!

### Step 4: Add Details

**Useful techniques:**
- **Stencil images**: Add logos, labels
  - Texture Paint → Tool Settings → Use Stencil Image
  - Load logo/label PNG
  - Position with RMB drag, scale with Shift+RMB
  - Paint over it to apply

- **Text overlays**: Add port numbers, model labels
  - Use image editing software (Photoshop, GIMP, Krita)
  - Export UV layout: UV Editor → UV → Export UV Layout
  - Paint text in external software
  - Re-import texture: Image Editor → Image → Open

### Step 5: Save Texture & Export

1. **Image Editor** → **Image** → **Save As...** → Save your painted texture
2. **Export as glTF** (same settings as Method 1)

---

## Priority 2: Adding Emissive LED Materials

Make status LEDs glow in the web app!

### Step 1: Select LED Faces

1. **Edit Mode** → **Face Select** mode
2. **Select the LED faces** (front of LED indicators)
3. **P** → **Separate by Selection** (makes separate object)
4. **Or**: Keep as same object but assign different material

### Step 2: Create Emissive Material

1. **Shader Editor** → Create new material
2. **Name**: `LED_Green_Emissive`
3. **Principled BSDF settings:**
   - **Base Color**: Bright green (e.g., RGB 0, 255, 0)
   - **Emission Color**: Same bright green
   - **Emission Strength**: 2.0 to 5.0 (higher = brighter glow)
   - **Metallic**: 0.0
   - **Roughness**: 0.3-0.5

### Step 3: Preview in Blender

1. **Switch viewport shading** to **Rendered** mode (far right icon)
2. **Or**: Material Preview mode (second from right)
3. **LEDs should glow!**

### Step 4: Export

- Export as glTF with same settings
- Web app will render glowing LEDs automatically!

**Multiple LED colors:**
- Create separate materials for each color:
  - `LED_Green_Emissive` (power on)
  - `LED_Red_Emissive` (error)
  - `LED_Yellow_Emissive` (warning)
  - `LED_Blue_Emissive` (status)

---

## Advanced: Normal Maps for Detail

Add surface detail without adding geometry.

### Step 1: High-Poly Sculpt (Optional)

1. **Sculpting** workspace
2. **Add detail** with sculpting brushes
3. **Or**: Use high-poly reference model

### Step 2: Bake Normal Map

1. **Select high-poly** object first
2. **Shift+Select low-poly** object
3. **Rendering** → **Bake**
   - Bake Type: **Normal**
   - Selected to Active: **☑ Checked**
   - Extrusion: 0.1
   - Max Ray Distance: 0.1
4. **Bake** → saves normal map
5. **Connect normal map** to Normal input of Principled BSDF

### Step 3: Export

- glTF exporter automatically includes normal maps
- Web app renders with surface detail

---

## Troubleshooting

### Issue: Texture appears black or wrong in web app

**Cause:** Texture file not embedded in glTF
**Fix:**
- Re-export with **Images** checked
- Save textures before exporting
- Use glTF Binary (.glb) format (embeds textures)

### Issue: UVs look stretched or distorted

**Cause:** Bad UV unwrap
**Fix:**
- Re-unwrap with Smart UV Project
- Manually mark seams and unwrap
- Adjust UV islands in UV Editor

### Issue: Colors still don't match Blender exactly

**Check:**
1. View Transform in Blender: Should be **Filmic** or **Standard**
2. Color Management: Check if Display Device is sRGB
3. Texture color space: Should be **sRGB** (not Linear)
4. In web app: Check browser console for material loading logs

### Issue: Emissive not glowing

**Check:**
1. Emission Strength: Should be > 1.0 (try 2.0-5.0)
2. Material export: Check glTF includes emissive
3. Web app console: Should show "Using FULL glTF material with textures"

---

## Recommended Texture Sizes

| Model Complexity | Texture Size | File Size (approx) | Detail Level |
|-----------------|--------------|-------------------|--------------|
| Simple device   | 512x512      | 50-200 KB         | Basic        |
| Standard device | 1024x1024    | 200-800 KB        | Good         |
| Detailed device | 2048x2048    | 800 KB - 3 MB     | High         |
| Hero asset      | 4096x4096    | 3-12 MB           | Very High    |

**Recommendation for network devices:** 1024x1024 is the sweet spot.

---

## Quick Checklist

**Before Export:**
- [ ] Model is UV unwrapped
- [ ] All materials have textures connected
- [ ] Textures are saved (Image → Save As)
- [ ] Emissive LEDs have Emission Strength > 1.0
- [ ] Preview in Rendered viewport mode looks correct

**Export Settings:**
- [ ] Format: glTF Binary (.glb)
- [ ] Include: Materials ☑, Images ☑
- [ ] Transform: +Y Up ☑
- [ ] Geometry: UVs ☑, Normals ☑, Tangents ☑

**After Export:**
- [ ] Place .glb in correct folder: `public/models/{type}/{vendor}/`
- [ ] Refresh web app with hard reload (Cmd+Shift+R)
- [ ] Check browser console for material loading logs
- [ ] Verify textures display correctly
- [ ] Verify LEDs glow (if using emissive)

---

## Example Workflow Summary

**For Cisco Router with Textured Body and Glowing LEDs:**

1. **UV unwrap** router body, front panel, LEDs separately
2. **Body texture**: 1024x1024, paint gray-blue color
3. **Front panel**: 512x512, paint black with white labels
4. **LEDs**: Create emissive material (Emission: 3.0, green color)
5. **Export** as `cisco_router.glb` to `public/models/router/cisco/`
6. **Refresh** web app → Perfect color match + glowing LEDs!

---

## Next Steps

1. **Try Method 1** with your existing Cisco router
2. **Add emissive LEDs** (Priority 2)
3. **Test in web app** with hard refresh
4. **Iterate** - adjust colors, emission strength as needed
5. **Document** your vendor-specific workflow for reuse

**Need help?** Check console logs in web app for material loading details!
