# Cascade compare — architecture · phase · 2026-10-25-capability-statement
E items 158 · held 44 · lost at classify 10 · lost at strong 3 · beside a placed fact - · cascade only 17 · body 30 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### cross-cutting-patterns#1.1 · lost at strong · .andromeda/architecture-detail/cross-cutting-patterns.md:8
- (.andromeda/architecture-detail/cross-cutting-patterns.md:8) The statement's snapshot, text, diff and actionable-key capabilities must be stated as holding under the `accessibility` feature (a default feature), not unconditionally.
  > `accessibility` also to `dioxus-native-dom/accessibility`, which enables dioxus-native-dom's own optional `accesskit` and gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules

### cross-cutting-patterns#2.1 · lost at classify · .andromeda/architecture-detail/cross-cutting-patterns.md:14
- (.andromeda/architecture-detail/cross-cutting-patterns.md:14) Any claim the statement makes about what escher's binaries log (the `escher-session` host and its `mcp` role beside the MCP client's stdio) must match the sink as built: stderr only, records outside the two prefix sets dropped at every level, the rest scrubbed.
  > writes to stderr only, stamps `service.name` / `service.version` on every line, bridges `log` records, drops every record whose target is outside the engine and escher prefix sets (`ENGINE_TARGET_PREFIXES`, `ESCHER_TARGET_PREFIXES`) at every level and whatever `RUST_LOG` names, scrubs the records that print — a closed span's fields by the rule that judges an event's — and chains the panic hook

### cross-cutting-patterns#2.1 · lost at classify · .andromeda/architecture-detail/cross-cutting-patterns.md:15
- (.andromeda/architecture-detail/cross-cutting-patterns.md:15) A no-content-in-logs claim must be scoped to escher's binaries and must not reach the upstream apps, whose installs write stdout unscrubbed — a limit the statement states rather than hides.
  > escher-driver emits its command span and, since 2026-10-10-mcp-surface, its `tool_call` span through `tracing` ungated and installs no subscriber (packages/escher-driver/src/execute.rs:96-111); the upstream `tracing_subscriber::fmt::init()` installs run only under the `tracing` feature and write to stdout unscrubbed

### design-philosophy#2.1 · lost at strong · .andromeda/architecture-detail/design-philosophy.md:24
- (.andromeda/architecture-detail/design-philosophy.md:24) A document built without providers runs on no-op defaults, so the statement must not claim network or navigation behaviour for a document that was handed none.
  > when one is not supplied, `BaseDocument::new` falls back to `DummyNetProvider`, `DummyNavigationProvider`, `DummyShellProvider` and `DummyHtmlParserProvider`

### established-decisions#5.1 · lost at classify · .andromeda/architecture-detail/established-decisions.md:32
- (.andromeda/architecture-detail/established-decisions.md:32) The `text_extent` limit (RTL and vertical text read as LTR lines) sits beside this decision: vertical writing modes are in no default feature set, and the statement must not claim them.
  > `writing-mode`, CSS vertical writing modes, is in no default list

### inherited-defaults#1.1 · lost at classify · .andromeda/architecture-detail/inherited-defaults.md:8
- (.andromeda/architecture-detail/inherited-defaults.md:8) The statement must not present escher-driver, escher-telemetry, blitz-test-harness or seven_guis as published crates. A consumer reaches them from the repository, because they set `publish = false`.
  > Publishability: the root package is virtual and not meant to be published (Cargo.toml:252-263); app and example crates, blitz-tests, blitz-test-harness, escher-telemetry, escher-driver, blitz-vibey-script (since 2026-10-13-upstream-sync-polish-and-ship) and wpt set `publish = false`

### inherited-defaults#2.1 · lost at classify · .andromeda/architecture-detail/inherited-defaults.md:14
- (.andromeda/architecture-detail/inherited-defaults.md:14) The chunk only edits documents, but its tree still passes the same gates through `.github/scripts/ci-leg.sh`. That means fmt, clippy `-D warnings`, the workspace tests and CI scripts (the `fast` leg) and rustdoc `-D warnings` over every workspace library crate (the `doc` leg).
  > `cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` and, since 2026-10-14-quality-gates, upstream's blitz-dom clippy line under `--no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets` (all run through `.github/scripts/ci-leg.sh`), rustdoc `-D warnings` set workflow-wide and enforced over every workspace library crate — the docs job runs `cargo doc --workspace --no-deps --locked`

### inherited-defaults#3.1 · lost at classify · .andromeda/architecture-detail/inherited-defaults.md:20
- (.andromeda/architecture-detail/inherited-defaults.md:20) Where the statement mentions telemetry or optional features, it must match this rule. Capabilities are Cargo features, and engine tracing is gated at each call site. escher-telemetry and escher-driver's `command` and `tool_call` spans are ungated.
  > Optional capabilities: Cargo features, tracing gated per call site in the engine crates — escher-telemetry and escher-driver (its `command` span and, since 2026-10-10-mcp-surface, its `tool_call` span) are ungated (packages/escher-telemetry/Cargo.toml:1-16; packages/escher-driver/Cargo.toml:13-18; packages/blitz-dom/src/net.rs:340-341; packages/dioxus-native/Cargo.toml:16-33)

