# Task — a11y-plan · 2026-10-31-engine-debts

## The session
Settled the recorded engine debts by the founder's forks, for every document: `visible_region` reads clipping boxes through transforms, `hit_inner` gives no hit under a non-invertible transform, a focus fixup under `inert`, a window fallback for the tree's focus, native `toggled`, and an engine `change` on checkbox and radio.
Built the `multiple` select (keys, pointer, form, tree, default sheet, snapshot, bridge) and `optgroup`; built no popup, `packages/blitz-paint/` shows no diff, and a closed single select's option stays refused `covered`.
Gave Cells' `cells-grid` one Tab stop with arrow keys and a current-cell outline, and added keyboard and contrast rows for Cells and the load page, closing the gaps a11y-plan holds open on the served pages and on `:open`.
Added 5 checks and edited 13; measured the `a11y` leg at 14 · 85 → 14 · 112 with its fourteen files unmoved, `stand-repeat`'s named targets ten → fifteen, and CI#41192433140 green 17/17 at `40ac6ddac`.
Retired the limits those forks fixed, `snapshot.rs`'s private `select_parent`, the script runtime's `change` synthesis, `inert_subtree`'s focus-fixup limit lines and `off-screen`'s transformed-box clause.
47 files changed; the list below holds 40, leaving out seven that carry no a11y-plan rule: `Cargo.lock`, `.github/scripts/test_ci_workflows.py`, `packages/blitz-vibey-script/tests/dom.rs`, `packages/escher-driver/src/lib.rs`, `packages/escher-driver/src/schema_tests.rs`, `tests/blitz-tests/tests/transformed_client_rect.rs`, `tests/blitz-tests/tests/pseudo_element_flow_classification.rs`.

## Reached files
- packages/blitz-dom/src/accessibility.rs
- packages/blitz-dom/src/mutator.rs
- packages/blitz-dom/src/node/select.rs
- packages/blitz-dom/src/node/node.rs
- packages/blitz-dom/src/document.rs
- packages/blitz-dom/src/scrolling.rs
- packages/blitz-dom/src/events/keyboard.rs
- packages/blitz-dom/src/events/pointer.rs
- packages/blitz-dom/src/form.rs
- packages/blitz-dom/assets/default.css
- packages/blitz-vibey-script/src/runtime/events.rs
- packages/blitz-shell/src/event.rs
- packages/dioxus-native-dom/src/snapshot.rs
- packages/dioxus-native-dom/src/dioxus_document.rs
- packages/dioxus-native-dom/src/lib.rs
- packages/escher-driver/src/execute.rs
- packages/escher-driver/src/lint.rs
- packages/escher-driver/src/refusal.rs
- packages/escher-driver/src/schema.rs
- examples/seven_guis/src/tasks/cells.rs
- docs/capabilities.md
- .github/scripts/ci-leg.sh
- .github/workflows/ci.yml
- tests/blitz-tests/tests/common/mod.rs
- tests/blitz-tests/tests/accessibility_states.rs
- tests/blitz-tests/tests/inert_subtree.rs
- tests/blitz-tests/tests/select_interaction.rs
- tests/blitz-tests/tests/stand_act_select.rs
- tests/blitz-tests/tests/stand_act_transformed.rs
- tests/blitz-tests/tests/stand_keyboard.rs
- tests/blitz-tests/tests/stand_contrast.rs
- tests/blitz-tests/tests/stand_lint.rs
- tests/blitz-tests/tests/stand_load_page.rs
- tests/blitz-tests/tests/stand_cells_served.rs
- tests/blitz-tests/tests/stand_non_lean_tasks.rs
- tests/blitz-tests/tests/visible_region_transformed.rs
- tests/blitz-tests/tests/hit_non_invertible_transform.rs
- tests/blitz-tests/tests/open_state.rs
- tests/blitz-tests/tests/checkable_change_event.rs
- tests/blitz-tests/tests/text_range_no_change_event.rs

