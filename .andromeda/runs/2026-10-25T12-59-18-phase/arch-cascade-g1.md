# arch · cascade read · group 1 of 7

## Carried
- (.andromeda/architecture-detail/design-philosophy.md:23) A load in flight carried on the stand must reach the stand's document only through a `NetProvider` the stand's boot supplies in its `DocumentConfig`, the stand's offline default staying the no-op provider, never through a fetch of the stand's own.
  > - **Embedder services are injected, with no-op defaults** — networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies.
  (.andromeda/architecture-detail/design-philosophy.md:24)
  > `DocumentConfig` carries trait-object providers `NetProvider`, `NavigationProvider`, `ShellProvider`, `HtmlParserProvider` (packages/blitz-dom/src/config.rs:40-47); when one is not supplied, `BaseDocument::new` falls back to `DummyNetProvider`, `DummyNavigationProvider`, `DummyShellProvider` and `DummyHtmlParserProvider`
  (.andromeda/architecture.md:7)
  > - **Embedder services are injected, with no-op defaults** — networking, navigation, shell and parsing reach the DOM only through provider traits the embedder supplies; script fetches go through the embedder's net provider, never an internal HTTP client.
  (.andromeda/architecture.md:250)
  > fetches go through `NetProvider::fetch` with a `ResourceHandler` that sends on the channel then `request_redraw` (packages/blitz-dom/src/net.rs:116-125)
  (.andromeda/architecture.md:255)
  > tests dispatch loads through a pluggable `NetProvider` (tests/blitz-tests/tests/link_rel_attribute.rs:20-24; tests/blitz-tests/tests/link_rel_attribute.rs:34-41)
- (.andromeda/architecture-detail/design-philosophy.md:46) Every stand check the chunk re-pins or adds (keyboard, contrast, ids, actionable keys, snapshot, text, parts, diff, act, select, load wait) must run in both layout modes and read identically in each.
  > Incremental and non-incremental layout run the same style → damage → box construction → layout pipeline; non-incremental marks every node damaged each resolve (packages/blitz-dom/src/document.rs:2223-2240; packages/blitz-dom/src/resolve.rs:87-93), so the two must produce identical layouts after any sequence of mutations (tests/blitz-tests/tests/incremental_oracle.rs:1-7)
  (.andromeda/architecture-detail/design-philosophy.md:45)
  > - **One incremental pipeline, safe identities** — style, damage, box construction, layout and paint topology are invalidated incrementally through a single code path, over versioned node ids.
  (.andromeda/architecture.md:11)
  > both layout modes produce identical layouts, a dropped node's id stops resolving instead of aliasing a reused slot, the slotmap key is used only at the storage boundary and every public API takes `NodeId`
