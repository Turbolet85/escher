# Codebase Research — 2026-10-06-headless-stand

## Scope
- **Depth:** moderate · **Reads:** 16 · **Globs/Greps:** 22
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the gate tool reads a smoke's exit 124/137 as its own `timeout`); `.claude/rules/testing.md` — read in full, 0 Session Additions. This chunk names no live leg: its checks are in-process `cargo test` functions, so no firing-form invocation is needed beyond `cargo test -p blitz-tests --test {name}` and `ci-leg.sh fast`.
- **Platform issues consulted:** none — no runner-only bullet. Setup 5a's run was in progress, not red, and no CI-reading entry sits outside the operator leg.
- **External inputs:** none — every fact this chunk turns on lives in this repository.

## Files inspected
- `examples/seven_guis/src/app.rs` (full) — `enum Task` (line 6) and `fn TaskShell` (line 146) are private. `pub fn app` (line 68) always starts on `Home` (`use_signal(|| None)`) and wraps each task in `TaskShell`. `SHELL_CSS` (line 265) carries its own `html, body, #main` rules, so TaskShell styles itself without `HOME_CSS`.
- `examples/seven_guis/src/lib.rs`, `src/main.rs`, `Cargo.toml` (full) — the wasm entry uses `build_single_font_ctx(DEJAVU_SANS)` with `include_bytes!("../assets/DejaVuSans.woff2")`. `main` calls `escher_telemetry::init` then `dioxus_native::launch(seven_guis::app::app)`. The native target enables `dioxus-native/system-fonts` only; `woff` is enabled on wasm32 only. The crate is `crate-type = ["cdylib", "rlib"]`, default feature `hybrid`.
- `examples/seven_guis/src/tasks/{timer,counter}.rs` (full), `flight_booker.rs` / `crud.rs` (class and handler lines) — the timer ticks with `use_coroutine` over `futures_timer::Delay::new(100ms)` and renders `Elapsed: {:.1}s` plus `.progress-fill` width. The counter is `.counter-display` plus `.counter-btn`. The flight booker sets class `date-input invalid` on bad dates and `disabled: !dates_ok` on `.flight-btn`. CRUD has unlabeled inputs, and its Update/Delete buttons are `disabled: !has_selection`. No task element carries an `id` (stable ids are Epoch 2), so checks select by class.
- `packages/blitz-test-harness/src/{harness,lib}.rs` (full), `input.rs` / `inspect.rs` (signatures) — the `HarnessOptions` fields are width, height, scale, color_scheme, base_url and net_provider. `into_config` fills the rest with `..Default::default()`, so there is no font_ctx and no incremental field. `from_vdom` runs `DioxusDocument::new` → `initial_build` → `pump`. `wrap` is public and does not pump. `tick` advances only the harness clock passed to `resolve`.
- `packages/blitz-dom/src/config.rs` (lines 11-61) — `DocumentConfig` carries `font_ctx: Option<FontContext>` (line 49), `net_provider` (line 41), `style_threading` (line 55) and `incremental: Option<bool>` (line 61). `StyleThreading`'s `#[default]` is `Sequential` (lines 19-28), although the field doc at line 54 says "Defaults to Parallel".
- `packages/blitz-dom/src/document.rs` (lines 360-423) — a supplied `font_ctx` is used as-is. Without one, the collection's `system_fonts` follows the `system-fonts` feature plus `BULLET_FONT`. `net_provider` falls back to `DummyNetProvider` (lines 421-423).
- `packages/blitz-dom/src/lib.rs` (lines 126-150) and `util.rs` (lines 17-37) — `build_single_font_ctx` builds a collection with `system_fonts: false` and registers `decode_font_bytes(font_data)` as the fallback for every generic. `decode_font_bytes` decompresses `wOF2` only under `#[cfg(feature = "woff")]`. Without that feature the raw woff2 bytes pass through.
- `packages/dioxus-native-dom/src/dioxus_document.rs` (lines 84-146, 218-260) — `new` sets `base_url` to `dioxus://index.html`, adds `DEFAULT_CSS`, builds `html/head/body/main#main` and calls `set_event_converter` on every construction. `poll(None)` polls `vdom.wait_for_work()` with a static noop waker. `DomEventData::Input` is forwarded to Dioxus as `NativeFormData` (line 326).
- `packages/blitz-dom/src/events/keyboard.rs` (lines 117-123) — a text edit dispatches `DomEventData::Input(BlitzInputEvent { value })` with the editor's raw text.
- `packages/dioxus-native/Cargo.toml` (features), `src/dioxus_renderer.rs:27`, `src/lib.rs:63` — `compile_error!("At least one renderer feature must be enabled")`. `dioxus_native` re-exports `FontContext` and `build_single_font_ctx` from blitz-dom.
- `tests/blitz-tests/Cargo.toml` (full), `tests/blitz-tests/lib.rs`, `tests/harness_smoke.rs` (full) — the dev-dependencies are `blitz-dom` with features `accessibility, floats, system-fonts` (no `woff`) and `dioxus-native-dom`. `lib.rs` is a doc-only shell. `harness_smoke.rs:89-104` (a Dioxus counter driven through `from_component` plus `click`) is the nearest template.
- `tests/blitz-tests/tests/link_rel_attribute.rs:16` — `RecordingNetProvider` is a test-file-local fake, not a shared helper.

