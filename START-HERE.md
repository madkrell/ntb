# Network Topology Visualizer - Implementation Start Guide

## 📋 Context for New Conversation

This is a **pure Rust/WASM** web application for visualizing network topologies using **Leptos 0.8 with Islands Architecture**. All code must be Rust - no JavaScript dependencies.

### Current Status
- ✅ Project initialized with git repository
- ✅ All documentation written and corrected
- ✅ Leptos 0.8 configuration verified via Context7
- 🔄 **READY TO START: Phase 1 - Foundation**

### Repository Structure
```
/Users/mattearp/Documents/CodeProjects/ntv/
├── CLAUDE.md                              # ✅ VERIFIED configurations (PRIMARY SOURCE)
├── network-topology-visualizer-plan.md    # Full spec (with corrections notice)
├── README.md                              # Project overview
├── START-HERE.md                          # This file
└── .git/                                  # Git initialized
```

## 🎯 What to Build

A web app for network engineers to:
1. **View** network topologies in 3D (using three-d + glTF models from Blender)
2. **Edit** topologies with canvas-based drag-and-drop editor
3. **Monitor** real-time traffic data via Leptos native streaming
4. **Export** topologies as PNG/JSON
5. **Persist** everything to SQLite database

### Tech Stack (VERIFIED)
- **Frontend:** Leptos 0.8 (islands mode) + three-d for 3D rendering
- **Backend:** Leptos SSR + server functions (no separate Axum routes)
- **Database:** SQLite with sqlx
- **Build:** cargo-leptos (NOT Trunk, NO Leptos.toml file)

## ⚠️ CRITICAL: Read CLAUDE.md First!

**CLAUDE.md contains all verified, working configurations from Leptos 0.8 docs.**

The original plan document has 4 major errors that are corrected in CLAUDE.md:
1. ❌ References non-existent Leptos.toml → ✅ Use Cargo.toml only
2. ❌ Wrong hydration function → ✅ Use `leptos::mount::hydrate_islands()`
3. ❌ Manual Axum SSE setup → ✅ Use Leptos native streaming
4. ❌ Some outdated syntax → ✅ See CLAUDE.md for correct patterns

## 📚 Key Files to Reference

### 1. CLAUDE.md - PRIMARY SOURCE (159 lines)
**Read this file completely before coding!**
Contains:
- ✅ Verified Cargo.toml configuration for islands
- ✅ Correct hydration setup
- ✅ Native Leptos streaming examples
- ✅ List of all corrections to original plan
- ✅ cargo-leptos commands
- ✅ IDE configuration (rust-analyzer)

### 2. network-topology-visualizer-plan.md (2,287 lines)
**Use for strategy, verify code in CLAUDE.md**
Contains:
- Phase 1-5 breakdown with timelines
- Detailed architecture decisions
- Database schema
- Code examples (BUT check CLAUDE.md for correct syntax)
- File structure details

## 🚀 Phase 1 Tasks (Start Here)

### Step 1: Install Prerequisites
```bash
# Install cargo-leptos
cargo install --locked cargo-leptos

# Add WASM target
rustup target add wasm32-unknown-unknown

# Verify installed
cargo leptos --version
```

### Step 2: Initialize Project

**OPTION A: Use Template (Recommended)**
```bash
cd /Users/mattearp/Documents/CodeProjects/ntv
cargo leptos new --git leptos-rs/start-axum
# This will scaffold inside ntv/ directory
```

**OPTION B: Manual Setup**
Create project structure as shown in plan.md Phase 1.1, using Cargo.toml configuration from CLAUDE.md.

### Step 3: Configure for Islands

**In Cargo.toml:**
```toml
[dependencies]
leptos = { version = "0.8", features = ["ssr", "islands"] }
leptos_meta = { version = "0.8", features = ["ssr"] }
leptos_router = { version = "0.8", features = ["ssr"] }
leptos_axum = { version = "0.8", optional = true }

# ... see CLAUDE.md for complete configuration
```

### Step 4: Create Minimal Test Island

**Goal:** Verify islands code splitting works BEFORE building complex features.

1. Create a simple Counter island in `src/islands/test_counter.rs`
2. Use `#[island]` macro (see CLAUDE.md for exact syntax)
3. Build: `cargo leptos build --release`
4. **VERIFY:** `ls -lh target/site/pkg/*.wasm` shows MULTIPLE .wasm files
5. If only ONE .wasm file → islands not working, check CLAUDE.md config

### Step 5: Set Up Database

1. Create SQLite schema from plan.md section 1.4 (migrations/001_initial.sql)
2. Create data models (src/models/topology.rs)
3. Test database connection via server function
4. Verify `use_context::<SqlitePool>()` works