## Touches
- Focus order (keyboard-focus-order, `.andromeda/a11y-plan.md:107`, `.andromeda/a11y-plan-detail/keyboard-navigation.md:8`) and `.andromeda/a11y-plan-detail/a11y-scope-summary.md:23`: the statement that HTML's focus fixup is not built, and that a node focused as it becomes inert keeps focus, is retired, along with its residual owner.
- Role derivation (strategy, `.andromeda/a11y-plan.md:35`): the statements that nothing else writes `toggled` and that the tree never states a native checkbox or radio checked are retired, along with the residual owner of that fix; the report's `native-toggled` sweep moves 9 of 10 sites across the masters without listing them all.
- ARIA state attributes (aria-patterns, `.andromeda/a11y-plan.md:89`): the mapping points at Role derivation, which now holds a native write beside the attribute reads.
- Single-choice select keys (keyboard-handling, `.andromeda/a11y-plan.md:123`, `.andromeda/a11y-plan-detail/keyboard-navigation.md:38`): the statement that a `multiple` select and an `optgroup`'s options are not modelled is retired; no popup is painted stays true beside the moved limit.
- Pointer click rule (keyboard-focus-restoration, `.andromeda/a11y-plan.md:113`, `.andromeda/a11y-plan-detail/keyboard-navigation.md:25`): `handle_click` gains an `option` arm for a `multiple` select's rows, and the option-click limit narrows to a closed select's option.
- Enter and Space activation (keyboard-handling, `.andromeda/a11y-plan.md:125`): indirect, a checkbox or radio reached by Space runs through `handle_click`, which now dispatches `change` after `input`.
- Tree focus (sr-tree, `.andromeda/a11y-plan-detail/screen-reader-support.md:52`): gains the window fallback; the other focus-agreement sites stay true.
- Screen reader sites for the select (`.andromeda/a11y-plan-detail/screen-reader-support.md:11`): the `multiple-select` limit moves there, while the closed select's options keeping its client rect stays true at `:11` and `:45`.
- Snapshot model (sr-snapshot, `.andromeda/a11y-plan.md:192`): indirect, `value` reads `select_value`, now a `multiple` select's first chosen option in tree order, and `pressed` never begins to read on a native control.
- Actionable keys and reacting elements (sr-actionable-keys, `.andromeda/a11y-plan-detail/screen-reader-support.md:38`): Cells' reacting-element count moves 676 → 677 because `cells-grid` joins the 676 data cells; the site also gains `cells-grid`; 676 data cells stays a true count.
- Hidden content and visibility exception (aria-patterns and visual-visibility, `.andromeda/a11y-plan.md:91`, `:174`): indirect, `is_select_option` now holds under an `optgroup`, so the `option` under a `select` exception can need restating; the sheet adds group and row display rules.
- Removing `inert` (visual-visibility, `.andromeda/a11y-plan.md:174`): the inert path now also clears focus at flush, and `inert_subtree::removing_inert_restores_focus_hit_and_tree` lost its focus-fixup limit lines with its name and first assertions standing.
- `changed_nodes` statement (strategy, `.andromeda/a11y-plan.md:29`): indirect, a `multiple` write or removal records its select and an `optgroup` write records its select through `maybe_record_node`; check the list of what marks the set.
- `multiple-select` no-change sites (`.andromeda/a11y-plan-detail/a11y-strategy.md:10`, `.andromeda/a11y-plan-detail/keyboard-navigation.md:64`): the report read them unmoved; a popup token at `.andromeda/a11y-plan-detail/aria-patterns-roles.md:39` is also unmoved.
- OPEN on the served pages (keyboard-harness, `.andromeda/a11y-plan.md:142`): closes, with `load-fetch` and the load page's rows now asserted for SC 2.1.1 and 1.4.3, and its residual owner absorbed.
- `:open` gap (`.andromeda/a11y-plan-detail/keyboard-navigation.md:28`, `.andromeda/a11y-plan-detail/visual-design-verification.md:56`): the no-test-asserts wording retires, since `open_state` 4 asserts the `open` attribute read by presence and `:open` matching.
- Keyboard harness inventory (keyboard-harness, `.andromeda/a11y-plan.md:62`, `:140`, `.andromeda/a11y-plan-detail/keyboard-navigation.md:102`): `stand_keyboard` no longer covers the four lean tasks alone, and `select_interaction` 7 → 18, `stand_act_select` 5 → 10.
- Contrast harness inventory (visual-contrast, `.andromeda/a11y-plan.md:64`, `:156`, `.andromeda/a11y-plan-detail/visual-design-verification.md:14`): `stand_contrast` 5 → 7 tests with Cells and load page coverage; its row-table counts and citations can have moved; `a11y-tally` also changes at `:14`.
- Visual lint (visual-contrast, `.andromeda/a11y-plan.md:158`, `.andromeda/a11y-plan-detail/visual-design-verification.md:15`): `stand_lint`'s `at_boot` `[0, 1, 0, 0]` → `[0, 0, 0, 0]`, since `overlaps` skips a closed select's options.
- Harness test inventory (`.andromeda/a11y-plan.md:53`, `:55`): `accessibility_states` 5 → 6, `inert_subtree` 4 → 6; the report warns its patterns miss the N tests form, and `:53` holds that form.
- CI counts (ci, `.andromeda/a11y-plan-detail/ci-integration.md:10`, `:11`): `a11y` tally 14 · 85 → 14 · 112, `stand_keyboard` 15 → 21, `stand_contrast` 5 → 7, `inert_subtree` 4 → 6, `accessibility_states` 5 → 6, `select_interaction` 7 → 18, `stand_act_select` 5 → 10.
- CI leg lists (ci, `.andromeda/a11y-plan.md:237`): the `a11y` leg's fourteen files are unmoved; `stand-repeat` names five more targets, which the report does not name, so read the eight-of-them count against that without assuming it moved.
- Line citations into edited files (`.andromeda/a11y-plan.md:32`, `:33`, `:53`, `:55`, `:62`, `:64`, `:85`): `accessibility.rs`, `accessibility_states.rs`, `inert_subtree.rs`, `stand_keyboard.rs` and `stand_contrast.rs` all gained lines, so their cited ranges can be off.
- Residual owner clauses across a11y-plan: residual lines 31, 32, 33, 39 and 40 are absorbed into `v030-04`, and line 34 but for its popup half; a clause naming one of those lines as owner of a now-built fix no longer holds; the windowed AT witness line stands.
- Disabled-ness (keyboard-focus-order, `.andromeda/a11y-plan.md:109`): indirect, the statement stands for form controls, and `option_disabled` is now the one rule for an option under its group, shared by `readings` and `form.rs:281`.
- Focus ring (visual-focus-ring): indirect, the group's standing text may state the stand's focus cues, and Cells now holds a current-cell outline on grid focus.
- State colour (visual-state-color): indirect, a chosen row now reads an engine-default pair; the group's standing text may state how a choice reads.
- CRUD listbox (keyboard-shortcuts, `.andromeda/a11y-plan.md:133`): indirect, lint reads the element and never the role, so CRUD's `role="option"` rows are not read as a select's; the CRUD key rule is unmoved.

