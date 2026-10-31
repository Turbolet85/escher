proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/ci-integration.md
    near: 15
    section: "## 9. CI Integration"
    change: In the Session state directory row, the seven_guis list of check directories under `target/tmp/` reads `hb-*`, `hl-trace`, `hl-client`, `hl-client-rec`, `hl-cells`, `hl-popup`, `cc-*`, `cf-*` and `ht-timer`, adding `hl-popup` and changing nothing else in the row.
    sidecar: obs-plan CI Integration, Session state directory row gains the check directory `hl-popup` (2026-10-31-select-popup).
    rationale: The report's Symbols / APIs bullet Test-only state directories adds `cc-popup` and `hl-popup` under `CARGO_TARGET_TMPDIR`; the row's `cc-*` already covers `cc-popup` but its `hl-` names are listed one by one and omit `hl-popup`; the report also states no `ml-popup` exists, so no other name is owed.
    basis: escher-0.3.0/chunks/2026-10-31-select-popup/report.md:22
