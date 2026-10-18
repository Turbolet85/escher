# Codebase Research — 2026-10-18-one-virtual-clock

## Scope
- **Depth:** deep · **Reads:** 26 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — 1 addition (2026-10-06 boot-smoke `timeout` exit); none applies, since this chunk names no live leg beyond `ci-leg.sh` legs and blitz-tests targets. `.claude/rules/testing.md` — 7 additions, all read; the 2026-10-10 rule (search `tests/` and `examples/*/tests/` for a changed table's readers and call builders) is what the sweeps below follow.
- **Platform issues consulted:** none. No runner-only bullet: Setup 5a read `f45bb570a94a` in progress, with no red folded.
- **External inputs:**
  - `inputs#I1` — the operator's directive at take-up. The founder is away; his forks go to one `founder-forks.md` in the run dir, are answered by the operator as delegate and recorded PROVISIONAL; technical forks go to the dialog.
  - `inputs#I2` — the founder forks as answered at P4 (F1 B · F2 A · F2b A · F3 A, delegate operator, PROVISIONAL), with the technical forks the operator answered in the P4 dialog.
  - Every fact this chunk turns on besides those lives in this repository.

## Files inspected
- `packages/blitz-dom/src/document.rs` (316-327, 539-544, 979, 1860-1945, 2128-2150) — the fade: `scrollbar_activity: HashMap<NodeId, Instant>` (327), stamped `Instant::now()` in `show_scrollbars` (1877-1881, only under `cfg!(feature = "scrollbars")`), read by `last.elapsed()` in `scrollbar_opacity` (1870) and `scrollbars_animating` (1885-1890). `scrollbars_animating` feeds `is_animating` (2148). Double click: `last_mousedown_time: Option<Instant>` (316), `click_count: u16` (320). Pruned on node removal (979).
- `packages/blitz-dom/src/resolve.rs` (39, 66-72, 83) — `resolve(current_time_for_animations)`. It runs the scroll animation, then drops finished fade entries by `last.elapsed()` (wall), then resolves Stylo at the given time.
- `packages/blitz-dom/src/stylo.rs` (135, 808, 820) — Stylo's `current_time_for_animations` takes the `resolve` time. CSS animations and transitions already read the caller's clock.
- `packages/blitz-dom/src/node/scrollbar.rs` (1-30) — `FADE_DELAY` 500 ms, `FADE_DURATION` 200 ms, and `opacity_at(elapsed: Duration)`, a pure function of a `Duration`.
- `packages/blitz-dom/src/events/pointer.rs` (240-272, 385-412, 420-425, 530-575, 640-652, 705-725, 960-975):
  - `handle_mousedown` counts a second click when `last_mousedown_time.elapsed() < 500 ms` (wall) and the point lies within 2 px (394-403), then stamps `Instant::now()` (407).
  - `click_count` 2 selects the word in a text input (548-556); 3 or more selects the hard line; on a password input any count above 1 selects all (647-651).
  - A count of 2 dispatches `DomEventData::DoubleClick` after the click (968-974).
  - Panning starts only for `BlitzPointerId::Finger` (248-252). Its `SystemTime::now()` reads (267, 717) and the fling (724) live there.
  - `scrollbar_opacity` gates a thumb press at pointerdown (423).
- `packages/blitz-dom/src/scrolling.rs` (335-350, 505-530, 532-560, 666-713, 865-900) — `write_scroll_offset` calls `show_scrollbars` on every scroll write (347). Smooth scroll: `start_scroll_animation` reads `SystemTime::now()` (515), as does its frame (896). The fling frame reads it too (871). `scroll_into_view` writes nested boxes at once with `ScrollBehavior::Instant` (713); only the viewport takes the requested behaviour.
- `packages/blitz-dom/src/config.rs` (1-80) — `DocumentConfig` carries providers (`net_provider`, `navigation_provider`, `shell_provider`, `html_parser_provider`), each `Option<Arc<dyn …>>` with a `Dummy*`/default fallback. It has no clock today.
- `packages/blitz-test-harness/src/harness.rs` (14-70, 120-155) — `HarnessOptions::into_config` maps the options onto a `DocumentConfig` (48-70). `Harness.time: f64` starts at 0.0. `pump` resolves at `self.time` (146-149), and `tick(dt)` adds and pumps (152-155). `tick` has no caller in the tree (`grep -rn '\.tick(' tests/ packages/ examples/ --include=*.rs`: 0 hits).
- `packages/blitz-test-harness/src/settle.rs` (35-38, 133-175) — `Settled::animating` reads `doc.is_animating()` (174), so today it reads the wall-clock fade.
- `packages/blitz-test-harness/src/input.rs` (237-241) — the harness's `scroll_into_view` passes `ScrollBehavior::Instant`.
- `packages/blitz-test-harness/Cargo.toml` (13-20) — `blitz-paint` with `default-features = false`.
- `tests/blitz-tests/Cargo.toml` (20) — `blitz-paint = { …, features = ["scrollbars", "svg"] }`, so every blitz-tests build paints overlay scrollbars.
- `examples/seven_guis/Cargo.toml` (1-50) — no `scrollbars` feature, and `dioxus-native`'s defaults (`packages/dioxus-native/Cargo.toml` `[features]`) do not include `scrollbars`. A per-package `escher-session` build paints none; a workspace build unifies it on through `apps/browser`, `apps/readme` and blitz-tests.
- `packages/escher-driver/src/execute.rs` (105-106, 170-190, 194) — the `advance` arm: no time step → `Refusal::new(Cause::TimeUnavailable)`, otherwise `advanced_ms = step(harness, ms).min(ms)` inside one settled step. `scroll` → `harness.scroll_into_view(target.node)`.
- `packages/escher-driver/src/session.rs` (23, 90, 122-127) — `TimeStep = Box<dyn FnMut(&mut Harness<DioxusDocument>, u32) -> u32>` and `with_time`.
- `packages/escher-driver/src/schema.rs` (276-277) and `schema_tests.rs` (83-84) — the `screenshot` help's sentence "in a build that paints overlay scrollbars, a scrollbar thumb painted within 0.7 s of a scroll fades by wall time", and its pin.
- `examples/seven_guis/src/stand.rs` (80-100) — `TIMER_TICK_MS` 100, and `timer_step`, which delivers whole ticks, drops the remainder, and leaves "the harness's animation clock … where it was".
- `examples/seven_guis/src/session_host.rs` (30-44) — only the Timer's session is handed `with_time(stand::timer_step(handle))`.
- `examples/seven_guis/src/tasks/timer.rs` (55) — outside a `TimerTicks` context the Timer waits on `futures_timer::Delay` (wall, windowed only).
- `examples/seven_guis/src/tasks/circle_drawer.rs` (72) — the Circle Drawer opens its dialog on `ondoubleclick`.
- `tests/blitz-tests/tests/session_common/mod.rs` (36-52) — `hold` hands the Timer alone a step.
- `tests/blitz-tests/tests/stand_act_timer.rs` (1-10, 38, 73-74, 90-91, 156, 175-208) — asserts "the animation clock is where it was" after every call, and `time-unavailable` on a session with no step.
- `tests/blitz-tests/tests/stand_screenshot_repeat.rs` (1-30) — 12 shots, "No scroll precedes a shot: an overlay scrollbar thumb fades by wall time".
- `tests/blitz-tests/tests/stand_non_lean_tasks.rs` (70-89) — opens the Circle Drawer dialog by two `click_at`s, commenting "A click within 500 ms of wall time and 2 px of the last one counts as a double click".
- `tests/blitz-tests/tests/scrollbars.rs` (18-40, 57-205) — fixture checks that scroll and paint at once on `HtmlDocument` + `resolve(0.0)`, within the wall window by construction.
- `packages/blitz-shell/src/window.rs` (97, 280-302, 403-416) — the windowed clock: `current_animation_time()` returns seconds since the first query (wall), handed to `resolve`; `redraw` reads `is_animating()` to keep frames coming.

## Graph impact (from the code-graph query)
Trace `.andromeda/runs/2026-10-18T07-18-53-phase/tree-query-2026-10-18-one-virtual-clock.json`, one query, rust plane, 33 rows (editor lines = SCIP line + 1):
- **show_scrollbars** — 3 callers: `set_hover_to` @ `packages/blitz-dom/src/document.rs:1940`, `handle_pointerup` @ `packages/blitz-dom/src/events/pointer.rs:713`, `write_scroll_offset` @ `packages/blitz-dom/src/scrolling.rs:347`. Every scroll write and a thumb hover-out restart the fade.
- **scrollbar_opacity** — 3 callers: `set_hover_to` @ `document.rs:1928`, `handle_pointerdown` @ `pointer.rs:423`, `draw_scrollbars` @ `packages/blitz-paint/src/render.rs:752`. The fade decides paint, and also whether a hover or press hits a thumb.
- **scrollbars_animating** — 1 caller: `is_animating` @ `document.rs:2148`.
- **is_animating** — 20 callers: `resolve` @ `resolve.rs:162`, `View::redraw` @ `window.rs:417`, `Harness::settle` @ `settle.rs:174`, and 17 test sites (`fragment_navigation.rs` ×11, `render_blocking_stylesheet.rs:81`, `scroll_into_view_nested.rs:331`, `stand_act_timer.rs:186,209`, `stand_settle.rs:294`). The test sites read smooth scroll or CSS animation, none the fade.
- **with_time** — 3 callers: `session_host.rs:41`, `session_common/mod.rs:47`, `stand_act_timer.rs:157`. **timer_step** — 3: the same host and `hold`, plus the stand's own unit test at `stand.rs:139`.
- `tick` returned 0 call rows. The name exists on the plane (`Harness::tick` is defined at `harness.rs:152`), and the grep above finds no caller, so it is a genuine no-callers result.

## Patterns detected
- **Injected provider with a default** (`packages/blitz-dom/src/config.rs:31-60`): each embedder service is an `Option<Arc<dyn Trait>>` on `DocumentConfig` that falls back to a no-op or default. A clock the embedder supplies takes this shape (arch extract, "Embedder services are injected, with no-op defaults").
- **The caller's time already reaches style** (`resolve.rs:39,83` → `stylo.rs:135`): CSS animations and transitions move only by the time `resolve` is handed. The harness hands its own `time`, and the windowed shell hands wall seconds. The fade, double click, smooth scroll and fling are the engine's time reads that bypass it.
- **A switchable virtual clock precedent** (`packages/blitz-vibey-script/src/clock.rs:37-48`): `ClockMode::Real` reads `Instant::now()`, a virtual mode holds its own `now`, and `ScriptDocument::with_virtual_time` selects it.
- **Time handed in by the session's caller** (`session.rs:122-127`, `session_host.rs:40-42`): the driver names no app, and time reaches it as a closure.

## Conventions to follow
- **Engine telemetry gated**: any blitz-dom log site sits under `#[cfg(feature = "tracing")]` (obs extract §2). None is planned.
- **Both layout modes**: every new pipeline check runs `for incremental in [false, true]` (CLAUDE.md Critical Warnings).
- **Failure messages name a mode, a task and an index**, never an id or a screen reading (`stand_screenshot_repeat.rs:1-9`, test-plan §3 via the tests extract).
- **Fixed limit texts follow the built behaviour** (arch history, 2026-10-10-driver-cli and 2026-10-10-scrolling-box-bounds-and-hit): the `screenshot` sentence is retired once its reading is fixed.

## New files to create
- `packages/blitz-dom/src/clock.rs` — the clock a document reads (F1 B, inputs#I2): a trait, its wall default and a virtual implementation
- `tests/blitz-tests/tests/virtual_clock_engine.rs` — the engine's reads on a virtual clock, fixture-proven, and the wall default
- `tests/blitz-tests/tests/stand_virtual_clock.rs` — the repeat-shot proof on the stand's CRUD list (v020-06), and the double-click reading on the one clock

## Files to modify
- `packages/blitz-dom/src/document.rs` — the fade's stamp and reads, the double-click stamp's type, and the clock the document holds
- `packages/blitz-dom/src/resolve.rs` — the finished-fade prune on the document's clock
- `packages/blitz-dom/src/config.rs` — the clock field and its default
- `packages/blitz-dom/src/lib.rs` — the clock module's re-export
- `packages/blitz-dom/src/iframe.rs` — a sub-document's config carries its parent's clock
- `packages/blitz-dom/src/events/pointer.rs` — the double-click window and the pan and fling times on the document's clock
- `packages/blitz-dom/src/scrolling.rs` — the smooth-scroll start and frame and the fling frame on the document's clock
- `packages/blitz-test-harness/src/harness.rs` — the harness's time as the document's one clock, `tick` and `advance_time` moving it
- `packages/escher-driver/src/execute.rs` — `advance` moving the instance's clock (F2)
- `packages/escher-driver/src/session.rs` — `with_time` and the time step's docs (F2)
- `packages/escher-driver/src/schema.rs` — the `screenshot` sentence retired, the `advance` and `click` help restated (F2, F3)
- `packages/escher-driver/src/schema_tests.rs` — the help pins
- `packages/escher-driver/src/refusal.rs` — `time-unavailable`'s fate (F2)
- `packages/escher-driver/src/lib.rs` — the crate docs' account of `advance` and the causes
- `examples/seven_guis/src/stand.rs` — `timer_step` against the one clock (F2)
- `tests/blitz-tests/tests/stand_act_timer.rs` — the "animation clock is where it was" assertions restated
- `tests/blitz-tests/tests/stand_screenshot_repeat.rs` — the doc comment's wall-time sentence
- `tests/blitz-tests/tests/stand_non_lean_tasks.rs` — the double-click comment's wall-time sentence
- `tests/blitz-tests/tests/stand_act_spans.rs` — the `time-unavailable` span line (F2)
- `tests/blitz-tests/tests/stand_mcp_tools.rs` — the `advance` tool result and its `ending` (F2)
- `examples/seven_guis/tests/host_timer.rs` — the hosted `advanced_ms` readings (F2)
- `examples/seven_guis/tests/cli_commands.rs` — the `advance` and `time-unavailable` command-line readings (F2)
<!-- Settled at P4 on the founder forks F1 B · F2 A · F2b A · F3 A (delegate operator, PROVISIONAL, inputs#I2): the F1 A branch is dropped; `node/scrollbar.rs` needs no change (`opacity_at` already takes a Duration); `iframe.rs` joins (sub-document config, iframe.rs:74). -->

## Sweeps
- `TimeUnavailable|time-unavailable` over `packages examples tests` (`*.rs`, `grep -rln`): 6 files — 5 on the list above (`execute.rs`, `refusal.rs`, `stand_act_spans.rs`, `stand_act_timer.rs`, `stand_mcp_tools.rs`) plus `examples/seven_guis/tests/cli_commands.rs`, also listed; 0 no-change until F2 is answered.
- `wall time|wall clock|0\.7 s` over `packages examples tests` (`*.rs`): 6 hits in 5 files — `schema.rs:277`, `schema_tests.rs:83-84`, `stand_screenshot_repeat.rs:5`, `stand_non_lean_tasks.rs:82` are changed; `stand_act_timer.rs:9` ("no check here … reads a wall clock") is no-change, since the rule it states stands.
- `advanced_ms` over `packages examples tests` (`*.rs`, `grep -rln`): 11 files — `execute.rs`, `json.rs`, `schema.rs`, `schema_tests.rs`, `session_common/mod.rs`, `stand_act_ids.rs`, `stand_act_spans.rs`, `stand_act_timer.rs`, `stand_mcp_tools.rs`, `host_timer.rs`, `cli_commands.rs`. Their disposition waits on F2 (if `advanced_ms` keeps its meaning, `json.rs`, `session_common/mod.rs` and `stand_act_ids.rs` are no-change).
- `\.tick\(` over `tests/ packages/ examples/` (`*.rs`): 0 hits — `Harness::tick` has no caller.
- `last_mousedown_time|scrollbar_activity` over `packages tests` (`*.rs`): 11 hits, all in `document.rs`, `resolve.rs` and `pointer.rs`, all on the list.
- `Instant|SystemTime|Delay|futures_timer|time::` over `examples/seven_guis/src packages/dioxus-native-dom/src`: 1 time read, `timer.rs:55` (`futures_timer::Delay`, outside a `TimerTicks` context, windowed only) — no-change; the rest are `ScrollBehavior::Instant` and `NamedKey::InstantReplay` tokens.

## Open questions
- none — F1, F2, F2b and F3 were answered at P4 by the operator as the founder's delegate, PROVISIONAL (inputs#I2): B · A · A · A.
