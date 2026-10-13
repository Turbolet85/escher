# Codebase Research — 2026-10-13-range-input-interaction

## Scope
- **Depth:** deep (engine event paths, the Dioxus bridge, the driver's click point, the standing checks that touch the
  slider) · **Reads:** 19 · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole (42 lines), 1 Session Addition (a
  `timeout`-bounded smoke reads as the gate's own bound; not applicable: no smoke here) · `.claude/rules/testing.md` —
  read whole (57 lines), 7 Session Additions applied (no platform-bound key through `press` without reading the
  `#[cfg]` lines; search `tests/` and `examples/*/tests/` for a pinned behaviour before calling a check unaffected; a
  hand mutation control's restore by sha256 — not used, the mutation tier stays unstarted, inputs#I1)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator relays the founder: no mutation runs (the mutation tier stays
  unstarted); pipeline tools run bare or redirected to a file read whole

## Files inspected
- `packages/blitz-dom/src/layout/construct.rs` (430-470) — the input special-casing at 437-458: `textarea` and the text
  types get `create_text_editor`, `checkbox`/`radio` get `create_checkbox_input`, `range` gets nothing.
- `packages/blitz-dom/src/events/keyboard.rs` (full, 175 lines) — `handle_key_or_input_event`: Tab moves focus (23-41);
  otherwise a key acts only when the target is the focused node AND it holds `text_input_data` (68-100). A range
  input holds none, so every key on it is dropped. `apply_generated_text_input_event`'s `Input` arm (119-127) is the
  house shape for an `input` event: mark `changed_nodes`, dispatch `DomEventData::Input(BlitzInputEvent { value })`,
  request a redraw.
- `packages/blitz-dom/src/events/pointer.rs` (375-540, 560-849) — `handle_pointerdown` hits the point (`doc.hit(x, y)`,
  413), maps a text input's click into its content box from the hit's node-local `hit.x`/`hit.y` minus
  `padding + border` (459-462, 506-507), and treats `disabled` presence as `ClickTarget::Disabled` (457). `handle_click`
  (614-827): a `disabled` element matches and does nothing (635-638); checkbox (645-666) and radio (667-695) set state,
  dispatch `Input`, and focus the control; no arm matches a range input, so the walk ends unmatched and focus is
  cleared (814-817) — the measured "a click does not focus it".
- `packages/blitz-dom/src/events/mod.rs` (60-315) — `handle_dom_event`: `PointerDown` → `handle_pointerdown` (192),
  `Click` → `handle_click` (215), `KeyDown` → `handle_key_or_input_event` (218); `Input` has no default action
  (242-244).
- `packages/blitz-dom/src/mutator.rs` (274-330) — `set_attribute` snapshots the node, restyles, marks ancestors dirty
  and inserts the node into `changed_nodes` when it is in the document (321-325).
- `packages/blitz-dom/src/accessibility.rs` (120-140, 285-310) — `range` → `Role::Slider` (302); the builder sets no
  numeric value on any node (no `set_numeric_value` in the file; `set_value` only on a `TextRun`, 132).
- `packages/blitz-dom/assets/default.css` (47-106) — `input` gets `border: 1px solid #999; padding: 2px; background-color:
  white` (76-81) and the `#4D90FE` focus outline (93-96); there is no range rule, and blitz-paint names no range input
  (grep `"range"` over `packages/`: hits only in `accessibility.rs:302` and a snapshot unit test), so a range input
  paints as an empty bordered box — no track, no thumb.
- `packages/dioxus-native-dom/src/dioxus_document.rs` (360-400) — `DomEventData::Input(data)` becomes
  `NativeFormData { value: data.value.clone(), values: vec![] }` (377-380): an engine-dispatched `Input` carrying the new
  value reaches the app's `oninput` with no bridge change.
- `packages/dioxus-native-dom/src/snapshot.rs` (11-60, 198-225) — `NodeState.value`: an input with no editor reads its
  `value` attribute (219-221); a range input is such an input.
- `packages/escher-driver/src/execute.rs` (117, 287-366, 472, 499-500) — `action_point` (351-366): a `click` lands at the
  centre of the target's bounds plus the viewport scroll, after the `disabled` · `off-screen` · `covered` refusals;
  `press` maps `arrow-right`/`home`/… to `keyboard_types::Key` (499-500).
- `packages/escher-driver/src/command.rs` (45-95, 470-480) — the twelve `KEY_NAMES`, the arrows, `home` and `end` among
  them; no range text anywhere in escher-driver (grep `-i "range input\|slider"` over `packages/escher-driver/src/`: 0).
- `packages/blitz-test-harness/src/input.rs` (98-121) — `click(selector)` clicks `center_of`; `click_at(x, y)` dispatches
  `PointerDown` + `PointerUp` at page coordinates and pumps — the route to a click at a chosen point on the track.
- `packages/blitz-test-harness/src/harness.rs` (160-176) — `dispatch_recorded` records the name of every DOM event
  dispatched to application code (an `HtmlDocument` check can read that `input` fired).
- `packages/blitz-dom/src/node/node.rs` (1928-1953) — `synthetic_click_event_data`: a label's synthetic click lands at
  the input's centre (from `absolute_position`).
- `examples/seven_guis/src/tasks/timer.rs` (76-103, 151-175) — the slider: `min 0`, `max 30`, `step 0.5`,
  `value: "{duration()}"`, `oninput` parses `evt.value()` as `f64` into `duration` (87-91); `.timer-slider { flex: 1;
  accent-color: #4a6cf7 }` (164-167), no padding or border of its own, so the default sheet's symmetric 1px + 2px
  applies.
- `tests/blitz-tests/tests/stand_act_range.rs` (1-60 of 107) — pins today's no-op in both layout modes, sequences A
  (click then four keys) and B (two Tabs then four keys); it is rewritten here.
- `tests/blitz-tests/tests/stand_act_ids.rs` (1-55) — `changes_at_boot(LeanTask::Timer) => &[]` (40): the slider's
  driver click is pinned accepted with an EMPTY diff, equal to a twin's selector click.
- `tests/blitz-tests/tests/stand_snapshot_state.rs` (319-335) — the slider reads role Slider and value `15` at boot
  (unchanged by this chunk: no boot reading moves).
- `escher-0.1.0/chunks/2026-10-07-act-by-id/evidence/range-input.md` (whole) — the measured starting point; still true
  at HEAD by the code paths above (no range arm in the key or click paths).

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-13T19-01-34-phase/tree-query-2026-10-13-range-input-interaction.json`)
- **handle_click** — callers `events/handle_dom_event()` at `packages/blitz-dom/src/events/mod.rs:215` and itself
  (the label arm) at `pointer.rs:731`; no signature change is planned, so the change is a new arm inside it.
- **handle_key_or_input_event** — callers `events/handle_dom_event()` at `events/mod.rs:218` (`KeyDown`) and `:232`
  (`AppleStandardKeybinding`); a new branch inside it, no signature change.
- **handle_pointerdown** — caller `events/handle_dom_event()` at `events/mod.rs:192`; a new arm of its `ClickTarget`
  match, no signature change.
- **handle_dom_event** — called from `BaseDocument::handle_dom_event` (`document.rs:661`) and
  `EventDriver::run_default_action` (`events/driver.rs:425`); not changed.
- Harness `click` / `press` / `click_at` / `center_of` — 127 call sites over 27 files (trace query 2); the sweep below
  narrows them to the two checks that touch the slider.

## Patterns detected
- **Form-control default action in `handle_click`** (`pointer.rs:645-695`): set the state, dispatch `Input` with the new
  value as text, then focus the control through `generate_focus_events`.
- **Point-in-control from the hit** (`pointer.rs:459-462, 506-507`): `hit.x - (padding.left + border.left)` is the
  click's offset into the content box, already correct under scrolled boxes since 2026-10-10 (the hit stops at and
  reads each clipping box where it stands).
- **`input` event shape** (`keyboard.rs:119-127`): mark changed, dispatch `Input { value }`, request redraw.
- **Value in an attribute** (`pointer.rs:776-777`): the file input writes its chosen path into the `value` attribute in
  the click default action — a precedent for an engine default action that writes `value` (it reaches past the mutator;
  this chunk uses `doc.mutate().set_attribute`, which also marks the changed set).

## Conventions to follow
- **Spec citation beside the code**: arch §Design Philosophy — cite the HTML "range state" value sanitization and the
  step rules (`https://html.spec.whatwg.org/multipage/input.html#range-state-(type=range)`) at the implementation.
- **No panic on markup**: parse `min`/`max`/`step`/`value` fallibly to finite `f64`, defaults 0 / 100 / 1 / the
  midpoint (spec default value); `step` ≤ 0, non-numeric or `any` → 1 for a key step (spec: `any` steps by 1); `max <
  min` → the value is `min`.
- **Stale ids**: resolve the focused or hit node with `get`, never by index (arch §Standard Contracts → Scrolling,
  selection, tree).
- **Engine tracing**: none added; if one were, `#[cfg(feature = "tracing")]` with `node_id` only (obs-plan §8).
- **Tests**: one blitz-tests file per behaviour with a `//!` doc, `for incremental in [false, true]`, the condition
  asserted first; stand driver checks through `session_common` (`hold`, `act`, `click`, `press`).

## New files to create
- `packages/blitz-dom/src/node/range.rs`
- `tests/blitz-tests/tests/range_input_interaction.rs`

## Files to modify
- `packages/blitz-dom/src/node/mod.rs`
- `packages/blitz-dom/src/node/element.rs`
- `packages/blitz-dom/src/layout/construct.rs`
- `packages/blitz-dom/src/mutator.rs`
- `packages/blitz-dom/src/events/keyboard.rs`
- `packages/blitz-dom/src/events/pointer.rs`
- `packages/blitz-dom/src/form.rs`
- `packages/dioxus-native-dom/src/snapshot.rs`
- `tests/blitz-tests/tests/stand_act_range.rs`
- `tests/blitz-tests/tests/stand_act_ids.rs`

## Resolved at P4 (the founder's answers, inputs#I2, inputs#I3)
The three open questions were answered by the founder (relayed verbatim by the overseer as his chosen option labels;
the English wording is the overseer's): Q1 «Отдельное текущее значение» — a separate dirty value; Q2 «Стрелки + Home/End
(Рекомендую)»; Q3 «Клик даёт фокус (Рекомендую)». Q1 widens the write set past the P3 lists; the reads that place it:
- `packages/blitz-dom/src/node/element.rs` (325-420, 500-640, 893-913) — `SpecialElementType` is `#[non_exhaustive]`
  (332-345) and has no reader (grep `SpecialElementType::` over `packages/ tests/ apps/ wpt/ examples/`: 0);
  `SpecialElementData` is not (348-373) — its exhaustive matches are the `Clone` impl (375-391) and the `Debug` impl
  (893-913) here and the removal walk in `mutator.rs:1080-1103` (grep `SpecialElementData::None =>` over `packages/
  apps/ wpt/ examples/`: only blitz-dom); the per-variant accessors (`checkbox_input_checked` 594-608) are the shape for
  a range accessor. Outside blitz-dom only blitz-paint names the enum, by `let … else` on `TableRoot` (`border.rs:513`,
  `background.rs:231`) — no exhaustive match.
- `packages/blitz-dom/src/layout/mod.rs` (376-472) — the `special_data` match runs only for `is_replaced_element`
  (`layout/replaced.rs:10-17`: img, svg, canvas, video, embed, iframe); an `input` never reaches it, so its
  `_ => unreachable!()` (471) holds for a new variant.
- `packages/blitz-dom/src/layout/construct.rs` (917-925) — `create_checkbox_input` seeds its data only when the element
  does not already hold it: the precedent for seeding a range's current value once, so a re-construction keeps it.
- `packages/blitz-dom/src/mutator.rs` (274-380) — a `value` attribute write sets a text input's editor text
  (`input_data.set_text`, 359-368): the precedent for an attribute write re-seeding a control's current value — the
  "reset of the dirty value on a Dioxus re-render's attribute write" the founder's choice takes on (inputs#I3).
- `packages/blitz-dom/src/form.rs` (305-320) — the entry list reads a text input's editor, else the `value` attribute
  (316-319): with a separate current value, a moved range would submit its default unless this reads it too.
- `packages/dioxus-native-dom/src/snapshot.rs` (198-225, 591-605) — the reader and the unit test
  `a_range_input_reads_its_value_attribute`, which the new reader renames and keeps (a boot value reads the same).

## Sweeps
- `timer-duration\|"range"\|type="range"\|r#type: "range"` over `tests/ examples/ packages/` (`*.rs`): 12 files ·
  2 changed (`stand_act_range.rs`; `stand_act_ids.rs` only if a click focuses the slider — P4 fork) · 10 no-change
  (`accessibility_roles.rs`, `stand_accessibility_ids.rs`, `stand_element_ids.rs`, `stand_contrast.rs`,
  `stand_snapshot_state.rs`, `stand_timer.rs` and `common/mod.rs` read the slider at boot only;
  `circle_drawer.rs` holds a range input in its dialog, driven by no check — grep `circle_drawer\|CircleDrawer` over
  `tests/ examples/seven_guis/tests`: 0; `accessibility.rs` and `snapshot.rs` are readers this chunk does not change).
- `arrow-\|ArrowRight\|ArrowLeft\|Key::Home\|Key::End\|"home"\|"end"` over `tests/blitz-tests/tests
  examples/seven_guis/tests`: 2 files · 1 changed (`stand_act_range.rs`) · 1 no-change (`button_alignment.rs` — its
  hit is CSS `text-align: end`, no key).
- `controls(\|click(\|timer-duration` over the seven Timer-holding checks (`stand_id_persistence`, `stand_act_spans`,
  `stand_mcp_tools`, `stand_snapshot_state`, `stand_diff`, `stand_session_state`, `stand_settle`): 0 click the slider
  — no change.
- `-i "range input\|slider"` over `packages/escher-driver/src/` and `examples/seven_guis/{src,tests}`: 0 driver or help
  text names a range input — no schema text moves.
- Keyboard conflicts: `ArrowDown\|ArrowUp\|Key::Home\|Key::End\|PageDown` over `packages/blitz-dom/src` outside the
  editor: hits only in `node/text.rs` (the editor) — no viewport keyboard scrolling to compete with a range key step.
- Upstream: `git log --oneline -i --grep=range upstream/main` (434e0661): 5 commits, none a range input interaction —
  nothing upstream to merge instead.

## Open questions
All three were answered at P4 (see Resolved at P4 above); kept as asked:
- Where the moved value lives — the `value` attribute written through `doc.mutate().set_attribute` (the snapshot and the
  changed set read it with no new reader) or a new `SpecialElementData` variant holding a dirty value apart from the
  default (spec-faithful, a public type change and a construct.rs edit) → blocks: plan-decision
- Which keys step — the four arrows (right/up increase, left/down decrease) with or without Home/End (both already
  `KEY_NAMES`), or right/left only → blocks: plan-decision
- Whether a pointer click focuses the range input, as the checkbox/radio arms do (`pointer.rs:658-664`) — changes the
  measured reading and moves `stand_act_ids`' pinned Timer row (`changes_at_boot` 40) → blocks: plan-decision
