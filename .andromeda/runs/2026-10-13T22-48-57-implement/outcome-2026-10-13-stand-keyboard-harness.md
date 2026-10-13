# Implement outcome — 2026-10-13-stand-keyboard-harness

**Outcome: green + smoke.** Every non-operator gate passed on the block's first firing (14 green, 0 red, 3 operator
entries not run), and the `escher-session` binary was driven by hand as a smoke. Next → the OPERATOR PASS, entries 15
(hygiene), 16 (pre-push gate + push) and 17 (CI read: the windows and macos witness of the delete binding, MSRV, ios,
android). Then `/andromeda-wrap-session`.

## Files changed (18; scope read `clean` — changed 18 · listed 18 · recorded 0 · absorbed 3)
- Engine (three changes for every document, F1–F3, ratified by the founder, inputs#I3):
  - `packages/blitz-dom/src/node/element.rs` — F3: `flush_is_focussable` reads `disabled` by presence (`has_attr`).
  - `packages/blitz-dom/src/events/keyboard.rs` — F1:
    - a pressed Enter on the focused control dispatches a `Click` DomEvent (data from `synthetic_click_event_data`):
      `button`, `input` of type `submit`/`reset`/`button`, `a[href]`, `summary`;
    - new `handle_key_up`: a released Space does the same on `button`, `input` of type
      `submit`/`reset`/`button`/`checkbox`/`radio`, `summary`;
    - both are gated on `disabled` presence.
  - `packages/blitz-dom/src/events/mod.rs` — F1: `KeyUp` routed to `handle_key_up`.
  - `packages/blitz-dom/src/events/pointer.rs` — F2:
    - `handle_click`'s submit arm becomes a button arm: a `button`, or an `input` of type `submit`/`reset`/`button`;
    - it focuses the control, submits the form owner for a submit button, and ends the walk matched.
- Stand: `examples/seven_guis/src/tasks/crud.rs` — F5 (the operator, inputs#I4):
  - `crud-list` gains `tabindex: "0"` and an `onkeydown`;
  - ArrowDown selects the next visible row (the first when none is selected), ArrowUp the previous, Home the first and
    End the last. Each selection fills the fields as a row click does;
  - the filter predicate moved into a private `shown` fn, read by the render and the handler.
- Pins and tables:
  - `packages/dioxus-native-dom/src/snapshot.rs` — the unit pin is renamed
    `…_reads_not_enabled_and_not_focusable_whatever_its_value`. The falsy and bare forms now read not focusable.
  - `tests/blitz-tests/tests/common/mod.rs` — `tab_order` and `focused()` moved in line for line. CRUD now reads
    `back-btn, crud-filter, crud-list, crud-name, crud-surname, crud-create`.
  - `stand_accessibility_ids.rs` · `stand_snapshot_state.rs` — they read the moved items. The parsed-markup pin now
    reads `fx-falsy`/`fx-bare` not focusable.
  - `focusability_updates.rs` — the note is restated, and there is a new test,
    `a_bare_disabled_set_after_creation_takes_the_focusability_away`.
  - `dioxus_falsy_boolean_attrs.rs` — `inert` is the 28th name. The falsy form is absent on both paths and the truthy
    form is kept on both.
  - `stand_act_keys.rs` — new test `enter_and_space_on_a_focused_button_count_as_its_click_does`. On Counter: two Tabs,
    then `press enter` and `press space` each name `counter-value` alone, with `differs: [name]`.
- New: `tests/blitz-tests/tests/stand_keyboard.rs` — 11 tests, both layout modes:
  - the Tab walk and its wrap;
  - the activation table (20 rows) and the non-activating rows (10 rows);
  - the fixture keys, disabled-after-focus, and a self-scrolled button;
  - click focus, the disabled-skip, the list keys, the Apple delete binding, and `inert` not honoured.
- Restated by step 11 under F2 (the derived `stand_*.rs` row):
  - `stand_act_obstructed.rs` — four accepted clicks now also name the clicked button, plus the button that lost focus
    where one did. After the click, `fx-before` reads focused and is the tree's focus.
  - `stand_act_diff.rs` — the counter's first click names `counter-increment`. Create names `crud-create`. The row click
    names `crud-create` losing focus.
  - `stand_act_ids.rs` — `changes_at_boot` lists every accepted control, and the count of changing rows goes 9 → 15.
  - `stand_act_spans.rs` — the child's counter click returns 2 changed nodes.
  - `stand_diff.rs` — the counter click names `counter-increment`. Book: 7 nodes are bounds-only, and `flight-book`
    reads `[Focused, Bounds]` (test renamed `…_eight_nodes_that_moved_and_book_focused`). The label fixture's rename
    names `fx-rename`.
  - `examples/seven_guis/tests/*` and the flows: none turned red, so none were restated.

## Measured (step 12)
- Activation table: the predicted changing rows held — `counter-increment`, `flight-return`, `flight-book`,
  `crud-create` change the screen beyond focus, the other six do not.
- Workspace (`target/ci-logs/test.log`): 182 result lines · 928 passed · 0 failed · 12 ignored (was 181 · 915 · 0 · 12).
- blitz-tests: 121 files, 120 targets (was 120 · 119). `stand_*` files: 38. `common` readers: 19 (was 18).
- `run stand`: run.end passed 153 · failed 0 · ignored 7, outcome passed (was 141).
- Probes: Cargo.lock 0 new packages · added-line census 0 · `stand_keyboard` silence 0 · untouched-paths diff quiet.

## Deviations from plan (with justification)
- `stand_keyboard`'s three-way focus reading drops the root element from the engine's reading.
  `BaseDocument::get_focussed_node_id` falls back to the root element when nothing is focused. The check's first run
  read that fallback as a focused node.
- A button disabled after it took focus is disabled by a direct `DocumentMutator::set_attribute(…, "")` in the test, not
  through app state. The test asserts first that the button still holds focus and carries the attribute.
- Step 6: ArrowUp with no row selected changes nothing. The plan states only the end-of-list rule, so this case was
  settled as no change.
- No scope-record lines: every changed file is on research's lists or absorbed by its derived `stand_*.rs` row.

## Surfaced
- None.
- Not this chunk's, for the wrap's amendments:
  - a11y-plan's `rules/a11y.md` text ("focusability on the PARSED bool"; `inert` "no longer makes content inert") now
    reads against the measured truth;
  - the plan's expected amendments list them.

## Smoke
- Smoke ✓ — fired by hand: `crud.rs` and the engine compile into `escher-session`, and the plan has no smoke entry.
- `escher-session` on crud:
  - three Tabs, then `arrow-down`: `crud-person-0` selected, the fields filled `Hans`/`Emil`, Update and Delete
    enabled;
  - `end`: `crud-person-2` selected.
- On counter: two Tabs, then `enter` and then `space` each read `counter-value` named 1 and then 2.
- One call was refused `malformed` because I spelled the key `arrowdown`. It was re-run as `arrow-down`.
- State dirs: `target/tmp/kbd-smoke-*`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| escher-session host (crud, pid 2154030) | this run | terminated — `stop` answered `{"stopped":true}` |
| escher-session host (crud, pid 2155432) | this run | terminated — `stop` answered `{"stopped":true}` |
| escher-session host (counter, pid 2155454) | this run | terminated — `stop` answered `{"stopped":true}` |
| cargo / test binaries of the gate block | the gate tool (entries 1–14) | terminated — each entry exited; `ps -eo pid,comm,args` after the smoke shows no escher-session or seven_guis process |

## Decisions & corrections
- Guard refusal: `bash-guard.sh` refused a `cat` heredoc to a scratch file. Re-issued once through the Edit tool.
- Standing learning met (testing.md 2026-10-10, search for pins before calling a check unaffected): step 11's full runs
  found 5 red targets that no list named by file. All were inside the derived row.
