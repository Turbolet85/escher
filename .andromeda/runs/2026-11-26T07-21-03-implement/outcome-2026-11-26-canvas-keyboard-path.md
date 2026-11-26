# Outcome — 2026-11-26-canvas-keyboard-path

**Outcome:** green — 14 of 14 local gate entries green on the final tree (`gate-2026-11-26-canvas-keyboard-path.json`,
the second full firing); smoke skipped — no boot-path change, no headful self-verify in this project. The four
`operator` entries (hygiene, push, CI read, windows job-log read) are the operator pass's.

**Next:** the OPERATOR PASS — entries 15 (`gate.py hygiene`), 16 (the push), 17 (`ci.py conclusion`) and 18 (the
windows job-log read, `<id>` the CI run id for the final HEAD) — then `/andromeda-wrap-session`.

## Files changed (20)
- `packages/blitz-dom/src/node/custom_widget.rs` — `VirtualNodeDeclaration.selected: Option<bool>`
- `packages/blitz-dom/src/events/mod.rs` — a focused custom-widget element passes Tab and Shift+Tab on after the widget
  hears the key (widening, the operator's word, inputs#I7, PROVISIONAL; flagged upstreamable)
- `packages/dioxus-native-dom/src/{dioxus_document,snapshot,virtual_node}.rs` — `selected` written on the virtual
  node's tree node and read by the snapshot; the unit builder takes `selected: None`
- `packages/escher-driver/src/{execute,actions,refusal}.rs` — `covered` by a later-declared node, shared by the run
  path and `actions` through `Target::into_virtual`; the `covered` meaning and its pin restated
- `examples/seven_guis/src/tasks/circle_drawer.rs` — `flex-shrink: 0`, the `selected` signal, click-selects,
  `tabindex`, `onkeydown`, `data-selected` / `data-adjusting`, `SELECTED_FILL`, two unit tables
- `examples/seven_guis/tests/host_binary.rs`, `tests/blitz-tests/tests/stand_act_error_text.rs` — imports behind
  `cfg(unix)`
- `tests/blitz-tests/tests/{common/mod,fixtures/mod,virtual_nodes_declared,virtual_nodes_as_elements,stand_circle_nodes,stand_keyboard,stand_canvas_click,stand_circle_drawer_canvas}.rs`
- `docs/capabilities.md`

## Deviations (with justification)
- **The engine Tab trap** — not in the plan: a focused element carrying a custom widget kept focus on Tab and
  Shift+Tab (measured 4 presses each way, both layout modes), blitz-dom's widget arm returning before any default
  action. Fixed on the operator's word (inputs#I7) in `events/mod.rs`, with the engine-level check
  `stand_canvas_click::tab_and_shift_tab_leave_a_focused_widget_which_still_hears_them` and its red control C7.
- **C6 read inert; the app's `set_focus` dropped** — step 4c's `drop(handle.set_focus(true))` guarded nothing:
  blitz-dom's widget arm focuses the element on PointerDown. Under testing.md's inert-guard rule the call was dropped;
  the focus is pinned by `a_click_focuses_the_canvas_and_a_press_acts_on_its_circle` and
  `the_focus_events_of_a_canvas_click`.
- **Close below the view** — with `flex-shrink: 0` (step 4a) `circle-close` sits at y 607–639, refused `off-screen`.
  `GRAPH_DRIVES` reaches it by `Press("tab")` + `Press("enter")` from the diameter, then `Click("circle-canvas")`
  refocuses the canvas before `arrow-right` and `enter` (step 7 put the presses right after a Close click, whose
  removal drops focus); stand checks `scroll circle-close` first.
- **App circles apart by harness clicks** — a `data-circles` write sets no app state, so the click and key cases
  (`a_click_focuses…`, `the_keys_move…`) and the undo case add their circles by `Harness::click_at` off the canvas
  centre (`click_inside`); `write_circles` stays for the widget-level `covered` and `disabled` readings.
- **Re-read cases step 8 said stand** — `an_undone_circle_reads_stale_and_its_redo_keeps_its_id` (its second centre
  click now selects), `each_circle_s_commands_carry_its_own_index`'s tie block (now written through `data-circles`)
  and `a_kept_check_lists_the_unclicked_circle_s_click` (rows re-read: `click c1` exercised, `click canvas` leaves
  `c1` unexercised).
- **Step 3's fixture lives in its check** — the selecting canvas is in `virtual_nodes_as_elements.rs`, a page with
  one reader (testing.md §Fixtures), not `fixtures/mod.rs`, whose existing declarations take `selected: None`.
- **The step 10 sweep record** is not written under research.md's `## Sweeps` (research is immutable at implement);
  it is carried in the report.
- **Scope record** (`gate.py scope`: clean — changed 20 · listed 13 · recorded 7):
  - companion: `packages/escher-driver/src/actions.rs` (step 5), `tests/blitz-tests/tests/stand_circle_drawer_canvas.rs`
    (step 10) — self;
  - in-intent: `packages/escher-driver/src/execute.rs`, `packages/escher-driver/src/refusal.rs` (step 5),
    `tests/blitz-tests/tests/virtual_nodes_as_elements.rs` (step 3) — self;
  - widening: `packages/blitz-dom/src/events/mod.rs`, `tests/blitz-tests/tests/stand_canvas_click.rs` (step 9) — the
    operator, 2026-11-26, PROVISIONAL (inputs#I7).

## Guard entries
none — the plan lists no `gate.py guard` entry.

## Process hygiene
| process | started by | final state |
|---|---|---|
| cargo, test binaries, `escher-session` hosts of the gate entries and controls | this run | terminated — `gate.py census` (escher-session · cargo · seven_guis_nati · args stand_, blitz-tests): `census: none running` |

## Decisions & corrections
- The operator, 2026-11-26 (inputs#I7): a focusable widget that traps Tab is an engine defect, fixed where it is —
  PROVISIONAL, with an engine-level check, its red control, and an upstreamable flag.
- Guard refusals: the bash guard refused a `cat` heredoc with a file target twice and a `cd` out of the root once;
  each re-issued once as a scratchpad script or an absolute path.
- Recurrence of a standing learning (testing.md, 2026-11-25): the `covered` text's pins were swept by its opening
  clause — two in `refusal.rs` (meaning and pin), none elsewhere.
- Measured: a custom-widget element is focused by the engine on pointer down, so an app's own focus call on its click
  is redundant (control C6).