## New facts
- F1 · keyboard-navigation · On a `multiple` select, ArrowDown, ArrowUp, Home and End alone choose that row alone, with Shift span from the anchor, and with Ctrl or Meta move the current row and change nothing.
- F2 · keyboard-navigation · On a `multiple` select, Space with no typed text standing flips the current row and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone.
- F3 · keyboard-navigation · Disabled rows of a `multiple` select are never chosen, and Ctrl+A is not modelled.
- F4 · keyboard-navigation · Shift with no cursor yet spans from the last chosen row, so a set the markup seeds is stepped and spanned from its last chosen row.
- F5 · keyboard-navigation · `click_select_row`: a disabled select stops the click; otherwise it focuses the select and clears the typeahead and text selection; plain chooses the row alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing.
- F6 · keyboard-navigation · `optgroup`: a select's options include its groups' options in tree order, reachable by keys and text, with a disabled group's rows treated as disabled through `option_disabled`.
- F7 · keyboard-navigation · Enter and Space open nothing on a single select, and an option of a closed single select has no click arm and stays refused `covered`, under a group too.
- F8 · keyboard-navigation · A `multiple` write or removal records the select, so removal settles it to one, and `set_option_selectedness` leaves a `multiple` select's other options alone.
- F9 · keyboard-navigation · Focus fixup: `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, covering a subtree moved under an inert box, firing no DOM `blur` or `focusout`; `disabled` and `hidden` read as before.
- F10 · keyboard-navigation · After `input`, `handle_click` dispatches `change` with the same value: a checkbox always, a radio only where it was not checked before the click; a page's script hears one `change`, from the engine.
- F11 · keyboard-navigation · The driver's `type` on a `multiple` select focuses it with no click and clears the typeahead, and a text opening on a space types nothing; no verb, argument, key name, result field, cause, session error or exit is added.
- F12 · keyboard-navigation · The driver's `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver, a limit its help states and a residual line owns.
- F13 · keyboard-navigation · Cells: `cells-grid` is one Tab stop; arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`.
- F14 · keyboard-navigation · Cells' editor: Enter on the grid opens the editor on `current`, focused; its Enter commits, stops propagation and returns focus to the grid; a click inside a cell's own open editor returns early, so typed text stays.
- F15 · keyboard-navigation · `tests/blitz-tests/tests/common/mod.rs` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, the stated Tab sequences of the load page and Cells.
- F16 · keyboard-navigation · Hazard: a `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed` (`events.rs:162`), measured headlessly, so Cells calls its handles synchronously.
- F17 · aria-patterns-roles · `optgroup` maps to `Role::Group`, its non-empty `label` attribute its name, and a group's label is named in the tree and unpainted.
- F18 · aria-patterns-roles · A `multiple` select reads `set_multiselectable` on its `ListBox`.
- F19 · a11y-strategy · A native checkbox or radio on a checkable role writes `Toggled::True` or `False` from `checkbox_input_checked()`; under any other role it writes nothing, and neither author attribute is read on a native control.
- F20 · screen-reader-support · `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build.
- F21 · screen-reader-support · A `multiple` select's rows and groups read their own client rects in the snapshot, unlike a closed select's options.
- F22 · screen-reader-support · `evt.values()` reads a `multiple` select's set through `multiple_select_values`, one form entry per chosen option on `Input` and `Change`, while `value` stays `select_value`.
- F23 · screen-reader-support · `cells-grid` takes `aria_label` Cells and no role; role and cell semantics stay out by residual line 24.
- F24 · visual-design-verification · Engine default chosen-row pair: `select[multiple]` options under `:checked` read `#c8c8c8` on `#000000`, Chromium's inactive selected pair, 12.56:1 measured by `select_interaction`.
- F25 · visual-design-verification · Cells' focus cue: `.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5` at `outline-offset: -2px`, the accent's held value, asserted through `stand_contrast`.
- F26 · visual-design-verification · A transform whose determinant is 0 or not finite answers `None` from `Node::hit_inner` for the whole subtree, so that subtree takes no hit.
- F27 · ci-integration · `accessibility_states` holds a row moving a native control in the tree by a click and a Space, read through `&dyn Document`, and `inert_subtree` asserts the fixup and the tree's window fallback; what an assistive technology reads stays unmeasured.