### occupied-resources#3.1 · lost at strong · .andromeda/architecture-detail/occupied-resources.md:24
- (.andromeda/architecture-detail/occupied-resources.md:24) How a consumer starts a session must match the state directory as occupied — caller-named, never an env read or default, owner-only, removed at stop or idle expiry.
  > the driver session's state directory — a path its caller names (the `--session <dir>` option for `escher-session`; never an env read or a cwd-relative default), created mode `0700` by `escher_driver::serve` when absent (its parent must exist) and refused as `StateDirNotPrivate` when present and open to its group or to others, holding the socket file `session.sock` alone while a session is up, and removed at `stop` or at the host's idle expiry

### project-intent#1.0 · lost at classify · .andromeda/architecture-detail/project-intent.md:9
- (.andromeda/architecture-detail/project-intent.md:9) If the statement names escher's stack versions as current with 0.2.0, they must match these pins. The upgrades that are waiting must be stated as next-version work owned by the residuals line, not as capabilities.
  > Dioxus 0.8.0-alpha.3 (`286874cc1`) wait for escher's next version, owned by a `.andromeda/residuals.md` line (target: next), so Stylo stays "0.22.0", Taffy the registry "0.15" and Dioxus "0.7.3", with no git dependency

### stack-and-technologies#1.1 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:9
- (.andromeda/architecture-detail/stack-and-technologies.md:9) The statement's toolchain floor for a consumer building on escher must read edition 2024 and rust-version 1.91.0, as the workspace manifest states.
  > Rust Cargo workspace, resolver "2", edition "2024", rust-version "1.91.0" (Cargo.toml:1; Cargo.toml:32; Cargo.toml:40-41)

### stack-and-technologies#1.2 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:12
- (.andromeda/architecture-detail/stack-and-technologies.md:12) A statement "current with 0.2.0" must keep escher's version (0.2.0) apart from the crates' shared workspace version, which reads 0.3.0-beta.4, and never present one as the other.
  > version "0.3.0-beta.4", license "MIT OR Apache-2.0", category "gui" (Cargo.toml:34-39); in-repo crates pinned `=0.3.0-beta.4`

### stack-and-technologies#1.15 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:51
- (.andromeda/architecture-detail/stack-and-technologies.md:51) If the statement names platform reach, the wasm builds of the examples (seven_guis among them) must be stated as browser-target builds via Trunk, not as a target the driver or stand runs on.
  > Trunk via a `data-trunk rel="rust"` link (examples/seven_guis/index.html:6; examples/todomvc/index.html:6; examples/wasm_hello/index.html:6; examples/wasm_hello/src/lib.rs:4)

### stack-and-technologies#1.23 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:76
- (.andromeda/architecture-detail/stack-and-technologies.md:76) Though the chunk changes no code, its tree must still pass the local pre-push gate before it is done.
  > with `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts, stopping at the first red) the local pre-push gate

### stack-and-technologies#1.23 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:77
- (.andromeda/architecture-detail/stack-and-technologies.md:77) Where the statement says a capability is proven on the stand, that claim must rest on the stand checks the `stand-repeat` leg holds to a zero-failure budget.
  > the `stand-repeat` leg holds the stand checks to a zero-failure budget over three rounds

## Held

- cross-cutting-patterns#5.1
- design-philosophy#1.1
- design-philosophy#4.1
- design-philosophy#5.1
- established-decisions#10.1
- established-decisions#11.1
- established-decisions#11.2
- established-decisions#11.3
- established-decisions#14.1
- established-decisions#15.1
- established-decisions#15.3
- established-decisions#19.1
- established-decisions#24.1
- established-decisions#25.1
- existing-scopes#1.23
- existing-scopes#1.26
- existing-scopes#1.31
- existing-scopes#1.32
- occupied-resources#1.1
- occupied-resources#2.1
- occupied-resources#4.1
- occupied-resources#4.2
- occupied-resources#5.1
- occupied-resources#8.1
- project-intent#3.1
- stack-and-technologies#1.10
- stack-and-technologies#1.13
- stack-and-technologies#1.14
- stack-and-technologies#1.20
- stack-and-technologies#1.22
- standard-contracts#12.1
- standard-contracts#13.1
- standard-contracts#17.1
- standard-contracts#18.1
- standard-contracts#18.2
- standard-contracts#18.3
- standard-contracts#18.4
- standard-contracts#21.1
- standard-contracts#22.2
- standard-contracts#23.1
- standard-contracts#26.1
- standard-contracts#3.1
- standard-contracts#6.1
- standard-contracts#6.2

## Cascade only

- design-philosophy#3.1
- established-decisions#4.1
- existing-scopes#1.10
- existing-scopes#1.14
- existing-scopes#1.24
- stack-and-technologies#1.7
- standard-contracts#1.1
- standard-contracts#14.1
- standard-contracts#15.1
- standard-contracts#2.1
- standard-contracts#2.2
- standard-contracts#22.1
- standard-contracts#4.1
- standard-contracts#5.1
- standard-contracts#8.1
- standard-contracts#9.1
- standard-contracts#9.2
