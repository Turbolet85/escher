# Scope — 2026-10-13-range-input-interaction

**Working entry (escher-0.1.0/working-route.md:90, verbatim title and hint):** Range input interaction — arrow keys step
a range input's value, a click sets it by position, an input event fires (the founder's ruling, 2026-10-07)

**Epoch:** Epoch 6 — Polish & ship. **Freight on the entry:** two CARRYs (`route.py pins` lists two blocks on line 90,
853 and 1442 characters), both folded below; no PREREQ, WATCH, BLOCKED-ON or CONTEXT.

## What it builds
- A minimal range interaction model in the engine, for every document with an `input type="range"`:
  - the arrow keys step the value of a focused range input by its `step` (default 1), clamped to `[min, max]` (defaults
    0 and 100) and kept on the step grid from `min`;
  - a pointer click on the range input sets its value by the click's position along the track, snapped to the step grid
    and clamped;
  - each change fires an `input` event, so an app's `oninput` handler runs with the new value (the stand Timer's
    `oninput` parses `evt.value()` as `f64`, `examples/seven_guis/src/tasks/timer.rs:87-91`).
- Whether Home/End (to `min`/`max`) and up/down arrows belong to "minimal", and whether a pointer click focuses the
  slider (measured absent today: a click does not focus it), are P4 forks; the entry names arrows, click and `input`
  only. Verified as open at P3: Page Up/Page Down have no driver key name (`KEY_NAMES` holds the four arrows, `home`,
  `end` — `packages/escher-driver/src/command.rs`), and blitz-traits has no `change` event to fire
  (`DomEventData` holds `Input`, no `Change`) — both stay out.
- What the value change reaches beyond the event, read at P3: the snapshot's `value` reads the element's `value`
  attribute for an input with no editor (`packages/dioxus-native-dom/src/snapshot.rs:219-221`), so it follows whatever
  writes that attribute; the accessibility node carries no numeric value (`packages/blitz-dom/src/accessibility.rs`
  sets none) and none is added; blitz paints no track or thumb — a range input paints as the default sheet's white box
  with a 1px border (`packages/blitz-dom/assets/default.css:76-81`, no range rule; no `range` in blitz-paint) — so no
  painted pixel follows the value, and no paint is added (scope Boundaries).
- The P4 forks, answered by the founder (relayed verbatim by the overseer as his chosen option labels; the English
  wording is the overseer's — inputs#I2, inputs#I3): Q1 «Отдельное текущее значение» — the moved value is the control's
  own current value, held apart from its `value` attribute; Q2 «Стрелки + Home/End (Рекомендую)» — the four arrows step
  it and Home/End go to its minimum and maximum; Q3 «Клик даёт фокус (Рекомендую)» — a click focuses it. Validation-1
  (intent-incomplete, amended at P5): Q1 makes that current value the control's value for every reader of it, so the
  snapshot's `value` and the form entry list (`packages/blitz-dom/src/form.rs:316-319`, which reads the `value`
  attribute today) read it, and a `value` attribute write re-seeds it so an app re-render keeps control.
- `stand_act_range` (tests/blitz-tests/tests/stand_act_range.rs, 107 lines) pins today's no-op reading and turns red by
  design when this lands; it is rewritten here to the new reading, not deleted (the measured-starting-point CARRY).

## The founder's ruling and the boundary it widens (CARRY 1, folded)
- The ruling: "Пусть будет а" — the founder, 2026-10-07, relayed verbatim by the overseer, choosing option (a), whose
  wording is the overseer's: a minimal range interaction model in the engine.
- It is a boundary widening by the founder's own word: a behaviour change for every document with a range input, the
  browser app's remote pages included. So the plan states it in security-plan §Input Validation
  (`.andromeda/security-plan.md:65`), and this chunk's wrap records the ratification.
- No capability was added to the ledger with it; a requirement line, if one is wanted, is the operator's to add.
- Placed ahead of the first work that needs it: the keyboard harness ("Stand keyboard harness", working-route.md:92)
  and SC 2.1.1 keyboard reach on every stand control. Neither is built here.

## The measured starting point (CARRY 2, folded — hypotheses re-verified at P3)
- Measured at 2026-10-07-act-by-id (`escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/range-input.md`, present):
  the Timer's `timer-duration` slider (role Slider, `min 0`, `max 30`, `step 0.5`, an `oninput` handler) reads `15`
  and its display `15.0s` after each of eleven driver calls, in both layout modes; a driver `click` on its id is
  accepted, changes nothing and does not focus it; two Tab presses focus it; right arrow, left arrow, Home and End
  change nothing, focused or not.