## Untouched
- Platform adapters: `Adapter` creation and the per-platform gaps live in `packages/accesskit_xplat/`, which the session did not edit.
- The accessibility feature's reach and the absence of an adapter on the stand: manifests unedited, and the `accesskit_(xplat|winit)` tree reading stays 0.
- The contrast formula and thresholds: `contrast_ratio`, `Harness::text_contrast` and 3.0 and 4.5 live in `packages/blitz-test-harness/`, which the session did not edit.
- Upstream builder test counts: `accessibility_label` 13, `accessibility_description` 3, `accessibility_merged_states` 3 and `accessibility_active_descendant_unread` 1, whose files did not change.
- `accessibility_hidden`, `accessibility_roles` and `focusability_updates`: files and their cited helpers unedited.
- Harness focus queries at `packages/blitz-test-harness/src/inspect.rs` and the Tab check at `tests/blitz-tests/tests/stand_snapshot_state.rs`.
- The obs log format note in §3: no log site, print or sink change on the added lines, and the `focussed node` needles read 0.
- The flight booker's invalid cue and `aria-invalid` dates (cognitive-error-recovery), and the stand's statement of its attribute-only control states: its task file did not change.
- Language and direction, and text alternatives on fixtures (sr-language, sr-text-alternatives): no fixture markup of theirs moved.
- Motion: the observed absence of `prefers-reduced-motion` and `prefers-color-scheme` in the searched s02 files; the session adds no motion, and reduced motion is the popup chunk's fork.
- The tier: `0 — SC 2.1.1 · 1.4.3 · 2.4.3` stands, the session asserting within it.
- §10, §11 and §12: each reads no recorded intent, and the session records none.
