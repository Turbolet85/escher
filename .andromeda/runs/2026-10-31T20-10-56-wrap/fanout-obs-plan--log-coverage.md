proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 75
    section: "## 6. Log Coverage"
    change: In this row's list of readers name `dioxus_native_dom::is_select` beside `closed_select_of` and `is_multiple_select`, new and re-exported at 2026-10-31-select-popup, instrumentation n/a with no log site or print; the rest of the row stands.
    sidecar: log-coverage - the no-site row of select readers gains the new public `is_select` (2026-10-31-select-popup).
    rationale: The report's Counts / qualifiers moved, `focus-select`, reads log-coverage:75 as no-change, its silence holding, and says add `is_select`; Symbols / APIs, Bridge, names the new public `is_select`, re-exported.
    basis: .andromeda/obs-plan-detail/log-coverage.md:75
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 75
    section: "## 6. Log Coverage"
    change: New item after this row, in the blitz-dom group - no site added, the select list - blitz-dom's `node::select_list`, `BaseDocument`'s list methods with the public `open_select`, `select_list_bounds`, `select_list_view`, `select_row_layout` and `close_select_list`, the hit's list answer, `reconcile_select_list` in the resolve, the pointer and key arms (`hover_list_row`, `click_list_row`, `close_list_outside`, `press_open_list_key`, `choose_current_and_close`), the list answers of `visible_region` and `scroll_into_view`, the `size` hint on a `multiple` select, the tree's `expanded` on a single select, and dioxus-native-dom's `closed_select_of` open answer and the snapshot builder's `select_list_bounds` read add no log site, span, log field or env read, and no wall-clock read in blitz-dom outside `clock.rs`; `SelectListRow.label` holds page text in a read type and is never logged (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, its gate entries 4, 5 and 6).
    sidecar: log-coverage - new no-site item for the select list's engine and bridge code (2026-10-31-select-popup).
    rationale: The report's Coverage of new surfaces reads the select list as instrumentation none added, none required, no log site, and the public readers as instrumentation n/a with `SelectListRow.label` never logged; Schema / config adds no env var, log field or span; the gates read the added lines and the new module clean.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 100
    section: "## 6. Log Coverage"
    change: New item after this row, in the blitz-shell, blitz-paint group - no site added - blitz-paint's `render::select_list` (`paint_select_list`, `device_rect`, `paint_list_box`, `paint_list_row`, `paint_list_thumb`) and `text.rs`'s `stroke_text_as` with `RunPaint` add no log site, the censuses reading clean at the pre-CI tree and the `text.rs` fix at `715798ce2` adding no such token (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, its gate entries 4 and 5 and its (obs) acceptance).
    sidecar: log-coverage - new no-site item for blitz-paint's select list painting and `stroke_text_as` (2026-10-31-select-popup).
    rationale: The report's Symbols / APIs, Paint, adds these names; its gate entry on `packages/blitz-paint/src/render/select_list.rs` reads exit 1 and its (obs) acceptance states the fix adds no such token.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 53
    section: "## 6. Log Coverage"
    change: New item after this row, in the escher-driver group - no span or field moves with the select list - `Target.select`, `focus_select`, `Entry.in_select_box`, `clip_region`'s open-row read, `covered`'s restated meaning and the restated fixed help of `click`, `type` and `press` add no span, log field or env read; `command` keeps its eight fields, pinned by `stand_act_spans`, and escher-driver its 2 `info_span!` sites (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, its gate entries 3 and 4).
    sidecar: log-coverage - the driver's select list changes recorded as moving no span or field (2026-10-31-select-popup).
    rationale: The report's Symbols / APIs, Driver, makes these changes; Coverage of new surfaces reads span fields eight, unmoved; Schema / config adds no log field or span; the gate reads 2 `info_span!`.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 53
    section: "## 6. Log Coverage"
    change: New item after this row, in the escher-driver group - a select list opened and chosen through the host and through the `mcp` role logs no option - `host_log`'s `a_select_list_opened_and_chosen_through_the_host_logs_no_option_at_trace`, under `#[cfg(unix)]` as every host-driving test there, and `mcp_log`'s `a_select_list_opened_and_chosen_through_the_role_logs_no_option_at_trace`, which records nothing, read 0 needles at `trace`, the shared module gaining `option_needles` (examples/seven_guis/tests/host_log.rs:282-393; examples/seven_guis/tests/mcp_log.rs:68-113; examples/seven_guis/tests/common/mod.rs:425-453; as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).
    sidecar: log-coverage - new item for the host and `mcp` role log checks over an opened and chosen select list (2026-10-31-select-popup).
    rationale: The report's (security) acceptance reads `host_log`, `mcp_log` 0 needles at `trace`; Coverage counts them +1 each; Reverted and Spec claims disproved state that the `mcp_log` test records nothing and that the `host_log` test carries `#[cfg(unix)]`; New text, by line lists the blocks.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 71
    section: "## 6. Log Coverage"
    change: Add to this row after its once-per-focus-move claim - at 2026-10-31-select-popup the `Focussed node` count is not settled, 0 lines through the sink on the workspace build with no base reading, so neither moved nor unmoved, `stand_keyboard`'s `holder` holding the forecast's premise instead (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/evidence/counts.md).
    sidecar: log-coverage - the `Focussed node` row records the select popup's unsettled sink reading (2026-10-31-select-popup).
    rationale: The report's Decisions and corrections states this reading; the row claims one line per focus move under the workspace build, and a reader of it should see that the latest reading through the sink read 0 with no base to compare.
    basis: .andromeda/obs-plan-detail/log-coverage.md:71
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 131
    section: "## 6. Log Coverage"
    change: "New item after this row - observed absent — a print or other site of the silence census in the three checks `select_popup`, `select_popup_painted` and `stand_act_popup` · searched: the silence census, `println!|…` as the report's gate entry 2 prints it, over those files, exit 1, last line 0 (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md, its gate entry 2)."
    sidecar: log-coverage - new absence row for the three select popup checks (2026-10-31-select-popup).
    rationale: The report's Expected amendments for `obs-plan` call for new silence rows for three checks, and its new files name these three; the row follows the form of the engine-debts row at this line.
note: No drift under D-obs-instrumentation, D-obs-stack, D-obs-pii or D-retired in this file; the census pattern is elided in the absence row because the report elides it; this file was written twice, the second Write replacing the first whole to double-quote two YAML values that a parser would have misread.
