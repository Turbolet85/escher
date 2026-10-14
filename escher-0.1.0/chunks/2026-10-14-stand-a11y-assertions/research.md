# Codebase Research — 2026-10-14-stand-a11y-assertions

## Scope
- **Depth:** moderate · **Reads:** 19 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition, none binding (no
  bounded boot smoke here); `.claude/rules/testing.md` and `.claude/rules/a11y.md` loaded in full on the stand files read
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's arguments at take-up: tools bare or read whole; the founder's word
  stands, no mutation runs; every founder fork raised as a dialog for the operator to take to him ·
  `inputs#I2` — the founder's answers to F1–F4, relayed by the overseer (P4) · `inputs#I3` — the overseer's record of
  those answers, his Russian labels verbatim (P4) · `inputs#I4` — the operator's F5 accent choice (P4)

## Files inspected
- `tests/blitz-tests/tests/stand_contrast.rs` (full) — `RECORD`, 18 rows of `(LeanTask, id, Reading)` at :42-129;
  `measure` classifies Pass / Fail / ExemptInactive / NoText at :161-184; the record test asserts the reading equals
  the stated row and the two layout modes agree (:194-225); the pixel cross-check samples the left padding (:227-277).
  It asserts the record, not the threshold: a `Fail` row passes the test.
- `tests/blitz-tests/tests/common/mod.rs` (full) — `controls` (:17-46, 18 rows, `crud-list` absent), `tab_order`
  (:50-70, CRUD six stops with `crud-list` third), `focused` (:75-92: snapshot focus equals the tree focus's
  `author_id`), `INPUT_NAMES` (:95-102, six rows), `rendered` (:107-144).
- `tests/blitz-tests/tests/stand_keyboard.rs` (index of items) — 11 tests; `BUTTONS` 10 rows (:36), `TEXT_INPUTS` 4
  rows (:50); the Tab walk test at :118-146 walks `tab_order` from boot only; `the_engine_honours_no_inert` at :597.
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` (index of items) — `element_nodes_carry_their_stable_id`
  (:80), `non_elements_carry_no_id` (:97), `controls_carry_role_and_name` (:129, over `controls` and
  `INPUT_NAMES`), `tab_order_is_unchanged` (:203). Every read is `harness.doc.accessibility_tree()`.
- `examples/seven_guis/src/tasks/crud.rs` (full) — `crud-list` `role: "listbox"`, `tabindex: "0"`, `onkeydown`
  (:57-86), no name attribute; rows `role: "option"`, `aria_selected` (:92-106); `.list-item.selected` is white on
  `#4a6cf7` (:230-233); the CRUD buttons are white on `#4a6cf7` (:261-270), hover `#3a5ce5` (:272-274), and
  `crud-update` / `crud-delete` take that accent once enabled (`btn-off` only while nothing is selected, :139, :154).
- `examples/seven_guis/src/tasks/flight_booker.rs` (CSS :1-241) — `.type-btn` `#444` on `#fafafa` (inactive toggle),
  `.type-btn-active` white on `#4a6cf7` (:170-174), `.date-input.invalid` `#c53030` on `#fff5f5`, `.flight-btn` white on
  `#4a6cf7` with hover `#3a5ce5` and active `#2a4cd3`, `:disabled` white on `#a0aec0`, `.booked-msg` `#276749` on
  `#ebf8ee`.
- `examples/seven_guis/src/tasks/counter.rs`, `timer.rs` (CSS) — `.counter-btn` and `.timer-reset-btn` white on
  `#4a6cf7`, hover `#3a5ce5`, active `#2a4cd3`; `.timer-label` `#444`, `.timer-duration-label` and the displays `#1a1a1a`.
- `examples/seven_guis/src/app.rs` (:160-330) — the shell: `#task-header` `#fff`, `#back-btn` `#333` on `#f0f0f0`, hover
  `#e4e4e4`, `#task-title` `#111`; the shell's CSS selects by id (the task CSS does not).
- `examples/seven_guis/src/tasks/{circle_drawer,cells,temp_converter}.rs` (accent sites) — circle drawer's toolbar and
  dialog buttons are white on `#4a6cf7` (:150-163, :218-231); cells' selected outline (:470) and temp converter's focus
  border (:74-78) use the accent as a non-text colour.
- `.github/scripts/ci-leg.sh` (full) — the `a11y` leg is one line, `cargo test --workspace --locked --test
  accessibility_hidden --test accessibility_roles --test focusability_updates` (:34).
- `.github/workflows/ci.yml` (:246-266) — the `a11y` job: needs the four fast jobs, restore-only cache, bounded install,
  the leg, a failure-only log upload.
