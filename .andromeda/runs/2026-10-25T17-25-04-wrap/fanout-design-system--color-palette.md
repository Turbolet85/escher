proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 27
    section: "## Color Palette › ### Core Colors"
    change: "In the seven_guis accent row's Usage cell, replace «on the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500–600, at boot)» with «on the four accent controls (`counter-increment`, `flight-book`, `timer-reset`, `crud-create`, at boot) — `flight-one-way`, the fifth until 2026-10-25-cold-agent-bar, retired with the flight booker's two mode buttons and their CSS (`.flight-type-row`, `.type-btn`, `.type-btn-active`)»; every other clause of the row (5.47, 6.83, 8.27, the re-colour history) unchanged."
    sidecar: "color-palette.md:27 — the accent's measured controls five → four: `flight-one-way` retired with the flight booker's mode buttons (2026-10-25-cold-agent-bar)."
    rationale: "The report's Retired bullet names the flight booker's two `aria-pressed` mode buttons (`flight-one-way`, `flight-return`) and their CSS; F5 retires the `flight-one-way` accent row (white on `#3a5ce5`, 5.47, 14px 600) from `stand_contrast` (control rows 18 → 17); Expected amendments names this site («design-system §Color Palette (`flight-one-way` leaves the accent rows …) — site color-palette.md:27»). The «14–16 px, weight 500–600» span included the retired 14px control and the report states no re-read span for the four that remain, so it is dropped rather than restated."
    basis: escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md:91
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 27
    section: "## Color Palette › ### Core Colors"
    change: "Add a row after the seven_guis accent row: «| seven_guis flight-type select | `#1a1a1a` on `#ffffff` (border `1px solid #c0c0c0`; focus drops the outline for a `#3a5ce5` border, as the date inputs do) | The flight booker's `select#flight-type` (class `type-select`, 15px, weight 400) since 2026-10-25-cold-agent-bar, in place of the two mode buttons; measured by `stand_contrast` at 17.40 at boot and with the return flight chosen, beside `flight-return-date` 17.40 (examples/seven_guis/src/tasks/flight_booker.rs:160-171) |»"
    sidecar: "color-palette.md — new row: the flight booker's `flight-type` select, `#1a1a1a` on `#ffffff`, border `#c0c0c0`, focus border `#3a5ce5`, 17.40 (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs: the select's CSS (flight_booker.rs:160-171) is 15px, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, `:focus` the `#3a5ce5` border; F5: `flight-type` (`#1a1a1a` on `#ffffff`, 17.40, 15px 400) and the return-chosen rows; Expected amendments «the select's pair measured»; Coverage `select#flight-type` tokens design-token✓."
    basis: examples/seven_guis/src/tasks/flight_booker.rs:160-171
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 27
    section: "## Color Palette › ### Core Colors"
    change: "Add a row after the flight-type select row: «| seven_guis load page | `#3a5ce5` / white (hover `#2a4cd3`, active `#1f3fc1`) on page `#f5f5f5`, text `#1a1a1a` | The served load page's `load-fetch` button takes the stand accent and its two state shades, `load-done` the stand text colour (`.load-btn` · `:hover` · `:active` · `.load-done`, examples/seven_guis/src/stand.rs:203-243); the page is no `LeanTask`, so `stand_contrast` holds no row for it |»"
    sidecar: "color-palette.md — new row: the load page (`boot_load`) declares the stand accent, its hover and active shades, `#f5f5f5` and `#1a1a1a`; no contrast row (2026-10-25-cold-agent-bar)."
    rationale: "Report Symbols / APIs: `load-fetch` (Button \"Load\", the stand accent `#3a5ce5` / white, hover `#2a4cd3`, active `#1f3fc1`), its CSS at stand.rs:203-243; Coverage `stand::boot_load` tokens design-token✓ (the stand accent `#3a5ce5` / `#2a4cd3` / `#1f3fc1`, `#1a1a1a`, `#f5f5f5`) and «no keyboard or contrast row — the page is no `LeanTask`»."
    basis: examples/seven_guis/src/stand.rs:203-243
    fact: F17
note: D-design-tokens reads no `hardcoded✗` in the report's Coverage (load page and select design-token✓, the rest n/a), so it fires nothing; color-palette.md:27 also cites `tests/blitz-tests/tests/stand_contrast.rs:49-132` and `:294-406`, a file this chunk edited at 68-69, 279, 295, 299-300 and 413-423 — the F5 sweep lists no citation row here, so it is left to the P2 citation sweep.