- (.andromeda/architecture-detail/design-philosophy.md:60) The extended stand (`cells` served, the flight booker's select, the held load) must be exercised headless, with no window, its input synthesized through the real event-dispatch pipeline, its checks in blitz-tests' `tests/` directory.
  > The test harness runs headless: no window, GPU or compositor is required, and synthesized input routes through the real event-dispatch pipeline (packages/blitz-test-harness/src/lib.rs:1-19); integration tests live in the `tests/` directory of the blitz-tests crate (tests/blitz-tests/lib.rs:1)
  (.andromeda/architecture-detail/design-philosophy.md:59)
  > - **Headless, measurable, testable** — the engine is exercised without a window through a headless harness
  (.andromeda/architecture.md:13)
  > - **Headless, measurable, testable** — the engine is exercised without a window through a headless harness, a WPT runner and phase-split benchmarks;
- (.andromeda/architecture-detail/stack-and-technologies.md:9) The stand's new code in seven_guis must build as edition 2024 under rust-version 1.91.0, with no manifest or toolchain change.
  > **Technology:** Rust Cargo workspace, resolver "2", edition "2024", rust-version "1.91.0" (Cargo.toml:1; Cargo.toml:32; Cargo.toml:40-41); edition 2024 in browser, browser-persistence, rdme, counter, seven_guis,
  (.andromeda/architecture.md:19)
  > The stack: a Rust 2024 Cargo workspace (rust-version 1.91.0)
- (.andromeda/architecture.md:249) The flight booker's select and any stand task change outside the native `stand` module must keep seven_guis building as a wasm cdylib under its `hybrid` feature.
  > seven_guis and todomvc built as wasm cdylib with `--no-default-features --features hybrid` (.github/workflows/ci.yml:122-144)
- (.andromeda/architecture.md:246) The widened headless boot must keep registering the crate-root bundled font for every task it boots, `cells` and the held-load page included.
  > seven_guis' bytes are the crate-root `DEJAVU_SANS`, compiled on native too, where the headless stand registers the same font
- (.andromeda/architecture-detail/stack-and-technologies.md:33) A stand check or task module that glob-imports the Dioxus prelude and names keys (the select's arrows and typeahead, the Tab and Enter/Space tables) must name the workspace's keyboard-types crate as `::keyboard_types`.
  > keyboard-types "0.8" (lock 0.8.3) since the 2026-10-17 upstream sync, while Dioxus 0.7.3 carries its own keyboard-types 0.7, so two versions stand in the graph and a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`
  (.andromeda/architecture.md:21)
  > Two keyboard-types versions stand in the graph, Dioxus carrying its own 0.7, so a module that glob-imports `dioxus::prelude::*` names the workspace's crate as `::keyboard_types`;
- (.andromeda/architecture-detail/stack-and-technologies.md:45) The flight booker's select, the served `cells` and any held-load page must be written against the workspace's Dioxus 0.7.3 crates, no Dioxus version moved.
  > **Technology:** Dioxus crates "0.7.3" (dioxus, dioxus-core, dioxus-html, dioxus-hooks, dioxus-signals, dioxus-stores, dioxus-asset-resolver, manganis, dioxus-document, dioxus-history, dioxus-devtools, dioxus-cli-config, dioxus-core-macro) (Cargo.toml:85-102)
  (.andromeda/architecture-detail/stack-and-technologies.md:46)
  > **Role:** Dioxus virtual DOM driving Blitz through dioxus-native / dioxus-native-dom
- (.andromeda/architecture-detail/stack-and-technologies.md:48) The acceptance's check that passes on replay must be read and compared through escher-driver's own serde_json, with no crate added for it.
  > since 2026-10-10-mcp-surface serde_json "1" — resolved 1.0.151, already in the graph through dioxus-html, taken by escher-driver alone as its reader of MCP requests, of kept checks and of the live answer line a replay compares (packages/escher-driver/Cargo.toml:17) —
- (.andromeda/architecture-detail/stack-and-technologies.md:66) The re-pinned stand checks must take the stand and the driver through blitz-tests' existing dev-dependencies, and the edited `flight.sh` must stay a POSIX `sh` script run under plain `sh` by `cli_flow`, no dependency or framework added.
  > **Technology:** blitz-tests dev-dependencies: blitz-test-harness, escher-telemetry, blitz crates, dioxus-native-dom, seven_guis (the headless stand, tests/blitz-tests/Cargo.toml:22), escher-driver (the driver session, tests/blitz-tests/Cargo.toml:23),
  (.andromeda/architecture-detail/stack-and-technologies.md:67)
  > since 2026-10-10-driver-cli two shell flows — seven_guis' unix-gated test target `cli_flow` runs two POSIX `sh` scripts under plain `sh` against the built `escher-session` binary, a shell being that one target's requirement and no dependency, manifest line or test framework added for it (examples/seven_guis/tests/cli_flow.rs:31-74)
- (.andromeda/architecture-detail/stack-and-technologies.md:73) The live run must use the operator host's `claude` client (a `run` precondition, never a CI requirement), the operator-built `escher-session mcp` as its server and the pipe's stdlib python3 judge with no package added, while CI runs only the pipe's shim tests.
  > the cold-agent pipe's agent client, the Claude Code CLI `claude` — a host tool on the operator host only, a `run` precondition (exit 3) and no CI requirement (CI runs the pipe's shim tests only), read at 2.1.288 on the dev host (as measured at escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/evidence/live-run.md) and at 2.1.296 (as measured at escher-0.1.0/chunks/2026-10-15-cold-agent-test/evidence/operator-9-claude-version.txt) — its MCP server, since 2026-10-15-cold-agent-test, the workspace's own `escher-session mcp` (a `run` precondition built by the operator, never by the pipe; the stub retired with that chunk), and its judge stdlib python3 (`datetime json os queue re subprocess sys threading`) with no package added (ratified by the founder, 2026-10-06) (scripts/cold-agent.sh:452-462)
  (.andromeda/architecture-detail/stack-and-technologies.md:72)
  > escher's `test_blitz_tests_targets.py`, which reads a manifest with the stdlib `tomllib`, so the `ci-scripts` leg needs Python 3.11 or later, no package added
  (.andromeda/architecture.md:241)
  > cold-agent.{sh,ps1} (cold-agent pipe)
- (.andromeda/architecture-detail/stack-and-technologies.md:77) The re-pinned and new stand checks must pass the `stand-repeat` leg's zero-failure budget over three rounds, and the change must pass the `fast` gate first.
  > the `stand-repeat` leg holds the stand checks to a zero-failure budget over three rounds (.github/scripts/ci-leg.sh:21-25; .github/scripts/ci-leg.sh:48-65)
  (.andromeda/architecture-detail/stack-and-technologies.md:76)
  > with `bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → ci-scripts, stopping at the first red) the local pre-push gate
  (.andromeda/architecture.md:247)
  > stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates: `ubuntu-latest`, needing the fast four, the test job's `workspace-test` cache restored with `save-if: false`, a failure-only `ci-log-stand-repeat` upload, no job-level permission; .github/workflows/ci.yml:300-323)

## Bindings
- **design-philosophy (embedder services) ↔ the body's Infrastructure Patterns (headless loading)**: the stand's held load must arrive through an injected `NetProvider` as tests already dispatch loads, so the P4 fork over how the stand carries a load is decided on the provider seam, never an in-app fetch, (.andromeda/architecture-detail/design-philosophy.md:23) and (.andromeda/architecture.md:255)
- **design-philosophy (both layout modes) ↔ stack-and-technologies (code quality)**: every re-pinned stand check runs in both modes, and that two-mode set is what `stand-repeat` holds to zero failures over three rounds, (.andromeda/architecture-detail/design-philosophy.md:46) and (.andromeda/architecture-detail/stack-and-technologies.md:77)
- **stack-and-technologies (UI framework) ↔ stack-and-technologies (windowing and input)**: the stand's Dioxus 0.7.3 modules carry Dioxus's own keyboard-types 0.7, so a module that names keys beside the prelude glob names `::keyboard_types`, (.andromeda/architecture-detail/stack-and-technologies.md:45) and (.andromeda/architecture-detail/stack-and-technologies.md:33)
- **stack-and-technologies (CI/CD) ↔ stack-and-technologies (code quality)**: the edited `test_cold_agent.py` runs in the `ci-scripts` leg of the `fast` gate under Python 3.11 or later with no package, while the live `claude` run stays off CI, (.andromeda/architecture-detail/stack-and-technologies.md:72) and (.andromeda/architecture-detail/stack-and-technologies.md:76)
