# Network Topology Visualizer

A web-based network topology visualization tool built entirely in **Rust** using **Leptos 0.8.11 with islands architecture**.

## Features

- **3D Visualization**: Interactive 3D rendering of network topologies using three-d
- **Topology Editor**: Canvas-based drag-and-drop editor for creating network diagrams
- **Real-time Monitoring**: Server-Sent Events (SSE) for live traffic data
- **Data Persistence**: SQLite database with full CRUD operations
- **Export**: PNG, SVG, and JSON export capabilities
- **Pure Rust**: No JavaScript dependencies - all interactive functionality in Rust/WASM

## Tech Stack

- **Frontend**: Leptos 0.8.11 (islands mode) + three-d (3D rendering)
- **Backend**: Leptos SSR with server functions (Axum integration)
- **Database**: SQLite with sqlx
- **Build Tool**: cargo-leptos
- **3D Models**: Blender → glTF/GLB format

## Architecture

This project uses **Leptos Islands Architecture** for optimal performance:
- Only interactive components compile to WASM
- Each island is a separate, lazy-loaded bundle
- Static content remains as server-rendered HTML
- Dramatically reduced bundle sizes (<640KB total)

## Prerequisites

- Rust 1.75+ (with wasm32-unknown-unknown target)
- cargo-leptos
- SQLite 3

## Installation

```bash
# Install Rust target for WASM
rustup target add wasm32-unknown-unknown

# Install cargo-leptos
cargo install cargo-leptos

# Clone repository
git clone <repository-url>
cd ntv

# Create database
touch topologies.db

# Run migrations
sqlx migrate run

# Start development server
cargo leptos watch
```

The application will be available at `http://localhost:3000`

## Development

```bash
# Development mode (hot reload)
cargo leptos watch

# Build for production
cargo leptos build --release

# Verify code splitting worked
ls -lh target/site/pkg/*.wasm
```

## Project Structure

```
ntv/
├── src/
│   ├── app.rs              # Main app shell (SSR)
│   ├── lib.rs              # Hydration entry point
│   ├── main.rs             # Server entry point
│   ├── islands/            # Interactive WASM islands
│   │   ├── topology_viewport.rs
│   │   ├── topology_editor.rs
│   │   └── traffic_monitor.rs
│   ├── components/         # Server-rendered components
│   ├── pages/              # Route pages
│   ├── models/             # Data structures
│   ├── server_fns/         # Leptos server functions
│   └── rendering/          # 3D rendering helpers
├── public/
│   └── models/             # glTF/GLB 3D models
├── migrations/             # SQLite migrations
├── Cargo.toml              # All configuration here (NO Leptos.toml)
├── CLAUDE.md               # Verified working configurations
└── README.md
```

## Configuration

Key configuration files:
- `Cargo.toml` - Rust dependencies, features, and project configuration
- `CLAUDE.md` - **✅ Verified configurations and critical corrections**

**Important:** The original implementation plan contains some outdated patterns. Always refer to `CLAUDE.md` for verified, working configurations based on Leptos 0.8 documentation.

## Documentation

- [Implementation Plan](network-topology-visualizer-plan.md) - Complete technical specification ⚠️ See CLAUDE.md for corrections
- [CLAUDE.md](CLAUDE.md) - **✅ Verified configurations and implementation notes**

## License

TBD

## Status

🚧 **In Development** - Phase 1: Foundation