- [premise-corrected: construct.rs is not where the no-op is decided — the key and click paths are] The CARRY's
  hypothesis ("read from construct.rs:442-460 … not traced at run time") holds as a fact — construction gives a range
  input no special data (`packages/blitz-dom/src/layout/construct.rs:437-458`) — but the run-time reason no key or click
  moves it is in the event paths: a key acts only on a focused node holding text-input data
  (`packages/blitz-dom/src/events/keyboard.rs:68-100`), and `handle_click` has no range arm, so a click on the slider
  matches nothing and clears focus (`packages/blitz-dom/src/events/pointer.rs:614-817`, the clear at 814-817) — which is
  also why the measured click does not focus it. No construction change is needed when the value lives in the `value`
  attribute (a P4 fork, research Open questions).
- The engine's input files are upstream-owned merge surface: verified — upstream/main at 434e0661 carries no range
  interaction (`git log -i --grep=range upstream/main`: none), and `events/{keyboard,pointer,mod}.rs` are upstream's; the
  change is kept additive, its value rules in a new module of ours.
- A driver `click` lands at the element's centre: verified (`packages/escher-driver/src/execute.rs:351-366`, centre of
  the bounds plus the viewport scroll). On the Timer's `0..30` track, whose border and padding are the default sheet's
  symmetric 1px + 2px (`.timer-slider` sets neither, `examples/seven_guis/src/tasks/timer.rs:164-167`), the centre of the
  box is the centre of its content box, so a centre click maps to the midpoint, 15 — the boot value. The rewritten
  check therefore proves "a click sets it by position" by clicking after a key step has moved the value off 15 (the
  centre click brings it back), and an engine check clicks chosen points through the harness's `click_at` — the route
  is P4's.
- Not measured, and out: a pointer drag (the verb table has no drag); `arrow-up`/`arrow-down` with the slider focused
  (read once unfocused) — whether they step like right/left is a P4 decision under the same minimal model.

## Boundaries
- Out: `select` — by the founder's ruling of 2026-10-09 a residual outside 0.1.0 (`.andromeda/residuals.md:11`, origin
  2026-10-06-snapshot-state-fidelity); no stand task holds one.
- Out: a driver drag verb or any new driver verb; the keyboard harness and the SC 2.1.1 sweep (working-route.md:92,
  :94); vertical or right-to-left ranges, `list`/datalist ticks, and a styled track or thumb beyond what blitz already
  paints.
- Out: AccessKit Increment/Decrement/SetValue actions and a numeric value on the slider node — verified not needed: the
  stand ships no platform adapter, and either would be a new class on the adapter crossing (a founder question).

## Surfaces and contracts it touches
- blitz-dom's event handling — `handle_key_or_input_event` (`events/keyboard.rs`), `handle_pointerdown` and
  `handle_click` (`events/pointer.rs`), reached from `handle_dom_event` (`events/mod.rs:192, 215, 218`) — with the
  `input` event built as the text input's is (`events/keyboard.rs:119-127`). Verified at P3; no signature changes.
- [premise-corrected: no bridge change is needed] dioxus-native-dom's event conversion already carries an engine
  `Input`'s value to the app: `DomEventData::Input(data)` becomes `NativeFormData { value: data.value.clone(), … }`
  (`packages/dioxus-native-dom/src/dioxus_document.rs:377-380`), so the Timer's `oninput` reads the new value with no
  change there.
- The driver's behaviour on the stand's Timer through `click` and `press`, with no driver code change: no driver or
  help text names a range input (grep `-i "range input\|slider"` over `packages/escher-driver/src/`: 0), so no schema
  text moves. Verified at P3.
- Standing checks that pin the old reading: `stand_act_range` (rewritten), and `stand_act_ids`, whose Timer row pins
  the slider's driver click accepted with an empty diff (`tests/blitz-tests/tests/stand_act_ids.rs:40`) — it moves only
  if a click focuses the slider (P4 fork). Found by the P3 sweep (research.md, Sweeps).
- security-plan §Input Validation (the widening statement); architecture §Established Decisions (Scrolling and input)
  and the test-plan's tallies are what the wrap amends.

## Directives
- No mutation runs — the mutation tier stays unstarted; pipeline tools run bare or redirected to a file read whole
  (the operator relaying the founder, inputs#I1).

## CI read at Setup (5a)
- `97ce760ddfb4` (the last wrap's flip commit, HEAD) — green · checks 16/16 · wall 1325 s · CI#38448217675
  completed/success. No red and no `not green` to fold.

## Verification matrix
- No capability names range input interaction (CARRY 1: none was added); the one unclaimed id, v010-15 (Cold-agent
  test), is owned by "Cold-agent test" (working-route.md:100). Verified against `matrix.py coverage` at session start
  (verified 16/17, unclaimed v010-15 only). This chunk is expected to claim none; P4 re-reads the pool through
  `matrix.py show --unclaimed`.
