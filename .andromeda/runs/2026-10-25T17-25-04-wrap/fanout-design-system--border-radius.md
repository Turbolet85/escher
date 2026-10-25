proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/border-radius.md
    near: 12
    section: "## Border Radius"
    change: "the seven_guis (untokenized) row's Value cell gains `, flight-type select 6px` and its Usage cell the cite `examples/seven_guis/src/tasks/flight_booker.rs:160-171` — the row reads cards 8px, buttons 6px, inputs 4px, task cards 6px, tags 3px, flight-type select 6px"
    sidecar: "border-radius: seven_guis row names the flight booker's new `select#flight-type` at radius 6px (2026-10-25-cold-agent-bar)"
    rationale: "the report's Symbols / APIs: the flight booker's mode is now one `select#flight-type` (class `type-select`) whose CSS (flight_booker.rs:160-171) sets radius 6, beside the row's inputs at 4px; Coverage marks it tokens design-token✓, so no D-design-tokens drift — the radius stays an untokenized literal as the row already records"
    basis: examples/seven_guis/src/tasks/flight_booker.rs:160-171
    fact: F2
note: the retired `.type-btn` / `.type-btn-active` / `.flight-type-row` CSS has no site in border-radius.md (no D-retired); the load page's `.load-btn` CSS (stand.rs:220-229) is new, but the report states no radius for it, so no value is proposed — the orchestrator may read it if the row should carry it.
