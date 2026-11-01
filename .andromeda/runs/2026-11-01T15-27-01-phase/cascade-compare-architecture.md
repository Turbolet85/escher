# Cascade compare — architecture · phase · 2026-11-01-upstream-sync-ui-graph
E items 254 · held 114 · lost at classify 3 · lost at strong 0 · beside a placed fact - · cascade only 68 · listed 5 · body 50 · unplaced 0

## Lost — the full read carried these and the cascade did not; the planner takes each, marked «(sampler)»

### project-intent#8.1 · lost at classify · .andromeda/architecture-detail/project-intent.md:51
- (.andromeda/architecture-detail/project-intent.md:51) A take that edits blitz-shell's manifest or crate doc, as the winit beta.4 shape may, must re-read this statement and move its citations, including the undeclared hot-reload feature it names.
  > blitz-shell "Blitz application shell" / "Event loop, windowing and system integration." (packages/blitz-shell/Cargo.toml:3; packages/blitz-shell/src/lib.rs:3), whose crate doc lists a hot-reload feature its manifest does not declare (packages/blitz-shell/src/lib.rs:8; packages/blitz-shell/Cargo.toml:13-25)

### project-intent#8.1 · lost at classify · .andromeda/architecture-detail/project-intent.md:51
- (.andromeda/architecture-detail/project-intent.md:51) A take that edits the Dioxus crates' manifests or `dioxus-native`'s crate root must re-read this statement and move its citations.
  > Dioxus crates with keywords dom, ui, gui, react and repository `https://github.com/DioxusLabs/dioxus/` (packages/dioxus-native-dom/Cargo.toml:8-10; packages/dioxus-native/Cargo.toml:8-10), `launch` starting "an interactive HTML/CSS renderer driven by the Dioxus virtualdom" (packages/dioxus-native/src/lib.rs:95)

### project-intent#9.1 · lost at classify · .andromeda/architecture-detail/project-intent.md:57
- (.andromeda/architecture-detail/project-intent.md:57) Upstream test files the take carries must keep the citations of this validation line true, the cited test files staying where the line names them or the line moving with them at the wrap.
  > blitz-tests is "Integration tests for Blitz" (tests/blitz-tests/Cargo.toml:3), some of them Rust ports of script-driven WPT tests the runner cannot run (tests/blitz-tests/tests/oof_dynamic_cb.rs:7-12)

### stack-and-technologies#1.21 · lost at classify · .andromeda/architecture-detail/stack-and-technologies.md:69
- (.andromeda/architecture-detail/stack-and-technologies.md:69) The flake's toolchain pin must keep matching the workspace `rust-version`.
  > the flake's pin matches the workspace `rust-version` "1.91.0" its comment says to keep in sync with (flake.nix:31; Cargo.toml:41)

## Listed — the full read carried these and the cascade dropped them as standing conventions: they reach the planner on the standing list, never as items; the blind judge reads them here

### conventions#2.1 · listed (lost at strong) · .andromeda/architecture-detail/conventions.md:15
- (.andromeda/architecture-detail/conventions.md:15) Any manifest edit the take carries (`e5929f9e2` names manifests, and a pin shape moves them) must keep one declaration in `[workspace.dependencies]`, in-repo crates at `default-features = false`, and the `seven_guis` exception.
  > **Manifests:** dependencies declared once in `[workspace.dependencies]` and consumed as `{ workspace = true }` (Cargo.toml:43; Cargo.toml:270-306; packages/blitz-traits/Cargo.toml:14-22; packages/blitz/Cargo.toml:24-36; packages/dioxus-native-dom/Cargo.toml:29-47; packages/dioxus-native/Cargo.toml:73-119; packages/stylo_taffy/Cargo.toml:15-18), except the WPT runner's direct `mimalloc = "0.1.48"` (wpt/runner/Cargo.toml:15); in-repo crates declared with `default-features = false` (Cargo.toml:47-58), except the `seven_guis` path entry, which keeps its default `hybrid` renderer feature because `dioxus-native` raises `compile_error!` with no renderer (Cargo.toml:62; packages/dioxus-native/src/dioxus_renderer.rs:27)

