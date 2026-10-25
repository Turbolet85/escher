group: design-system+layout-templates
read: .claude/docs/design-summary.md · 43 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:13   class: stale
  leaf-text: «the five accent controls, 14–16 px»
  source: .andromeda/design-system-detail/color-palette.md:27 «measured as the background of white `#ffffff` button text on the four accent controls (`counter-increment`, `flight-book`, `timer-reset`, `crud-create`, 14–16 px, weight 500, at boot)»
  replace-with: «the four accent controls `counter-increment` · `flight-book` · `timer-reset` · `crud-create`, 14–16 px, weight 500»

- leaf: .claude/docs/design-summary.md:27   class: stale
  leaf-text: «the headless stand skips Home and mounts one lean task in TaskShell under `main#main` at the pinned 800 × 600 Light viewport»
  source: .andromeda/layout-templates.md:10 «the headless stand skips Home and mounts one lean task (Counter, FlightBooker, Timer, Crud) or Cells in TaskShell through `task_in_shell`, and the load page in a `TaskShell` of its own (`boot_load`)»
  replace-with: «the headless stand skips Home and mounts one lean task or Cells in TaskShell, or the load page in a TaskShell of its own (`boot_load`: the button `load-fetch`, white on the accent, and the paragraph `load-done`, `display: none` until the stylesheet its click links lands, answered by the in-process stand-in `LoadNet` after 200 ms), under `main#main` at the pinned 800 × 600 Light viewport»

- leaf: .claude/docs/design-summary.md:27   class: stale
  leaf-text: «and the flight booker and CRUD state `aria-pressed` · `aria-invalid` · `role="listbox"`/`role="option"` with `aria-selected` by attributes alone»
  source: .andromeda/layout-templates.md:10 «the classes that style the invalid and selected states staying, and the stand states no `aria-pressed`, the flight booker's mode a select whose options state its choice by `selected` (below)»
  replace-with: «and the flight booker's date inputs state `aria-invalid` and CRUD `role="listbox"`/`role="option"` with `aria-selected` by attributes alone, the stand stating no `aria-pressed` — the flight booker's mode is one `select#flight-type` (named by `aria-label` "Flight type"), its options `flight-type-one-way` · `flight-type-return` stating the choice by `selected`»

- leaf: .claude/docs/design-summary.md:27   class: stale
  leaf-text: «at boot each of the four lean tasks reads 0 findings in both layout modes, through the verb as through the reader (no overlap, clipping, truncated text, off-screen element, zero-size element holding content or contrast failure)»
  source: .andromeda/layout-templates.md:54 «and the flight booker reads one `overlap`, `flight-type-one-way` × `flight-type-return`, the snapshot giving each option its select's client rect»
  replace-with: «at boot the counter, the timer and CRUD read 0 findings in both layout modes, through the verb as through the reader (no overlap, clipping, truncated text, off-screen element, zero-size element holding content or contrast failure), the flight booker reads one `overlap`, `flight-type-one-way` × `flight-type-return` — the snapshot giving each option its select's client rect, the driver lint's reading of a select's options and not a layout defect, owned by a `.andromeda/residuals.md` line — and the load page reads 0»

- leaf: .claude/docs/design-summary.md:28   class: stale
  leaf-text: «its one task `stand`, the four lean apps in one client session»
  source: .andromeda/layout-templates.md:82 «and the one task `stand` — six apps — counter, flight booker, timer, CRUD, cells and the load page — in one client session»
  replace-with: «its one task `stand`, six apps — counter, flight booker, timer, CRUD, cells and the load page — in one client session, the live run's server child running `mcp --record` into the owner-only `target/cold-agent/record`, its kept checks never printed,»

- leaf: .claude/docs/design-summary.md:28   class: omission
  leaf-text: «escher's driver command line, built at 2026-10-10-driver-cli,»
  source: .andromeda/design-system-detail/surface.md:131 «`escher-session` serves six tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — Cells and the load page mounted in the same `TaskShell` as the lean four»
  replace-with: «escher's driver command line, built at 2026-10-10-driver-cli, serving six tasks — `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` — its usage line's `apps:` and the MCP `start` tool's `enum` listing all six,»

- leaf: .claude/docs/design-summary.md:24   class: omission
  leaf-text: «so after a driver `scroll` a settle reads `animating` true until 700 ms of that clock pass;»
  source: .andromeda/design-system-detail/motion.md:14 «so a driver `click` on a Cells cell in that band right after a `scroll` lands on the scrollbar, and an `advance` past the fade comes first»
  replace-with: «so after a driver `scroll` a settle reads `animating` true until 700 ms of that clock pass, and the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag, so a driver `click` on a Cells cell in that band right after a `scroll` lands on the scrollbar and an `advance` past the fade comes first;»
