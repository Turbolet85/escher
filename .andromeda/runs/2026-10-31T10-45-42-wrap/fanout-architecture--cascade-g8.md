proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 64
    section: "## Existing Scopes"
    change: "The blitz-shell unit's What it is reads: Modules application, convert_events, event (holding, since 2026-10-31-engine-debts, the crate's first `#[cfg(test)]` module, one unit test, a redraw request reaching the receiver and waking the loop once: packages/blitz-shell/src/event.rs:128-172), net, window, accessibility."
    sidecar: "existing-scopes: blitz-shell's `event` module holds the crate's first unit test (2026-10-31-engine-debts)."
    rationale: "Report, Changes, Crates / modules: added `#[cfg(test)]` module in `blitz-shell/src/event.rs` (`:128-172`), its first unit test; Counts: blitz-shell 0 to 1. The second occurrence of F22, whose primary is the row beside line 145."
    dependent-of: D-new-detail
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 76
    section: "## Existing Scopes"
    change: "In the dioxus-native-dom unit's `snapshot` clause, «its three types, `is_masked_input` and the `MASKED_VALUE` constant re-exported» reads «its three types, `is_masked_input`, the `MASKED_VALUE` constant and, since 2026-10-31-engine-debts, `closed_select_of` and `is_multiple_select` re-exported»."
    sidecar: "existing-scopes: dioxus-native-dom's snapshot module also re-exports `closed_select_of` and `is_multiple_select` (2026-10-31-engine-debts)."
    rationale: "Report, Changes, Sole-caller facts: `closed_select_of` and `is_multiple_select` are public and re-exported (`lib.rs:35-38`); Deviations 4 names them, so the clause's list of re-exports is short by two."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "In the `accessibility_states` clause, after «neither attribute on a native checkbox or radio» add «and, since 2026-10-31-engine-debts, a native checkbox's or radio's `toggled` written from its checked state under a checkable role, nothing under any other role, a click and a Space moving it», and «5 tests in both layout modes» reads «6 tests in both layout modes»."
    sidecar: "existing-scopes: `accessibility_states` 5 to 6 tests, native `toggled` written on checkable roles (2026-10-31-engine-debts)."
    rationale: "Report, Counts, the lines over 2,000 chars: `existing-scopes.md:102`, `native-toggled` @c453 changed and `n-a11y-states`' fact @c745 changed (5 tests); Symbols F5; `accessibility_states` 5 to 6."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 102
    section: "## Existing Scopes"
    change: "In the `inert_subtree` clause, «the focus-fixup limit pinned, 4 tests» reads «and, since 2026-10-31-engine-debts, a node that holds focus when it comes under `inert` giving focus up with no DOM `blur` or `focusout` and taking no key, the tree's focus reading the window when the focused node was not built, 6 tests»."
    sidecar: "existing-scopes: `inert_subtree` 4 to 6 tests, its focus-fixup limit retired for the fixup itself (2026-10-31-engine-debts)."
    rationale: "Report, Retired: `inert_subtree::removing_inert_restores_focus_hit_and_tree`'s focus-fixup limit lines; Counts, `existing-scopes.md:102`: `focus-fixup` @c1195 changed and `n-inert`'s fact @c1254 changed (4 tests); Symbols F3, F4; Outcome: `inert_subtree` 6."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 104
    section: "## Existing Scopes"
    change: "In the `transformed_client_rect` clause, «a target inside a translated scrolling box reading outside its visible region» reads «a target inside a translated scrolling box reading inside its visible region»; its 4 tests stand."
    sidecar: "existing-scopes: `transformed_client_rect`'s pin inverted, a target in a translated scrolling box reads inside its visible region (2026-10-31-engine-debts)."
    rationale: "Report, Retired: the pin name `a_target_inside_a_translated_scrolling_box_reads_outside_its_visible_region`, renamed `…reads_inside_its_visible_region` and inverted (`transformed_client_rect.rs:183-216`); Symbols F1."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: "## Existing Scopes"
    change: "In the `tests/common/mod.rs` clause, after «`REGION_NAMES` (`crud-list` → `People`)» add «and at 2026-10-31-engine-debts by `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, the load page's and Cells' Tab sequences (tests/blitz-tests/tests/common/mod.rs:93-98)»."
    sidecar: "existing-scopes: the shared stand module states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER` (2026-10-31-engine-debts)."
    rationale: "Report, Changes, F13: `common/mod.rs:93-98` states `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`; the entity names this module as the one statement of the per-task tables."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: "## Existing Scopes"
    change: "«`stand_keyboard`, 15 tests in both layout modes (11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg; the fifteenth the flight type select moving its choice by its arrows)» reads «`stand_keyboard`, 21 tests in both layout modes (11 until 2026-10-14-stand-a11y-assertions, which added three and put it in the `a11y` CI leg; the fifteenth the flight type select moving its choice by its arrows; 15 until 2026-10-31-engine-debts, which added the load page's stated walk with Enter and Space on its button, Cells' stated walk, Cells' arrows and Enter opening the moved-to cell and its value committed by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, and the arrows clamping at the grid's ends and bringing a cell into view)»."
    sidecar: "existing-scopes: `stand_keyboard` 15 to 21 tests, the load page's and Cells' keyboard paths (2026-10-31-engine-debts)."
    rationale: "Report, Counts: `stand_keyboard` 15 to 21, `n-keyboard` changed at `existing-scopes.md:106`; the six new tests named in New text, by line under `tests/blitz-tests/tests/stand_keyboard.rs`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "In the `stand_act_transformed` clause, «a `click` on a translated button accepted and landing and its snapshot `bounds` read where it shows, 2 tests in both layout modes» reads «a `click` on a translated button accepted and landing and its snapshot `bounds` read where it shows and, since 2026-10-31-engine-debts, a control in a scrolling box under a zoomed world reached where it shows, 3 tests in both layout modes»."
    sidecar: "existing-scopes: `stand_act_transformed` 2 to 3 tests, a control under a zoomed world (2026-10-31-engine-debts)."
    rationale: "Report, Counts: `stand_act_transformed` 2 to 3, and its statement that an `N` tests form escapes the count patterns, the detectors' sweeps the control; New text, by line names `a_control_in_a_scrolling_box_under_a_zoomed_world_is_reached_where_it_shows`; Symbols F1."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "In the `stand_act_select` clause, «a disabled select and an unchosen option refused — 5 tests in both layout modes» reads «a disabled select and an unchosen option refused and, since 2026-10-31-engine-debts, a `multiple` select's name, role and rows each read with its own bounds, a click on a row accepted and choosing it, a press with Shift spanning the rows, `type` choosing its match alone with no stray click, and an option under a closed select's group refused `covered` — 10 tests in both layout modes»."
    sidecar: "existing-scopes: `stand_act_select` 5 to 10 tests, a `multiple` select driven by id (2026-10-31-engine-debts)."
    rationale: "Report, Counts: `stand_act_select` 5 to 10, `n-act-select` listing no line of this file, an `N` tests form its patterns do not reach; the five new tests named in New text, by line under `tests/blitz-tests/tests/stand_act_select.rs`; Symbols F6, F8."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "In the `select_interaction` clause, «a disabled select, a re-seed — 7 tests in both layout modes» reads «a disabled select, a re-seed and, since 2026-10-31-engine-debts, Enter and Space opening nothing on a single select, a `multiple` select's rows laid out with boxes of their own, its keys and typed text, its clicks choosing, flipping and spanning, a disabled one taking no click and no key, `multiple` removed settling the set to one option, the chosen row's and a plain row's ratios, a seeded set stepped and spanned from its last chosen row, and `optgroup` under a single and a `multiple` select — 18 tests in both layout modes»."
    sidecar: "existing-scopes: `select_interaction` 7 to 18 tests, the `multiple` select and `optgroup` (2026-10-31-engine-debts)."
    rationale: "Report, Counts: `select_interaction` 7 to 18, `n-select` listing no line of this file, an `N` tests form its patterns do not reach; the eleven new tests named in New text, by line under `tests/blitz-tests/tests/select_interaction.rs`; Symbols F6, F7, F8."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 113
    section: "## Existing Scopes"
    change: "«`stand_contrast`, 5 tests in both layout modes (2 until 2026-10-14-stand-a11y-assertions)» reads «`stand_contrast`, 7 tests in both layout modes (2 until 2026-10-14-stand-a11y-assertions, 5 until 2026-10-31-engine-debts, which added the load page's ratios and Cells' ratios)»."
    sidecar: "existing-scopes: `stand_contrast` 5 to 7 tests, the load page and Cells (2026-10-31-engine-debts)."
    rationale: "Report, Counts: `stand_contrast` 5 to 7, `n-contrast` changed at `existing-scopes.md:113`; New text, by line names `the_load_page_meets_its_ratios` and `cells_meets_its_ratios`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 116
    section: "## Existing Scopes"
    change: "«the per-task count of elements that react to a click and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 676, measured and pinned)» reads «the per-task count of elements that react to input and state no role (0 on the four lean tasks, Home and Temperature Converter, Circle Drawer 1, Cells 677, its 676 data cells and `cells-grid` with its `onkeydown`, measured and pinned)»."
    sidecar: "existing-scopes: Cells' reacting-element count 676 to 677, `cells-grid` joining the data cells (2026-10-31-engine-debts)."
    rationale: "Report, Spec claims disproved, Cells' count: `existing-scopes.md:116`'s 676 measured 677, `cells-grid` taking an `onkeydown` and no role; Deviations 1: the pin moved and its doc now says react to input; `cells-676` changed at this line."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 135
    section: "## Existing Scopes"
    change: "«`stand_cells_served`, 4 tests in both layout modes» reads «`stand_cells_served`, 5 tests in both layout modes», and after «the lint at boot reading its measured findings» add «and, since 2026-10-31-engine-debts, the keyboard walking to a cell and committing a value read in the first part»; «`stand_load_page`, 2 tests in both layout modes» reads «`stand_load_page`, 3 tests in both layout modes»; the closing «as measured at» gains «, the 5 and the 3 at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»."
    sidecar: "existing-scopes: `stand_cells_served` 4 to 5 and `stand_load_page` 2 to 3 tests (2026-10-31-engine-debts)."
    rationale: "Report, Counts: `stand_load_page` 2 to 3 (`n-load-page`) and `stand_cells_served` 4 to 5 (`n-cells`), both changed at `existing-scopes.md:135`; New text, by line names `the_keyboard_walks_to_a_cell_and_commits_a_value_read_in_the_first_part`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 142
    section: "## Existing Scopes"
    change: "«by 24 (22 `stand_*` files, `font_face_sniffed_source` and `select_interaction`)» reads «by 28 (24 until 2026-10-31-engine-debts: 22 `stand_*` files, `font_face_sniffed_source` and `select_interaction`)», and the closing measure gains «, the 28 at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md»; `session_common`'s 31 and `common`'s 21 stand as written."
    sidecar: "existing-scopes: fixture readers 24 to 28, `session_common` 31 unmoved (2026-10-31-engine-debts)."
    rationale: "Report, Counts: fixture readers 24 to 28 (`evidence/census.md`), `session_common` 31 unmoved. The report states the count and not the four new readers' names, so the breakdown stays the old count's."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "«the file does not run both layout modes» reads «each of the three runs both layout modes, since 2026-10-31-engine-debts»."
    sidecar: "existing-scopes: `pseudo_element_flow_classification` runs both layout modes, upstream's exception ended (2026-10-31-engine-debts)."
    rationale: "Report, Retired: upstream's both-modes exception, the three tests now running `for incremental in [false, true]`; `flow-class` changed at `existing-scopes.md:145`."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 145
    section: "## Existing Scopes"
    change: "New item after line 145: - blitz-tests' targets since 2026-10-31-engine-debts, each one file and one target, in both layout modes: `visible_region_transformed`, 6 tests — a world under an untransformed viewport showing a node and panning one out, a target in a node body under a transformed world reading where it shows, a page with no transform reading its padding box in the client rect's frame, a translate moving the region, a box mapped to a point leaving nothing visible inside it, and a rotation reading the rotated box's bounding box (tests/blitz-tests/tests/visible_region_transformed.rs:132-276); `hit_non_invertible_transform`, 2 tests — a box under a non-invertible transform taking no hit and its invertible twin hit where it shows (tests/blitz-tests/tests/hit_non_invertible_transform.rs:33-71); `open_state`, 4 tests — `open` read from the attribute at construction, a write and a removal through the mutator moving it, and a summary's click and Enter flipping its details (tests/blitz-tests/tests/open_state.rs:54-143); `checkable_change_event`, 3 tests — a click and a Space on a checkbox each firing `input` then `change`, a radio's `change` firing on an actual change alone, and a label's click reading as its control's (tests/blitz-tests/tests/checkable_change_event.rs:68-136); `text_range_no_change_event`, 2 tests — typing in a text input, and a range's key and pointerdown, firing `input` and no `change` (tests/blitz-tests/tests/text_range_no_change_event.rs:59-97). Beside them `packages/blitz-shell/src/event.rs` holds that crate's first `#[cfg(test)]` module (packages/blitz-shell/src/event.rs:128-172)."
    sidecar: "existing-scopes: the five new blitz-tests targets and blitz-shell's first unit-test module (2026-10-31-engine-debts)."
    rationale: "Report, Counts, New targets: `visible_region_transformed` 6, `hit_non_invertible_transform` 2, `open_state` 4, `checkable_change_event` 3, `text_range_no_change_event` 2; Crates / modules: `blitz-shell/src/event.rs` (`:128-172`); each test named in New text, by line."
    fact: F22
note: "Edits at existing-scopes.md:102, :106 and :112 may need the line re-wrapped to stay within 2,000 chars. `stand_lint` 7 to 8 and the bridge's `multiple` facts sit in no entity of this group. `existing-scopes.md:135`'s 47,949 bytes in 740 lines is neither moved nor re-measured by the report, though `cells-grid` gained an `aria_label` and a Tab stop, so it was left unedited and is worth a reading."
