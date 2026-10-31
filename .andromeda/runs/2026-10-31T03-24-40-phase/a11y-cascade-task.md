# Task — a11y · 2026-10-31-engine-debts

## The chunk
Takes six recorded engine debts, each to the founder as a fork at P4 and each ending as his word says: fixed and proven on the stand or a fixture, pinned as a stated limit, or kept as a residual. No answer is assumed; any fix below is conditional on his word.
The debts: a transformed scrolling box's `visible_region`; the missing `inert` focus fixup; native checkbox and radio checkedness absent from the tree; the select model's stated limits; the select-option `overlap` in `lint`; and the served pages' missing Tab-order, keyboard-activation and contrast rows.
Adds checks for three behaviours an upstream take brought: `:open` on `details` and `dialog`, the windowed shell's redraw route, and both layout modes in `pseudo_element_flow_classification`.
Re-asserts SC 2.1.1, 1.4.3 and 2.4.3 through the standing witnesses `stand_keyboard`, `stand_contrast`, `inert_subtree` and `select_interaction`; the served pages' rows are the ground of the route's later a11y re-read.
Adds no new verb, node key or refusal cause, sends nothing upstream and bumps no dependency.

## Reached files
- packages/blitz-dom/src/scrolling.rs · edits `visible_region`
- packages/blitz-dom/src/accessibility.rs · edits the `toggled` write for a native checkbox or radio
- packages/blitz-dom/src/node/node.rs
- packages/blitz-dom/src/events/
- packages/escher-driver/src/lint.rs · edits `overlaps`
- packages/escher-driver/src/refusal.rs · edits the meaning text of `off-screen` and `covered`
- tests/blitz-tests/tests/transformed_client_rect.rs · edits `a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`
- tests/blitz-tests/tests/inert_subtree.rs · edits `removing_inert_restores_focus_hit_and_tree`
- tests/blitz-tests/tests/stand_lint.rs · edits the pinned `[0, 1, 0, 0]`
- tests/blitz-tests/tests/stand_keyboard.rs · edits the served pages' Tab-order and keyboard-activation rows
- tests/blitz-tests/tests/stand_contrast.rs · edits the served pages' contrast rows
- tests/blitz-tests/tests/pseudo_element_flow_classification.rs · edits its run `for incremental in [false, true]`
- tests/blitz-tests/tests/select_interaction.rs · re-reads the standing witness of the select's interaction
- tests/blitz-tests/tests/incremental_oracle.rs · re-reads incremental and non-incremental layout staying identical
- .andromeda/residuals.md · edits lines 31, 32, 33, 34, 39 and 40
- .andromeda/architecture-detail/standard-contracts.md · edits the `ElementData::classes` row (the wrap's amendment)

## Needs
- `visible_region` and what it reads for a target inside a scrolling box under a CSS `transform`: translate as recorded, and scale as P3 measures it, on a pan/zoom container holding absolutely positioned nodes.
- Visibility and hit-testing as the driver reads them: a driver call refused `off-screen` or `covered` though the target shows, and the stated limits both refusals carry, which a fix moves along with the capability statement's pin.
- Focus order and focusability where a node or an ancestor becomes inert: HTML's focus fixup not built, focus kept until it moves, SC 2.4.3, and the recorded ownership of that fix.
- `inert` as a whole: skipped by Tab and Shift+Tab, passed through by a hit, absent from the tree, restored on removal; a fixup must leave these as `inert_subtree` asserts them.
- Agreement of the engine's focus, the snapshot's `focused` node and the tree's focus at every step, which a focus fixup must keep (indirect, through `stand_keyboard`).
- The accessibility builder's state writes: `toggled` from `aria-pressed` and `aria-checked` over the five checkable roles, nothing else writing it, and the recorded ownership of the native checkbox and radio gap.
- `accessibility_states`' tables, which assert unchecked natives reading neither attribute and `aria-pressed` on a checkable role reading nothing; a checkedness fix must stay inside them or move them openly.
- The snapshot's readings: `checked` from a native's state, `selected`, `pressed` and `invalid` from the tree; whether a checkedness fix moves `pressed` is P3's to read (indirect, the same node property).
- A native's checked state crossing to the platform adapter as a closed value, and the tree the shell hands the adapter equalling the harness's tree node for node.
- The refresh path: a focus or checked change marks `changed_nodes`, so a fixup or a checkedness fix reaches the rebuilt tree through it; a check reading `has_changes()` drains first (indirect).
- Select keyboard handling: a focused single-choice select's keys and typeahead, one `input` then one `change` per change, a disabled select's keys ignored, and the stated limits on `multiple`, `optgroup` and the unpainted popup.
- A select's options in tree and snapshot: the chosen state, the carve-out of an option from `display: none`, and each `ListBoxOption` carrying its select's client rect.
- The driver `click` on an option refused `covered`, and `stand_act_select`'s reading of it.
- A checkbox, radio or text input firing no engine `change`, and any record of that event's absence.
- The space activation of a native checkbox or radio, as the route by which a key reaches a state a checkedness fix or a `change` fix would read.
- The select-option `overlap` on the flight booker at boot (`flight-type-one-way` × `flight-type-return`) and how a painted popup would change what an option's bounds read.
- The visual lint's findings written as ids, closed words and numbers only, which a change to `overlaps` must keep.
- The served pages' OPEN gap: Cells and the load page holding no Tab-order, keyboard-activation or contrast row, SC 2.1.1 and 1.4.3 unasserted on `load-fetch`.
- `stand_keyboard`'s `tab_order`, `ENABLED_TAB_ORDER` and its rule that every stand control lies in one of the two orders, which the served pages' controls now enter.
- Enter and Space activation of a `button` such as `load-fetch`, and the app's handlers running before `handle_click`, as a keyboard-activation row asserts them.
- `stand_contrast`'s tables and their exempt rule (`enabled` reading `Some(false)`), and the required ratios 4.5 and 3.0 for large text, as a served page's row applies them; a red row is a further fork, not a re-pin.
- The focus redraw `focus_redraw` reads on the windowed shell's route through its event loop, beside the boot smoke, as the ground of the new redraw check.
- `:open` on `details` and `dialog`, and what it does to their content's presence in the tree and the Tab order, as the new check reads it.

## Standing conventions
- S1 · Every a11y check runs in both layout modes, `for incremental in [false, true]` · §3 A11y Assertion Harness Contract
- S2 · An a11y assertion is a Rust test in blitz-tests over the AccessKit tree, built with `build_accessibility_tree` after `resolve` · §3 A11y Assertion Harness Contract
- S3 · A state check reads each carrier off the built tree, one stated table per attribute with its row count asserted · §3 A11y Assertion Harness Contract
- S4 · A keyboard check drives through the harness's `press` and `press_with` and reads the focused node back at each step · .andromeda/a11y-plan-detail/keyboard-navigation.md
- S5 · A contrast row is read through `Harness::text_contrast` and `contrast_ratio`, by the one shared reader, with no second formula or threshold · .andromeda/a11y-plan-detail/visual-design-verification.md
- S6 · Expected roles and names are stated once in the shared module's tables, never a second role or name mapping · .andromeda/a11y-plan-detail/screen-reader-support.md
- S7 · An element an agent can act on reads an author key, so `unkeyed_actionable` returns nothing on every stand task · .andromeda/a11y-plan-detail/screen-reader-support.md
- S8 · A new accessibility test file joins the `a11y` leg's list, and the `stand-repeat` leg's where it qualifies, with the stated file count moved · .andromeda/a11y-plan-detail/ci-integration.md

## Not needed
- The per-platform adapter gaps, the `Adapter` creation order in `accesskit_xplat`, and the windowed AT witness: the platform adapter is out of this version (residual line 14).
- The Dioxus crates' `accessibility` feature wiring: this entry bumps no dependency and changes no manifest.
- Cells' selected-cell state: residual line 24, out of this entry; only Cells' keyboard and contrast rows are in it.
- Language and direction records and their fixtures: no debt touches `lang` or text direction.
- Input method handling: no debt touches IME.
- Text alternatives and hidden decoration in the fixtures: no debt touches them.
- Cognitive accessibility, the flight booker's invalid-date cue and orientation cues included: the flight booker enters only through the option `overlap`.
- Motion rules and focus-ring tokens: no debt paints or measures them.
- Not-colour-alone state cues: this entry asserts contrast rows, not whether a state is carried by colour alone.
- Page zoom and colour-scheme or user-preference handling: the pan/zoom here is a CSS `transform` on a container, not a user's zoom setting.
- The `input type="range"` step keys and pointer fraction: no debt touches a range input.
- The unread `aria-activedescendant` and its pin `accessibility_active_descendant_unread`: SC 4.1.2 work outside this entry.
- The canvas renderer, the custom-paint gap, `drag` and virtual-node production: 0.4.0 capabilities this entry does not take.
