# Codebase Research — 2026-10-13-stand-keyboard-harness

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/testing.md` — read in full, 7 Session Additions applied (the platform-bound-key rule at :52 decides the `apple_keybinding` lean below; the count-under-`--no-fail-fast` rule at :55; the table-pin search at :57) · `.claude/rules/verification-harness.md` — read in full, 1 Session Addition (no live leg with a `timeout` bound in this chunk)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (CI on `e40d9437` green, 16/16)
- **External inputs:** `inputs#I1` — the operator's directive at take-up: pipeline tools bare or redirected and read whole; no mutation runs (the founder's word); a P4 fork changing engine behaviour for every document is raised as a dialog for the founder

## Files inspected
- `packages/blitz-dom/src/events/keyboard.rs` (full) — Tab and Shift+Tab move focus through `focus_next_node` / `focus_prev_node` inside `generate_focus_events` (:26-45); every other key reaches only the focused node, and only two arms act there: a text input's editor (:80-101) and a non-`disabled` range input's arrows and `Home` / `End` (:102-117). No arm activates a `button`, a checkbox, a radio, a `summary` or a link on Enter or Space.
- `packages/blitz-dom/src/events/pointer.rs` (:640-866) — `handle_click` walks up from the target: a `disabled` attribute (presence, :661-664) or a text input stops it, a range input, a checkbox, a radio and a first `summary` focus themselves (:670-760), a label re-runs its input's click (:762-773), a submit button submits its form (:800-806); no arm matches a plain `button`, so the walk ends unmatched and focus is cleared (:853-856).
- `packages/blitz-dom/src/node/element.rs` (:702-734) — `flush_is_focussable` reads `disabled` by `attr_parsed` (a bare `""` fails the parse and reads `false`, so a bare or `"false"` `disabled` stays focusable) and `tabindex >= 0`, else the default-focusable names.
- `packages/blitz-dom/src/document.rs` (:1739-1752, :1755-1795) — `focus_next_node` / `focus_prev_node` start from `get_focussed_node_id` (the root when nothing is focused) and `set_focus_to`, which logs `Focussed node` under `#[cfg(feature = "tracing")]` (:1776-1777).
- `packages/blitz-dom/src/traversal.rs` (:195-285) — `next_node` and `prev_node` walk document order and wrap through the root, returning `None` only on coming back to the start: Tab wraps forward, Shift+Tab backward; neither reads visibility.
- `packages/blitz-dom/src/node/text.rs` (:333-342, :372) — the editor's `Key::Backspace` arm is `#[cfg(not(target_os = "macos"))]`; `apply_apple_standard_keybinding` carries no platform gate, so `Harness::apple_keybinding("deleteBackward:")` deletes on every OS.
- `packages/blitz-dom/src/node/node.rs` (:1923-1953) — `synthetic_click_event_data` takes its point from `absolute_position` (:1929).
- `packages/blitz-test-harness/src/input.rs` (:79-94, :202-233) — `key_event` fills `text` only for a pressed `Key::Character`; `press_with` dispatches KeyDown then KeyUp and pumps; `apple_keybinding` dispatches `UiEvent::AppleStandardKeybinding` and pumps.
- `packages/escher-driver/src/schema.rs` (:14-27) and `packages/escher-driver/src/execute.rs` (:472-495) — the driver's twelve key names include `enter` and `space` (`Key::Character(" ")`); `press` adds `deleteBackward:` on macOS only.
- `packages/dioxus-native-dom/src/mutation_writer.rs` (:428, :443-475) — a falsy value of an `is_bool_attr` name is removed; `inert` is one of them (:458).
- `packages/dioxus-native-dom/src/events.rs` (:660-664) — `synthetic_click_event` is defined and has no caller in the crate (grep below): nothing in the bridge turns a key into a click.
- `examples/seven_guis/src/tasks/crud.rs` (:56-82) — each row is a `div` with `role: "option"`, `aria_selected` and an `onclick` only: no `tabindex`, no key handler.
- `tests/blitz-tests/tests/common/mod.rs` (:16-45) — `controls(task)`, the 18 rows naming 15 controls with their roles.
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` (:18-38, :86-102) — the per-task `tab_order` table, private to this file, read by `focus_next_node` from the root until it repeats (not by a key press); the FlightBooker sequence omits `flight-return-date` and the CRUD one `crud-update` / `crud-delete` (disabled at boot by a truthy `disabled`).
- `tests/blitz-tests/tests/stand_snapshot_state.rs` (:61-77, :228-249, :339-375) — `focused()` reads the snapshot's `focused` nodes and asserts them equal to the accessibility tree's focus `author_id`; the parsed-markup pin that `fx-falsy` / `fx-bare` are focusable and read not enabled; the one key-press focus check (CRUD, click then Tab then Shift+Tab, a fresh boot's first Tab on `back-btn`).
- `packages/dioxus-native-dom/src/snapshot.rs` (:552-575) — the unit pin `a_present_disabled_attribute_reads_not_enabled_whatever_its_value`, asserting `is_focussable()` true on the falsy and bare forms.
- `tests/blitz-tests/tests/focusability_updates.rs` (:58-76) — writes `disabled="true"`, its doc noting a bare `""` would not count.
- `.github/scripts/test_blitz_tests_targets.py` (:18-30) — asserts `autotests` is not false and the one `[[test]]` table; it names no file, so a new file moves nothing in it.
- `.andromeda/playbook.md` (:36, :48, :60, :64) — the `verdict: escalate` patterns (Boundary widening · Provisional discharge · Boundary widening by upstream merge) and the routine engine-change rule, on the founder's own word at the P4 fork.

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-13T22-07-35-phase/tree-query-2026-10-13-stand-keyboard-harness.json`)
- **is_focussable** — read at `packages/blitz-dom/src/document.rs:1741` and `:1749` (the Tab walk), `packages/blitz-dom/src/mutator.rs:1042` (autofocus), `packages/dioxus-native-dom/src/actionable.rs:70` (`unkeyed_actionable`), and pinned by `snapshot.rs:563`, `stand_snapshot_state.rs:240`, `focusability_updates.rs:38-75`, `stand_actionable_keys.rs:68` — a presence-keyed fix moves the Tab walk, autofocus and the actionable census for every document and rewrites three pins.
- **focus_next_node / focus_prev_node** — called by `keyboard.rs:34` / `:36` and by the checks `stand_accessibility_ids.rs:89` and `stand_snapshot.rs:226`.
- **absolute_position** — read by `handle_dom_event` (`packages/blitz-dom/src/events/mod.rs:109`), itself recursively (`node.rs:1689`), `synthetic_click_event_data` (`node.rs:1929`), `Node::focus` (`node.rs:732`, the IME cursor area), `scroll_into_view` (`scrolling.rs:720`), the harness's `layout_rect_of` (`inspect.rs:60`) and `write_node` (`inspect.rs:142`), and by three checks (`anonymous_block_restyle.rs:100`, `stale_dirty_descendants.rs:81-100`, `scrolled_box_absolute_position.rs:44-74`) — a fix moves seven engine and harness readers for every document.
- **synthetic_click_event_data** — called by `handle_click`'s label arm (`pointer.rs:769`), `Node::synthetic_click_event` (`node.rs:1925`) and the bridge's uncalled `synthetic_click_event` (`events.rs:662`): the one engine path a key activation would reuse.
- **apple_keybinding** — the graph returned no `calls` row for it though its `symbol` row stands (`input.rs:230`, a second query); grep finds its one caller, `packages/escher-driver/src/execute.rs:483`, under `#[cfg(target_os = "macos")]`, which the Linux-built index does not see — not a leaf: no standing check calls it on this host.

