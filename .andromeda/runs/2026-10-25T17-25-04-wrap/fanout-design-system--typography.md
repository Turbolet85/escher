proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 18
    section: "## Typography"
    change: "add a row after the seven_guis row — `| seven_guis flight booker's flight-type select (`select#flight-type`, class `type-select`) | not stated | 400 | 15px | not stated | (examples/seven_guis/src/tasks/flight_booker.rs:160-171) |`"
    sidecar: "Typography: the flight booker's `select#flight-type` reads 15px weight 400, a seven_guis control size beside its 14px base (2026-10-25-cold-agent-bar)."
    rationale: "The report's Symbols / APIs gives the select's CSS (flight_booker.rs:160-171) as 15px, and F5 measures its `stand_contrast` row at 15px 400. That is a type-scale step the chunk's new control uses, and the seven_guis row (base 14px; home title 36px) does not hold it. The retired buttons and their `.type-btn` CSS were never in this file, so there is no D-retired site. D-design-tokens is not violated: the Coverage line marks `select#flight-type` and the load page design-token✓ and every other new surface tokens n/a."
    basis: examples/seven_guis/src/tasks/flight_booker.rs:160-171
    fact: F5
note: D-design-tokens held (no hardcoded✗ flag in Coverage); the report gives no font size for the load page's CSS (stand.rs:203-243), so it brings no typography row.
