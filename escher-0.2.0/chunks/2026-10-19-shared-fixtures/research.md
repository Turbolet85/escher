# Codebase Research — 2026-10-19-shared-fixtures

## Scope
- **Depth:** moderate · **Reads:** 24 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (1 Session Addition, a gate-tool
  timeout note; no live leg here) · `.claude/rules/testing.md` — read in full (9 Session Additions; applied: the
  2026-10-19 off-unix compile check for a moved `cfg(unix)` item, the 2026-10-10 search of `tests/` for a pinned
  table before calling a check unaffected)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** none — every fact this chunk turns on lives in this repository

## Files inspected
- `tests/blitz-tests/tests/stand_settle.rs` (22-26, 127-260, 308-330) — file-private `fixture(app, incremental)` boot;
  `ManualNetProvider` (held requests, `held()`, `answer(file, css)` asserting the url's tail); two
  `Session::start(label, || fixture(…))` holds; its `restless_fixture` ids `fx-root`/`fx-label`
- `tests/blitz-tests/tests/font_face_sniffed_source.rs` (1-125) — `HeldNetProvider` (held requests, `urls()`,
  `answer(body)`); declares no shared module
- `tests/blitz-tests/tests/stand_boot.rs` (1-130) — escher-authored `RecordingNetProvider`: records the url, drops the
  handler; its two checks read the url list alone; declares no shared module
- `tests/blitz-tests/tests/render_blocking_stylesheet.rs` (14-70) · `link_rel_attribute.rs` (11-35) — upstream's own
  stand-ins (4322e86f0 upstream #695; e255bf7df upstream #846), left as written
- `tests/blitz-tests/tests/{stand_act_diff,stand_diff,stand_snapshot_text,stand_mcp_masked,stand_session_recording,stand_act_filled,stand_act_screenshot}.rs`
  — the seven password fixtures (below)
- `tests/blitz-tests/tests/{stand_act_obstructed,stand_act_scroll,stand_act_transformed,stand_act_spans,stand_act_timer,stand_mcp_tools,stand_replay}.rs`
  — the session holds of a fixture (below)
- `tests/blitz-tests/tests/{stand_keyboard,stand_snapshot_state}.rs` — file-private `boot_fixture`
- `tests/blitz-tests/tests/common/mod.rs` (1-12) · `session_common/mod.rs` (1-80) — `#![allow(dead_code)]`, each
  reader declares the module; `session_common::hold(task, incremental)` the lean-task hold
- `packages/blitz-traits/src/net.rs` (17-32) — `NetProvider::is_noop` defaults to `false`
- `packages/blitz-test-harness/src/harness.rs` (45-62) — `into_config` wraps a supplied provider whose `is_noop()` is
  false in `LoadCounter`
- `examples/seven_guis/src/stand.rs` (58-69) — `stand::options(incremental)`: 800 × 600, `net_provider: None`,
  bundled font, `incremental`
- `scripts/agent-run.sh` (170-215) — `run` selections `stand` (every `stand_*.rs`) · `all` · one file stem; `boot` runs
  `cargo test -p blitz-tests --locked --no-run`
- `.github/scripts/test_blitz_tests_targets.py` (1-30) — pins `autotests` not false and the one `[[test]]` `all`
- `.github/scripts/test_agent_run.py` (141-265) — the contract cases; `run stand` asserted on `stand_*` stems
- `.github/scripts/ci-leg.sh` (48-59) — `stand-repeat` takes every `stand_*.rs`

## Graph impact (from the code-graph query)
- **duplicated definitions** — `symbol` query over the stand-in and fixture names (trace
  `.andromeda/runs/2026-10-19T22-00-26-phase/tree-query-2026-10-19-shared-fixtures.json`, 23 rows): `password_fixture`
  ×7 (`stand_act_diff.rs:270`, `stand_act_filled.rs:47`, `stand_act_screenshot.rs:224`, `stand_diff.rs:687`,
  `stand_mcp_masked.rs:24`, `stand_session_recording.rs:200`, `stand_snapshot_text.rs:333`), `covered_fixture` ×3
  (`stand_act_obstructed.rs:35`, `stand_act_spans.rs:534`, `stand_mcp_tools.rs:398`), `hold_fixture` ×2, `boot_fixture`
  ×3, `fixture` ×2, the net stand-ins ×5 (two upstream). Each is file-private to one test target: no caller crosses a
  target, so a move has zero cross-target blast radius beyond the files listed.
- **name `fixtures`** — existence query (same trace): 0 rows, the module name is free.

## Patterns detected
- **The fixture boot** (`stand_settle.rs:24-26`, `stand_keyboard.rs:434-436`, `stand_snapshot_text.rs:352-354`,
  `stand_snapshot_state.rs:511-513`, `stand_diff.rs:91-96`): `Harness::from_vdom(VirtualDom::new(app),
  stand::options(incremental))` — five file-private helpers, one body.
- **The session hold of a fixture** (`stand_act_obstructed.rs:155-160`, `stand_act_scroll.rs:85-90`,
  `stand_act_transformed.rs:42-50`, `stand_session_recording.rs:208-215`, `stand_replay.rs:199-204`, and inline at
  `stand_act_diff.rs:284`, `stand_act_filled.rs:203`, `stand_act_screenshot.rs:243`, `stand_act_spans.rs:669` and
  `:702`, `stand_act_timer.rs:170` and `:216`, `stand_mcp_tools.rs:532`, `stand_mcp_masked.rs:33`, `stand_settle.rs:171`
  and `:326`): `Session::start(label, || <the fixture boot>).expect(…)` — 16 sites, one shape, each its own label.
- **Password fixtures, four variants:** labelled `Passphrase` in `fx-root` — `stand_act_diff.rs:270` and
  `stand_diff.rs:687`, byte-identical bodies; `stand_snapshot_text.rs:333` the same without the root's id, its checks
  reading only the `fx-secret` line and the whole text (`stand_snapshot_text.rs:362-390`), so the labelled page serves
  it unchanged · named `Secret` by `aria-label` in `fx-root` — `stand_mcp_masked.rs:24` and
  `stand_session_recording.rs:200`, byte-identical bodies · `stand_act_filled.rs:47` (a password and a plain input) and
  `stand_act_screenshot.rs:224` (`fx-input` `Field`, paired with its `text_fixture`) — one reader each.
- **The simple covered page:** `stand_act_spans.rs:534-547` and `stand_mcp_tools.rs:398-411`, identical bodies (one
  button under one `z-index: 5` overlay); `stand_act_obstructed.rs:35` is a different, richer page.
- **Not shared:** the two `restless_fixture`s differ in ids (`stand_settle.rs:310` `fx-root`/`fx-label`;
  `stand_act_spans.rs:552` `fx-restless`/`fx-restless-label`) — one reader each as written.
- **The held network stand-in, three escher copies of one shape:** a `Mutex<Vec<(url, handler)>>` filled by
  `fetch`, nothing else — `stand_settle.rs:182-214` (`held()`, `answer(file, css)`),
  `font_face_sniffed_source.rs:30-66` (`urls()`, `answer(body)`), and `stand_boot.rs:11-21` (urls only, the handler
  dropped). None overrides `is_noop`.

## Conventions to follow
- **A shared module is `tests/{dir}/mod.rs`, `#![allow(dead_code)]`, a `//!` doc naming what it states and that it
  holds no test**, read by `mod {dir};` (`common/mod.rs:1-6`, `session_common/mod.rs:1-8`). Cargo auto-discovers
  `tests/*.rs` and `tests/*/main.rs` as targets; a `mod.rs` in a subdirectory is none, and the target pin reads the
  manifest only (`test_blitz_tests_targets.py:18-26`).
- **No cross-module reference:** a shared module cannot name another (`crate::common` resolves only where every reader
  declares both), so each shared module is self-contained.
- **Assertion messages name a layout mode, a task or fixture and an index, never an id, a url or a typed text**
  (security-plan §Input Validation, obs-plan §3); the stand-in's own failure messages follow it.
- **Both layout modes** for every moved check (`for incremental in [false, true]`), unchanged.

## Mechanism equalities (verified at HEAD)
- **The shared stand-in keeps the load counter:** a provider that does not override `is_noop` reads false
  (`net.rs:27-29`), and `HarnessOptions::into_config` wraps every supplied provider reading false in `LoadCounter`
  (`harness.rs:52-56`) — so `stand_settle`'s `NotSettled { busy: Loads }` while the import is held reads as today;
  both current copies take the same default.
- **`stand_boot` reads the same through a held stand-in:** its checks assert only the list of urls asked
  (`stand_boot.rs:98-127`); a stand-in that keeps the handler records the same urls in the same order. A kept handler
  is a load in flight for the harness's counter, which neither check settles or reads.
- **The fixture boot is the stand's:** every site above passes `stand::options(incremental)` unchanged
  (`stand.rs:58-69`); the shared boot is that one call.

## New files to create
- `tests/blitz-tests/tests/fixtures/mod.rs` — the fixture boot, the session hold of a fixture, the shared fixture pages and the held network stand-in, stated once

## Files to modify
- `tests/blitz-tests/tests/stand_settle.rs` — boot, holds and held stand-in from the shared module
- `tests/blitz-tests/tests/font_face_sniffed_source.rs` — the held stand-in from the shared module
- `tests/blitz-tests/tests/stand_boot.rs` — the held stand-in in place of its recording copy
- `tests/blitz-tests/tests/stand_act_diff.rs` — the labelled password page and the hold
- `tests/blitz-tests/tests/stand_diff.rs` — the labelled password page and the boot
- `tests/blitz-tests/tests/stand_snapshot_text.rs` — the labelled password page and the boot
- `tests/blitz-tests/tests/stand_mcp_masked.rs` — the named password page and the hold
- `tests/blitz-tests/tests/stand_session_recording.rs` — the named password page and the hold
- `tests/blitz-tests/tests/stand_act_spans.rs` — the covered page and the holds
- `tests/blitz-tests/tests/stand_mcp_tools.rs` — the covered page and the hold
- `tests/blitz-tests/tests/stand_act_filled.rs` — the hold
- `tests/blitz-tests/tests/stand_act_screenshot.rs` — the hold
- `tests/blitz-tests/tests/stand_act_obstructed.rs` — the hold
- `tests/blitz-tests/tests/stand_act_scroll.rs` — the hold
- `tests/blitz-tests/tests/stand_act_transformed.rs` — the hold
- `tests/blitz-tests/tests/stand_act_timer.rs` — the holds
- `tests/blitz-tests/tests/stand_replay.rs` — the hold
- `tests/blitz-tests/tests/stand_keyboard.rs` — the boot
- `tests/blitz-tests/tests/stand_snapshot_state.rs` — the boot

## Sweeps
- `Harness::from_vdom` over `tests/blitz-tests/tests/*.rs` (`grep -n -A3`): 27 hits · 22 changed (the fixture boots and
  holds above) · 5 no-change (`dioxus_falsy_boolean_attrs.rs:20` and `stale_node_mapping.rs:56` take their own
  options; `stand_actionable_keys.rs:35`, `stand_id_persistence.rs:193` and `stand_non_lean_tasks.rs:26` boot the
  whole app's `app_root`, not a fixture; `stand_id_edits.rs:73`/`:347` boot an edited app through props — the app,
  not a fixture page)
- `^fn [a-z_]*_fixture` over `tests/blitz-tests/tests/*.rs`: 33 hits · 9 changed (the five password pages and two
  covered pages that become shared, the two `hold_fixture`s) · 24 no-change (one reader each: `stand_act_obstructed`'s
  four pages, `stand_act_scroll`'s three, `stand_act_screenshot`'s two, `stand_act_filled`'s, `stand_act_transformed`'s,
  `stand_act_timer`'s, both `restless_fixture`s (ids differ), `stand_diff`'s `hidden`/`label`, `stand_keyboard`'s
  `activation`/`inert`, `stand_settle`'s `mount`/`hover`, `stand_snapshot_text`'s `file`; the `boot_fixture` helpers
  counted under the first sweep)
- `NetProvider for` over `tests/blitz-tests/tests/*.rs`: 5 hits · 3 changed (`stand_settle`, `font_face_sniffed_source`,
  `stand_boot`) · 2 no-change (upstream's `render_blocking_stylesheet`, `link_rel_attribute`)
- `session_common\|common/mod.rs` over `.github/scripts/*.py`, `ci-leg.sh`, `agent-run.sh`: 0 hits — no script names
  a shared module, so a third one moves no pin

## Open questions
- Where the shared fixtures live — a third module `fixtures/mod.rs`, or the two existing ones (the boot and pages in
  `common`, the hold in `session_common`) → blocks: plan-decision
- What «run through agent-run» asks of `scripts/agent-run.sh` — the existing selections (`run stand` takes every
  fixture-holding `stand_*` check; `font_face_sniffed_source` runs under `run all` or by name) with test-plan §3's
  bootstrap line measured, or a new `run fixtures` selection (then `scripts/agent-run.sh` and
  `.github/scripts/test_agent_run.py` join the files to modify) → blocks: plan-decision
