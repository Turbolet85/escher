proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 11
    section: "## Threat Model Summary"
    change: "Remove the sentence «A remote page can also reach a key-press panic ... listed under Error Handling's panic paths (as measured at escher-0.3.0/chunks/2026-10-31-select-popup/report.md).»; line 11 then ends at «each held under Input Validation.»"
    sidecar: "Threat Model Summary, boundary-remote-content: the key-press panic retires with the fix in `BaseDocument::deallocate_anonymous_block`, together with Error Handling's panic row."
    rationale: "The report's Retired bullet (1) names the key-press panic in §Threat Model Summary; a key press on a control whose anonymous block was dropped no longer panics, and the Counts `panic` row marks threat-model-summary.md:11 ×3 changed, retired together (report.md:33, :23, :50, :96)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:50
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 11
    section: "## Threat Model Summary"
    change: "Add after line 11, in boundary-remote-content: « A stylesheet that fails to load, comes back empty or is not CSS stops blocking render, and one whose `href` does not resolve starts no load, so a remote page's failed sheet no longer holds a document's rendering; the error text stops in `ResourceLoadResponse` and the `Resource load failed` log line, whose `url` and `error` the sink scrubs, and no carrier takes it (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»"
    sidecar: "Threat Model Summary, boundary-remote-content: a failed, empty or non-CSS stylesheet no longer blocks render, a bad `href` starts no load, and the error text reaches no carrier."
    rationale: "`f6edb9701` changes what a remote page's stylesheet can do to a document, on every document, and the report reads the error text's reach; this group holds the remote vector's boundary guards (report.md:14, :31, :103)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/threat-model-summary.md
    near: 19
    section: "## Threat Model Summary"
    change: "Add after line 19, in boundary-script-execution: « A page's script also reaches the global `URLSearchParams` and `URL.prototype.searchParams`, with `href` and `search` live both ways, a realization on this crossing and no widening; the native helper behind them, `__blitz_parse_search_params`, is deleted from `globalThis` before any page script; the checks are held under Input Validation (as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/report.md).»"
    sidecar: "Threat Model Summary, boundary-script-execution: `URLSearchParams` and `URL.prototype.searchParams` named as a realization on the registered script crossing, their native helper gone before any page script."
    rationale: "`91ef65e21` adds a surface a page's script reaches, which the report reads on the registered script crossing and states is unnamed in security-plan; line 11's select-list sentence is the precedent for naming a page-reachable surface here with its checks under Input Validation (report.md:17, :46, :92, :102)."
    basis: .andromeda/cache/docs/2026-11-01-upstream-sync-ui-graph/report.md:102
note: "D-security-input, D-security-auth and D-security-deps fired nothing here (URLSearchParams validated per report.md:102, no auth touched, Dependencies none); the third proposal's pointer needs the Input Validation row report.md:96 expects, and outside this file `.andromeda/security-plan.md:326`'s «the handler is not called on error» reads stale against `f6edb9701` (report.md:14)."