### Step 6: Create SSR Shell

1. Build static components (Navbar, Footer) using `#[component]` NOT `#[island]`
2. Set up routing with leptos_router
3. Create basic pages (Home, Viewer, Editor, List)
4. Verify SSR renders without loading any WASM

## 📋 Phase 1 Success Criteria

✅ **Before moving to Phase 2:**
1. `cargo leptos build --release` succeeds without errors
2. Multiple .wasm files exist in target/site/pkg/ (code splitting working)
3. Test island hydrates and functions in browser
4. Can load/save topology to SQLite via server function
5. Static pages render without WASM

## 🔄 Subsequent Phases

### Phase 2: 3D Viewer (or 2D Canvas fallback)
- Start with 2D canvas viewer to validate architecture
- Upgrade to three-d for 3D rendering
- Load glTF models asynchronously

### Phase 3: Editor Island
- Canvas-based device placement
- Connection drawing
- Properties panel
- Save/load functionality

### Phase 4: Traffic Monitoring
- **Use Leptos native streaming** (see CLAUDE.md)
- `#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]`
- Client: `Signal::from_stream(stream)`
- NO manual Axum SSE endpoints needed!

### Phase 5: Export & Polish
- PNG export from canvas
- JSON data export
- UI polish and optimization
- Verify bundle size targets met (<640KB total)

## 🛠 Development Commands

```bash
# Development with hot reload
cargo leptos watch

# Build for production
cargo leptos build --release

# Check code splitting worked
ls -lh target/site/pkg/*.wasm

# Run server
./target/site/server
```

## 🧭 When Things Go Wrong

### Islands Not Code Splitting
- Check Cargo.toml has `leptos = { features = ["ssr", "islands"] }`
- Verify using `cargo leptos build` not `cargo build`
- Ensure `#[island]` macro is used correctly
- Check CLAUDE.md hydration setup

### Hydration Errors
- Use `leptos::mount::hydrate_islands()` NOT `stop_hydrating()`
- Ensure `<HydrationScripts options=options islands=true/>`
- Check browser console for errors

### three-d Not Working
- Fallback to 2D canvas approach (web-sys CanvasRenderingContext2d)
- Still pure Rust, just simpler rendering
- Plan includes 2D alternative (Phase 2A in plan.md)

## 📞 Context7 Integration

If you need to verify Leptos APIs during implementation:
```
Use mcp__context7__resolve-library-id with "leptos"
Then mcp__context7__get-library-docs with "/websites/book_leptos_dev"
```

## 🎬 Starting a New Conversation

**Prompt to use:**
```
I'm implementing a network topology visualizer in Rust using Leptos 0.8 with islands architecture.

The project is at: /Users/mattearp/Documents/CodeProjects/ntv/

Please read these files in order:
1. START-HERE.md (this gives context)
2. CLAUDE.md (verified configurations)

I'm ready to start Phase 1: Foundation. Let's begin by [choose one]:
- Installing prerequisites and using cargo leptos new template
- Creating the minimal test island to verify code splitting
- Setting up the database schema
- [Or specify where you want to start]

If you need to verify any Leptos patterns, use Context7 mcp server to check /websites/book_leptos_dev documentation.
```

## 📊 Project Files Summary

| File | Lines | Purpose | Status |
|------|-------|---------|--------|
| CLAUDE.md | 159 | ✅ Verified configs | Primary reference |
| network-topology-visualizer-plan.md | 2,287 | Full specification | Has corrections notice |
| README.md | 117 | Project overview | Points to CLAUDE.md |
| START-HERE.md | This file | Implementation guide | You are here |

## 🎯 Success Metrics

**Phase 1 (Foundation):** 2-3 days
- Working islands architecture with code splitting
- Database connectivity
- Basic SSR shell

**Phase 2 (Viewer):** 4-5 days
- 2D or 3D topology visualization
- Load topology data from database

**Phase 3 (Editor):** 4 days
- Canvas-based editor
- Device placement and connections
- Save functionality

**Phase 4 (Monitoring):** 2 days
- Native Leptos streaming
- Real-time data display

**Phase 5 (Polish):** 3 days
- Export functionality
- UI improvements
- Bundle size optimization

**Total:** ~18 days to MVP

## ✅ Pre-Implementation Checklist

Before starting Phase 1, verify:
- [ ] Read CLAUDE.md completely
- [ ] Understand islands vs components difference
- [ ] Know that Leptos.toml does NOT exist
- [ ] Understand native Leptos streaming (no manual SSE)
- [ ] Have cargo-leptos installed
- [ ] Have wasm32-unknown-unknown target added
- [ ] Have Context7 mcp server available for docs lookup

**You are ready to begin! Start with Phase 1, Step 1.**