## Graph impact
- **app / TaskShell / tasks** — 22 call rows (trace `tree-query-2026-10-06-headless-stand.json`, query 1, db_state fresh). `app` is called only from `main.rs:8` (`launch`) and the wasm entry in `lib.rs`. `TaskShell` and the task components are called only inside `app.rs`'s `app()`. Exposing a named-task root has no external caller to rethread.
- **seven_guis crate edges** — 4 rows (query 2): seven_guis → blitz-dom, dioxus-native, escher-telemetry, and blitz-tests → blitz-test-harness. No crate depends on seven_guis today, so `blitz-tests → seven_guis` is a new edge (an arch §Occupied Resources / crate-graph fact for wrap).
- **HarnessOptions / from_vdom / into_config** — 19 ref rows (query 3). The struct-literal sites are `dir_attribute.rs:23`, `pointer_events.rs:11` and `touch_events.rs:28` (re-derived: `grep -rn "HarnessOptions {" --include=*.rs`, 3 hits outside the definition). All 3 end in `..Default::default()`, so a new `HarnessOptions` field breaks no caller.

## Patterns detected
- **Bundled font only under `woff`** (`util.rs:22-37`, `lib.rs:131-150`): `build_single_font_ctx(DejaVuSans.woff2)` yields a registered face for these inputs only when blitz-dom's `woff` feature is on. Off, the woff2 bytes are registered undecoded, no family registers, and text has no face. That is the vacuous 0×0 trap. `cargo tree -p blitz-tests -e features -i blitz-dom --locked` and `-p seven_guis` both list no `woff`. `cargo tree --workspace -i wuff` shows it reached only through `blitz`'s default `blitz-dom`. So `cargo test --workspace` unifies `woff` on, but `cargo test -p blitz-tests --test {name}` builds it off. The stand's font path must enable `woff` itself, and `wuff` is already in `Cargo.lock`.
- **Supplied font context replaces system fonts** (`document.rs:370-376`): a `font_ctx` built by `build_single_font_ctx` has `system_fonts: false`, so host fonts cannot win the generic lookup on that path. The design extract's host-font-leakage concern is closed by construction.
- **No live network by default** (`document.rs:421-423`): `net_provider: None` → `DummyNetProvider`. The stand needs no new provider to be offline. Asserting "zero requests" needs a recording fake, and the only one is file-local in `link_rel_attribute.rs:16`.
- **Real-time timer, noop-waker pump** (`timer.rs`, `dioxus_document.rs:220-238`): `futures_timer::Delay` completes on wall-clock time from its own global timer thread. `Harness::tick` moves only the animation clock handed to `resolve`. Its equality: for the timer task, N `tick`s with no wall-clock wait yield `Elapsed: 0.0s`. No harness-side knob advances `elapsed`, so a deterministic elapsed advance needs a stand-side tick seam.
- **Sequential is the real default** (`config.rs:19-28`): harness documents resolve sequentially, so parallel `cargo test` threads do not share Stylo's pool. The issue-430 panic the arch extract flags does not apply to the harness path. The doc line `config.rs:54` contradicts the derive, which is upstream doc drift and not this chunk's to fix.