## Patterns detected
- **The focus read-back helper** (`tests/blitz-tests/tests/stand_snapshot_state.rs:61-77`): the snapshot's `focused` ids asserted equal to the tree focus's `author_id`, returned for the caller's assertion — private to that file today.
- **A stated sequence read by the engine's walk** (`tests/blitz-tests/tests/stand_accessibility_ids.rs:18-38`): the expected order is a table, never computed from the engine.
- **Measured-and-pinned, red by design when fixed** (`tests/blitz-tests/tests/scrolled_box_absolute_position.rs`): the shape for every defect a fork leaves standing.
- **A row table with its count asserted** (`tests/blitz-tests/tests/range_input_interaction.rs:113-135`): the shape for an activation table, one row per control and key.

## Conventions to follow
- **One behaviour, one target file**: `//!` doc, `for incremental in [false, true]`, fixture condition asserted first; shared tables in `tests/blitz-tests/tests/common/mod.rs` only (testing.md:15, :21-23).
- **A `stand_` prefix puts a file under `run stand`** with no script change (`scripts/agent-run.sh`, verification-harness.md:18); `range_input_interaction` chose no prefix and runs on the `test` leg alone.
- **No platform-bound key through `press`** (testing.md:52): leaned — a direct `apple_keybinding("deleteBackward:")` is not `press` and is not platform-bound (`text.rs:372` has no gate), so it is the every-OS witness CARRY 5 asks for.
- **Failure messages name a layout mode, a task and an index, never an id or screen text** (tests-history, 2026-10-07-act-by-id).

## Measured at P3 (read-only: no code written, the built binary run through the driver)
- `target/debug/escher-session` on `counter` (state directory `target/tmp/kbd-probe-counter`), output read whole at `.andromeda/runs/2026-10-13T22-07-35-phase/probe-counter.txt`: Tab, Tab focuses `counter-increment` (`differs: ["focused"]`); `press enter` and `press space` each return `settled: true` with an empty diff, the value still `0`; a `click` on `counter-increment` reads the value `1` and `counter-increment` `focused: false`. The equality the chunk's activation rows turn on — Enter or Space on a focused stand button yields that button's click — reads false at HEAD for these inputs, the frame being `keyboard.rs:71-118` (no button arm).