- `.github/scripts/test_ci_workflows.py` (a11y sites) — `RESTORE_ONLY_JOBS = {"a11y"}` (:44), the job-name pin (:205),
  and `LegScriptTest`'s row pinning the leg's exact cargo argument list (:289).
- `packages/blitz-shell/src/accessibility.rs` (full) — `AccessibilityState::update_tree(&dyn Document)` hands the adapter
  `doc.accessibility_tree()` inside `adapter.update_if_active(…)`: the platform tree is exactly the `Document` trait's
  output for the shell's document, built only while an AT is active.
- `packages/blitz-shell/src/window.rs` (accessibility sites :29-30, :111-113, :150-151, :377-381, :521-523, :585-587) —
  the state is built from a live winit window; no constructor without one.
- `packages/blitz-shell/Cargo.toml`, `packages/dioxus-native/Cargo.toml` (`[features]`) — `dioxus-native/accessibility`
  = `blitz-shell/accessibility` + `blitz-dom/accessibility` + `dioxus-native-dom/accessibility`; `blitz-shell/accessibility`
  pulls `accesskit` and `accesskit_xplat`. seven_guis names neither (manifest :22, :31, :41).
- `packages/blitz-dom/src/accessibility.rs` (focus site) — the tree's `focus` is the engine's focused node or the
  `Window` (:62); no `aria-activedescendant` read anywhere in `packages/` or `examples/seven_guis/src` (0 hits).
- `packages/blitz-dom/src/events/driver.rs` (:180-206) and `packages/blitz-test-harness/src/input.rs` (:104-136) — a
  `PointerDown` sets the hover node then `active_node()`, a `PointerUp` `unactive_node()`; `mouse_down_at` and
  `move_mouse_to` dispatch and pump, so `:active` and `:hover` are reachable through the harness.

## Graph impact
- **`controls`** (`tests/blitz-tests/tests/common/mod.rs:17`) — referenced from 8 stand files: `stand_accessibility_ids`
  3 · `stand_act_ids` 3 · `stand_act_obstructed` 1 · `stand_contrast` 4 · `stand_keyboard` 2 · `stand_session_fresh` 3 ·
  `stand_session_ids` 3 · `stand_snapshot` 2 (trace `tree-query-2026-10-14-stand-a11y-assertions.json`, query 1). A row
  added to `controls(Crud)` moves every one of them — the driver-action and session checks click or address each row.
- **`tab_order`** — 2 readers: `stand_accessibility_ids` 2 · `stand_keyboard` 5 (same query).
- **`INPUT_NAMES`** — 3 readers: `stand_accessibility_ids`, `stand_session_fresh`, `stand_snapshot` (same query).
- **`AccessibilityState::update_tree`** — 2 callers in `window.rs` (:379 the poll refresh, :523
  `build_accessibility_tree`), read from the file; both need a live window.

## Patterns detected
- **Stated record, asserted row for row** (`stand_contrast.rs:42-129`, `:194-225`): the table is the expectation, its
  row count asserted, both layout modes compared. An SC 1.4.3 assertion is a second predicate over the same rows (every
  non-exempt text-bearing row `Pass`), the record kept beside it.
- **Failure messages name mode, task and index** (`stand_contrast.rs:167`, `stand_keyboard.rs:21-22`), never an id or
  screen text.