## Conventions to follow
- **Integration file shape**: a `//!` doc naming the behaviour, snake_case behaviour names, `#[track_caller]` helpers, and assert the fixture first (`.claude/rules/testing.md` §File and naming; `harness_smoke.rs:1-3`).
- **Dioxus via input helpers**: `click` / `type_text` route through `handle_ui_event` → `DioxusEventHandler` (`dioxus_document.rs:258-264`). Never use `dispatch_recorded` for Dioxus behaviour (`verification-harness.md` §In-process Harness).
- **Manifests**: dependencies declared once in root `[workspace.dependencies]`, consumed `{ workspace = true }`. In-repo crates normally use `default-features = false`, but `seven_guis` cannot (`dioxus_renderer.rs:27` compile_error without a renderer). The edge keeps seven_guis' default `hybrid`, which `cargo build --workspace` already compiles.
- **No telemetry in checks**: tests install no subscriber (obs extract; `telemetry_*` files are one-process-per-behaviour), so stand checks never call `escher_telemetry::init`.

## New files to create
- `examples/seven_guis/src/stand.rs` — the headless stand surface: the named-task root, the pinned viewport constants, the bundled-font context and the timer tick seam's headless side
- `tests/blitz-tests/tests/stand_boot.rs` — boot invariants: each lean task mounts in TaskShell offline at the pinned viewport, bundled font measures non-zero, two boots lay out identically in both layout modes
- `tests/blitz-tests/tests/stand_counter.rs` — counter proof check
- `tests/blitz-tests/tests/stand_flight_booker.rs` — flight-booker proof check
- `tests/blitz-tests/tests/stand_timer.rs` — timer proof check
- `tests/blitz-tests/tests/stand_crud.rs` — CRUD proof check

## Files to modify
- `examples/seven_guis/src/app.rs` — make the task enum and TaskShell reachable for a named-task root without changing the windowed tree
- `examples/seven_guis/src/lib.rs` — declare the `stand` module
- `examples/seven_guis/src/tasks/timer.rs` — tick seam: default real `futures_timer` cadence, injectable ticks headlessly
- `examples/seven_guis/Cargo.toml` — enable `dioxus-native/woff` on native so the bundled woff2 decodes in the stand's own build
- `packages/blitz-test-harness/src/harness.rs` — the fork's harness-option branch: `HarnessOptions` gains a font context and an incremental switch
- `tests/blitz-tests/Cargo.toml` — `seven_guis` dev-dependency (which brings `woff` through seven_guis' native feature)
- `Cargo.toml` — `seven_guis` entry in `[workspace.dependencies]`
- `Cargo.lock` — blitz-tests' dependency list gains the path crate

## Open questions
- Font and incremental configuration: add `font_ctx` and `incremental` fields to `HarnessOptions` (a public harness addition, arch amendment, non-breaking per the 3 `..Default` sites), or have the stand build `DocumentConfig` and use `Harness::wrap` (no harness change, but it duplicates `from_vdom`'s three steps)? → blocks: plan-decision
- Timer determinism: a stand-side tick seam (the timer takes its ticks from a Dioxus context the headless root provides, defaulting to the real 100 ms `Delay`), or limit the timer check to boot plus Reset/duration with no elapsed advance? → blocks: plan-decision
- `type_text` into a Dioxus controlled `input` reaches its `oninput` with the typed value (`keyboard.rs:123` → `dioxus_document.rs:326`). This is not yet exercised by any Dioxus test, so the flight-booker and CRUD checks are its first witness → blocks: implementation-scope
