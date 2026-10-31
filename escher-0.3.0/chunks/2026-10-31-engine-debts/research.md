# Codebase Research — 2026-10-31-engine-debts

## Scope
- **Depth:** deep · **Reads:** 29 source and check files · **Globs/Greps:** 11 `git grep` calls; one throwaway probe target (see §Patterns detected)
- **Harness rules consulted:** none — no live leg in this chunk's research; the plan's gate entries read `.claude/rules/verification-harness.md` at P4's resume
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's word for this run · `inputs#I2`, `inputs#I3`, `inputs#I4` — the founder's 0.4.0 adaptation and its reconnaissance · `inputs#I5` — HTML's focus fixup with its blur note, inert's rules, HTML-AAM's checkbox and radio mapping, as a fetch agent quoted them · `inputs#I6` — AccessKit consumer's `validate_global` · `inputs#I7` — winit-core's `EventLoopProxy::new`

## Files inspected
- `packages/blitz-dom/src/scrolling.rs` (full) — `visible_region` at :744 walks `containing_block_chain` and reads each clipping box through `physical_unrounded_geometry`, no transform; `scroll_into_view`, `scroll_state`.
- `packages/blitz-dom/src/document.rs` (:1000-1065, :1710-1930, :2470-2650) — the removal path's focus reset, `snapshot_node_and`'s changed-set mark, `set_focus_to`, `clear_focus`, `get_client_bounding_rect`, the private `transformed_bounds` :2544 and `is_transformed_chain` :2589.
- `packages/blitz-dom/src/node/node.rs` (:430-456, :558-581, :745-802, :1420-1540, :1749-1777) — `set_transform`, `is_focussable` and `is_inert`, `focus` and `blur` with IME, `hit_inner`'s `t.inverse()` at :1472 and `matches_self` at :1482-1483, `containing_block`.
- `packages/blitz-dom/src/accessibility.rs` (full) — `focus` written from `focus_node_id` at :115 with no check it was built; `is_native_checkable` :409; the toggle at :190-202; `is_hidden_from_accessibility_tree` :259.
- `packages/blitz-dom/src/node/select.rs` (full) — `is_single_select`, `is_select_option` (parent must be a select), `options` (children only).
- `packages/blitz-dom/src/mutator.rs` (:279-533, :1030-1052) — `set_attribute` and `clear_attribute`, the `tabindex` · `href` · `disabled` hook, `flush`'s autofocus call of `set_focus_to`.
- `packages/blitz-dom/src/events/pointer.rs` (:730-1001) — `handle_click`: checkbox and radio dispatch `Input` alone; radio's unconditional `input` carries a TODO at :813.
- `packages/blitz-dom/src/events/keyboard.rs` (:120-330) and `events/focus.rs` (full) — activation, `press_select_key`, `apply_select_choice`, `generate_focus_events` (needs an event driver's dispatch).
- `packages/blitz-dom/assets/default.css` (:145-160) — `option { display: none }`, `select:not([multiple]) > option:checked { display: inline }`.
- `packages/dioxus-native-dom/src/snapshot.rs` (:150-265) — `checked` :169, `pressed` :178-181, `value`. `dioxus_document.rs:377` — `Input` and `Change` convert alike to `NativeFormData`, so an engine `change` reaches no `unimplemented!()` conversion.
- `packages/blitz-vibey-script/src/runtime/events.rs` (:165-192) — the script runtime already synthesises `change` after `input` on a checkbox or radio.
- `packages/escher-driver/src/lint.rs` (:240-430), `execute.rs` (:360-494), `refusal.rs` (:78-96), `schema.rs` (:165-175, :340-362).
- `packages/blitz-shell/src/event.rs` (full), `lib.rs` (:85-108), `application.rs` (:60-80) — the redraw route's hops.
- `examples/seven_guis/src/stand.rs` (full), `examples/seven_guis/src/tasks/cells.rs` (:368-412).
- `tests/blitz-tests/tests/{transformed_client_rect,inert_subtree,stand_load_page,stand_cells_served,pseudo_element_flow_classification,focus_redraw,stand_act_transformed}.rs` (full), `common/mod.rs` (full), `stand_keyboard.rs` (:575-605), `stand_contrast.rs` (:160-200), `docs/capabilities.md` (:160-180).

## Graph impact (from the code-graph query; trace `.andromeda/runs/2026-10-31T03-24-40-phase/tree-query-2026-10-31-engine-debts.json`)
- **`visible_region`** — 3 production callers: `in_view` @ `packages/escher-driver/src/execute.rs:454`, `crop` @ `execute.rs:465`, `clip_region` @ `packages/escher-driver/src/lint.rs:394`; 4 test callers. A fix moves the act gate's `off-screen`, the shot's crop and the lint's clip together.
- **`build_accessibility_tree`** — `accessibility_tree` @ `packages/blitz-dom/src/document.rs:197`, `handle_blitz_shell_event` @ `packages/blitz-shell/src/application.rs:82`, `accessibility_tree` @ `packages/dioxus-native-dom/src/dioxus_document.rs:298`.
- **`set_focus_to`** — production callers include `flush` @ `packages/blitz-dom/src/mutator.rs:1040`, the only focus move in the mutator; **`clear_focus`** — `handle_click` @ `events/pointer.rs:971`, script's `blur`, Dioxus `set_focus`, and no site in the mutator.
- **`is_select_option` · `options`** — read at `accessibility.rs:157,173,260`, `document.rs:739`, `keyboard.rs:176,280`, `form.rs:277`, `mutator.rs:1252`: the reach of an `optgroup` model.
- **`is_native_checkable`** — one caller, `build_accessibility_node` @ `accessibility.rs:190`.

## Patterns detected
Measured by a throwaway target, `tests/blitz-tests/tests/zz_p3_probe.rs`, run on the tree at `2109f1c2e`, stand options, both layout modes, every row reading alike in both, then removed; `git status --short -- tests packages examples` printed nothing afterwards.

- **Plain pan/zoom already reads right.** A `transform` on a world box holding absolutely positioned nodes, under an untransformed `overflow:hidden` viewport: translate, `translate + scale(2)` and `scale(0.5)` each read the target's centre inside its region and the hit on the target; a target panned out reads outside and is not hit; a driver `click` on `translate + scale(2)` is accepted. On that fixture the lint reads one finding, the zoomed world box `clipped` by the viewport, a true reading.
- **The limit fires where a box that clips lies under the transform.** An inner `overflow:auto` node body under a translated or a `scale(2)` world reads region (50, 30, 80, 40) with the target at (150, 80) or (100, 60): centre outside, hit on the target. Through the driver at `translate + scale(2)`: `click` refused `off-screen`, lint `off-screen` and `truncated` on the target. A clipping box whose own shown place happens to cover the target reads right by coincidence (`vp-transformed-*`, `world-clips-*`).
- **The fix's equality.** For a clipping box's padding box P and the mapping `transformed_bounds` already computes, region = viewport ∩ AABB(map(P)): exact under translate and scale, an over-reading under rotation (the AABB holds corners the rotated clip does not). `transformed_bounds` returns `None` with no transform on the chain, so a page with none reads as today. Not yet run: this equality is the plan's to prove.
- **A non-invertible transform swallows hits.** `scale(0)` and `matrix(0,0,0,0,0,0)` with no clip of their own: a hit at (300, 200) and (590, 390) answers the target inside, nothing painted there; with `overflow:hidden` on the same box, no hit. `t.inverse()` at `node.rs:1472` leaves NaN coordinates, and `matches_self`'s `!(x < 0 || …)` reads true on NaN. No panic. Client rect reads 0 × 0.
- **`inert` with focus held.** After `inert` lands on a focused button's box: engine focus still on it, snapshot `focused` count 0, the tree's `focus` id absent from its nodes, no `blur`; Enter still clicks it (count 1 → 2); a focused text input takes a typed character. Tab leaves, and the tree's focus is whole again. AccessKit's consumer panics when focus is not in the node list (`inputs#I6`, `src/tree.rs:89-94`); read from source, and no stand binary builds an adapter.
- **`:open` reads right on every path.** At boot, on `open`'s write, on its removal, on a `summary` click and on `summary` Enter, for `details` and `dialog` as each applies, both modes. `toggle_details_open` goes through the mutator.
- **`pseudo_element_flow_classification`.** The first of its three tests, re-run with `incremental: Some(_)`, reads y 200, height 50 in both modes. The other two not run.
- **The load page's rows are green.** Tab walk `back-btn` → `load-fetch`, wrapping; Enter and Space each leave a load in flight; contrast `load-fetch` 5.47 · hover 6.83 · active 8.27, `back-btn` 11.09, `task-title` 18.88, `load-done` 15.96, each required 4.5.
- **Cells fails SC 2.1.1.** Tab stays on `back-btn`; 0 of 676 cells focusable; a cell opens only on a pointer click, its editor then unfocused (`focused_on_input=false` after `harness.click`). Contrast: `back-btn`, `task-title` as above, `cell-input` 15.19; an empty cell reads `None`; a filled cell not measured.
- **Native checkedness moves little.** `checked` reads the element first and `pressed` filters on the role, so the fix moves `accessibility_states`' fourth test only, its four unchecked natives reading `Toggled::False` for nothing. The stand holds no checkbox or radio (`git grep` over `examples/seven_guis`), so no committed Counter check moves. A native with a non-checkable `role` attribute would begin to read `pressed` unless the write is gated on the role. The clicks already mark the changed set through `snapshot_node_and(CHECKED)`. No `indeterminate` model exists (`git grep indeterminate packages/blitz-dom/src`: 0), so HTML-AAM's `mixed` has no source.
- **The redraw route's headless reach.** `EventLoopProxy::new` takes a provider trait object at the locked winit-core (`inputs#I7`), so a blitz-shell unit test can read `BlitzShellProxy::send_event` → its `Receiver` and one `wake_up` with no window. `BlitzShellProvider::request_redraw` needs an `Arc<dyn Window>` and `application.rs:70` a live window: those two hops stay with the windowed boot smoke, a limit the check states. blitz-tests holds no `blitz-shell` dev-dependency, so the check lives in blitz-shell's own unit tests.
- **`0980cce5f`** — re-derived: `git show 0980cce5f`. It leaves `visible_region` alone. It changes `scroll_state`'s return to three values and every read of it, `physical_layout`'s signature in `writing_mode.rs`, `hit_inner`'s `matches_self` and `matches_content`, and `scrollable_overflow_rect`'s type behind a Taffy bump. A `visible_region` fix pre-empts nothing; a non-invertible-transform fix sits beside its `hit_inner` hunk, a conflict for Epoch 3's sync to resolve, not a collision of meaning.

## Conventions to follow
- **Widen the method itself** (`scroll_into_view`'s precedent): `visible_region` keeps its signature and `transformed_bounds` becomes `pub(crate)`.
- **A focus move's shape**: `snapshot_node_and` for `FOCUS | FOCUSRING` and each ancestor's `FOCUS_WITHIN`, then a redraw — `clear_focus` at `document.rs:1840` already does all of it, IME through `blur`.
- **`events/` arms stay private functions**, upstream parents at their floor (`click_button`'s precedent, `pointer.rs:985`).
- **Each new target**: `//!` doc, both layout modes, fixture condition first, stem matching `^[a-z0-9_]+$`, a non-`stand_` target joining `stand-repeat`'s literal `--test` list with its pin (`test_stand_repeat_leg_runs_three_rounds`).

## New files to create
- `tests/blitz-tests/tests/visible_region_transformed.rs` — F1: the pan/zoom shapes, the inner scroller, the degenerate row.
- `tests/blitz-tests/tests/hit_non_invertible_transform.rs` — F2's reading.
- `tests/blitz-tests/tests/open_state.rs` — CARRY 2's `:open` check.
- `tests/blitz-tests/tests/checkable_change_event.rs` — F10's reading on a Dioxus page.
- `tests/blitz-tests/tests/text_range_no_change_event.rs` — F11's stated limit, pinned.

## Files to modify
- `packages/blitz-dom/src/scrolling.rs` — F1: `visible_region` reads each clipping box through its transforms.
- `packages/blitz-dom/src/document.rs` — F1: `transformed_bounds` `pub(crate)`; F6, F7: `select_value`, `select_values`, `select_of`, the chosen set's writer.
- `packages/blitz-dom/src/node/node.rs` — F2: a non-invertible transform takes no hit.
- `packages/blitz-dom/src/mutator.rs` — F3: the fixup at the flush; F6, F7: a `multiple` write, the owner select through a group.
- `packages/blitz-dom/src/accessibility.rs` — F4, F5; F6: `multiselectable`; F7: a group's role and name.
- `packages/blitz-dom/src/node/select.rs` — F6, F7: the list through a group, the disabled rule, the set's rules.
- `packages/blitz-dom/src/events/keyboard.rs` — F6: the keys of a `multiple` select; the one write path takes a set.
- `packages/blitz-dom/src/events/pointer.rs` — F6: a row's click; F10: `change` after a checkbox's or radio's `input`.
- `packages/blitz-dom/src/form.rs` — F7: a group's disabled rule; a named select's entries, a test of their own.
- `packages/blitz-dom/assets/default.css` — F6, F7: rows and groups, in every document.
- `packages/blitz-vibey-script/src/runtime/events.rs` — F10: the script runtime's own synthesis removed.
- `packages/blitz-vibey-script/tests/dom.rs` — F10: a radio's `change` on an actual change alone.
- `packages/dioxus-native-dom/src/snapshot.rs` — F6, F7: a row's own bounds, a group's.
- `packages/dioxus-native-dom/src/dioxus_document.rs` — F6: `values()` on a `multiple` select's events.
- `packages/escher-driver/src/execute.rs` — F6: `type` on a `multiple` select focuses it with no click on a row.
- `packages/escher-driver/src/lint.rs` — F9: a closed single select's options exempt in `overlaps`.
- `packages/escher-driver/src/refusal.rs` — F1: `off-screen`'s text and its inline pins.
- `packages/escher-driver/src/schema.rs` — F6: `type`'s and `press`'s help.
- `packages/escher-driver/src/lib.rs` — F1: the crate docs' count of readings off.
- `packages/blitz-shell/src/event.rs` — CARRY 2: the redraw route's first hop, a unit test.
- `examples/seven_guis/src/tasks/cells.rs` — F13: the grid as one Tab stop with its keys.
- `docs/capabilities.md` — every limit his word moves, by hand; `## Where each claim is proven`.
- `tests/blitz-tests/tests/common/mod.rs` — F12, F13: the served pages' Tab orders.
- `tests/blitz-tests/tests/transformed_client_rect.rs` — F1: the limit's pin renamed and inverted.
- `tests/blitz-tests/tests/stand_act_transformed.rs` — F1: the driver's reading on the inner scroller.
- `tests/blitz-tests/tests/inert_subtree.rs` — F3, F4: the limit restated, a test added.
- `tests/blitz-tests/tests/accessibility_states.rs` — F5: the fourth test's natives restated.
- `tests/blitz-tests/tests/select_interaction.rs` — F6, F7: the set's keys and clicks, the groups.
- `tests/blitz-tests/tests/stand_act_select.rs` — F6, F7: a row driven by id, a closed option still refused.
- `tests/blitz-tests/tests/stand_lint.rs` — F9: `[0, 0, 0, 0]`, the flight booker's row, the rows linted.
- `tests/blitz-tests/tests/stand_keyboard.rs` — F12, F13: the served pages' Tab and activation rows.
- `tests/blitz-tests/tests/stand_contrast.rs` — F12, F13: the served pages' contrast rows.
- `tests/blitz-tests/tests/stand_load_page.rs` — F12: Enter through the driver waits out the load.
- `tests/blitz-tests/tests/stand_cells_served.rs` — F13: the keyboard's walk, the re-measured parts and findings.
- `tests/blitz-tests/tests/pseudo_element_flow_classification.rs` — CARRY 2: both layout modes.
- `.github/scripts/ci-leg.sh` — `stand-repeat`'s literal list, for each new non-`stand_` target.
- `.github/scripts/test_ci_workflows.py` — that list's pin.
- `Cargo.lock` — pulldown-cmark 0.13.4 → 0.13.5, RUSTSEC-2026-0414's fix, one package.

## Sweeps
- `visible_region` over `packages tests examples docs` (`git grep -c`): 13 hits in 8 files · changed under F1 A: `transformed_client_rect` 3 · no change: `scrolling.rs`'s own definition, `execute.rs` 2 and `lint.rs` 1 (callers keep their call), `scroll_into_view_nested` 2, `scrolled_box_client_rect` 2, `hit_clipped_at_scrolling_box` 1 and `stand_act_obstructed` 1 (no transform on their chains).
- `flight-type-one-way|\[0, 1, 0, 0\]` over `tests examples packages docs README.md` (`git grep -n`): 14 lines · changed under F9 A: `stand_lint.rs:282,289` · no change: 12 (an id in a stated table; F8 B alone moves `stand_act_ids.rs:60`).
- `remove_attribute` over `.andromeda/architecture-detail .andromeda/security-plan-amendments.md .andromeda/architecture-amendments.md` (`git grep -l`): 3 files · changed here: 0 · all three the wrap's amendment (CARRY 3).

## Reads added at P4, for the select model and Cells
Read at `2109f1c2e`, after his word (inputs#I8, inputs#I9).
- `packages/blitz-dom/src/node/select.rs` (full) — `options` walks children alone; `settled`, `stepped` and `matched` are pure over `OptionReading`, so a set's rules extend them.
- `packages/blitz-dom/src/events/keyboard.rs` (:96-297) — `press_select_key` gated on `is_single_select`; `apply_select_choice` the one write path, `set_select_choice` taking one target.
- `packages/blitz-dom/src/events/pointer.rs` (:736-982) — `handle_click`'s `select` arm; an option has no arm, and an unmatched walk clears focus.
- `packages/blitz-dom/src/mutator.rs` (:1025-1043, :1236-1257) — the flush, `set_option_selectedness`; a `select_nodes` entry comes only from a parent `select`.
- `packages/blitz-dom/src/form.rs` (:276-290) — a chosen option's entry; no test of it exists (`git grep -l construct_entry_list -- tests` prints nothing, and the harness holds no navigation provider).
- `packages/blitz-dom/assets/default.css` (:78-96, :151-159) — no `optgroup` rule; `picture` already takes `display: contents`.
- `packages/blitz-dom/src/layout/construct.rs` (:647) — a `::before`'s `content` reads `ContentItem::String` alone, so `content: attr(label)` paints nothing: a group's label stays unpainted, a limit rather than an engine change he was not asked.
- `packages/dioxus-native-dom/src/dioxus_document.rs` (:377-382) and `events.rs` (:310-332) — `NativeFormData { value, values: vec![] }`, where a Dioxus handler reads a set through `values()`; `FormValue::Text` (dioxus-html 0.7 `events/form.rs:178`).
- `packages/dioxus-native-dom/src/events.rs` (:233-305) — `MountedData::set_focus` is `set_focus_to` with no focus events (its TODO); `scroll_to` is `scroll_into_view`. The async handler form is `dioxus_document.rs:743`'s.
- `packages/escher-driver/src/execute.rs` (:212-231, :429-498, :546-555) — `type` clicks the centre first, which on a list lands on a row; `press` carries Shift alone, no Ctrl (`schema.rs`'s arguments), a new argument being a widening.
- `examples/seven_guis/src/tasks/crud.rs` (:57-90) and `cells.rs` (:347-410) — `crud-list`'s shape; Cells' `onblur` writes `edit_val` to its own cell, so a pointer click on another cell while an editor is focused writes the other cell's raw text there: reachable once the keyboard path focuses the editor.
- `tests/blitz-tests/checks/` — `git grep -c 'off-screen\|covered'` 0 hits, exit 1: no committed Counter check holds a moved text.

## Open questions
- None. F1 to F13 are answered (inputs#I8, inputs#I9). What his answers leave to the plan is stated there as a lean and shown on the review card.