## New files to create
- `tests/blitz-tests/tests/stand_keyboard.rs` — the stand keyboard harness: per lean task the forward Tab sequence and its Shift+Tab reverse with wrap-around, the three focus readings at every step, the activation rows, the disabled-attribute Tab reach and the `apple_keybinding` every-OS witness

## Files to modify
- `packages/blitz-dom/src/events/keyboard.rs` — F1 (ruled, inputs#I3): Enter on keydown and Space on keyup dispatch a click at the focused control
- `packages/blitz-dom/src/events/mod.rs` — F1: the KeyUp default action routed to the keyboard module for Space
- `packages/blitz-dom/src/events/pointer.rs` — F2 (ruled, inputs#I3): a click on a button focuses it
- `packages/blitz-dom/src/node/element.rs` — F3 (ruled, inputs#I3): focusability reads `disabled` by presence
- `examples/seven_guis/src/tasks/crud.rs` — F5 (the operator, inputs#I4): the list takes focus, arrows and Home and End move the selection
- `packages/dioxus-native-dom/src/snapshot.rs` — F3: the unit pin restated, a bare and a falsy `disabled` not focusable
- `tests/blitz-tests/tests/common/mod.rs` — the per-task Tab sequence and the focus read-back helper stated once, moved line for line
- `tests/blitz-tests/tests/stand_accessibility_ids.rs` — reads the moved Tab sequence, CRUD's restated with `crud-list`
- `tests/blitz-tests/tests/stand_snapshot_state.rs` — reads the moved focus read-back helper; F3 restates the parsed-markup pin
- `tests/blitz-tests/tests/focusability_updates.rs` — F3: a bare `disabled` takes focusability away, the note restated
- `tests/blitz-tests/tests/dioxus_falsy_boolean_attrs.rs` — the falsy `inert` read beside its 27 names
- `tests/blitz-tests/tests/stand_act_keys.rs` — F1 through the driver: `press enter` and `press space` on a focused Counter button
- `tests/blitz-tests/tests/stand_act_obstructed.rs` — F2: the restated plain-button reading, the clicked button focused
- `tests/blitz-tests/tests/stand_act_diff.rs` — F2: the Create step names `crud-create` gaining focus
- derived `tests/blitz-tests/tests/stand_*.rs` by `cargo test -p blitz-tests --locked --no-fail-fast` — F2: each further check that pins a button click leaving nothing focused, restated
- derived `examples/seven_guis/tests/*.rs` by `cargo test -p seven_guis --locked --no-fail-fast` — F2: each command-line check that pins a button click's diff, restated
- derived `examples/seven_guis/tests/flows/*.sh` by `cargo test -p seven_guis --locked --test cli_flow` — F2: each flow step that pins a button click's answer, restated

## Sweeps
- `synthetic_click_event\(` over `packages/dioxus-native-dom/src`: 1 hit (the definition, `events.rs:660`) · 0 changed · 1 no-change (no caller)
- `Key::Enter|"enter"|Character\(" "|"space"` over `tests/blitz-tests/tests/*.rs`: 0 hits — no standing check presses Enter or Space (re-derived: `grep -rn` over the directory)
- `click\(.*"#?(back-btn|counter-increment|flight-one-way|flight-return"|flight-book|timer-reset|crud-create|crud-update|crud-delete)` over `tests/blitz-tests/tests/*.rs examples/seven_guis/tests/*.rs`: 24 files hit (`command grep -lE`) · 0 changed · 24 no-change at HEAD — each is a candidate pin of "a button click leaves nothing focused" should fork F2 make a click focus the button; enumerated per file at P4 on that branch only
- `inert` over `packages/blitz-dom/src`: 1 hit, a doc comment (`mutator.rs:166`) · 0 changed — the engine implements no `inert`: a truthy `inert` neither removes focusability nor stops a hit (re-derived: `grep -rn 'inert'`)
- `onkey|tabindex|onfocus|onblur` over `examples/seven_guis/src/`: 2 hits, both Cells (`cells.rs:386`, `:390`) · 0 changed — no lean task handles a key

## Open questions
- Under F2, which of the 24 button-clicking files pin a diff or a focus reading that moves? → blocks: implementation-scope (the derived rows of Files to modify are read by the named runs)
- Resolved at P4: F1 activates by dispatching a `Click` DomEvent at the focused control — the pointerup's own route (`pointer.rs:626-629` dispatches `DomEventData::Click`; `events/mod.rs` runs the app's handlers, then `handle_click` as its default action) — so the app's `onclick` fires, which the label arm's direct `handle_click` call (`pointer.rs:769`) would skip.