### conventions#3.1 · listed (lost at classify) · .andromeda/architecture-detail/conventions.md:21
- (.andromeda/architecture-detail/conventions.md:21) New test files, a public item the take carries and any engine fix of ours must hold the doc form; the workspace rustdoc gate reads it, and a behaviour fix cites its spec where one governs.
  > **Documentation:** modules and files open with `//!` docs (packages/blitz-dom/src/scrolling.rs:1-2; packages/blitz-dom/src/selection.rs:1-5; packages/blitz-dom/src/stylo_device.rs:1-5; packages/blitz-dom/src/tree.rs:1; packages/blitz-dom/src/stylo_to_parley.rs:1; packages/blitz-traits/src/devtools.rs:1; packages/blitz-traits/src/navigation.rs:1; packages/blitz-traits/src/net.rs:1; packages/blitz-vibey-script/src/clock.rs:1; packages/blitz-vibey-script/src/timers.rs:1), public items carry `///` (packages/blitz-net/src/lib.rs:1-3; packages/blitz-dom/src/node/scrollbar.rs:1-3; packages/blitz-dom/src/node/serialize.rs:1; packages/blitz-dom/src/node/svg.rs:1; packages/blitz-test-harness/src/lib.rs:1-19; packages/blitz-shell/src/lib.rs:3-9); spec URLs cited above or next to implementations

### existing-scopes#3.2 · listed (lost at strong) · .andromeda/architecture-detail/existing-scopes.md:144
- (.andromeda/architecture-detail/existing-scopes.md:144) A check added or edited for the panic fork, the `font-embolden` gate or upstream's tests must keep these three modules no target and must move these reader counts when it reads one.
  > blitz-tests holds three shared modules that are no target — `tests/common/mod.rs`, read through `mod common;` by 21 checks, `tests/session_common/mod.rs`, by 32, and `tests/fixtures/mod.rs`, the fixture boot and hold, the shared fixture pages and the held network stand-in `HeldNet`, by 31

### inherited-defaults#3.1 · listed (lost at classify) · .andromeda/architecture-detail/inherited-defaults.md:20
- (.andromeda/architecture-detail/inherited-defaults.md:20) Every `tracing` call site the taken delta brings into an engine crate must stay gated per call site, escher-telemetry and escher-driver staying ungated, and this is the shape the sink allowlist re-check reads the delta against; new optional capability a take carries must arrive as a Cargo feature.
  > Optional capabilities: Cargo features, tracing gated per call site in the engine crates — escher-telemetry and escher-driver (its `command` span and, since 2026-10-10-mcp-surface, its `tool_call` span) are ungated (packages/escher-telemetry/Cargo.toml:1-16; packages/escher-driver/Cargo.toml:13-18; packages/blitz-dom/src/net.rs:340-341; packages/dioxus-native/Cargo.toml:16-33)

### occupied-resources#5.1 · listed (lost at classify) · .andromeda/architecture-detail/occupied-resources.md:36
- (.andromeda/architecture-detail/occupied-resources.md:36) The free-disk fix in `ci.yml` and a `font-embolden` run joining a local gate must keep this log path the failure artifacts' only one, and these citations are re-pointed if those edits move the lines.
  > CI leg logs `target/ci-logs/{leg}.log` and `target/ci-logs/matrix-{platform}.log`, merged cargo/python output truncated at each leg's start — the only path the CI failure artifacts upload (.github/scripts/ci-leg.sh:90-91; .github/workflows/ci.yml:422-423)

### occupied-resources#5.1 · listed (lost at classify) · .andromeda/architecture-detail/occupied-resources.md:38
- (.andromeda/architecture-detail/occupied-resources.md:38) P3's own trial worktree under `target/tmp/` must take a name clear of these occupied directory names, so the stand checks re-run after the take collide with nothing.
  > the session checks' own state directories are `ss-life`, `ss-idle` and `ss-quiet` (blitz-tests) and `hb-*`, `hl-trace`, `hl-client`, `hl-client-rec`, `hl-cells`, `hl-popup`, `cc-*`, `cf-*` and `ht-timer`

## Held

