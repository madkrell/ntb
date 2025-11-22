# Network Topology Builder - Quick Session Guide

## 🎯 Start New Conversation With This Prompt

```
I'm continuing work on the Network Topology Builder project at:
/Users/mattearp/Documents/CodeProjects/ntb/

Please read CLAUDE.md for complete architecture context.

Current Status: Scene Objects Outliner ✅ (2025-01-22)
All core features complete PLUS native Blender coordinate system AND Scene Objects panel.
Features: Blender-style outliner, visibility toggle, node selection from sidebar.

Ready for: [Traffic Dashboard / WebSocket Streaming / UX Polish / your goal here]
```

## 📋 Essential Context

**Read These Files (in order):**
1. **CLAUDE.md** - Complete architecture, all patterns, all lessons (lines 1-221)
2. **This file** - Quick reference only

**Current State:**
- Native Blender Z-up coordinate system ✅
- Native Blender scale (no 0.3x multiplier) ✅
- Scene Objects panel (Blender-style outliner) ✅
- Git tag: `v0.1.0-scene-objects`
- All core features working (see CLAUDE.md lines 8-20)
- Direct Blender → Viewport workflow (no transformations, what you see is what you get)

## 🚀 Quick Start Examples

### Traffic Dashboard (Recommended)
```
Continuing NTB at /Users/mattearp/Documents/CodeProjects/ntb/
Read CLAUDE.md for context. Native Blender Z-up complete.

Goal: Implement traffic dashboard with metrics panel, top-N connections,
historical charts, CSV export.

See CLAUDE.md lines 21-22 for feature details.
```

### WebSocket Streaming
```
Continuing NTB. Native Blender Z-up complete.
Read CLAUDE.md for architecture.

Goal: Add WebSocket streaming for real-time traffic updates using Leptos
WebSocket server functions.
```

### UX Polish
```
Continuing NTB. Native Blender Z-up complete.

Goal: Add [multi-select / keyboard shortcuts / feature name]
```

### Custom Feature
```
Continuing NTB at /Users/mattearp/Documents/CodeProjects/ntb/
Read CLAUDE.md for complete context. Native Blender Z-up complete.

Goal: [Describe your specific feature or fix]
```

## 💡 Development Reminders

### Quick Commands
```bash
# Development (run both in separate terminals)
./tailwindcss -i style/input.css -o style/output.css --watch
cargo leptos watch

# Test: http://127.0.0.1:3000

# Production
./tailwindcss -i style/input.css -o style/output.css --minify
cargo leptos build --release
```

### Key Facts
- **Database**: `ntv.db` (NOT ntb.db - preserved during rename)
- **Architecture**: Leptos 0.8, SQLite, three-d WebGL2
- **Coordinates**: Native Blender Z-up (X→X, Y→Y, Z→Z, no swapping)
- **Scale**: Native Blender scale (no multipliers, direct 1:1)
- **Blender export**: UNCHECK "+Y Up" to preserve Z-up, Apply Transforms before export
- **Model validation**: `./validate_models.py` for size/material checks
- **Code structure**: See CLAUDE.md lines 30-69
- **Critical patterns**: See CLAUDE.md lines 28-221
- **Common issues**: See CLAUDE.md lines 333-348
- **Scene Objects**: Outliner in left sidebar, visibility toggle with eye icon

## 🔗 References

- **Complete architecture**: CLAUDE.md (read first!)
- **GitHub**: https://github.com/madkrell/ntb.git
- **Leptos docs**: Use Context7 MCP with `/websites/book_leptos_dev`
- **three-d docs**: https://github.com/asny/three-d

## ✅ That's It!

CLAUDE.md contains everything. This file is just for quick session starts.
