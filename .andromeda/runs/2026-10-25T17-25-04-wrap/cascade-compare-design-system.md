# Cascade compare — design-system · wrap · 2026-10-25-cold-agent-bar
E items 8 · held 4 · lost at classify 0 · lost at strong 0 · beside a placed fact 2 · cascade only 7 · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### surface#1.2 · lost at classify · beside a placed fact F10 (near 35, surface#4.1) · .andromeda/design-system-detail/surface.md:9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 9
    section: "## Surface: desktop-native › ### Tokens (platform-specific)"
    change: "add after line 9 one list item — since 2026-10-25-cold-agent-bar the stand's flight booker overrides those select defaults on its one select, `select#flight-type` (class `type-select`, `aria-label` \"Flight type\"): 15px text, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, and on `:focus` no outline but a `#3a5ce5` border, as the stand's date inputs do; its two options (`one-way flight`, `return flight`) show the chosen one alone by the engine rule above (examples/seven_guis/src/tasks/flight_booker.rs:66-86; examples/seven_guis/src/tasks/flight_booker.rs:160-171; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "surface.md desktop-native-tokens: the stand's flight-type select and its own CSS over the user-agent select defaults (2026-10-25-cold-agent-bar)"
    rationale: "the report's Symbols / APIs names the new `select#flight-type` and its CSS (report.md:41-48, flight_booker.rs:66-86 and 160-171), Coverage reads it `tokens design-token✓` (report.md:304-307); line 9 states only the engine's select defaults, so the one select the stand now builds, its states (focus cue) and its carve-out from those defaults are new detail for this surface's component record"
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:41

### typography#1.9 · lost at classify · beside a placed fact F4 (near 30, typography#2.2) · .andromeda/design-system-detail/typography.md:18
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

## Held

- border-radius#1.3
- color-palette#2.3
- motion#1.6
- spacing#2.4

## Cascade only

- color-palette#5.1
- color-palette#5.2
- color-palette#7.3
- depth-strategy#1.3
- motion#4.1
- surface#15.1
- typography#2.2
