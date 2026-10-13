# P5 review — 2026-10-13-stand-keyboard-harness ("Stand keyboard harness")

Plan: `escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/plan.md` (364+ lines · 12 steps · 12 acceptance
criteria · 17 gate entries) · research: `escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/research.md` · scope
(premises closed at P3): `escher-0.1.0/chunks/2026-10-13-stand-keyboard-harness/scope.md`.

## Goal
Make the stand operable by keyboard alone and prove it headlessly. Today no key activates anything. P3 measured it
through the driver: Enter and Space on a focused `counter-increment` change nothing, so SC 2.1.1 is unmet on every
stand button. The chunk lands:
- **F1** «Клавиши нажимают (Рекомендую)»: Enter on keydown and Space on keyup dispatch a `Click` at the focused
  control, so the app's `onclick` fires. This is an engine change for every document and a boundary widening.
- **F2** «Клик даёт фокус (Рекомендую)»: a click on a button focuses it.
- **F3** «По наличию (Рекомендую)»: any `disabled` attribute removes focusability.
- **F5**, your "List takes focus": `crud-list` takes Tab focus, and ArrowUp, ArrowDown, Home and End move the
  selection.
- **F4** «Отложить (Рекомендую)»: `Node::absolute_position` is not touched here.

The founder's labels are relayed verbatim (inputs#I2, inputs#I3).

## Top acceptance criteria
- Each lean task's Tab sequence, forward and back with wrap-around. At every step the stable id, the snapshot's one
  `focused` node and the tree focus's `author_id` agree. Both layout modes.
- Enter and Space on each of the 10 stand buttons give the same snapshot text as a click on it (20 rows, counted). At
  least one row changes the screen. The 10 non-activating rows read as stated.
- Fixture rows:
  - a checkbox or radio toggles by Space
  - a link activates by Enter
  - a summary activates by Enter and by Space
  - an `input type="button"` activates by Enter and by Space
  - a control disabled after it took focus fires no click
  - a self-scrolled button gets its own `onclick` on Enter
- A click focuses each stand button, and the next Tab continues from it.
- Tab skips a bare, `"false"` and `"true"` `disabled`.
- On CRUD, the third Tab stop is `crud-list`. The arrows move `selected`. `focused()` reads `crud-list` throughout:
  this is the known gap, stated plainly in the plan's Goal.
- `apple_keybinding("deleteBackward:")` deletes on linux, and the windows and macos jobs of the pushed sha's CI read
  green.
- Through the driver, `press enter` and `press space` on a Tab-focused `counter-increment` each name `counter-value`
  changed.
- `inert`:
  - absent under `inert: false` and present under `inert: true`
  - Tab reaches content under both (the engine honours no `inert`; pinned)

## Touchpoints
- Engine:
  - `packages/blitz-dom/src/events/keyboard.rs`, `packages/blitz-dom/src/events/mod.rs` (F1)
  - `packages/blitz-dom/src/events/pointer.rs` (F2)
  - `packages/blitz-dom/src/node/element.rs` (F3)
- Stand: `examples/seven_guis/src/tasks/crud.rs` (F5).
- Pins: `packages/dioxus-native-dom/src/snapshot.rs`, plus `tests/blitz-tests/tests/` `common/mod.rs`,
  `stand_accessibility_ids.rs`, `stand_snapshot_state.rs`, `focusability_updates.rs`, `dioxus_falsy_boolean_attrs.rs`,
  `stand_act_keys.rs`, `stand_act_obstructed.rs` and `stand_act_diff.rs`.
- Derived under F2: the further checks the step-11 runs name, out of 24 files that click a stand button.
- New: `tests/blitz-tests/tests/stand_keyboard.rs`.
- Untouched, guarded by entry 10: `node.rs`, `scrolling.rs`, the driver, the harness, the bridge writer, the other
  stand tasks.

## Test commands
- Units: `cargo test -p blitz-dom --locked` · `cargo test -p dioxus-native-dom --locked`.
- Per package: `cargo test -p blitz-tests --locked --test stand_keyboard`, and the eight restated targets.
- The same under `--workspace`, plus `cargo test -p seven_guis --locked`.
- Probes:
  - no package in `Cargo.lock`
  - no print, log, env, unwrap, expect or panic in the added engine, bridge or stand lines
  - the new check is silent
  - nothing outside the write set moves
- Harness and CI legs: `bash scripts/agent-run.sh boot` / `run stand` · `ci-leg.sh fast` · `ci-leg.sh doc`.
- Operator pass: hygiene, push, `ci.py conclusion --sha HEAD --wait 1800`.
- No mutation runs (inputs#I1).

Baselines on the untouched tree:
- The predicted reds: the new target is absent (entries 3 and 5) and the new file is absent (entry 9).
- Entry 4 is green on the old readings that steps 1–6 turn red by design.
- `run stand` reads 141 · 0 · 7.
- Probes 7–10 each fired over a scratch clone, on a must-fail input and a must-pass input
  (`.andromeda/runs/2026-10-13T22-07-35-phase/controls-p5.md`).

## Concretization preview
none — no capability is claimed. The one unclaimed id, `v010-15`, is listed by "Cold-agent test"
(escher-0.1.0/working-route.md:100).

## Leans for your confirm
1. F1 and F2 treat a button as a `<button>` or an `input` of type `submit`, `reset` or `button`. This is leaned from
   the F1 option text "a submit button submits", and kept the same for F2, so a Tab-reachable input button behaves as
   a `<button>` does.
2. F3 reads `disabled` by presence on every element, per "any `disabled` attribute removes focusability" and
   `handle_click`'s own presence read on every element. Known divergence from browsers: a `<div disabled tabindex=0>`
   becomes unfocusable.
3. The wrap halts once on F1 as the Boundary widening answer card (`.andromeda/playbook.md:36`). The founder answered
   it himself, relayed verbatim.

## Beyond the plan
| # | proposal | where |
|---|---|---|
| 1 | New CARRY: the CRUD list takes focus but the tree's focus is the list, not the row. The accessibility builder reads no `aria-activedescendant`, and a click on a row or on the list still leaves nothing focused (F2 covers buttons only). This is the operator's condition, inputs#I4. | "Stand a11y assertions" (escher-0.1.0/working-route.md:94), beside the listbox-name CARRY |
| 2 | The CARRY "the second position reader reads a scrolled box shifted" moves off this entry, by the founder's F4 word «Отложить (Рекомендую)». Its hypothesis is restated as measured here: an activation key reaches the self-scrolled node, and only the click event's coordinates are off. | "Stand requirement sweep" (escher-0.1.0/working-route.md:96) |
| 3 | New CARRY: the engine honours no `inert`. A truthy `inert` neither removes focusability nor stops a hit. Pinned by `stand_keyboard`; a fix is an engine change for every document. | "Stand a11y assertions" (escher-0.1.0/working-route.md:94) |
| 4 | New CARRY: the merge surface of the three ratified engine changes, against 434e0661 (`events/keyboard.rs`, `events/mod.rs`, `events/pointer.rs`, `node/element.rs`). This follows the range model's precedent. | "Quality gates" (escher-0.1.0/working-route.md:98) |

Apply? (yes / review / cancel)
