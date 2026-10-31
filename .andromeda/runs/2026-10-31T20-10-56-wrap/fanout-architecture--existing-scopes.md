proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 28
    section: ## Existing Scopes
    change: After the text selection clause, add: since 2026-10-31-select-popup, an open select's painted list, held in the one field `select_list` of `BaseDocument`, opened and reconciled at resolve by crate-private methods, its hit answered first in `hit_with_scrollbar`, its public readers `open_select`, `select_list_bounds`, `select_list_view` and `select_row_layout` and its public writer `close_select_list` (packages/blitz-dom/src/document.rs:343-344; packages/blitz-dom/src/document.rs:761-1089; packages/blitz-dom/src/document.rs:2404-2406)
    sidecar: blitz-dom interaction unit gains the open select's list on `BaseDocument`
    rationale: The report's Symbols / APIs add the field, its readers and writer and the first answer in `hit_with_scrollbar`; the interaction unit lists this file's hit and overlay areas and holds none of it.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:11
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 49
    section: ## Existing Scopes
    change: After the `select` item, add: select_list (since 2026-10-31-select-popup, an open select's list: its crate-private state `SelectList`, seven fields, and the public read types `SelectListView` and `SelectListRow`, re-exported from the crate root; packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-dom/src/node/select_list.rs:78-91; packages/blitz-dom/src/node/select_list.rs:93-108)
    sidecar: blitz-dom node unit gains the module `select_list`
    rationale: The report's Crates / modules names the new module blitz-dom `node::select_list`, its seventh field `fresh` a stated deviation; this unit lists the node modules and omits it.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:24
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 58
    section: ## Existing Scopes
    change: After the module list, add: `render` holding since 2026-10-31-select-popup the child `select_list`, whose `paint_select_list` paints an open select's list after the root's render and before the devtools overlays (packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/render.rs:246-247)
    sidecar: blitz-paint unit gains the child module `render::select_list`
    rationale: The report's Crates / modules names the new module blitz-paint `render::select_list`; this unit names the crate's modules and holds no such child.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 76
    section: ## Existing Scopes
    change: In the `snapshot` item, the re-exports read `closed_select_of`, `is_select` and `is_multiple_select` re-exported, in place of `closed_select_of` and `is_multiple_select` re-exported.
    sidecar: dioxus-native-dom's snapshot re-exports `is_select`
    rationale: The report's Bridge entry adds a public `is_select`, re-exported, and keeps `is_multiple_select` re-exported; the sweep's `focus-select` row lists this line.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: ## Existing Scopes
    change: `stand_keyboard` reads 23 tests in both layout modes, in place of 21 tests; the two added tests are stated in the new row following line 151.
    sidecar: `stand_keyboard` 21 to 23 tests
    rationale: The report's `kbd-count` moves `stand_keyboard` 21 to 23 and lists this line as changed; the digit alone is edited here since the line already runs long.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:34
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: ## Existing Scopes
    change: In the `select_interaction` item, `Enter and Space opening nothing on a single select` reads `Enter and Space opening its list on a single select`; the rest of the line stands.
    sidecar: `select_interaction`'s row states Enter and Space opening a single select's list
    rationale: The report's Retired bullet ends «Enter and Space open nothing on a single select», the test renamed `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`; the over-length listing records this line as `enter-space` changed and its other facts true.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:28
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 113
    section: ## Existing Scopes
    change: `STATE_ROWS` reads `(`STATE_ROWS`, 20: 19 pass, 1 exempt`, in place of `(`STATE_ROWS`, 18: 17 pass, 1 exempt`; the rest of that clause stands.
    sidecar: `stand_contrast`'s `STATE_ROWS` 18 to 20, 19 pass and 1 exempt
    rationale: The report's `state-rows` moves `STATE_ROWS` 18 to 20, tally (19, 0, 1, 0), and lists this line as changed.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:35
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 116
    section: ## Existing Scopes
    change: `by the fifteen `stand_act_*`` reads `by the sixteen `stand_act_*``.
    sidecar: `stand_act_*` readers of `session_common` fifteen to sixteen
    rationale: The report's `act-files` moves the `stand_act_*` files fifteen to sixteen and lists this line as changed.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 142
    section: ## Existing Scopes
    change: `tests/session_common/mod.rs` reads by 32 in place of by 31, and `tests/fixtures/mod.rs` reads by 31 (28 at 2026-10-31-engine-debts — 22 `stand_*` files, `font_face_sniffed_source`, `select_interaction`, `inert_subtree`, `accessibility_states`, `checkable_change_event` and `text_range_no_change_event` — and three more since 2026-10-31-select-popup); the closing provenance reads as measured at escher-0.2.0/chunks/2026-10-20-large-screen-in-parts/report.md, the 32 and the 31 at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the 28's roll at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md.
    sidecar: checks declaring `mod session_common;` 31 to 32 and `mod fixtures;` 28 to 31
    rationale: The report's `mod-decl` moves both counts, anchored by `git grep -l -E`; its changed list omits this line, which states both old counts as standing. The report names no roll of the three added declarers, so the row names none.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 151
    section: ## Existing Scopes
    change: New row: blitz-tests' `select_popup`, since 2026-10-31-select-popup, 25 tests in both layout modes, a named `--test` of the `stand-repeat` list and outside the `a11y` leg's fourteen files — a click, Enter and Space opening a single select's list on its chosen option and firing nothing, keys and the pointer moving the highlight over enabled rows, Enter, Space, Tab and a click choosing with one `input` then one `change`, a disabled row changing nothing, Escape, a second click, a click outside and the page scrolling closing with no event, a label's click focusing and opening nothing, a select that cannot open and one made inert, disabled, hidden or removed with its list open, the key press skipped in the ancestor-`hidden` row for the `position_in_containing_block` panic the report routes, the tree's reading, an author's `aria-expanded` and `open` steering nothing, clips, a pointer at a row's centre, groups as headers, a long list's twenty rows, hostile markup and `size` on a `multiple` select (tests/blitz-tests/tests/select_popup.rs:202-1186; .github/scripts/ci-leg.sh:59-60).
    sidecar: New row for the engine check `select_popup`, 25 tests
    rationale: The report's Files, Coverage and Harness entries add this target; the file names each blitz-tests target and holds no row for it, and the Spec claims entry states the one skipped key press.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 151
    section: ## Existing Scopes
    change: New row: blitz-tests' `select_popup_painted`, since 2026-10-31-select-popup, 6 tests in both layout modes, a named `--test` of the `stand-repeat` list — the highlight and a disabled row read as painted against `SelectListView`'s constants, the plain pair measured at the default sheet and at the stand, a list painted whole past an opacity ancestor and a box that clips its select, a closed select's frame byte-equal before opening and after a close with no choice, a group's header darker than a row by weight and a scrolled list's thumb painted and taking no hit (tests/blitz-tests/tests/select_popup_painted.rs:119-351; .github/scripts/ci-leg.sh:59-60).
    sidecar: New row for the engine check `select_popup_painted`, 6 tests
    rationale: The report's Files, Coverage and Harness entries add this target; the file names each blitz-tests target and holds no row for it.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:110
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 151
    section: ## Existing Scopes
    change: New row: blitz-tests' sixteenth driver-action file `stand_act_popup`, since 2026-10-31-select-popup, 12 tests in both layout modes, reading `tests/session_common/mod.rs` and in `run stand` and `stand-repeat` by its prefix — a click opening the list and a click on a row choosing it, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, a closed option staying refused `covered`, a row past the scrollport off screen until `scroll` brings it in, a button under the list refused `covered`, `type` leaving no list open and choosing by its text, a shot of a row cropping to it and leaving the list open, the lint reading no `overlap` on a row with the planted pair still read, the open screen under its budget and every actionable element keyed, two clicks on one point opening and closing each focusing, and a screen read after a step reading the new state with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423).
    sidecar: New row for the driver-action check `stand_act_popup`, 12 tests
    rationale: The report's Files, Coverage and Harness entries add this target, the sixteenth by `act-files`; lines 110-112 enumerate the driver-action files through the fifteenth and that long line cannot take it.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:70
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 151
    section: ## Existing Scopes
    change: New row: blitz-tests' `stand_keyboard`, its twenty-second and twenty-third tests since 2026-10-31-select-popup — the flight type walk holding with its list open and after each close, and a click opening the list with focus staying on the select and the walk going on from it, in the `a11y` CI leg with the rest of the file (tests/blitz-tests/tests/stand_keyboard.rs:1082-1111; tests/blitz-tests/tests/stand_keyboard.rs:1113-1134).
    sidecar: New row for `stand_keyboard`'s two list tests
    rationale: The report's `kbd-count` and `a11y-tally` move the file by two tests and the leg by the same two; line 106 enumerates each test's arrival and already runs long, so the detail takes its own row.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:33
note: Lines 106 and 112 already run past the 2 000-char limit, so their edits hold length and their new detail goes to rows after line 151; D-arch-resources and D-arch-decisions found no violation in this file, no dependency moving and the registry sections lying outside it.
