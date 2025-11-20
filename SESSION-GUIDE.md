# Network Topology Builder - Quick Session Guide

## 🎯 Start New Conversation With This Prompt

```
I'm continuing work on the Network Topology Builder project at:
/Users/mattearp/Documents/CodeProjects/ntb/

Please read CLAUDE.md for complete architecture context.

Current Status: Phase 6.4.4 COMPLETE ✅ (2025-01-20)
All core features implemented: 3D visualization, traffic monitoring, particle animation,
simplified connection creation, auto-save, undo functionality.

Ready for: [Traffic Dashboard / WebSocket Streaming / UX Polish / your goal here]
```

## 📋 Essential Context

**Read These Files (in order):**
1. **CLAUDE.md** - Complete architecture, all patterns, all lessons (lines 1-221)
2. **This file** - Quick reference only

**Current State:**
- Phase 6.4.4 complete ✅
- Git tag: `v0.1.0-phase6.4.4-complete`
- All core features working (see CLAUDE.md lines 7-18)

## 🚀 Quick Start Examples

### Traffic Dashboard (Recommended)
```
Continuing NTB at /Users/mattearp/Documents/CodeProjects/ntb/
Read CLAUDE.md for context. Phase 6.4.4 complete.

Goal: Implement traffic dashboard with metrics panel, top-N connections,
historical charts, CSV export.

See CLAUDE.md lines 20-21 for feature details.
```

### WebSocket Streaming
```
Continuing NTB. Phase 6.4.4 complete.
Read CLAUDE.md for architecture.

Goal: Add WebSocket streaming for real-time traffic updates using Leptos
WebSocket server functions.
```

### UX Polish
```
Continuing NTB. Phase 6.4.4 complete.

Goal: Add [multi-select / keyboard shortcuts / feature name]
```

### Custom Feature
```
Continuing NTB at /Users/mattearp/Documents/CodeProjects/ntb/
Read CLAUDE.md for complete context. Phase 6.4.4 complete.

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
- **Code structure**: See CLAUDE.md lines 140-156
- **Critical patterns**: See CLAUDE.md lines 25-76
- **Common issues**: See CLAUDE.md lines 208-221

## 🔗 References

- **Complete architecture**: CLAUDE.md (read first!)
- **GitHub**: https://github.com/madkrell/ntb.git
- **Leptos docs**: Use Context7 MCP with `/websites/book_leptos_dev`
- **three-d docs**: https://github.com/asny/three-d

## ✅ That's It!

CLAUDE.md contains everything. This file is just for quick session starts.
