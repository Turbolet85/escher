# Codebase Research — 2026-10-14-stand-requirement-sweep

## Scope
- **Depth:** deep · **Reads:** 19 · **Globs/Greps:** 24
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (the gate's
  exit-124 reading; no boot smoke in this chunk, so it applies to nothing planned) · `.claude/rules/testing.md` —
  read in full, 7 Session Additions applied (no deleting key outside `stand_act_keys`; a table-pinning check is searched
  for before a table moves; `--no-fail-fast` for red counts)
- **Platform issues consulted:** none — no runner-only bullet (CI#38632479079 green) and no CI-reading entry outside the
  operator leg; C9's macOS reading is a check this chunk adds, read at the wrap from the run that carries it
- **External inputs:** `inputs#I1` — the operator's directive at take-up: the founder's word stands, no mutation runs;
  a founder's fork is raised as a dialog for the operator to take to him

## Files inspected
- `escher-0.1.0/verification-matrix.json` (through `matrix.py show --id`, v010-01…17) — every cap's `ref`, acceptance
  and surface: v010-01 · 02 · 05 · 16 are witnessed by in-process readers (`DioxusDocument::element_ids`, `snapshot`),
  v010-03 · 06 · 09 · 10 · 11 · 17 through `Session::run`, v010-04 · 12 · 14 through the `escher-session` binary,
  v010-07 · 13 on all three surfaces, v010-08 in process across two child runs
- `tests/blitz-tests/tests/stand_*.rs` and `examples/seven_guis/tests/*.rs` (per-test probe,
  `$TMPDIR/{scratchpad}/per_test.py` over HEAD 19756f2d: 192 `#[test]` functions, 38 `stand_*` files + 9 seven_guis
  targets) — which task each test boots and which surface it drives
- `tests/blitz-tests/tests/stand_actionable_keys.rs` (full) — `the_non_lean_tasks_are_measured` reaches the three
  non-lean tasks through Home (`app_root`, a card click after `scroll_into_view`), pins `UNKEYED` 2 · 3 · 676, both
  modes; the boot path a non-lean check has
- `examples/seven_guis/src/app.rs` (full) — Home's seven cards are `button` elements (`app.rs:143`), keyed
  `task-card-{slug}`; `task_in_shell` mounts any `Task`
- `examples/seven_guis/src/tasks/temp_converter.rs` (full) — two unkeyed `input`s (`:12`, `:24`) with `oninput`
- `examples/seven_guis/src/tasks/circle_drawer.rs` (full) — Undo · Redo `button`s (`:28`, `:38`), the canvas `div`
  with `onclick` and `ondoubleclick` and no role (`:50-55`), circles `pointer-events: none` with no listener (`:91`), the
  dialog's range `input` (`:102`) and Close `button` (`:121`) only while open; the rule `.cd-dialog
  input[type="range"] { width: 240px; }` at `:215`
- `examples/seven_guis/src/tasks/cells.rs` (`:340-408`) — 26 × 26 data cells, each a `div` keyed `{r}-{c}` with
  `onclick` and no role (`:373-376`); the selected cell holds an `input` with `oninput` · `onblur` · `onkeydown`
  (`:382-395`); headers carry no listener
- `examples/seven_guis/src/tasks/{counter,flight_booker,timer,crud}.rs` (handler lines) — every lean-task click
  handler sits on a `button` or a role-bearing element (`crud` rows `role="option"`, the list `role="listbox"`)
- `packages/escher-driver/src/execute.rs` (`:330-390`) — `action_point`: `disabled`, then `in_view` (geometry through
  `BaseDocument::visible_region`), then `covered` (a hit at the centre); `crop` through `visible_region`
- `packages/blitz-dom/src/document.rs` (`:1860-1890`, `:1777`) — `show_scrollbars` stamps `Instant::now()` under
  `cfg!(feature = "scrollbars")` (`:1879`), `scrollbars_animating` reads `last.elapsed()` against
  `FADE_DELAY + FADE_DURATION` (`:1886-1889`); `tracing::info!("Focussed node {focus_node_id}")` at `:1777`
- `packages/blitz-dom/src/events/pointer.rs` (`:336-366`, `:476-495`, `:549-560`, `:792`, `:797`) — a click in a text
  input places the caret through `driver.move_to_point` on the editor's own layout (`text_input_data.editor`), the real
  characters; the href `warn` (`:792`) and the attributes `info` (`:797`)
- `packages/blitz-paint/src/render.rs` (`:912`, `:975`) — a password paints one U+2022 per character in its own layout,
  apart from the editor's
- `packages/blitz-dom/src/mutator.rs` (`:356`, `:377`, `:421`, `:499`, `:506`) and `layout/construct.rs` (`:928-946`) —
  `clear_attribute` re-seeds on `value` and re-sanitizes on `min`/`max`/`step`; `flush_range_input` clears
  `RangeInputData` off an element no longer `type="range"`
- `tests/blitz-tests/tests/range_input_interaction.rs` (greps) — 2 `set_attribute` sites, 0 `clear_attribute` /
  `remove_attribute`, no `type` write
- `tests/blitz-tests/tests/inline_fragment_rects.rs` (greps) — three writing modes (`:127`), 0 `scroll` tokens
- `examples/seven_guis/tests/cli_commands.rs` (`:13`, `:541-543`) — the file is `#![cfg(unix)]`; the 94/95 bound test is
  `#[cfg(target_os = "linux")]`
- `packages/escher-driver/src/error.rs` (`:61`, `:64`, `:310`) — the two fixed messages, 94 on Linux and 90 elsewhere,
  and `let bound = if cfg!(target_os = "linux") { 94 } else { 90 }`
- `.github/workflows/ci.yml` (`:300-398`) — the matrix job runs `cargo test --all --tests --target {t} --locked` on
  macOS (`:318-331`, `:392`); every `seven_guis` test target runs there
- `examples/seven_guis/tests/{host_log,mcp_log}.rs` (assert lines) — both assert the needle counts at 0, one `command`
  line per call and `lines >= 1`; neither pins a total line or byte count
- Feature manifests (`writing-mode`, `scrollbars` over every `Cargo.toml`) — `wpt/runner/Cargo.toml:15` turns
  `blitz-dom/writing-mode` on, so a workspace build compiles the second bounds body; `tests/blitz-tests/Cargo.toml:20`
  turns `blitz-paint/scrollbars` on, so blitz-tests paints overlay scrollbars in both builds

## Graph impact (from the code-graph query)
- **`absolute_position`** — 14 call rows (trace `tree-query-2026-10-14-stand-requirement-sweep.json`, editor lines):
  `handle_dom_event` `packages/blitz-dom/src/events/mod.rs:109` · `Node::focus` `packages/blitz-dom/src/node/node.rs:732`
  (the IME cursor area) · its own recursion `node.rs:1689` · `synthetic_click_event_data` `node.rs:1929` ·
  `scroll_into_view` `packages/blitz-dom/src/scrolling.rs:720` · `Harness::layout_rect_of`
  `packages/blitz-test-harness/src/inspect.rs:60` (and through it `layout_rect` `:51` and `center_of` `:72`) ·
  `write_node` `inspect.rs:142` (`dom_string`) · tests `anonymous_block_restyle.rs:100`,
  `scrolled_box_absolute_position.rs:44,57,74`, `stale_dirty_descendants.rs:81,90,100` — C14's reader list holds, with
  `dom_string` and two upstream tests added; this chunk changes the function only on a founder's "change"

## Patterns detected
- **A requirement witnessed in process, not by a driver call** (`stand_element_ids.rs`, `stand_id_persistence.rs`,
  `stand_snapshot_state.rs`, `stand_id_edits.rs`): v010-01 · 02 · 05 · 16 read `DioxusDocument` directly; their driver
  readings exist only for some tasks (`stand_session_ids` on all four for ids across a command, `stand_act_states` on
  two). This is the sweep's gap class: "by an agent" (intent §Principles) against an in-process witness.
- **A requirement proven on one task** (matrix refs): v010-10 on the Timer (`stand_act_timer`, the only task with a
  delayed update), v010-11's `off-screen` on CRUD and `covered` on a fixture, v010-17 on flight booker and CRUD. By
  construction, a requirement whose case exists on one task only is proven there, and the sweep records that.
- **Non-lean tasks reached through Home** (`stand_actionable_keys.rs:29-37`, `:146-172`): `Harness::from_vdom(app_root)`
  with `stand::options`, a card `scroll_into_view` then `click`; no driver session boots them (`LeanTask` is four, and
  `escher-session start` takes four).
- **A pinned measured limit** (`scrolled_box_absolute_position.rs`, `hit_clipped_at_scrolling_box.rs`'s last test,
  `stand_actionable_keys::the_non_lean_tasks_are_measured`): a reading left as it reads is pinned by a test that turns
  red by design when the reading is fixed.
- **An in-file fixture beside the stand** (`stand_act_obstructed.rs`, `stand_act_scroll.rs`, `stand_snapshot_state.rs`):
  booted with the stand's options; used where the lean tasks lack the case.

## Conventions to follow
- **One behaviour per file, one target each, auto-discovered** (`tests/blitz-tests/Cargo.toml`, test-plan §2;
  `BlitzTestsTargetsTest`): no `[[test]]` table added.
- **Both layout modes** (`for incremental in [false, true]` in every stand check, e.g. `stand_actionable_keys.rs:87`).
- **A failure names a task, a mode and an index, never an id or screen text** (test-plan §4, testing.md).
- **Stand edits by attribute only** (layout-templates §Surface: desktop-native; `crud.rs` role lines, `app.rs:144`).
- **No mutation run in this chunk** (inputs#I1): the test-plan §4 mutation-control pattern (`evidence/controls.md`) is
  not run; each new check's witness is its assertion of the fixture's condition first (testing.md, "Assert first that
  the fixture produces the condition under test").

## New files to create
- `tests/blitz-tests/tests/stand_requirement_sweep.rs` — the sweep check through `Session::run` over the four lean tasks, with the requirement-to-witness table (form decided at P4)
- `tests/blitz-tests/tests/stand_non_lean_tasks.rs` — the three non-lean tasks booted through Home: the C3 reading (elements that react to a click and state no role) and the C12 dialog rule
- `tests/blitz-tests/tests/password_caret_click.rs` — the C11 measurement: a click inside a filled password field and the caret index it places

## Files to modify
- `tests/blitz-tests/tests/common/mod.rs` — the sweep's shared table, if P4 places it there
- `tests/blitz-tests/tests/stand_actionable_keys.rs` — the non-lean pin moves to 0 for each task keyed (C2)
- `examples/seven_guis/src/tasks/temp_converter.rs` — author ids on the two inputs (C2)
- `examples/seven_guis/src/tasks/circle_drawer.rs` — author ids on Undo, Redo, the canvas and the dialog's two controls (C2)
- `examples/seven_guis/src/tasks/cells.rs` — author ids on the 676 data cells and the cell input (C2), and the selected cell's state if C8 is taken
- `tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs` — C6(1) the transformed-and-scrolled box; C4's unmeasured hits at an image's, a sub-document's and a text input's own box
- `tests/blitz-tests/tests/scrolled_box_client_rect.rs` — C6(2) a scrolled box under a vertical writing mode, read under both builds
- `tests/blitz-tests/tests/stand_act_obstructed.rs` — C5 a driver `click` on a target outside a one-axis-clipped box along its visible axis
- `tests/blitz-tests/tests/range_input_interaction.rs` — C13 attribute removal through `clear_attribute` and a `type` change
- `examples/seven_guis/tests/cli_commands.rs` — C9 the 90/91-byte bound off Linux, read on the macOS matrix leg
- `packages/blitz-dom/src/node/node.rs` — only on a founder's "change": C4 (hit stop), C14 (`absolute_position`), C15 (focusability)
- `packages/blitz-dom/src/scrolling.rs` — only on a founder's "change": C5 (`visible_region`) or C14 (`scroll_into_view`'s add-back)
- `packages/blitz-dom/src/accessibility.rs` — only on a founder's "change": C7 (`aria-checked`)
- `packages/blitz-dom/src/document.rs` — only on a founder's "change": C10 (the fade clock)
- `packages/blitz-dom/src/events/pointer.rs` — only on a founder's "change": C11 (the caret by the masked text)
- `packages/escher-driver/src/schema.rs` — only where a measured limit is stated in a verb's help (C5), or on a founder's "change" to C3

## Sweeps
- `contain` over `examples/seven_guis/src` (excluding `contains`): 0 hits · 0 changed · 0 no-change (C4 re-derived:
  `command grep -rn 'contain' examples/seven_guis/src | command grep -v contains`)
- `overflow[-a-z]*: *clip` over `examples/seven_guis/src`: 0 hits (C5 re-derived)
- `aria-checked|aria_checked` over `examples/seven_guis/src` and `packages/blitz-dom/src`: 0 hits (C7 re-derived)
- `clear_attribute|remove_attribute` over `tests/blitz-tests/tests/range_input_interaction.rs`: 0 hits; `set_attribute`
  2 hits (C13 re-derived)
- `scroll` over `tests/blitz-tests/tests/inline_fragment_rects.rs`: 0 hits (C6(2) re-derived)
- `onclick|onkeydown|onmousedown` over the four lean task files: 9 handler sites, every one on a `button` or a
  role-bearing element (C3 lean-task half re-derived)

## Measured facts beside the scope (re-derived at HEAD 19756f2d)
- C2: `UNKEYED` reads `[("Temperature Converter", 2), ("Circle Drawer", 3), ("Cells", 676)]` at
  `stand_actionable_keys.rs:152-156`, green at CI#38632479079 — the pin stands; 676 = 26 × 26 data cells
  (`cells.rs:340-341`).
- C3: on the lean tasks no element reacts to a click without a role (sweep above). Candidates by code: Circle Drawer's
  canvas (`circle_drawer.rs:50-55`) and Cells' 676 data cells (`cells.rs:373-376`). Home's cards are `button`s
  (`app.rs:143`), so the CARRY's "Home's cards" is not a candidate. The run-time reading is owed to this chunk.
- C5: `action_point` reads `in_view` from `visible_region` (geometry) and `covered` from a hit, in that order
  (`execute.rs:351-380`), so a target outside a one-axis-clipped box along its visible axis reads in view and is not
  reached by the hit — the CARRY's hypothesis (`covered`, not `off-screen`) follows from the code; the driver reading is
  owed.
- C9: the 90 is `error.rs:310`'s `else` arm; the macOS matrix leg runs `cli_commands` (`ci.yml:326`, `:392`), so a
  `#[cfg(not(target_os = "linux"))]` test there is read on that leg with no workflow edit.
- C10: the fade reads `Instant::now()` / `elapsed()` (`document.rs:1879`, `:1886-1889`), not the harness clock; the
  CARRY's `:1829-1848` moved to `:1860-1890`.
- C11: the click's caret comes from the editor's layout of the real text (`pointer.rs:549-560`), while paint lays out
  bullets (`render.rs:912`, `:975`) — the two layouts differ by construction; whether the caret index then differs for
  two values of equal length is the measurement owed.
- C12: unmeasured — Circle Drawer is reached only through Home and its dialog only after a canvas click and a double
  click; the rule's width (240px) is the reading.
- C16: `focussed node` at `document.rs:1777` and the href sites at `pointer.rs:792` / `:797` stand as obs-plan §6
  cites them; `host_log` and `mcp_log` pin no total line count (`lines >= 1`), so the dated 9 / 10, 4 / 5 and 742 / 860
  readings can have moved green; the re-read is a capture under both builds.
- Cells' snapshot: 702 cells of text plus the shell — the 10,000-byte `SNAPSHOT_TEXT_BUDGET` is held on the lean screens
  only (`stand_snapshot_text::every_screen_fits_the_budget`); Cells is outside the stand and is not read whole here.

## Open questions
- The sweep's form: a driver check over the four lean tasks with its requirement table, or a record of the standing
  checks with gaps filled where they lie → blocks: plan-decision
- C2 / C8: which non-lean tasks are keyed, and whether Cells states its selected cell → blocks: plan-decision
- The founder's forks C3 · C4 · C5 · C7 · C10 · C11 · C14 · C15 (each an engine change for every document or a new
  reading) → blocks: plan-decision (raised as a dialog for the operator to take to the founder, inputs#I1)
