# Cascade compare — architecture · phase · 2026-10-26-codebase-health
E items 81 · held 24 · lost at classify 6 · lost at strong 5 · beside a placed fact - · cascade only 10 · listed 5 · body 16 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### conventions#6.1 · lost at strong · .andromeda/architecture-detail/conventions.md:39
- (.andromeda/architecture-detail/conventions.md:39) The refactored lint walk in escher-driver must keep the driver's tracing form: no `#[instrument]` on a new helper and no `cfg(feature = "tracing")` gate added.
  > escher-driver, since 2026-10-07-driver-command-spans, takes `tracing` with no feature and builds its spans by hand with `tracing::info_span!` — the `command` span and, since 2026-10-10-mcp-surface, the `tool_call` span, the parent of `command` on an MCP tool call (packages/escher-driver/src/mcp.rs:118-121) — no `#[instrument]`, no `cfg(feature = "tracing")` gate

### cross-cutting-patterns#1.1 · lost at classify · .andromeda/architecture-detail/cross-cutting-patterns.md:8
- (.andromeda/architecture-detail/cross-cutting-patterns.md:8) `lint.rs::findings` reads a `Snapshot`, and the snapshot modules are gated under `accessibility`. Every helper the factoring introduces must therefore sit behind the same gating as the code it comes from, and must build in every feature set the workspace builds, so the both-build checks stay green.
  > `accessibility` also to `dioxus-native-dom/accessibility`, which enables dioxus-native-dom's own optional `accesskit` and gates its `accessibility_tree` override and its `snapshot`, `snapshot_diff`, `snapshot_text` and `actionable` modules (packages/dioxus-native/Cargo.toml:16-33; packages/dioxus-native-dom/Cargo.toml:15-23; packages/dioxus-native-dom/Cargo.toml:41)

### design-philosophy#3.1 · lost at classify · .andromeda/architecture-detail/design-philosophy.md:35
- (.andromeda/architecture-detail/design-philosophy.md:35) The engine function the founder ruled out of scope, `clear_interaction_state_for_removed_node`, carries this browser-semantics principle; the chunk must leave it and its WebKit/Blink-matching teardown exactly as it stands, untouched by any factoring.
  > interaction-state teardown on node removal matches WebKit/Blink semantics (packages/blitz-dom/src/document.rs:987-1006; tests/blitz-tests/tests/interaction_state_teardown.rs:1-14)

### design-philosophy#4.1 · lost at strong · .andromeda/architecture-detail/design-philosophy.md:46
- (.andromeda/architecture-detail/design-philosophy.md:46) The lint's findings walk is read in both layout modes by `stand_lint`; the factored walk must return the same findings under incremental and non-incremental layout, both modes staying identical and green.
  > Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2223-2240; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)

### established-decisions#16.2 · lost at classify · .andromeda/architecture-detail/established-decisions.md:111
- (.andromeda/architecture-detail/established-decisions.md:111) This cite points at lines inside `clear_interaction_state_for_removed_node` (document.rs:1007 onward). Since the function stays untouched, the cite must still resolve to the same lines.
  > a removed focus clears its chain (packages/blitz-dom/src/document.rs:1025-1039)

### existing-scopes#1.7 · lost at strong · .andromeda/architecture-detail/existing-scopes.md:28
- (.andromeda/architecture-detail/existing-scopes.md:28) `packages/blitz-dom/src/document.rs`, which holds `clear_interaction_state_for_removed_node`, must be left unchanged, so the interaction unit's cited spans stay as they are.
  > Hit testing and `elementFromPoint`/`elementsFromPoint` (packages/blitz-dom/src/document.rs:1732-1793); hover, focus, active (packages/blitz-dom/src/document.rs:1824-1960; packages/blitz-dom/src/document.rs:2045-2172)

### occupied-resources#6.2 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:46
- (.andromeda/architecture-detail/occupied-resources.md:46) "Every check green" includes the committed Counter checks. Their kept answers must stay byte-identical: `stand_replay` must still replay them and seven_guis `recording` must still compare a fresh recording byte for byte. So the lint refactor must change no answer byte that a recorded step holds.
  > read by `stand_replay` through `Check::from_bytes`, compared byte for byte with a fresh recording by seven_guis `recording` in `rc-fresh` under `CARGO_TARGET_TMPDIR`, uploaded by no CI leg

