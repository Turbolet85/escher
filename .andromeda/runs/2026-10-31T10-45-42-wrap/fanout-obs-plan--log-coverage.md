proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 70
    section: "## 6. Log Coverage"
    change: "The `focussed node` row also states: re-read at 2026-10-31-engine-debts beside the focus fixup (`DocumentMutator::flush` calling `BaseDocument::clear_focus`, no DOM `blur` or `focusout`), which adds no log site, the line's needles reading 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/sink.md)."
    sidecar: "§6 blitz-dom `focussed node` row: gains the 2026-10-31-engine-debts re-read beside the focus fixup, no log site added, needles 0."
    rationale: "The report's Expected amendments carries obs-plan §6's `focussed node` re-read, and its Coverage of new surfaces row for the focus fixup reads: no log site added, `focussed node` re-read, its needles at 0 (`evidence/sink.md`); the row at line 70 holds no such statement."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 126
    section: "## 6. Log Coverage"
    change: "New row after line 126: - observed absent — a log site, print, env read, listener, wall-clock read or panic path on the added lines of the 2026-10-31-engine-debts engine and driver changes (`visible_region`'s `padding_box_in_view`, `hit_inner`'s non-invertible answer, the focus fixup, the tree's window fallback and native `toggled`, the `multiple` select's rules, keys, click, mutator record and bridge, `optgroup`, the engine's `change` on a checkbox and a radio, `lint.rs`'s `in_closed_select`, `focus_multiple_select`) · searched: gate entry 4's added-lines census reading 0 (exit 1), `packages/blitz-shell/src/event.rs` held out, its addition an inline test; the clock census reading 0 (exit 1); `grep -rh 'info_span!('` reading 2; `host_log` and `mcp_log` 0 needles under both builds (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md, its gates)"
    sidecar: "§6 absence group: gains the 2026-10-31-engine-debts added-lines row, no log site, print, env read, listener, wall-clock read or panic path, 2 `info_span!` unmoved."
    rationale: "The report's Schema / config states no log site, print, env read, listener, wall-clock read or panic path on the added lines (entry 4 reads 0, `blitz-shell/src/event.rs` held out); Outcome's closed-sets criterion reads 2 `info_span!`, entry 4 at 0, `host_log` and `mcp_log` 0 needles under both builds; D-obs-instrumentation and D-obs-pii hold, every new surface reading instrumentation n/a, and this file's absence group is where earlier chunks record that reading."
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 126
    section: "## 6. Log Coverage"
    change: "New row after the added-lines row: - observed absent — a print, env read, subscriber or listener in the five new checks `visible_region_transformed`, `hit_non_invertible_transform`, `open_state`, `checkable_change_event` and `text_range_no_change_event` · searched: the silence census, exit 1, last line 0 (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/evidence/census.md)"
    sidecar: "§6 absence group: gains the silence-census row for the five checks 2026-10-31-engine-debts adds."
    rationale: "The report's Expected amendments carries obs-plan §3 and §6's `the census's new checks` to `evidence/census.md`, its Changes names the five new checks, and its gates read the silence census at exit 1, last line 0; lines 123 to 126 record the like census for earlier chunks' checks."
note: The report does not list the silence census's file set and evidence/census.md and evidence/sink.md were outside this read, so the third proposal's five-check reach is inferred from the report and wants checking against evidence/census.md; D-obs-stack and D-retired found nothing here, and the report's body hits (obs-plan.md:76, :87, :92) fall outside this file.
