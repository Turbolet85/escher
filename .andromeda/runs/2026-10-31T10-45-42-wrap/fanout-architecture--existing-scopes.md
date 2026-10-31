proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 49
    section: "## Existing Scopes"
    change: "In the blitz-dom node module list, after the range input data clause, add: select, the crate-private rules of the select model, since 2026-10-31-engine-debts holding `SelectCursor`, `RowGesture` and `Gestured` (packages/blitz-dom/src/node/select.rs:56-65; packages/blitz-dom/src/node/select.rs:67-78; packages/blitz-dom/src/node/select.rs:80-89)."
    sidecar: "2026-10-31-engine-debts: blitz-dom's node record names the crate-private select rules and their three new types."
    rationale: "Report Changes, Symbols / APIs, F6 names these rule types in `node/select.rs`; the body's `node::select` line at architecture.md:151 is the other side of the relation."
    basis: packages/blitz-dom/src/node/select.rs:56-89
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 64
    section: "## Existing Scopes"
    change: "After the blitz-shell module list add: `event` holding, since 2026-10-31-engine-debts, the crate's first unit test in an inline `#[cfg(test)]` module (packages/blitz-shell/src/event.rs:128-172)."
    sidecar: "2026-10-31-engine-debts: blitz-shell's record gains its first unit test module."
    rationale: "Report Changes, Crates / modules: an added `#[cfg(test)]` module in `blitz-shell/src/event.rs`, its first unit test."
    basis: packages/blitz-shell/src/event.rs:128-172
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 76
    section: "## Existing Scopes"
    change: "In dioxus-native-dom's `snapshot` clause, the re-exports read: its three types, `is_masked_input`, `closed_select_of`, `is_multiple_select` and the `MASKED_VALUE` constant re-exported (packages/dioxus-native-dom/src/snapshot.rs:242-252; packages/dioxus-native-dom/src/lib.rs:35-38)."
    sidecar: "2026-10-31-engine-debts: dioxus-native-dom's snapshot record names the two new public re-exports."
    rationale: "Report Changes, Sole-caller facts: `closed_select_of` and `is_multiple_select` are public and re-exported; Deviations 4 gives the reason."
    basis: packages/dioxus-native-dom/src/lib.rs:35-38
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "In the `accessibility_states` clause, `neither attribute on a native checkbox or radio` reads: `neither attribute on a native checkbox or radio, whose `toggled` reads its own checked state on a checkable role and is unwritten under any other`."
    sidecar: "2026-10-31-engine-debts: `accessibility_states`' clause states the native control's `toggled`."
    rationale: "Report Changes, F5, and Lines over 2,000 chars: `existing-scopes.md:102` `native-toggled` @c453 changed."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "`accessibility_states`' `5 tests` reads `6 tests (5 until 2026-10-31-engine-debts, which added a click and a Space moving a native control in the tree)`."
    sidecar: "2026-10-31-engine-debts: `accessibility_states` 5 to 6 tests."
    rationale: "Report Counts / qualifiers moved: `accessibility_states` 5 to 6, and `existing-scopes.md:102` `n-a11y-states`' fact @c745 changed; Deviations 9 names the added row."
    basis: tests/blitz-tests/tests/accessibility_states.rs:386-427
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "In the `inert_subtree` clause, `the focus-fixup limit pinned` reads `a node made inert giving up focus with no `blur` or `focusout` and taking no key, the tree's focus then the window`."
    sidecar: "2026-10-31-engine-debts: `inert_subtree`'s focus-fixup limit retired; the fixup and tree-focus fallback stated."
    rationale: "Report Retired names `inert_subtree`'s focus-fixup limit lines; F3 and F4 replace the limit, and `existing-scopes.md:102` `focus-fixup` @c1195 is listed changed. The body's limit at architecture.md:74 is the other side."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "`inert_subtree`'s `4 tests` reads `6 tests (4 until 2026-10-31-engine-debts)`."
    sidecar: "2026-10-31-engine-debts: `inert_subtree` 4 to 6 tests."
    rationale: "Report Counts / qualifiers moved: `inert_subtree` 4 to 6, and `existing-scopes.md:102` `n-inert`'s fact @c1254 changed."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 104
    section: "## Existing Scopes"
    change: "In the `transformed_client_rect` clause, `a target inside a translated scrolling box reading outside its visible region` reads `a target inside a translated scrolling box reading inside its visible region`."
    sidecar: "2026-10-31-engine-debts: `transformed_client_rect`'s visible-region pin inverted with its rename."
    rationale: "Report Retired: the pin `...reads_outside_its_visible_region` is renamed `...reads_inside_its_visible_region` and inverted; F1 is the fix. The body's limit at architecture.md:145 is the other side."
    basis: tests/blitz-tests/tests/transformed_client_rect.rs:183-216
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: "## Existing Scopes"
    change: "In the `tests/common/mod.rs` clause, after `REGION_NAMES` (`crud-list` -> `People`), add: and at 2026-10-31-engine-debts by `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` (tests/blitz-tests/tests/common/mod.rs:93-98)."
    sidecar: "2026-10-31-engine-debts: the shared stand tables gain the load page's and Cells' Tab orders."
    rationale: "Report Changes, F13: `common/mod.rs:93-98` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`."
    basis: tests/blitz-tests/tests/common/mod.rs:93-98
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: "## Existing Scopes"
    change: "`stand_keyboard`'s `15 tests` reads `21 tests`, and its parenthesis gains `, 15 until 2026-10-31-engine-debts`."
    sidecar: "2026-10-31-engine-debts: `stand_keyboard` 15 to 21 tests."
    rationale: "Report Counts / qualifiers moved: `stand_keyboard` 15 to 21, `n-keyboard` changed at `existing-scopes.md:106`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 107
    section: "## Existing Scopes"
    change: "Before this line's citations, add to `stand_keyboard`: and, since 2026-10-31-engine-debts, the load page walking its stated sequence with its button taking Enter and Space, Cells walking its stated sequence, its arrows and Enter opening the moved-to cell and the value committing by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, and the arrows clamping at the grid's ends and bringing a cell into view (tests/blitz-tests/tests/stand_keyboard.rs:157-177; tests/blitz-tests/tests/stand_keyboard.rs:193-335)."
    sidecar: "2026-10-31-engine-debts: `stand_keyboard`'s six added tests described."
    rationale: "Report Changes, F13, and New text, by line, name these six tests, which take the target from 15 to 21."
    basis: tests/blitz-tests/tests/stand_keyboard.rs:157-335
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "`stand_act_transformed`'s `2 tests` reads `3 tests`; what the third reads is the new record after this file's line 145."
    sidecar: "2026-10-31-engine-debts: `stand_act_transformed` 2 to 3 tests."
    rationale: "Report Counts / qualifiers moved: `stand_act_transformed` 2 to 3. The file's `N tests` form is one the report's pattern does not reach, and the detectors' own sweeps stay the control."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "`stand_act_select`'s `an unchosen option refused` reads `a closed select's unchosen option refused`, and its `5 tests` reads `10 tests`; what the five added read is the new record after this file's line 145."
    sidecar: "2026-10-31-engine-debts: `stand_act_select` 5 to 10 tests, its refused option narrowed to a closed select's."
    rationale: "Report Counts / qualifiers moved: `stand_act_select` 5 to 10; F6 and F8: a `multiple` select's row takes a click, and only a closed single select's option stays refused `covered`. The `N tests` form is outside the report's pattern."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "`select_interaction`'s `7 tests` reads `18 tests`; what the eleven added read is the new record after this file's line 145."
    sidecar: "2026-10-31-engine-debts: `select_interaction` 7 to 18 tests."
    rationale: "Report Counts / qualifiers moved: `select_interaction` 7 to 18. The `N tests` form is outside the report's pattern."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 113
    section: "## Existing Scopes"
    change: "`stand_contrast`'s `5 tests` reads `7 tests`; what the two added read is the new record after this file's line 145."
    sidecar: "2026-10-31-engine-debts: `stand_contrast` 5 to 7 tests."
    rationale: "Report Counts / qualifiers moved: `stand_contrast` 5 to 7, `n-contrast` changed at `existing-scopes.md:113`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 116
    section: "## Existing Scopes"
    change: "`stand_non_lean_tasks`' clause reads `the per-task count of elements that react to input and state no role`, and its `Cells 676` reads `Cells 677`; the reason is the new record after this file's line 145."
    sidecar: "2026-10-31-engine-debts: Cells' reacting-element count 676 to 677; the count's words read input."
    rationale: "Report Spec claims disproved, pattern `cells-676`: `existing-scopes.md:116` measured 677; Deviations 1: the check's doc now says react to input."
    basis: tests/blitz-tests/tests/stand_non_lean_tasks.rs:102-103
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 135
    section: "## Existing Scopes"
    change: "`stand_cells_served`'s `4 tests` reads `5 tests (4 until 2026-10-31-engine-debts)`, and after `the lint at boot reading its measured findings` add `, and the keyboard walking to a cell and committing a value read in the first part (tests/blitz-tests/tests/stand_cells_served.rs:179-214)`."
    sidecar: "2026-10-31-engine-debts: `stand_cells_served` 4 to 5 tests, its keyboard path described."
    rationale: "Report Counts / qualifiers moved: `stand_cells_served` 4 to 5, `n-cells` changed at `existing-scopes.md:135`; New text, by line, names the test."
    basis: tests/blitz-tests/tests/stand_cells_served.rs:179-214
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 135
    section: "## Existing Scopes"
    change: "`stand_load_page`'s `2 tests` reads `3 tests (2 until 2026-10-31-engine-debts)`."
    sidecar: "2026-10-31-engine-debts: `stand_load_page` 2 to 3 tests."
    rationale: "Report Counts / qualifiers moved: `stand_load_page` 2 to 3, `n-load-page` changed at `existing-scopes.md:135`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 142
    section: "## Existing Scopes"
    change: "`tests/fixtures/mod.rs`'s `by 24 (22 `stand_*` files, `font_face_sniffed_source` and `select_interaction`)` reads `by 28`, and the closing measure gains `, the 28 at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md`."
    sidecar: "2026-10-31-engine-debts: `tests/fixtures/mod.rs` readers 24 to 28."
    rationale: "Report Counts / qualifiers moved: fixture readers 24 to 28 against `evidence/census.md`, `session_common` 31 unmoved. This site is not among the `fixture-readers` changed rows, and the report does not break the 28 down, so the old breakdown leaves with the old count."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "`pseudo_element_flow_classification`'s `the file does not run both layout modes` reads `its three tests run in both layout modes since 2026-10-31-engine-debts, `for incremental in [false, true]``."
    sidecar: "2026-10-31-engine-debts: `pseudo_element_flow_classification`'s both-modes exception ended."
    rationale: "Report Retired: `upstream's both-modes exception`; `flow-class` changed at `existing-scopes.md:145`. Expected amendments name the same end at `conventions.md:53`, the other side of the relation."
    basis: tests/blitz-tests/tests/pseudo_element_flow_classification.rs:44-54
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `visible_region_transformed`, 6 tests in both layout modes, since 2026-10-31-engine-debts: a world under an untransformed viewport showing a node and panning one out, a target in a node body under a transformed world reading where it shows, a page with no transform reading its padding box in the client rect's frame, a translate moving the region by the translation, a box mapped to a point leaving nothing visible inside it, and under a rotation the region reading the rotated box's bounding box (tests/blitz-tests/tests/visible_region_transformed.rs:132-276)."
    sidecar: "2026-10-31-engine-debts: new target `visible_region_transformed` recorded."
    rationale: "Report Changes, New checks and New targets: `visible_region_transformed` 6, the witness of F1; New text, by line, names its tests."
    basis: tests/blitz-tests/tests/visible_region_transformed.rs:132-276
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `hit_non_invertible_transform`, 2 tests in both layout modes, since 2026-10-31-engine-debts: a box under a non-invertible transform taking no hit, and its invertible twin hit where it shows (tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71)."
    sidecar: "2026-10-31-engine-debts: new target `hit_non_invertible_transform` recorded."
    rationale: "Report Changes, New checks and New targets: `hit_non_invertible_transform` 2, the witness of F2; the `non-invertible` control is unfired, so this is new text."
    basis: tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `open_state`, 4 tests in both layout modes, since 2026-10-31-engine-debts: at construction `open` read from the attribute, a write and a removal through the mutator moving it, and a summary's click and its Enter each flipping its details (tests/blitz-tests/tests/open_state.rs:54-143)."
    sidecar: "2026-10-31-engine-debts: new target `open_state` recorded."
    rationale: "Report Changes, New checks and New targets: `open_state` 4; Outcome CARRY 2 names it among the three checks; New text, by line, names its tests."
    basis: tests/blitz-tests/tests/open_state.rs:54-143
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `checkable_change_event`, 3 tests in both layout modes, since 2026-10-31-engine-debts: a click and a Space on a checkbox each firing `input` then `change`, a radio's `change` firing on an actual change alone, and a label's click reading as its control's (tests/blitz-tests/tests/checkable_change_event.rs:68-136)."
    sidecar: "2026-10-31-engine-debts: new target `checkable_change_event` recorded."
    rationale: "Report Changes, New checks and New targets: `checkable_change_event` 3, the witness of F10; New text, by line, names its tests."
    basis: tests/blitz-tests/tests/checkable_change_event.rs:68-136
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `text_range_no_change_event`, 2 tests in both layout modes, since 2026-10-31-engine-debts: typing in a text input and a range's key and pointerdown each firing `input` and no `change` (tests/blitz-tests/tests/text_range_no_change_event.rs:59-97)."
    sidecar: "2026-10-31-engine-debts: new target `text_range_no_change_event` recorded."
    rationale: "Report Changes, New checks and New targets: `text_range_no_change_event` 2; New text, by line, names its tests."
    basis: tests/blitz-tests/tests/text_range_no_change_event.rs:59-97
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `select_interaction`, its eleven tests added at 2026-10-31-engine-debts, each in both layout modes: Enter and Space opening nothing on a single select; a `multiple` select's rows laid out with boxes of their own, its keys choosing, spanning and flipping them, typed text choosing its row alone and a Space joining it while it stands, a click choosing, flipping or spanning a row, a disabled one taking no click and no key, `multiple` removed settling the set to one option, a chosen and a plain row meeting their ratio at the user agent's pair, and a seeded set stepped and spanned from its last chosen row; a group's options chosen by a single select's keys and shown alone; and a `multiple` select's groups laid out with a disabled group's rows not chosen (tests/blitz-tests/tests/select_interaction.rs:355-894)."
    sidecar: "2026-10-31-engine-debts: `select_interaction`'s eleven added tests described."
    rationale: "Report Changes, F6, F7 and F8, and New text, by line, name these eleven tests, which take the target from 7 to 18."
    basis: tests/blitz-tests/tests/select_interaction.rs:355-894
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `stand_act_select`, its five tests added at 2026-10-31-engine-debts, each in both layout modes: a `multiple` select read with its name, its role and its rows each with its own bounds, a click on a row accepted and choosing it, a press with Shift spanning the rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered` (tests/blitz-tests/tests/stand_act_select.rs:311-457)."
    sidecar: "2026-10-31-engine-debts: `stand_act_select`'s five added tests described."
    rationale: "Report Changes, F6 and F8, and New text, by line, name these five tests, which take the target from 5 to 10."
    basis: tests/blitz-tests/tests/stand_act_select.rs:311-457
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `stand_act_transformed`, its third test added at 2026-10-31-engine-debts, in both layout modes: a control in a scrolling box under a zoomed world reached where it shows (tests/blitz-tests/tests/stand_act_transformed.rs:138-176)."
    sidecar: "2026-10-31-engine-debts: `stand_act_transformed`'s third test described."
    rationale: "Report Outcome (F1) and New text, by line, name this test, which takes the target from 2 to 3."
    basis: tests/blitz-tests/tests/stand_act_transformed.rs:138-176
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `stand_contrast`, its two tests added at 2026-10-31-engine-debts, each in both layout modes: the load page and Cells each meeting its ratios (tests/blitz-tests/tests/stand_contrast.rs:647-704)."
    sidecar: "2026-10-31-engine-debts: `stand_contrast`'s two added tests described."
    rationale: "Report Coverage of new surfaces and New text, by line, name these two tests, which take the target from 5 to 7."
    basis: tests/blitz-tests/tests/stand_contrast.rs:647-704
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New record: - blitz-tests' `stand_non_lean_tasks` counts every element with the listener mark that reads `GenericContainer`; Cells' 677, since 2026-10-31-engine-debts, is its 676 data cells and `cells-grid`, which takes an `onkeydown` and no role (tests/blitz-tests/tests/stand_non_lean_tasks.rs:102-103)."
    sidecar: "2026-10-31-engine-debts: the reason for Cells' 677 recorded."
    rationale: "Report Spec claims disproved, Cells' count: the forecast 676 measured 677, with this reason; the 676 data cells stay a true count elsewhere."
    basis: tests/blitz-tests/tests/stand_non_lean_tasks.rs:102-103
note: "The report does not name stand-repeat's five added targets or the 28 fixture readers' breakdown, so the new records state no leg and line 142 drops its breakdown; lines 112, 113 and 116 take digit-level edits with their detail in new records to hold the 2,000-char limit, and line 102 stands over it already."
