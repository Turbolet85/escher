proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 28
    section: "## Existing Scopes"
    change: "In the What it is line, after the hit testing clause's own citation add the clause: `; since 2026-10-31-select-popup, a single select's open list, its state, readers and writer and its hit answered first in `hit_with_scrollbar` (packages/blitz-dom/src/document.rs:761-1089; packages/blitz-dom/src/document.rs:2404-2406)`; every other clause of the line stands."
    sidecar: "existing-scopes: blitz-dom interaction names the open select list held in document.rs."
    rationale: "Report Changes, Symbols / APIs, `BaseDocument` and The hit: `document.rs:761-1089` adds the list's field readers and writer and `hit_with_scrollbar` answers `select_list_hit` first at `document.rs:2404-2406`, so the unit's list of document.rs's interaction areas omits one."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 49
    section: "## Existing Scopes"
    change: "In the What it is line, after the `select (...)` item and before `scrollbars`, add the item: `select_list (since 2026-10-31-select-popup, the crate-private state and geometry of a single select's open list, with 6 unit tests, its two public read types `SelectListView` and `SelectListRow` re-exported from the crate root; blitz-paint's `render::select_list` paints it, and `select_popup`, `select_popup_painted` and `stand_act_popup` are its checks; packages/blitz-dom/src/node/select_list.rs:28-46; packages/blitz-dom/src/node/select_list.rs:78-108; packages/blitz-dom/src/lib.rs:108-111)`."
    sidecar: "existing-scopes: blitz-dom node gains the module select_list, its painter and its checks named."
    rationale: "Report Changes, Crates / modules: two new modules, blitz-dom `node::select_list` and blitz-paint `render::select_list`; Coverage of new surfaces: unit (`select_list` 6) and the three checks."
    fact: F27
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 58
    section: "## Existing Scopes"
    change: "In the What it is line, the module `render` becomes `render (with, since 2026-10-31-select-popup, the child module `select_list`, `paint_select_list` painting a single select's open list after the root's render and before the devtools overlays: packages/blitz-paint/src/render/select_list.rs:21-40; packages/blitz-paint/src/render.rs:246-247)`."
    sidecar: "existing-scopes: blitz-paint's render names its child module select_list."
    rationale: "Report Changes, Crates / modules and Symbols / APIs, Paint: `paint_scene` calls `paint_select_list` after the root's render and before the devtools overlays (`render.rs:246-247`), in the new `render/select_list.rs`."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 76
    section: "## Existing Scopes"
    change: "In the snapshot item, `closed_select_of` and `is_multiple_select` re-exported becomes `closed_select_of`, `is_multiple_select` and, since 2026-10-31-select-popup, `is_select` re-exported."
    sidecar: "existing-scopes: dioxus-native-dom's snapshot re-exports is_select."
    rationale: "Report Changes, Symbols / APIs, Bridge: new public `is_select`, re-exported; Counts, `closed-option` and `focus-select` each list existing-scopes:76 as changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 106
    section: "## Existing Scopes"
    change: "The stand keyboard check `stand_keyboard`, 21 tests in both layout modes becomes `stand_keyboard`, 23 tests in both layout modes; the rest of the line stands."
    sidecar: "existing-scopes: stand_keyboard counts 23 tests."
    rationale: "Report Counts, `kbd-count`: `stand_keyboard` 21 to 23, existing-scopes:106 changed."
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "In `select_interaction`'s list, Enter and Space opening nothing on a single select becomes Enter and Space opening its list on a single select; its 18 tests and every other clause stand."
    sidecar: "existing-scopes: select_interaction's row on Enter and Space states the list they open."
    rationale: "Report Retired: the limit Enter and Space open nothing on a single select; Test renames: `enter_and_space_open_nothing_on_a_single_select` became `enter_and_space_open_the_list_and_a_released_space_does_nothing_more`; `enter-space` lists existing-scopes:112 as changed, `select-model` holding the 18."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 112
    section: "## Existing Scopes"
    change: "Add a continuation line following this one: ` — since 2026-10-31-select-popup a sixteenth driver-action file, `stand_act_popup`, 12 tests in both layout modes — a click opening the list and a click on a row choosing it, the keys alone opening, moving and choosing, Escape closing with the value unchanged and the rows back at the select's box, a closed option still refused `covered`, a row past the scrollport off screen until `scroll` brings it in, a button under the list refused `covered`, `type` leaving no list open and choosing by its text, a shot of a row cropped to it with the list left open, the lint reading no `overlap` on a row and the planted pair still reading, the open screen under its budget with every actionable element keyed, two clicks on one point opening and closing and each focusing, a screen read after a step holding the new state with no `advance` (tests/blitz-tests/tests/stand_act_popup.rs:58-423), in the `stand-repeat` leg by its `stand_` prefix; the engine checks `select_popup`, 25 tests — the list's opening, keys, choice, closing, tree, clips, hostile markup and a `multiple` select's `size` (tests/blitz-tests/tests/select_popup.rs:202-1186) — and `select_popup_painted`, 6 tests — the highlight and a disabled row against their constants, the plain pair, opacity and clips, a closed select's frame byte-equal, a group's header darker than a row by weight and a thumb that takes no hit (tests/blitz-tests/tests/select_popup_painted.rs:119-351) —, both in both layout modes and named `--test`s of the `stand-repeat` list (.github/scripts/ci-leg.sh:59-60), outside the `a11y` leg's fourteen files; and `stand_keyboard`'s twenty-second and twenty-third, the flight type walk holding with its list open and after each close, and a click opening the list with focus staying on the select and the walk going on from it (tests/blitz-tests/tests/stand_keyboard.rs:1082-1134)`."
    sidecar: "existing-scopes: blitz-tests names stand_act_popup, select_popup, select_popup_painted and stand_keyboard's two new tests."
    rationale: "Report Changes, Files and New text, by line: the three new test files and `stand_keyboard` 1082-1134; Coverage: `select_popup` 25, `select_popup_painted` 6, `stand_act_popup` 12, `stand_keyboard` +2; Harness / gate surface: `ci-leg.sh:59-60`, `stand_act_popup` by its prefix, the `a11y` leg stays fourteen files."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 113
    section: "## Existing Scopes"
    change: "`STATE_ROWS`, 18: 17 pass, 1 exempt becomes `STATE_ROWS`, 20: 19 pass, 1 exempt; the rest of the line stands."
    sidecar: "existing-scopes: stand_contrast's STATE_ROWS reads 20, 19 pass and 1 exempt."
    rationale: "Report Counts, `state-rows`: `STATE_ROWS` 18 to 20, tally (19, 0, 1, 0), existing-scopes:113 changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 116
    section: "## Existing Scopes"
    change: "by the fifteen `stand_act_*` becomes by the sixteen `stand_act_*`; the rest of the line stands."
    sidecar: "existing-scopes: session_common's readers count sixteen stand_act_* files."
    rationale: "Report Counts, `act-files`: `stand_act_*` files fifteen to sixteen, existing-scopes:116 changed."
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/existing-scopes.md
    near: 142
    section: "## Existing Scopes"
    change: "`tests/session_common/mod.rs`, by 31 becomes by 32; `tests/fixtures/mod.rs`'s by 28 (22 `stand_*` files, becomes by 31 (28 until 2026-10-31-select-popup: 22 `stand_*` files, with the six named checks standing; the closing the 31 at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md, the 28 at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md becomes the 32 and the 31 at escher-0.3.0/chunks/2026-10-31-select-popup/report.md."
    sidecar: "existing-scopes: blitz-tests' shared modules read 32 session_common and 31 fixtures declarers."
    rationale: "Report Counts, `mod-decl`: checks declaring `mod session_common;` 31 to 32 and `mod fixtures;` 28 to 31, anchored at both revs; its changed sites omit this line, a listing reaching its patterns' wording and no further. The report names the count and not the three new declarers, so the 28's breakdown stays dated."
note: "F28 is occupied-resources' and F1-F26 are other categories', none carried here; existing-scopes:106 and :112 already stand over the category's 2 000-char line limit, so their edits add no length beyond a few characters and the new detail takes its own line."
