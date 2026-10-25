proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 27
    section: "## Color Palette › ### Core Colors"
    change: "In the seven_guis accent row, the measured accent controls read four, not five: `counter-increment`, `flight-book`, `timer-reset`, `crud-create` (white `#ffffff` on `#3a5ce5`, 5.47; hover 6.83, active 8.27) — since 2026-10-25-cold-agent-bar the flight booker's `flight-one-way` mode button (14px 600, the active one of its two `aria-pressed` buttons) is retired with its row, its non-text accent use surviving as the `flight-type` select's focus border; and re-cite tests/blitz-tests/tests/stand_contrast.rs's two ranges (49-132 · 294-406), moved by the lines the chunk added at 68-69, 279, 295, 299-300 and 413-423 (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)."
    sidecar: "color-palette#2.3: the accent controls five → four, `flight-one-way` retired (2026-10-25-cold-agent-bar); stand_contrast citations re-read."
    rationale: "The report's Retired bullet names the flight booker's two `aria-pressed` mode buttons and their CSS; F5 states the `flight-one-way` accent row (white on #3a5ce5, 5.47, 14px 600) gives way to `flight-type` (#1a1a1a on #ffffff, 17.40); New text lists stand_contrast.rs additions at 68-69, 279, 295, 299-300, 413-414, 417-423."
    basis: "tests/blitz-tests/tests/stand_contrast.rs:68"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 27
    section: "## Color Palette › ### Core Colors"
    change: "Add after the accent row: | seven_guis load page (stand) | `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`), `#1a1a1a`, `#f5f5f5` | The served load page's Load button `load-fetch` is white on the accent at rest, on hover and when active, from CSS outside the task files (`.load-btn` and its `:hover` · `:active`); the page also uses `#1a1a1a` and `#f5f5f5`; it holds no contrast row, being no lean task (examples/seven_guis/src/stand.rs:203-243; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md) |"
    sidecar: "color-palette: the load page's accent button and its colours recorded (2026-10-25-cold-agent-bar, F2)."
    rationale: "Report Symbols / APIs: `load-fetch` (Button \"Load\", the stand accent #3a5ce5 / white, hover #2a4cd3, active #1f3fc1), CSS at stand.rs:203-243; Coverage: tokens design-token✓ (#3a5ce5 / #2a4cd3 / #1f3fc1, #1a1a1a, #f5f5f5), no keyboard or contrast row."
    basis: "examples/seven_guis/src/stand.rs:220"
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 99
    section: "## Color Palette › ### Border Progression"
    change: "Add after the blitz-dom focus-outline row: | Field — seven_guis flight-type select | `1px solid #c0c0c0`; focused, `#3a5ce5` | The flight booker's `flight-type` select (`.type-select`) paints `#1a1a1a` on `#ffffff` (17.40, 15px 400) inside a `1px solid #c0c0c0` border; on focus it drops the outline for a `#3a5ce5` border, as the date inputs do (examples/seven_guis/src/tasks/flight_booker.rs:160-171; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md) |"
    sidecar: "color-palette: the flight-type select's border, focus border and text pair recorded (2026-10-25-cold-agent-bar, F1)."
    rationale: "Report Symbols / APIs, the flight booker: its CSS (160-171) 15px, #1a1a1a on #ffffff, 1px solid #c0c0c0, radius 6, padding 8/10; :focus drops the outline for the #3a5ce5 border; F5 reads it 17.40."
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:160"
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 71
    section: "## Color Palette › ### Semantic Colors"
    change: "Keep the seven_guis invalid row's values (`#e53e3e` / `#fff5f5` / `#c53030`) and re-cite its examples/seven_guis/src/tasks/flight_booker.rs ranges (194-198 · 229-237), which lie past the blocks the chunk added at 66-80, 83-86 and 160-171 and so have moved."
    sidecar: "color-palette#5.1: flight_booker.rs citations re-read after the select replaced the mode buttons (2026-10-25-cold-agent-bar)."
    rationale: "flight_booker.rs is a reached file; New text lists additions at 66-80 · 83-86 · 160-162 · 165-166 · 169-171 and the Retired bullet removes the buttons and their CSS, so every cited line past 66 can have moved."
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:66"
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 72
    section: "## Color Palette › ### Semantic Colors"
    change: "Keep the seven_guis success row's values (`#ebf8ee` / `#68d391` / `#276749`) and re-cite its examples/seven_guis/src/tasks/flight_booker.rs ranges (194-198 · 229-237), moved past the blocks added at 66-80, 83-86 and 160-171."
    sidecar: "color-palette#5.2: flight_booker.rs citations re-read (2026-10-25-cold-agent-bar)."
    rationale: "The same reached file and added blocks as the invalid row; the report moves no status colour."
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:160"
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 72
    section: "## Color Palette › ### Semantic Colors"
    change: "Add after the success row: | seven_guis pressed | none since 2026-10-25-cold-agent-bar | The stand states no `aria-pressed` anywhere: the flight booker's mode buttons and `.type-btn-active` are retired, and the flight type's state is the `flight-type` select's chosen option (`selected`), so no pressed-state colour pair is left (examples/seven_guis/src/tasks/flight_booker.rs:66-86; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md) |"
    sidecar: "color-palette: the stand's pressed-state pair recorded gone (2026-10-25-cold-agent-bar, F3)."
    rationale: "Report Retired bullet (the stand's only aria-pressed controls) and F18 (the stand no longer states aria-pressed anywhere; the select states its choice by selected); flight_booker.rs:66-86 the select's markup."
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:66"
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 30
    section: "## Typography"
    change: "Keep the stand's bundled DejaVu Sans rule and re-cite examples/seven_guis/src/stand.rs:59-61 (`build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`), which the lines added at 1-9, 12 and 17 have moved; the same font context now also boots Cells and the load page through `boot_cells` and `boot_load` (stand.rs:131-152)."
    sidecar: "typography#2.2: stand.rs citation re-read after the new boots (2026-10-25-cold-agent-bar)."
    rationale: "stand.rs is a reached file; New text lists additions at 1-9 · 12 · 17 · 131-245 · 283-330; the report keeps every option field but `net_provider` in `boot_load` (`..options`)."
    basis: "examples/seven_guis/src/stand.rs:131"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 30
    section: "## Typography"
    change: "Add after the stand-font bullet: - The seven_guis flight booker's `flight-type` select sets its text at 15px, weight 400, where the retired mode buttons read 14px 600 (examples/seven_guis/src/tasks/flight_booker.rs:160-171; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "typography: the flight-type select's 15px 400 recorded (2026-10-25-cold-agent-bar, F4)."
    rationale: "Report Symbols / APIs: the select's CSS (160-171) 15px; F5 reads `flight-type` 15px 400 in place of the 14px 600 accent row."
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:160"
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 19
    section: "## Spacing"
    change: "Add after the seven_guis cards bullet: - The seven_guis flight booker's `flight-type` select pads 8px vertical, 10px horizontal (examples/seven_guis/src/tasks/flight_booker.rs:160-171; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "spacing: the flight-type select's 8/10 padding recorded (2026-10-25-cold-agent-bar, F6)."
    rationale: "Report Symbols / APIs: the select's CSS (160-171), padding 8/10."
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:160"
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 10
    section: "## Depth Strategy"
    change: "Keep the seven_guis card shadow values (`0 2px 8px rgba(0, 0, 0, 0.08)`, the circle dialog 0.10) and re-cite examples/seven_guis/src/tasks/flight_booker.rs:148, which lies past the blocks added at 66-80 and 83-86 and so has moved."
    sidecar: "depth-strategy#1.3: flight_booker.rs citation re-read (2026-10-25-cold-agent-bar)."
    rationale: "flight_booker.rs is a reached file; New text lists additions at 66-80 · 83-86 · 160-171 and the Retired bullet removes the mode buttons' markup and CSS; the report moves no shadow."
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:83"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/border-radius.md
    near: 12
    section: "## Border Radius"
    change: "In the seven_guis (untokenized) row, add the flight booker's `flight-type` select at 6px beside cards 8px, buttons 6px, inputs 4px, task cards 6px and tags 3px, citing examples/seven_guis/src/tasks/flight_booker.rs:160-171 (as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)."
    sidecar: "border-radius: the flight-type select's radius 6 recorded (2026-10-25-cold-agent-bar, F5)."
    rationale: "Report Symbols / APIs: the select's CSS (160-171), radius 6."
    basis: "examples/seven_guis/src/tasks/flight_booker.rs:160"
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 13
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: "Add after the overlay-scrollbar fade bullet: - Right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag (`hit_with_scrollbar` while `scrollbar_opacity > 0`) and dispatches nothing; on the virtual clock it stays shown until an `advance` passes its fade, and a pointer left on the thumb keeps it shown — so a driver `click` on a cell in that band right after a `scroll` lands on the scrollbar (tests/blitz-tests/tests/stand_cells_served.rs:176-192; tests/blitz-tests/tests/stand_cells_served.rs:91-133; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "motion: the shown overlay scrollbar's thumb-drag take of a click after a scroll recorded (2026-10-25-cold-agent-bar, F7)."
    rationale: "Report Decisions & corrections, Measured: right after scroll the overlay scrollbar takes a pointerdown at a cell in its band as a thumb drag; it stays shown until advance passes its fade; a pointer resting on the thumb keeps it shown; pinned by stand_cells_served's a_click_right_after_the_scroll_lands_on_the_shown_scrollbar (176-192)."
    basis: "tests/blitz-tests/tests/stand_cells_served.rs:177"
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 32
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: "Add after the load-wait bullet: - The stand's served load page exercises the wait: its in-process stand-in `LoadNet` answers the page's one sheet after `LOAD_ANSWER_DELAY` (200 ms) from a thread of its own, and the `Loaded` paragraph `load-done` stays `display: none` until that sheet lands, so a driver `click` on `load-fetch` waits it out and returns with the message shown (examples/seven_guis/src/stand.rs:141-152; examples/seven_guis/src/stand.rs:167-175; tests/blitz-tests/tests/stand_load_page.rs:34-68; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "motion: the load page's 200 ms stand-in under the load wait recorded (2026-10-25-cold-agent-bar, F8)."
    rationale: "Report Symbols / APIs: boot_load (141-152), LoadNet's fetch answering LOADED_CSS after LOAD_ANSWER_DELAY = 200 ms from its own thread (167-175), load-done display none until the sheet lands; witness stand_load_page::the_load_page_click_waits_out_its_load_and_shows_the_message (34-68)."
    basis: "examples/seven_guis/src/stand.rs:167"
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 123
    section: "## Surface: cli › ### Component Patterns"
    change: "Add after the escher-session output bullet: - Since 2026-10-25-cold-agent-bar `escher-session` serves six tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — Cells and the load page mounted in the same `TaskShell` as the lean four; its usage line's `apps:` and the MCP `start` tool's `enum` list all six, the program's own fixed text, and an unknown task stays a usage ending (examples/seven_guis/src/session_host.rs:12-27; as measured at escher-0.2.0/chunks/2026-10-25-cold-agent-bar/report.md)"
    sidecar: "surface: escher-session's six served tasks recorded on the cli surface (2026-10-25-cold-agent-bar, F9)."
    rationale: "Report Symbols / APIs: `escher-session` (session_host.rs:12-27) `enum Served { Lean(LeanTask), Cells, Load }` and `TASKS: [(&str, Served); 6]`, the usage line's apps and the MCP start enum following; Outcome (arch): six slugs served, an unknown task a usage ending."
    basis: "examples/seven_guis/src/session_host.rs:12"
    fact: F9
note: D-design-tokens did not fire (Coverage tokens read n/a or design-token✓ on all four new surfaces); F10 (the load page's keyed elements and classes) has no picked surface entity to carry it and is left to the new-detail pass.
