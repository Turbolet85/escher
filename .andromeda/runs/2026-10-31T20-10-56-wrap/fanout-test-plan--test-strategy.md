proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 28
    section: "## 2. Test Strategy"
    change: "After the clause naming the `select` model's two limit bullets and its keys, rows and groups pinned by `select_interaction` and `stand_act_select`, add: since 2026-10-31-select-popup the open list's own limits stand in two bullets more, the highlight unread and the thumb with one row height (docs/capabilities.md:208-210; docs/capabilities.md:211-213), the open list's keys, rows and groups pinned by `select_popup` and `stand_act_popup` and its pixels by `select_popup_painted`."
    sidecar: "Claims mapped to checks: the open select list's two limit bullets and the three checks that pin the list, from 2026-10-31-select-popup."
    rationale: "The report's New text lists capabilities 208-210 and 211-213 as added bullets beside the model's 205-207 and 223-226, and its Outcome (`v030-17`) rests the list on `select_popup`, `select_popup_painted` and `stand_act_popup`; the model's two bullets stay true (`select-model`, no change), so this is added detail and not a correction."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:148
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 34
    section: "## 2. Test Strategy"
    change: "New bullet after Pixel tests: **Painted select list (since 2026-10-31-select-popup):** `select_popup_painted` reads an open list on a frame's `Raster` in both layout modes, a highlighted and a disabled row within one channel of the engine's constants, a closed select's frame byte-equal before opening and after a close with no choice, and a group's header darker than a row by its ink; `font-embolden` is built by no local gate, so a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, which reproduced on linux the header-versus-row numbers that made CI#41233234207 red on `Test (macos)` alone (tests/blitz-tests/tests/select_popup_painted.rs:119-159; tests/blitz-tests/tests/select_popup_painted.rs:237-253; tests/blitz-tests/tests/select_popup_painted.rs:282-297; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/ci.md)"
    sidecar: "Pixel tests: a new bullet for `select_popup_painted` and the local `blitz-paint/font-embolden` witness, from 2026-10-31-select-popup."
    rationale: "The report adds `select_popup_painted` (New text, lines 119-159, 237-253, 282-297) and records under Decisions & corrections that a local gate block read green over a defect only macOS showed, with this flag its local witness; line 34 states the frame strategy and names neither."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:131
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 52
    section: "## 2. Test Strategy"
    change: "`tests/session_common/mod.rs`, read through `mod session_common;` by 32 checks (`stand_act_popup.rs` the latest; 31 with `stand_cells_served.rs` and `stand_load_page.rs`; 29 with `stand_snapshot_parts.rs`; the rest of the history as it stands)."
    sidecar: "Directory pattern: `mod session_common;` readers 31 to 32 with `stand_act_popup.rs`."
    rationale: "The report's `mod-decl` row moves `mod session_common;` from 31 to 32 and names test-strategy:52 as changed; `stand_act_popup` is the new file its New text shows importing from `session_common`."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 53
    section: "## 2. Test Strategy"
    change: "Among the `mod session_common;` readers, «the fifteen `stand_act_*.rs`» reads «the sixteen `stand_act_*.rs`»."
    sidecar: "Directory pattern: `stand_act_*` files fifteen to sixteen."
    rationale: "The report's `act-files` row moves `stand_act_*` files from fifteen to sixteen and names test-strategy:53 as changed."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 53
    section: "## 2. Test Strategy"
    change: "`tests/fixtures/mod.rs`, read through `mod fixtures;` by 31 checks, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md; the list that follows is kept as the 28 measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md, its 22 `stand_*.rs` and six other files unchanged."
    sidecar: "Directory pattern: `mod fixtures;` readers 28 to 31, the named list kept as the census of 28."
    rationale: "The report's `mod-decl` row moves `mod fixtures;` from 28 to 31 and names test-strategy:53 as changed, without naming the three new readers, so the named list stays dated to its census and is not extended."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:39
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 54
    section: "## 2. Test Strategy"
    change: "156 `.rs` files, 155 test targets and the unbuilt `all.rs`, as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, the latest `select_popup.rs`, `select_popup_painted.rs` and `stand_act_popup.rs`, each its own auto-discovered target with no manifest line (153 and 152 before them, the latest then `visible_region_transformed.rs`, `hit_non_invertible_transform.rs`, `open_state.rs`, `checkable_change_event.rs` and `text_range_no_change_event.rs`, as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md; 145 and 144 before `stand_cells_served.rs` and the rest of the history as it stands)."
    sidecar: "Directory pattern: blitz-tests `.rs` files 153 to 156 and targets 152 to 155, three new auto-discovered targets."
    rationale: "The report's `target-files` row moves blitz-tests `.rs` targets from 153 to 156 and its Dependencies row keeps every manifest unmoved; that row reads «0 changed» and «No master states the count in this wording», yet line 54 of this file states 153 as a live count, so the sweep missed it."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:41
note: "The report's `target-files` row counts no hit at test-strategy:54, which states the moved 153; `mod-decl` names none of the three new `mod fixtures;` readers, so the 22 and six breakdown is left at the engine-debts census and the body's own 22-reader lines are for its own pass."
