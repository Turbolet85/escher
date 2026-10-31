## 2026-10-31-engine-debts — citations re-pointed onto the moved lines
**Section:** the citation-only lines of a11y-scope-summary.md, a11y-strategy.md, aria-patterns-roles.md, ci-integration.md, cognitive-accessibility.md, keyboard-navigation.md, screen-reader-support.md, visual-design-verification.md and a11y-plan.md
**Change:** Those lines' citations, into `blitz-dom`, `dioxus-native-dom`, `blitz-vibey-script`, `seven_guis`, the CI files and the `blitz-tests` checks, now name where the cited code and checks stand; each line's claim is unmoved. cognitive-accessibility.md's one edit is of this kind.
**Why:** the chunk's edits moved or grew the cited lines, and no reading on these lines moved with them.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the stand's graph re-read on the lock's move
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: Dioxus crates)
**Change:** The stand-graph clause gains a re-read on `Cargo.lock`'s own move of `pulldown-cmark` to 0.13.5, no crate added or removed: the stand's graph still holds neither `accesskit_xplat` nor `accesskit_winit` (`cargo tree`, `accesskit_(xplat|winit)` 0 rows), so no stand binary builds a platform adapter still.
**Why:** `pulldown-cmark` moved 0.13.4 → 0.13.5 in the lock alone, RUSTSEC-2026-0414's fix, the operator's call and not put to the founder (inputs#I11, inputs#I12).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the load page's and Cells' Tab, keyboard and contrast rows, the OPEN gap closed
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: Served pages); §3 Keyboard test harness and Contrast verification harness (a11y-plan.md); §5 Test harness pattern (a11y-plan.md; keyboard-navigation.md: keyboard-harness); §6 Color contrast (a11y-plan.md; visual-design-verification.md: visual-contrast); §7 Screen Reader Support (screen-reader-support.md: the load page)
**Change:**
- The OPEN line retired: was the pages beside the lean four holding no Tab-order, keyboard-activation or contrast row, SC 2.1.1 and 1.4.3 unasserted on the load page's `load-fetch`, owned by a `.andromeda/residuals.md` line; now the load page and Cells hold Tab-order, keyboard and contrast rows (`stand_keyboard`, `stand_contrast`, `stand_load_page`, `stand_cells_served`), SC 2.1.1 and 1.4.3 asserted on `load-fetch` and on Cells' `cells-grid`, the owner clause gone with it.
- Served pages: was neither carrying a Tab-order, keyboard-activation or contrast row; now each, still no `LeanTask`, holds a stated Tab sequence (`LOAD_TAB_ORDER`, `CELLS_TAB_ORDER`, in the shared module) that `stand_keyboard` walks, keyboard rows (`load-fetch` taking Enter and Space, Cells' arrows and Enter on `cells-grid`) and contrast rows in `stand_contrast`.
- `stand_keyboard` walks both orders forward and back in both layout modes: the load page's button taking Enter and Space (`the_load_page_walks_its_stated_sequence_and_its_button_takes_enter_and_space`), Cells' arrows and Enter opening the moved-to cell's editor and its value committing by keyboard, Tab leaving an open editor and a click on another cell each committing it, the arrows clamping at the grid's ends and bringing a cell into view. The lean line's clause narrows: was every stand control in one of the two orders; now every lean task's control in the two lean orders.
- `stand_contrast` holds two tests beside its three tables, `the_load_page_meets_its_ratios` and `cells_meets_its_ratios` (five rows), each asserting its required ratios in both layout modes, so SC 1.4.3 is asserted on the load page and Cells as well as the lean four; the load page's colours read through the shared `read`, no hex copied.
**Why:** the founder's F12 and F13, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9), asked these rows; they are met, and so the gap the body held open closes.
**Kept:** the `a11y` leg stays fourteen files.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the capability statement's named limits
**Section:** §1 A11y Scope Summary (a11y-scope-summary.md: Capability statement)
**Change:** The named limits `docs/capabilities.md` publishes under `## Accessibility reach`: was the focus fixup not built, a `multiple` select and an `optgroup`'s options not modelled, `aria-activedescendant` unread, the native checked state never stated in the tree; now the focus fixup firing no `blur`, no popup painted and Enter and Space opening nothing on a single select, a `multiple` select's `size` unread, `aria-activedescendant` unread. The limits are cited at this chunk's report.
**Why:** the chunk built the fixup, the native write, the `multiple` select and `optgroup` on the founder's F3, F5, F6 and F7, and left the popup out on his F8, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); `size` goes to the popup's chunk on his word at the P5 card, relayed by the operator (inputs#I11, inputs#I12).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a native checkbox's or radio's checked state in the tree
**Section:** §2 Role derivation (a11y-plan.md; a11y-strategy.md); §3 A11y testing tool pick (a11y-plan.md: the state check); §4 ARIA Patterns & Roles (aria-patterns-roles.md: ARIA state attributes); §7 Screen Reader Support (screen-reader-support.md)
**Change:**
- Was nothing else writing `toggled`, so the tree never stated a native checkbox or radio checked, the fix owned by a `.andromeda/residuals.md` line (target: next). Now a native checkbox or radio on a checkable role writes `Toggled::True` or `False` from `checkbox_input_checked()`; under any other role it writes nothing, so `pressed` never begins to read on it; neither author attribute is read on a native control. The owner clause is retired; a11y-strategy.md holds the write as a line of its own, each attribute still read once.
- The state check: `accessibility_states` was 5 tests, now 6, the sixth, `a_click_and_a_space_move_a_native_control_in_the_tree`, moving a native control by a click and a Space and reading it in the tree.
- screen-reader-support.md states the tree's reading beside the snapshot's `checked`, which reads the native state from the element.
**Why:** the founder's F5, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9), ended the measured absence.
**Kept:** what an assistive technology reads of it stays unmeasured, the windowed witness owned by the `.andromeda/residuals.md` line that reads `re-carried:0.2.0`.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a `multiple` select in the tree, the snapshot and the form values
**Section:** §4 ARIA Patterns & Roles (aria-patterns-roles.md: Select, `multiple`; Hidden content); §7 Accessibility tree output (a11y-plan.md; screen-reader-support.md)
**Change:**
- A `multiple` select's `ListBox` writes `set_multiselectable`.
- Hidden content: was every option but a single-choice select's chosen one computing `display: none`; now every option of a single-choice select but its chosen one computes it and a `multiple` select's rows do not.
- The snapshot's `value`: was `select_value` the chosen option's value; now, for a `multiple` select, its first chosen option's in tree order, nothing with none chosen. A select's options under it with its client rect as `bounds` narrows to a closed select's; a `multiple` select's rows and groups read their own client rects, in both layout modes.
- `evt.values()` reads a `multiple` select's set through `multiple_select_values`, one `(name, FormValue::Text(value))` form entry per chosen option on `Input` and `Change`, while `value` stays `select_value`.
**Why:** the founder's F6, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); that its text reaches form entries and a value beyond one is the operator's reading of the fork text at P2 Validate, 2026-10-31 13:13Z, not a new word of the founder's (inputs#I16).
**Kept:** what an assistive technology reads of it stays unmeasured.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `optgroup`, its role, its name and its options
**Section:** §4 ARIA Patterns & Roles (aria-patterns-roles.md: Option group; Hidden content); §5 Keyboard handling (a11y-plan.md; keyboard-navigation.md: keyboard-handling); §6 Visual Design Verification (visual-design-verification.md: visual-visibility); §7 Accessibility tree output (a11y-plan.md; screen-reader-support.md)
**Change:**
- `optgroup` → `Role::Group`, its non-empty `label` attribute its name, named in the tree and unpainted.
- Was an `optgroup`'s options not modelled; now they are among the select's options in tree order, reached by keys and text, kept in the tree as `ListBoxOption` nodes.
- A disabled group's rows read as disabled through `option_disabled`, the one rule `readings` and the form's entries share.
- The `display: none` exception, was an `option` whose parent is a `select`; now an `option` of a `select`, its child or under its `optgroup` (`is_select_option`), `hidden`, `inert` and `visibility: hidden` still excluding it.
**Why:** the founder's F7, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9), the list through a group by the operator's reading of the fork text at P2 Validate, 2026-10-31 13:13Z, not a new word of the founder's (inputs#I16). The group's `label` as a name stands on the P5 lean and not on F7's text, recorded so on that same word of the operator's.
**Kept:** what an assistive technology reads of it stays unmeasured.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the a11y leg's tally and the stand-repeat list
**Section:** §5 Test harness pattern (keyboard-navigation.md: `stand_keyboard`'s count); §6 Visual Design Verification (visual-design-verification.md: visual-contrast); §9 CI Integration (ci-integration.md)
**Change:**
- The leg: was 14 files and 85 tests (`stand_keyboard` 15), over the tier-0 witnesses alone 9 files and 59; now 14 files and 112 (`stand_keyboard` 21), 9 files and 86. visual-design-verification.md's reading: was 85 tests over 14 targets, `stand_contrast` 5; now 112, `stand_contrast` 7.
- Its log: was the six unmoved (43) and the eight (41), 14 result lines · 84 passed, on CI#40134371519; now the six with `stand_keyboard` 21 and `stand_contrast` 7 (52) and accessibility_states 6 · accessibility_label 13 · accessibility_description 3 · accessibility_merged_states 3 · accessibility_active_descendant_unread 1 · inert_subtree 6 · select_interaction 18 · stand_act_select 10 (60): 14 result lines · 112 passed · 0 failed · 0 ignored, locally and on the job of CI#41192433140.
- `stand-repeat`'s fixed list: was ten words pinned by `test_stand_repeat_leg_runs_three_rounds`; now fifteen.
- A new line: `accessibility_states` holds a row moving a native control in the tree by a click and a Space, and `inert_subtree` asserts the focus fixup and the tree's window fallback, both read through `&dyn Document`.
**Why:** the chunk's checks moved the leg's tally on CI#41192433140, its fourteen files unmoved, and `ci-leg.sh`'s `stand-repeat` names five more targets.
**Kept:** the job's slow tier, its `needs`, its restore-only `workspace-test` cache, its failure-only log upload and no required status check; what an assistive technology reads stays unmeasured.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the focus fixup built
**Section:** §3 A11y testing tool pick (a11y-plan.md: `inert_subtree`); §5 Focus order per layout (a11y-plan.md; keyboard-navigation.md: keyboard-focus-order)
**Change:**
- Was HTML's focus fixup not built, a node holding focus when it becomes inert keeping it until focus moves, the fix owned by a `.andromeda/residuals.md` line (target: next). Now it is built, in every document: `DocumentMutator::flush` calls `BaseDocument::clear_focus` where `focus_node_id` resolves to a node reading `is_inert()`, with no `inert` arm joining the attribute writes, so a subtree moved under an inert box is covered too. The owner clause is retired.
- No DOM `blur` or `focusout` fires, and `disabled` and `hidden` take no part in it.
- `inert_subtree` was 4 tests, now 6, asserting a node made inert losing focus and taking no key.
**Why:** the founder's F3, focus taken away with no `blur`, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a `multiple` select's keys and row clicks
**Section:** §5 Focus restoration (a11y-plan.md; keyboard-navigation.md: keyboard-focus-restoration); §5 Keyboard handling (a11y-plan.md; keyboard-navigation.md: keyboard-handling); §5 Test harness pattern (keyboard-navigation.md: keyboard-harness)
**Change:**
- Was a `multiple` select not modelled and taking no key; now it takes its own keys and row clicks.
- Keys, on one focused and no disabled form control: ArrowDown, ArrowUp, Home and End alone choose that row alone, with Shift span from the anchor, with Ctrl or Meta move the current row and change nothing. A Space with no typed text standing flips the current row and with text standing joins it; a character with no Ctrl, Alt or Meta chooses its match alone. Disabled rows are never chosen; Ctrl+A is not modelled. Shift with no cursor yet spans from the last chosen row, so a set the markup seeds is stepped and spanned from it.
- Clicks: `click_select_row` behind `handle_click`'s `option` arm, in every document. A disabled select stops it; otherwise it focuses the select and clears the typeahead and the text selection; a plain click chooses the row alone, Ctrl or Meta flips it, Shift spans, and a disabled row changes nothing.
- Was an option not clicked; now an option of a closed single select, under a group too, is not clicked.
- A `multiple` write or removal records the select (`record_select_of_multiple_write`), so removal settles it to one; `set_option_selectedness` leaves a `multiple` select's other options alone.
- `select_interaction` was 7 tests, now 18, with a 17-row keys table and a 10-row click table on a `multiple` select; `stand_act_select` was 5, now 10.
**Why:** the founder's F6, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); that its text names Ctrl and Shift is the operator's reading of the fork text at P2 Validate, 2026-10-31 13:13Z, not a new word of the founder's (inputs#I16).
**Kept:** no popup is painted, Enter and Space open nothing on a single select, and a closed select's option stays refused `covered`, on the founder's F8, the same dialog.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `:open` asserted
**Section:** §5 Focus restoration (keyboard-navigation.md: keyboard-focus-restoration); §6 Visual Design Verification (visual-design-verification.md: visual-state-color)
**Change:** Was no test asserting `:open`; now `open_state` (4 tests, both layout modes) reads the `open` attribute by presence at construction, moves it through a mutator write and removal and a summary's click and Enter, and reads `:open` matching.
**Why:** the chunk carried this gap and closes it with `open_state`.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — `change` on a checkbox and a radio
**Section:** §5 Focus restoration (keyboard-navigation.md: keyboard-focus-restoration); §5 Test harness pattern (keyboard-navigation.md: keyboard-harness)
**Change:**
- After `input`, `handle_click`'s checkbox and radio arms dispatch a `DomEventData::Change` with the same value, in every document, a checkbox always and a radio only where it was not checked before the click. The script runtime synthesizes no `change`, so a page's script hears one `change`, from the engine.
- `text_range_no_change_event` (2 tests) pins, in both layout modes, typing in a text input and a range's key and pointerdown firing `input` and no `change`.
**Why:** the founder's F10 and F11, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9); F10's text names the engine's change, on the operator's reading at P2 Validate, 2026-10-31 13:13Z, not a new word of the founder's (inputs#I16).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the driver on a `multiple` select
**Section:** §5 Keyboard handling (keyboard-navigation.md: keyboard-handling)
**Change:**
- `type` on a `multiple` select focuses it with no click, which would land on the row at the centre, and clears the typeahead; a text opening on a space types nothing. The driver's closed sets hold no verb, argument, key name, result field, cause, session error or exit for it.
- `press` holds no Ctrl, so a non-adjacent `multiple` set cannot be built through the driver, a limit its help states, its fix owned by a `.andromeda/residuals.md` line (target: next).
**Why:** `type`'s meaning stands on the founder's F6, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9), as the operator read the fork text at P2 Validate, 2026-10-31 13:13Z, not a new word of the founder's (inputs#I16); a text opening on a space can match no label and its Space would flip a row. `press`'s limit and residual stand on the founder's word at the P5 card, relayed by the operator (inputs#I11, inputs#I12).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the `MountedData` hazard in a spawned task
**Section:** §5 Keyboard Navigation (keyboard-navigation.md: keyboard-framework)
**Change:** New: a `MountedData` call in a task spawned from a key handler that writes a signal panics with `RefCell already borrowed` (`events.rs:162`), the task first polled inside `render_immediate`; measured headlessly, the windowed path read and not run. Cells therefore calls its handles synchronously, each acting at the call.
**Why:** the async form for the handles was tried and panicked, and did not ship; a later app's key handler meets the same panic.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — Cells by keyboard
**Section:** §5 Per-surface keyboard shortcuts (a11y-plan.md; keyboard-navigation.md: keyboard-shortcuts); §6 Visual Design Verification (visual-design-verification.md: visual-focus-ring); §7 Screen Reader Support (screen-reader-support.md: the reacting elements, `cells-grid`)
**Change:**
- `cells-grid`, named `Cells` by `aria_label` with no role, is one Tab stop; its arrows move `current`, clamped, and bring the cell into view through a synchronous `MountedData::scroll_to(ScrollBehavior::Instant)`.
- Enter on the grid opens the editor on `current`, focused; its Enter commits, stops propagation and returns focus to the grid; Tab leaving an open editor commits it; a click inside a cell's own open editor returns early, so typed text stays.
- `.cells-grid:focus .cell.current` draws `outline: 2px solid #3a5ce5` at `outline-offset: -2px`, the accent's held value.
- The reacting elements' reading: was the elements that react to a click, Cells 676; now the elements that react to input, Cells 677, the data cells and `cells-grid`.
**Why:** the founder's F13, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9). 677 is measured: `cells-grid`'s `onkeydown` with no role joins the 676 data cells, disproving the plan's forecast. The early return restates the plan's criterion on a pointer-opened editor, the behaviour kept because the keyboard path needs it, on the operator's word by dialog at P2 Validate, 2026-10-31 13:13Z (Q2, inputs#I16); that criterion was the plan's, not the founder's, F13 A saying nothing of it.
**Kept:** role and cell semantics stay out by `.andromeda/residuals.md` line 24, and what a person reads at a window is `unrunnable-here`.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the tree's focus falls back to the window
**Section:** §3 A11y testing tool pick (a11y-plan.md: `inert_subtree`); §7 Screen Reader Support (screen-reader-support.md: tree focus)
**Change:** The focused node reported as tree focus now carries its bound: `build_accessibility_tree` writes `focus` only where `focus_node_id` is among the built nodes, else `u64::MAX`, the window, so the tree never names a node it did not build. `inert_subtree` asserts the fallback where the focused node was not built.
**Why:** the founder's F4, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the lint's closed-select overlap ended
**Section:** §6 Visual Design Verification (visual-design-verification.md: visual-contrast)
**Change:**
- Was the driver's visual lint reading a select's options as overlapping, the flight booker at boot one `overlap` (`flight-type-one-way` × `flight-type-return`), `the_stand_at_boot_reads_its_measured_findings` reading `[0, 1, 0, 0]`, the defect open and owned by a `.andromeda/residuals.md` line. Now it leaves a closed select's options out of its overlap reading in every document: `overlaps` skips such an entry on either side, `Entry.in_closed_select` reading the element through `closed_select_of` and never the role, so CRUD's `role="option"` rows are not read as a select's and a `multiple` select's rows are linted as usual. The flight booker at boot reads no `overlap` in both layout modes, `[0, 0, 0, 0]`, and the owner clause is gone.
- The stand's boot reading: was its one finding that `overlap`; now no finding of any kind at boot.
**Why:** the founder's F9, as the report records it restated, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — the default sheet's `multiple` rows and chosen-row pair
**Section:** §6 Visual Design Verification (visual-design-verification.md: visual-contrast)
**Change:**
- The user-agent layout narrows to a single select, a group not `hidden` reading `display: contents` and the chosen option under it `display: inline`; a `multiple` select lays out its groups and rows as blocks, each row (`padding: 0 2px`, `white-space: nowrap`) with a box of its own.
- New: an engine-default chosen-row pair, `select[multiple] > option:checked` and `select[multiple] > optgroup > option:checked` reading `#000000` on `#c8c8c8`, Chromium's inactive selected pair, hardcoded by design as the engine holds no tokens, 12.56:1 measured by `select_interaction` in both layout modes.
- Was the user-agent pair not measured; now not measured on a single select, while `select_interaction` measures a `multiple` select's plain and chosen rows at the pair.
**Why:** the founder's F6 and F7, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9), put a `multiple` select's rows and groups in the default sheet; the chosen pair is hardcoded because the engine holds no tokens.
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/

## 2026-10-31-engine-debts — a non-invertible transform takes no hit
**Section:** §6 Visual Design Verification (visual-design-verification.md: visual-visibility)
**Change:** New: a transform whose determinant is 0 or not finite answers `None` from `Node::hit_inner` for the whole subtree, so that subtree takes no hit, an invertible twin hit where it shows, asserted by `hit_non_invertible_transform` in both layout modes.
**Why:** the founder's F2, by dialog in the overseer session, 2026-10-31, relayed verbatim by the operator (inputs#I8, inputs#I9).
**Ref:** .andromeda/runs/2026-10-31T10-45-42-wrap/
