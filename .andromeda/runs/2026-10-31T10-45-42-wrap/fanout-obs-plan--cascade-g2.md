proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 42
    section: "## 6. Log Coverage"
    change: "  - Cells' `cells-grid` Tab stop, key handler and `MountedData` handles, and its `.cell.current` outline, are instrumentation n/a and add no log site, print or env read (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "log-coverage, examples emitter: Cells' `cells-grid` keys, handles and outline recorded as adding no log site, print or env read (2026-10-31-engine-debts)"
    rationale: "Report **Coverage of new surfaces** reads `cells-grid` Tab stop, keys and `.cell.current` outline as instrumentation n/a, and **Schema / config** reads no log site, print or env read on the added lines; the examples group is where an example's output is stated."
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 72
    section: "## 6. Log Coverage"
    change: "  - no site added: the transformed `visible_region` read, the non-invertible hit answer, the inert focus fixup, the tree's focus fallback, native `toggled`, the `multiple` select, `optgroup` and the engine's checkable `change` add no log site, print, env read, listener, wall-clock read or panic path on their added lines (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "log-coverage, blitz-dom logs: the engine-debts changes recorded as adding no log site, print, env read, listener, wall-clock read or panic path (2026-10-31-engine-debts)"
    rationale: "Report **Schema / config** reads no log site, print, env read, listener, wall-clock read or panic path on the added lines, entry 4 at 0, and **Coverage of new surfaces** reads these surfaces as instrumentation n/a or no log site added; the group lists blitz-dom's log sites, so the fact that the list gains none belongs beside it."
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 72
    section: "## 6. Log Coverage"
    change: "  - no site added: the readers `BaseDocument::select_values`, `select_of` and `clear_select_typeahead`, and `dioxus_native_dom::closed_select_of` and `is_multiple_select`, are instrumentation n/a and add no log site, field or print (as measured at escher-0.3.0/chunks/2026-10-31-engine-debts/report.md)"
    sidecar: "log-coverage, blitz-dom logs: the new select readers recorded as instrumentation n/a, adding no log site, field or print (2026-10-31-engine-debts)"
    rationale: "Report **Coverage of new surfaces** reads `select_of`, `select_values`, `clear_select_typeahead`, `closed_select_of` and `is_multiple_select` as instrumentation n/a; the first three are in `document.rs`, a file of this group, and **Deviations** 4 adds no type, verb or field."
    fact: F4
note: "F1 (the five new checks' silence) and F3 (blitz-shell's first unit test) fit no picked entity, so the new-detail pass places them; the reader count 24 to 28 at obs-plan.md:92 and the select statement at obs-plan.md:95 sit in the body's §3, outside this group; no scoped detector fired, the report reading no log site, 2 `info_span!` and no telemetry dependency moved."
