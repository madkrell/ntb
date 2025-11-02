# Network Topology Visualizer - Claude Development Notes

## Project Status
**Current Phase:** Phase 1 - Foundation & Verification
**Last Updated:** 2025-11-02

## ✅ VERIFIED Configuration (from Leptos 0.7/0.8 docs)

### Important: NO Leptos.toml Required!
Modern Leptos projects use `cargo-leptos` and configure everything in `Cargo.toml`.
The original plan referenced Leptos.toml which is NOT standard.

### Leptos Islands Architecture (Cargo.toml)
```toml
[dependencies]
leptos = { version = "0.8", features = ["ssr", "islands"] }
leptos_meta = { version = "0.8", features = ["ssr"] }
leptos_router = { version = "0.8", features = ["ssr"] }
leptos_axum = { version = "0.8", optional = true }

[features]
hydrate = ["leptos/hydrate", "leptos_meta/hydrate", "leptos_router/hydrate"]
ssr = [
    "leptos/ssr",
    "leptos_meta/ssr",
    "leptos_router/ssr",
    "dep:axum",
    "dep:leptos_axum"
]

[lib]
crate-type = ["cdylib", "rlib"]  # cdylib required for WASM

# Optional: WASM size optimization
[profile.wasm-release]
inherits = "release"
opt-level = 'z'
lto = true
codegen-units = 1
```

### Hydration Setup (VERIFIED)
```rust
// In shell function (app.rs or main.rs)
<HydrationScripts options=options islands=true/>

// In lib.rs hydrate entry point
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();  // NOT stop_hydrating()
}
```

## Islands vs Components
- `#[island]` - Compiles to separate WASM, loads on-demand
- `#[component]` - Server-rendered HTML only, no WASM

## Server Functions & Streaming (VERIFIED)
All backend logic uses `#[server]` macro. Leptos has NATIVE streaming support!

```rust
// Regular server function
#[server(FunctionName)]
pub async fn function_name(...) -> Result<T, ServerFnError> {
    let pool = use_context::<SqlitePool>()?;
    // Database operations...
}

// ✅ NATIVE SSE/STREAMING (no Axum SSE needed!)
#[server(protocol = Websocket<JsonEncoding, JsonEncoding>)]
async fn stream_data(
    input: BoxedStream<Message, ServerFnError>
) -> Result<BoxedStream<Message, ServerFnError>, ServerFnError> {
    // Leptos handles streaming natively
    Ok(input.into())
}

// Client side: create signal from stream
let signal = Signal::from_stream(my_stream);
```

## Project Initialization (VERIFIED)
```bash
# Install cargo-leptos
cargo install --locked cargo-leptos

# Create project from template
cargo leptos new --git leptos-rs/start-axum

# OR manual setup with correct structure
cargo new --lib my-project
# Configure Cargo.toml as shown above
```

## Verified Working Dependencies
- leptos = "0.8" (with "ssr" and "islands" features)
- leptos_axum = "0.8"
- cargo-leptos = latest
- sqlx = "0.7"
- three-d = "0.17" (WASM status: TBD - to verify in Phase 2)

## Build Commands (VERIFIED)
```bash
# Development with hot reload
cargo leptos watch

# Production build
cargo leptos build --release

# Verify code splitting (look for multiple .wasm files)
ls -lh target/site/pkg/*.wasm
```

## Target Bundle Sizes
- TopologyViewport: ~300KB (with three-d)
- TopologyEditor: ~150KB
- TrafficMonitor: ~80KB
- Total: <640KB

## Known Issues & Solutions
(To be populated as we encounter them)

## Database
- SQLite with sqlx
- Migrations in /migrations/
- Pool provided via Axum Extension and Leptos context

## Key Corrections to Original Plan

### ❌ INCORRECT in plan:
1. **Leptos.toml file** - Does NOT exist in modern Leptos
2. **leptos::leptos_dom::HydrationCtx::stop_hydrating()** - Wrong! Use `hydrate_islands()`
3. **Axum SSE endpoints** - NOT needed! Leptos has native streaming via server functions
4. **Manual EventSource setup** - NOT needed! Use `Signal::from_stream()`

### ✅ CORRECT approach:
1. Use `cargo leptos new --git leptos-rs/start-axum` for project template
2. Configure in `Cargo.toml` with "islands" feature
3. Use `leptos::mount::hydrate_islands()` in lib.rs
4. Use `#[server(protocol = Websocket<>)]` for streaming
5. Use `Signal::from_stream()` for reactive SSE/streaming data
6. Be sure to use context7 mcp server if needing to check correct leptos configuration

## IDE Configuration
All editors should enable all Cargo features for rust-analyzer:
```json
// VSCode settings.json
{
  "rust-analyzer.cargo.features": "all"
}
```

## Next Steps
1. ✅ Verified Leptos 0.8 islands architecture
2. Create project from template or manual setup
3. Create minimal test island
4. Confirm code splitting works
5. Set up database and migrations
