proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/conventions.md
    near: 54
    section: "## Conventions"
    change: "In the Tests convention's list of checks that hold a `Session`, «the fifteen driver-action checks `stand_act_*`» becomes «the sixteen driver-action checks `stand_act_*`»; the rest of the line stands."
    sidecar: "conventions: the driver-action checks `stand_act_*` fifteen → sixteen, `stand_act_popup` added by 2026-10-31-select-popup."
    rationale: "Report Counts / qualifiers moved, `act-files`: `stand_act_*` files fifteen → sixteen (`git ls-tree`: 15 → 16), naming conventions:54 among the sites changed; the new file `tests/blitz-tests/tests/stand_act_popup.rs` is in Files and its New text shows it reading `session_common`, so the sentence's rule holds and only the count moves."
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:31
note: "The `act-files` move fits neither scoped detector's invariant, so it is filed as D-new-detail; D-arch-resources, D-arch-decisions and D-retired find nothing in this file (no resource, dependency or retired item stands in it)."