- conventions#1.1
- conventions#4.1
- conventions#5.1
- conventions#6.1
- conventions#7.1
- conventions#8.1
- cross-cutting-patterns#1.1
- cross-cutting-patterns#2.1
- cross-cutting-patterns#3.1
- cross-cutting-patterns#4.1
- cross-cutting-patterns#5.1
- cross-cutting-patterns#6.1
- design-philosophy#1.1
- design-philosophy#2.1
- design-philosophy#3.1
- design-philosophy#4.1
- design-philosophy#5.1
- established-decisions#1.1
- established-decisions#10.1
- established-decisions#11.1
- established-decisions#11.10
- established-decisions#11.13
- established-decisions#11.2
- established-decisions#11.3
- established-decisions#11.4
- established-decisions#11.8
- established-decisions#13.1
- established-decisions#14.2
- established-decisions#14.3
- established-decisions#15.1
- established-decisions#16.1
- established-decisions#16.2
- established-decisions#16.3
- established-decisions#16.4
- established-decisions#16.6
- established-decisions#16.7
- established-decisions#16.8
- established-decisions#17.2
- established-decisions#21.1
- established-decisions#22.1
- established-decisions#6.1
- established-decisions#7.1
- existing-scopes#1.10
- existing-scopes#1.13
- existing-scopes#1.14
- existing-scopes#1.17
- existing-scopes#1.23
- existing-scopes#1.24
- existing-scopes#1.25
- existing-scopes#1.31
- existing-scopes#1.32
- existing-scopes#1.7
- existing-scopes#2.1
- existing-scopes#3.4
- existing-scopes#3.5
- existing-scopes#4.1
- inherited-defaults#1.1
- inherited-defaults#2.1
- occupied-resources#1.1
- occupied-resources#10.2
- occupied-resources#10.3
- occupied-resources#11.1
- occupied-resources#12.1
- occupied-resources#13.1
- occupied-resources#14.1
- occupied-resources#15.1
- occupied-resources#2.1
- occupied-resources#7.2
- occupied-resources#8.1
- project-intent#1.0
- project-intent#2.1
- project-intent#3.1
- project-intent#7.1
- stack-and-technologies#1.1
- stack-and-technologies#1.11
- stack-and-technologies#1.12
- stack-and-technologies#1.13
- stack-and-technologies#1.16
- stack-and-technologies#1.19
- stack-and-technologies#1.2
- stack-and-technologies#1.20
- stack-and-technologies#1.22
- stack-and-technologies#1.23
- stack-and-technologies#1.3
- stack-and-technologies#1.4
- stack-and-technologies#1.5
- stack-and-technologies#1.6
- stack-and-technologies#1.7
- stack-and-technologies#1.9
- standard-contracts#1.1
- standard-contracts#12.1
- standard-contracts#12.3
- standard-contracts#14.1
- standard-contracts#19.1
- standard-contracts#2.1
- standard-contracts#2.3
- standard-contracts#2.5
- standard-contracts#2.7
- standard-contracts#20.1
- standard-contracts#22.1
- standard-contracts#23.1
- standard-contracts#27.1
- standard-contracts#27.2
- standard-contracts#28.2
- standard-contracts#32.1
- standard-contracts#32.2
- standard-contracts#4.2
- standard-contracts#4.4
- standard-contracts#5.1
- standard-contracts#6.4
- standard-contracts#6.5
- standard-contracts#9.10
- standard-contracts#9.4
- standard-contracts#9.9

## Cascade only

- established-decisions#11.11
- established-decisions#11.12
- established-decisions#11.5
- established-decisions#11.6
- established-decisions#11.7
- established-decisions#11.9
- established-decisions#12.1
- established-decisions#16.5
- established-decisions#17.1
- established-decisions#19.1
- established-decisions#2.1
- established-decisions#20.1
- established-decisions#23.1
- established-decisions#24.1
- established-decisions#28.1
- established-decisions#28.2
- established-decisions#29.1
- established-decisions#3.1
- established-decisions#4.1
- established-decisions#5.1
- established-decisions#8.1
- established-decisions#9.1
- existing-scopes#1.11
- existing-scopes#1.18
- existing-scopes#1.8
- occupied-resources#10.1
- stack-and-technologies#1.10
- stack-and-technologies#1.14
- stack-and-technologies#1.15
- stack-and-technologies#1.8
- standard-contracts#10.1
- standard-contracts#12.2
- standard-contracts#15.1
- standard-contracts#16.1
- standard-contracts#17.1
- standard-contracts#18.1
- standard-contracts#18.2
- standard-contracts#18.3
- standard-contracts#2.2
- standard-contracts#2.4
- standard-contracts#2.6
- standard-contracts#21.1
- standard-contracts#21.2
- standard-contracts#21.3
- standard-contracts#21.4
- standard-contracts#22.2
- standard-contracts#24.1
- standard-contracts#25.1
- standard-contracts#28.1
- standard-contracts#28.4
- standard-contracts#28.5
- standard-contracts#3.1
- standard-contracts#3.2
- standard-contracts#3.3
- standard-contracts#3.4
- standard-contracts#4.1
- standard-contracts#4.3
- standard-contracts#6.1
- standard-contracts#6.2
- standard-contracts#7.1
- standard-contracts#8.1
- standard-contracts#9.1
- standard-contracts#9.2
- standard-contracts#9.3
- standard-contracts#9.5
- standard-contracts#9.6
- standard-contracts#9.7
- standard-contracts#9.8
