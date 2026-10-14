# Scope — 2026-10-14-stand-requirement-sweep

**Working entry (the intent anchor, verbatim title):** Stand requirement sweep — every 0.1.0 capability proven
headless by an agent across the stand tasks (per intent §Principles)

**Version:** escher-0.1.0 · Epoch 6 — Polish & ship · taken up 2026-10-14

## What this chunk builds

- **The sweep.** Every 0.1.0 capability that the stand proves is proven headless, by an agent, across the stand tasks:
  the four lean 7GUIs tasks (counter, flight booker, timer, CRUD) and, where they lack a case, a minimal fixture beside
  the stand, the fixtures counted with the tasks (intent §Principles, "The stand is the proof"; C1 below).
- "By an agent" reads as through the driver's own surface (`escher_driver::Session::run` in process, the
  `escher-session` command line or its `mcp` role), not through the test harness's selector or coordinate API, which
  intent §5 names as the OBSERVED state. (Closed at P3: architecture §Standard Contracts → Driver session names these
  three as the only ways a call runs; research.md §Patterns detected finds v010-01 · 02 · 05 · 16 witnessed by
  in-process readers only — the sweep's gap class.)
- The sweep's product is a record that maps every requirement of `escher-0.1.0/requirements.md` to the stand task(s)
  or fixture(s) and the check that proves it there, and fills each gap the record finds with a check. Its form (one
  check file with its table, a table in the shared module, a committed record) is a P4 fork. (Closed at P3: test-plan
  §3 → `run {selection}` asks that every requirement name the check and the selection that runs it; research.md's open
  question 1.)
- The requirements in reach are v010-01…v010-14, v010-16 and v010-17, every one `verified` in
  `verification-matrix.json` at take-up (16/17). v010-15 (Cold-agent test) is out of scope: the markerless entry
  "Cold-agent test" (working-route.md:100) owns it, unclaimed in the matrix and ordered after this entry. (Closed at P3:
  `matrix.py coverage` reads verified 16/17, unclaimed `v010-15` only.)
- The sweep re-proves; it does not re-claim. (Closed at P3: the unclaimed pool is `[v010-15]` alone, so no cap remains
  for this chunk to claim; a requirement whose witness widens records it in its notes at the wrap, never by a re-claim.)
- **The stand tasks are the lean four.** [premise-corrected: arch §Standard Contracts → Headless stand limits
  `LeanTask` and `escher-session start` to counter, flight booker, timer, CRUD; the three others are reached only
  through Home in process (`stand_actionable_keys.rs:29-37`), so the sweep runs over the four, and the non-lean work
  of C2 · C3 · C8 · C12 runs on those three through Home.]

## Folded freight (the 16 CARRYs at working-route.md:96, in entry order)

Every CARRY is folded as a hypothesis: each named artifact, line and count is re-verified at P3 before it shapes the plan.
Those the entry marks "decided at this entry's promotion" are P4 forks; those the entry marks as a founder's fork are
raised to the operator as a dialog (inputs#I1).

- **C1 · fixtures beside the stand** (the founder's ruling, 2026-10-06): where the 7GUIs tasks lack a case, a minimal
  fixture beside the stand proves it, and the sweep counts those fixtures with the tasks.
- **C2 · keys for the three non-lean tasks** (from 2026-10-06-id-stability-across-code-edits): every actionable element
  must read an author key; `DioxusDocument::unkeyed_actionable()` reads Temperature Converter 2 · Circle Drawer 3 ·
  Cells 676, both layout modes, pinned by `stand_actionable_keys.rs::the_non_lean_tasks_are_measured`; the four lean
  tasks and Home read 0; keying a task drops its pinned count to 0. **Whether and when the three are keyed is decided at
  this entry's promotion (P4 fork).**
- **C3 · a row that can be clicked reads no role that says so** (from the first live agent trial, 2026-10-10): CRUD rows
  now read `ListBoxOption` with `selected`/`unselected`; what stays is the general reading: an element that reacts to a
  click and states no role reads `GenericContainer` with nothing saying it reacts. **This entry measures first** which
  stand elements still read so; **whether a snapshot then says an element reacts to a click, or the reading rides to a
  later version, is asked of the founder with that measurement.** [premise-corrected: the candidates are Circle
  Drawer's canvas (`circle_drawer.rs:50-55`) and Cells' 676 data cells (`cells.rs:373-376`); Home's cards are `button`s
  (`app.rs:143`) and no lean-task element reacts to a click without a role (research.md §Sweeps).]
- **C4 · what a hit still reaches** (from 2026-10-10-scrolling-box-bounds-and-hit): the hit stops at the padding box of
  an `overflow`-clipping box and not at a `contain: paint` box with `overflow: visible`; such content is hit through and
  a control lying where it extends reads `covered` (pinned by the last test of `hit_clipped_at_scrolling_box`). Not
  measured: a hit at an image's, a sub-document's and a text input's own box. No stand file sets `contain`. **Whether
  the stop widens to paint's whole predicate is decided at this entry's promotion; a change there is an engine change
  for every document in upstream-owned merge surface (`packages/blitz-dom/src/node/node.rs`), a founder's fork.**
- **C5 · a box clipped on one axis reads in view and is not hit** (same chunk): `overflow-x: clip` beside
  `overflow-y: visible`; the hit stops at the whole padding box while `BaseDocument::visible_region` narrows the clipped
  axis only. No stand file sets `overflow: clip`. **Which of the two readers moves, or neither, is decided at this
  entry's promotion** (either reader moving is an engine change for every document).
- **C6 · two paths that chunk edited hold no check** (same chunk; the operator's word: "they are checks owed on code
  this chunk changed, and the sweep comes first"): (1) a box both transformed and scrolled, in the hit walk; (2) a
  scrolled box under a vertical writing mode, the bounds reader's second body under blitz-dom's `writing-mode` feature.
  Each is a fixture check owed in both layout modes, the second under the workspace build.
- **C7 · an ARIA checkbox's and a switch's reading** (from 2026-10-10-snapshot-states): the builder reads
  `aria-selected`, `aria-pressed`, `aria-invalid` and not `aria-checked`; a `role="checkbox"`/`role="switch"` element
  reads its role and no checked state (pinned by the fourth test of `accessibility_states` and a snapshot unit test). No
  stand task states `aria-checked`. **Whether it is read, and as `checked` or as `pressed`, is decided at this entry's
  promotion**: a new attribute read by the builder is an engine change for every document.
- **C8 · whether Cells states its selected cell** (from 2026-10-10-snapshot-states): **whether its selected cell states
  `aria-selected`, and under which role, is decided with its keying (C2) at this entry's promotion.**
- **C9 · the state directory's bound off Linux** (from 2026-10-12-self-description): `StateDirTooLong` states 94 bytes
  on Linux and 90 elsewhere; the 90 is read from the code and not measured; a check at 90 and 91 on the macOS leg is
  owed.
- **C10 · a shot within 0.7 s of a scroll can differ** (from 2026-10-12-headless-screenshot): the overlay scrollbar's
  fade reads the wall clock (`packages/blitz-dom/src/document.rs:1879-1889` at HEAD; the CARRY's `:1829-1848` moved);
  the `screenshot` help states the limit and no check shoots inside the window; **moving the fade onto the harness clock
  is an engine change for every document, a founder's fork, not asked there.**
- **C11 · a click inside a filled password field places the caret by the real characters** (from
  2026-10-12-headless-screenshot): no check measures it; measure it on a fixture beside the stand; **a fix in the
  pointer path is an engine change for every document, a founder's fork.**
- **C12 · Circle Drawer's dialog rule** (from 2026-10-13-upstream-sync-polish-and-ship): upstream #1186 builds Dioxus
  attribute names in the empty namespace, so `.cd-dialog input[type="range"]` (`circle_drawer.rs:215`) is predicted to
  apply for the first time.
- **C13 · two range paths that chunk edited hold no check** (from 2026-10-13-range-input-interaction): (1) removing a
  range input's `value` attribute re-seeds its current value and removing `min`/`max`/`step` re-sanitizes it
  (`clear_attribute` in `mutator.rs`); (2) construction clears `RangeInputData` when an element no longer reads
  `type="range"`. Each is a fixture check owed in both layout modes.
- **C14 · the second position reader reads a scrolled box shifted** (from 2026-10-10-scrolling-box-bounds-and-hit;
  moved here by the founder's answer to 2026-10-13-stand-keyboard-harness fork F4, «Отложить (Рекомендую)»):
  `Node::absolute_position` answers a self-scrolled box's position less its own scroll offset (pinned by
  `scrolled_box_absolute_position`, red by design when fixed — rewritten then, not deleted); readers built on it are
  named in the CARRY (`layout_rect`, `layout_rect_of`, `center_of`, `synthetic_click_event_data`, the IME cursor area,
  `handle_dom_event`'s target position, `scroll_into_view`'s add-back), and `dom_string`'s `write_node` beside them
  (research.md §Graph impact). **The fix is an engine change for every document in upstream-owned merge surface, a
  founder's fork.**
- **C15 · narrow `disabled`'s focusability reading to the form controls** (from 2026-10-13-stand-keyboard-harness):
  presence on any element makes it unfocusable (pinned by `stand_keyboard::a_disabled_attribute_of_any_value_is_skipped`);
  **narrowing to `button`, `input`, `select`, `textarea` is an engine change for every document, a founder's fork.**
- **C16 · two log readings a click's focus may move, not re-measured** (from 2026-10-13-stand-keyboard-harness): the
  engine's `focussed node` info event may fire on more steps than the dated `host_log` and `mcp_log` line counts in
  obs-plan §2, §6 and §8; Enter on a focused `a href` now reaches pointer.rs's href log site from the keyboard; re-read
  both on a stand capture. The CARRY itself says: "move it if it reads wrong".

## Mechanism claims carried in the freight (closed at P3 against HEAD 19756f2d; marker text kept verbatim)

- C2: "measured at that chunk": the unkeyed counts 2 · 3 · 676, both layout modes. (Spot-checked: the pin
  `stand_actionable_keys.rs:152-156` stands, green at CI#38632479079.)
- C3: "measured through the `escher-session` binary at that chunk": the CRUD list's line opens `ListBox`, each row
  `ListBoxOption … unselected`; "which stand elements still read so is not measured". (Spot-checked: `crud.rs` states
  `role: "listbox"` and `role: "option"`; the run-time reading on the candidates is owed.)
- C4: "measured and pinned by the last test of `hit_clipped_at_scrolling_box`, both layout modes, both builds"; "0 hits
  for the property in `examples/seven_guis`, read at that wrap". (Re-derived: 0 hits at HEAD.)
- C5: "hypothesis: a driver `click` on such a target is refused `covered`, not `off-screen` (read from the two readings,
  not measured on a driver call)". (Re-derived from code: `action_point` reads `in_view` by geometry through
  `visible_region`, then `covered` by a hit, `execute.rs:351-380` — the hypothesis follows; the driver reading is owed.)
- C6: "the bounds reader's second body ... leaves the node's own scroll offset out on its vertical-chain path too" and
  "the one blitz-tests file that does, `inline_fragment_rects`, scrolls nothing (measured at that chunk's wrap)".
  (Re-derived: `inline_fragment_rects` holds three writing modes and 0 `scroll` tokens; `wpt/runner` turns
  `writing-mode` on for a workspace build. The scroll-offset claim is the check's to read.)
- C7: "AccessKit holds one `toggled` property for pressed and checked ... (read from AccessKit 0.25.1 at that chunk's
  research, not built)". (Spot-checked: `Cargo.lock` pins `accesskit` 0.25.1; 0 `aria-checked` reads at HEAD.)
- C10: "the overlay scrollbar's fade reads the wall clock". (Verified: `Instant::now()` / `elapsed()` at
  `document.rs:1879`, `:1886-1889`; blitz-tests builds with `scrollbars` on.)
- C11: "hypothesis: a click-placed caret can say something of the value's glyph widths". (Re-derived from code: the
  click's caret comes from the editor's layout of the real text, `pointer.rs:549-560`, the paint from a bullet layout,
  `render.rs:912`, `:975`; the measurement is owed.)
- C12: "hypothesis: unmeasured, the four lean tasks read no moved number". (Verified for the lean half: 0 CSS attribute
  selectors in the lean task files or `app.rs`; Circle Drawer's reading is owed.)
- C13: "`range_input_interaction`'s attribute rows write through `set_attribute` only and no check changes a range
  input's `type` (measured at that chunk's wrap)". (Re-derived: 2 `set_attribute`, 0 `clear_attribute`, no `type` write.)
- C14: "hypothesis: only the activation click's coordinates are off by that offset (read from the code, not measured)".
  (Re-derived from code: `synthetic_click_event_data` reads `absolute_position`, `node.rs:1929`; not measured.)
- C16: "read from the code at that wrap, not measured". (Verified: `focussed node` at `document.rs:1777`, the href sites
  at `pointer.rs:792` / `:797`; `host_log` and `mcp_log` pin no total line count, so the dated counts can have moved
  green — the re-read is owed.)

## CI (Setup 5a)

- CI#38632479079 on `19756f2d60a0` (the last wrap's flip, HEAD): **green**, checks 16/16, wall 872 s. Nothing to fold.

## Boundaries

- No mutation run in this chunk: the founder's word stands (inputs#I1).
- A founder's fork (an engine change for every document, playbook :64; a boundary widening, :36; a new gate or
  direction) is raised to the operator as a question dialog, for the operator to take to the founder (inputs#I1); it is
  never decided as a lean.
- v010-15 and the cold-agent pipe are "Cold-agent test"'s; the CARRYs on "Quality gates" (working-route.md:98) are
  that entry's.
- A CARRY that the forks leave unbuilt keeps an owner: a `.andromeda/residuals.md` line or a CARRY on a later entry,
  placed at this chunk's wrap and listed in the P5 card's `## Beyond the plan`; it is never dropped. (Closed at P3: the
  playbook's Sequencing-deferral rule requires a route annotation for every deferred obligation.)

## Surfaces and contracts touched (grounded at P3 — research.md §New files / §Files to modify)

- The stand: `examples/seven_guis/src/tasks/{temp_converter,circle_drawer,cells}.rs` (keys if C2 is taken; Cells'
  selected cell if C8 is taken) and the stand checks `tests/blitz-tests/tests/stand_*.rs` with their shared `common/`
  and `session_common/`. (Verified: the three files exist; the lean task files are not edited.)
- New and extended fixture checks in `tests/blitz-tests/tests/` (C4, C5, C6, C11, C12, C13 and C3's reading), one file
  and one test target per behaviour, both layout modes. (Verified: each extended file exists.)
- The engine (`packages/blitz-dom`) only where a founder's fork answers "change": `node/node.rs` (hit walk,
  `absolute_position`, focusability), `scrolling.rs`, `accessibility.rs`, `document.rs` (the fade), `events/pointer.rs`.
- `examples/seven_guis/tests/cli_commands.rs` for C9 — [premise-corrected: no CI workflow edit — the macOS matrix leg
  already runs `cargo test --all --tests` over every seven_guis target (`ci.yml:326`, `:392`)]; C16 re-reads
  `examples/seven_guis/tests/{host_log,mcp_log}.rs` captures and obs-plan's dated counts.