### occupied-resources#11.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:84
- (.andromeda/architecture-detail/occupied-resources.md:84) escher-driver's registered shape must stay as listed. The helpers or table walk add no dependency (no complexity-tool or table crate), no feature and no binary target to escher-driver.
  > escher-driver, a `publish = false` library with no features of its own and no binary target, workspace member and `[workspace.dependencies]` path entry, depending on blitz-test-harness, on dioxus-native-dom with its `accessibility` feature named, since 2026-10-07-driver-command-spans, on `tracing`, since 2026-10-10-mcp-surface, on `serde_json` and, since 2026-10-12-headless-screenshot, on `png` (the workspace's, each with no feature)

### occupied-resources#4.1 · lost at classify · .andromeda/architecture-detail/occupied-resources.md:30
- (.andromeda/architecture-detail/occupied-resources.md:30) The `ci-leg.sh fast` and `doc` verdicts this chunk owes are read from the legs' logs at this registered path. The path is truncated at each leg's start, so it has to be read after each run.
  > CI leg logs `target/ci-logs/{leg}.log` and `target/ci-logs/matrix-{platform}.log`, merged cargo/python output truncated at each leg's start — the only path the CI failure artifacts upload

### stack-and-technologies#1.1 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:9
- (.andromeda/architecture-detail/stack-and-technologies.md:9) The factored `lint.rs::findings` and its new helpers must build in the workspace's Rust 2024 edition at its MSRV 1.91.0. They must use no language feature past that version.
  > Rust Cargo workspace, resolver "2", edition "2024", rust-version "1.91.0" (Cargo.toml:1; Cargo.toml:32; Cargo.toml:40-41)

### stack-and-technologies#1.20 · lost at strong · .andromeda/architecture-detail/stack-and-technologies.md:66
- (.andromeda/architecture-detail/stack-and-technologies.md:66) The sweep test's table walk or helper must use only the blitz-tests dev-dependencies listed here. It adds no dev-dependency or manifest line.
  > blitz-tests dev-dependencies: blitz-test-harness, escher-telemetry, blitz crates, dioxus-native-dom, seven_guis (the headless stand, tests/blitz-tests/Cargo.toml:22), escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23), anyrender, anyrender_vello_cpu, dioxus, test-that, usvg, tracing, tracing-log

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### conventions#5.1 · listed (lost at classify) · .andromeda/architecture-detail/conventions.md:33
- (.andromeda/architecture-detail/conventions.md:33) A helper split out of `lint.rs::findings` must stay a crate internal and widen no public API.
  > crate internals are `pub(crate)` / `pub(super)`

### conventions#7.1 · listed (lost at classify) · .andromeda/architecture-detail/conventions.md:46
- (.andromeda/architecture-detail/conventions.md:46) Flattening `findings`' branch ladder may use the workspace's let-chain form in `if` conditions.
  > let-chains in `if` conditions

### design-philosophy#5.1 · listed (lost at strong) · .andromeda/architecture-detail/design-philosophy.md:60
- (.andromeda/architecture-detail/design-philosophy.md:60) The lint reads the screen through the headless harness, and `stand_requirement_sweep` is a blitz-tests integration test; the factored test and its helpers must stay in the blitz-tests crate's `tests/` directory, and the lint must keep running headless through `Harness`.
  > The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19); integration tests live in the `tests/` directory of the blitz-tests crate (tests/blitz-tests/lib.rs:1)

### existing-scopes#3.1 · listed (lost at classify) · .andromeda/architecture-detail/existing-scopes.md:141
- (.andromeda/architecture-detail/existing-scopes.md:141) `mcp_tests.rs` must stay a `#[path]` unit-test child of `mcp`, not part of the crate's public tree, and must stay that way after the listing test is refactored.
  > escher-driver's `cli` and `recording` modules hold their unit tests in `#[path]` children of their own, as `schema` and `mcp` do — `cli_tests.rs`, `recording_tests.rs` and `recording_off_unix_tests.rs` (§Conventions, **Tests**) — none a module of the crate's public tree

### existing-scopes#3.2 · listed (lost at classify) · .andromeda/architecture-detail/existing-scopes.md:142
- (.andromeda/architecture-detail/existing-scopes.md:142) The shared non-target modules' reader counts must hold. A helper the sweep test gains goes into its own file, or into a shared module it already reads, so that 21 · 31 · 24 do not move.
  > blitz-tests holds three shared modules that are no target — `tests/common/mod.rs`, read through `mod common;` by 21 checks, `tests/session_common/mod.rs`, by 31, and `tests/fixtures/mod.rs`, the fixture boot and hold, the shared fixture pages and the held network stand-in `HeldNet`, by 24 (22 `stand_*` files, `font_face_sniffed_source` and `select_interaction`)

## Held

- conventions#1.1
- conventions#8.1
- cross-cutting-patterns#2.1
- cross-cutting-patterns#5.1
- established-decisions#15.1
- established-decisions#25.1
- established-decisions#26.1
- existing-scopes#1.26
- existing-scopes#1.31
- existing-scopes#2.1
- inherited-defaults#2.1
- inherited-defaults#3.1
- occupied-resources#1.1
- occupied-resources#7.1
- project-intent#3.1
- project-intent#4.1
- stack-and-technologies#1.14
- stack-and-technologies#1.22
- stack-and-technologies#1.23
- standard-contracts#18.1
- standard-contracts#19.3
- standard-contracts#20.1
- standard-contracts#28.1
- standard-contracts#6.2

## Cascade only

- established-decisions#16.3
- standard-contracts#1.1
- standard-contracts#13.1
- standard-contracts#14.1
- standard-contracts#15.1
- standard-contracts#16.2
- standard-contracts#23.1
- standard-contracts#24.2
- standard-contracts#25.1
- standard-contracts#6.1