- **Shared tables in `common`** (`common/mod.rs:1-4`): a new expectation is a table there, read by `mod common;`.
- **Attribute-only stand markup** (`crud.rs:57-59`, the flight dates' `aria_label`): a name is one attribute.

## Conventions to follow
- **Both layout modes**: `for incremental in [false, true]` (`stand_contrast.rs:207`; arch §Design Philosophy).
- **Stand boot**: `common::boot(task, incremental)` → `seven_guis::stand::boot` (`common/mod.rs:146-148`).
- **One target per file, no manifest line** (`.github/scripts/test_blitz_tests_targets.py`): a leg names files by
  `--test {name}` in `ci-leg.sh`.
- **The leg list is pinned** (`test_ci_workflows.py:289`): a change to `ci-leg.sh:34` moves that row in the same commit.

## New files to create
- none

## Files to modify
- `examples/seven_guis/src/tasks/counter.rs` — the accent re-colour (rest, hover, active)
- `examples/seven_guis/src/tasks/flight_booker.rs` — the accent re-colour on the active toggle and the Book button, its hover and active
- `examples/seven_guis/src/tasks/timer.rs` — the accent re-colour on Reset, its hover and active
- `examples/seven_guis/src/tasks/crud.rs` — the accent re-colour on the buttons and the selected row; a name on `crud-list`
- `examples/seven_guis/src/tasks/circle_drawer.rs` — the accent re-colour on its white-text buttons, if the token moves stand-wide
- `examples/seven_guis/src/tasks/cells.rs` — the accent's outline, if the token moves stand-wide
- `examples/seven_guis/src/tasks/temp_converter.rs` — the accent's focus border, if the token moves stand-wide
- `tests/blitz-tests/tests/common/mod.rs` — the stated tables the new assertions read
- `tests/blitz-tests/tests/stand_contrast.rs` — the record re-stated, the SC 1.4.3 pass predicate, the widened rows and states
- `tests/blitz-tests/tests/stand_keyboard.rs` — reach and order in the non-boot enabled states
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` — the listbox's name; the tree read through the trait the shell calls
- `examples/seven_guis/tests/host_log.rs` — the list's name joins the name needles
- `examples/seven_guis/tests/mcp_log.rs` — the list's name joins the name needles
- `.github/scripts/ci-leg.sh` — the `a11y` leg's test list
- `.github/scripts/test_ci_workflows.py` — the leg's pinned argument list

## Sweeps
- `#4a6cf7|#3a5ce5|#2a4cd3` (case-insensitive) over `examples/seven_guis/src`: 25 hits in 7 task files — counter 3 ·
  flight_booker 5 accent + `#c53030` + `#276749` · timer 5 · crud 3 · circle_drawer 5 · cells 1 · temp_converter 1
  (`grep -rn -i -o` over the directory, counted by `uniq -c`); app.rs 0. The same colour as `rgba(74, 108, 247, …)`: 3
  hits — `temp_converter.rs:78` (0.15), `circle_drawer.rs:185` (0.15), `:192` (0.35) (`grep -rn -i -E '74, ?108, ?247'`,
  run at P5 after the entry-5 control read one hit left; the P3 sweep searched hex only).
- `4a6cf7|3a5ce5|2a4cd3` over `tests/`, `examples/seven_guis/tests`, `packages` (`*.rs`): 8 hits — `stand_contrast.rs` 5
  (the fail rows, changed) · `packages/blitz-test-harness/src/contrast.rs` 3 (the reader's own unit fixture, no change:
  it tests the formula on a fixed pair, not the stand).
- `crud-list` over `tests/` and `examples/seven_guis/tests`: 14 files; none pins its name `""` (`ListBox` over the same
  files: 0 stand hits); `host_log.rs:56` and `mcp_log.rs:70` take the rows' text as name needles — no change, the list's
  name joins the needle census by the plan.
- `inert` over `examples/seven_guis/src`: 0 hits — no stand task renders `inert`.
- `aria-activedescendant|activedescendant` over `packages/` and `examples/seven_guis/src`: 0 hits.
- `2049|2,202|2202` (the CRUD text byte figures) over the blitz-tests and seven_guis test files: 0 hits — no check pins them.

## Measurements at take-up
- **Merge gating on the fork** (`gh api repos/Turbolet85/escher/branches/build%2Fescher-0.1.0/protection` → 404 "Branch
  not protected"; `…/branches/main/protection` → 404; `…/rulesets` → `[]`; `gh pr list -R Turbolet85/escher --state all`
  → none; default branch `build/escher-0.1.0`, read 2026-10-14): nothing blocks a merge on the fork today. Work lands by
  push to the build branch, and a red CI job reads as the run's verdict, which phase Setup 5a and the wrap read.
- **Declared colour pairs** (WCAG 2.x over the declared hex, a scratchpad script; the record's reader is
  `Harness::text_contrast`): white on `#4a6cf7` 4.39 · on `#3a5ce5` 5.47 · on `#2a4cd3` 6.83 · on `#1f3fc1` 8.27; the
  invalid date `#c53030` on `#fff5f5` 5.11; the success message `#276749` on `#ebf8ee` 6.15; the inactive toggle `#444`
  on `#fafafa` 9.33; `#back-btn` hover `#333` on `#e4e4e4` 9.94; `#task-title` `#111` on `#fff` 18.88; `.timer-label`
  `#444` on `#fff` 9.74; CRUD labels `#333` on `#f5f5f5` 11.59; the disabled Book white on `#a0aec0` 2.26 (inactive,
  exempt). Every declared text pair off the accent passes 4.5; every white-on-accent pair at rest fails.

## Open questions
- Does `Harness::text_contrast` read the `:hover` and `:active` backgrounds after `move_mouse_to` / `mouse_down_at`? The
  mechanism is read (`driver.rs:182-190`, `input.rs:112-136`); the equality is not measured — no probe was written at
  P3 → blocks: implementation-scope (the plan's first step measures it; a no reading keeps those two states recorded,
  not asserted).
- The founder's forks — the stand's platform adapter, the tree's current option, `inert`, the leg as a merge gate —
  and the operator's colour choice → blocks: plan-decision (P4 asks them; an engine branch adds
  `packages/blitz-dom/src/accessibility.rs` or the focusability path to the lists above).
