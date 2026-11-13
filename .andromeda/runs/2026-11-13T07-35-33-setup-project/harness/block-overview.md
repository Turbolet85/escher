escher is a fork of Blitz, a Rust library workspace for an HTML/CSS engine (high-level APIs for rendering HTML, a core DOM abstraction whose primary entry point is `BaseDocument`, a `Document` trait so wrappers around `BaseDocument` can all be driven by blitz-shell), being turned into an agent-first UI framework: a headless driver (CLI and MCP), stable ids for every element, a compact semantic snapshot of the screen, proven on an internal 7GUIs stand. Development is agent-driven: the framework's own driver is meant to become the harness.

**Stack:** Rust 2024 Cargo workspace 0.3.0-beta.4 (rust-version 1.91.0, no `rust-toolchain.toml`) · Stylo 0.22 · Taffy 0.15 · Parley 0.12 · anyrender 0.14 / Vello 0.11 · winit 0.31.0-beta.3 · AccessKit 0.25 · Dioxus 0.7.3 · Boa 0.22 · tokio 1.42 · reqwest 0.13 — a native-API engine library with no server and no database (the browser app keeps a rusqlite 0.32 history); versions as `Cargo.toml` pins them.

**Key directories:**
- `packages/` — the engine and integration crates and escher's own crates (telemetry bootstrap, driver)
- `tests/blitz-tests/` — integration tests, one file and one test target per behaviour; `checks/` holds the two committed kept checks
- `examples/` — example crates; `examples/seven_guis/` is the 7GUIs stand
- `apps/` — the reference browser and the markdown viewer
- `scripts/` — the agent-run test contract, the cold-agent run pipe, the code-graph tooling
