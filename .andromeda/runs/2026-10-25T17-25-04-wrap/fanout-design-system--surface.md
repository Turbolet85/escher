proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "add after line 9 one list item — since 2026-10-25-cold-agent-bar the stand's flight booker overrides those select defaults on its one select, `select#flight-type` (class `type-select`, `aria-label` \"Flight type\"): 15px text, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, and on `:focus` no outline but a `#3a5ce5` border, as the stand's date inputs do; its two options (`one-way flight`, `return flight`) show the chosen one alone by the engine rule above (examples/seven_guis/src/tasks/flight_booker.rs:66-86; examples/seven_guis/src/tasks/flight_booker.rs:160-171; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "surface.md desktop-native-tokens: the stand's flight-type select and its own CSS over the user-agent select defaults (2026-10-25-cold-agent-bar)"
    rationale: "the report's Symbols / APIs names the new `select#flight-type` and its CSS (report.md:41-48, flight_booker.rs:66-86 and 160-171), Coverage reads it `tokens design-token✓` (report.md:304-307); line 9 states only the engine's select defaults, so the one select the stand now builds, its states (focus cue) and its carve-out from those defaults are new detail for this surface's component record"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:41
note: D-design-tokens holds — every new UI surface in the report's Coverage reads `tokens n/a` or `design-token✓`, none `hardcoded✗`; the retired `flight-one-way` / `flight-return` buttons and their CSS are named nowhere in surface.md or the body's Surface sections, so no D-retired site; the six-task usage line and the load page belong to layout-templates' surfaces and color-palette.md:27 per the plan's expected amendments.
