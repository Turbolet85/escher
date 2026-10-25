proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 19
    section: "## Spacing"
    change: "add after line 19, in the same detail group: `- seven_guis flight booker's flight-type select (class type-select) pads 8/10 CSS px, 15px text, radius 6 (examples/seven_guis/src/tasks/flight_booker.rs:160-171)`"
    sidecar: "spacing: the flight booker's new flight-type select records its padding 8/10 px (2026-10-25-cold-agent-bar)"
    rationale: "report Changes › Symbols / APIs: the flight booker's two mode buttons give way to one select#flight-type whose CSS (160-171) sets padding 8/10; Coverage marks it tokens design-token✓, so it is new spacing detail, not D-design-tokens drift"
    basis: examples/seven_guis/src/tasks/flight_booker.rs:160-171
    fact: F2
note: D-design-tokens holds (both new UI elements, the load page and select#flight-type, read tokens design-token✓); the Retired CSS (.flight-type-row, .type-btn, .type-btn-active) and the lint-at-boot claim (plan's «spacing#2.4») have no passage in spacing.md or the body's Spacing section, so no D-retired proposal.
