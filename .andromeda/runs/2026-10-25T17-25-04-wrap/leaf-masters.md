group: masters
read: CLAUDE.md · 151 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:38   class: stale
  leaf-text: «its flight booker and CRUD state which control is pressed, invalid or selected by `aria-*` attributes alone»
  source: .andromeda/a11y-plan-detail/screen-reader-support.md:45 «the stand states no `aria-pressed` — the flight booker's mode is one `select#flight-type`»
  replace-with: «its flight booker states which date is invalid and its CRUD which row is selected by `aria-*` attributes alone, the flight booker's mode a `select#flight-type` whose options state its choice by `selected`, and the stand states no `aria-pressed`»

- leaf: CLAUDE.md:38   class: stale
  leaf-text: «its native `stand` module boots counter, flight booker, timer or CRUD headlessly in TaskShell (pinned viewport, bundled font, offline, fresh per boot)»
  source: .andromeda/architecture.md:163 «every boot offline but the load page's, whose one stylesheet fetch the in-process stand-in `LoadNet` answers»
  replace-with: «its native `stand` module boots counter, flight booker, timer or CRUD headlessly in TaskShell and, beside them, Cells (`boot_cells`, no `LeanTask`) and a load page (`boot_load`) (pinned viewport, bundled font, fresh per boot, every boot offline but the load page's, whose one stylesheet fetch the in-process stand-in `LoadNet` answers after 200 ms)»

- leaf: CLAUDE.md:38   class: stale
  leaf-text: «is the driver's command line over the four»
  source: .andromeda/architecture.md:187 «the driver's command line over six served stand tasks — the four lean ones, Cells and the load page»
  replace-with: «is the driver's command line over six served tasks — the four lean ones, Cells and the load page (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`)»

- leaf: CLAUDE.md:38   class: stale
  leaf-text: «booting the same four through the same boot»
  source: .andromeda/layout-templates.md:10 «booting one of the same six served tasks in process on a `start` tool call through the same boot»
  replace-with: «booting the same six through the same boot»

- leaf: CLAUDE.md:45   class: omission
  leaf-text: «so a stand colour change moves its record;»
  source: .andromeda/a11y-plan.md:142 «OPEN — the pages the stand serves beside the lean four hold no Tab-order, keyboard-activation or contrast row»
  replace-with: «so a stand colour change moves its record; the pages the stand serves beside the lean four hold no Tab-order, keyboard-activation or contrast row, so SC 2.1.1 and 1.4.3 are unasserted on the load page's `load-fetch` Button, a gap owned by a `.andromeda/residuals.md` line;»

- leaf: CLAUDE.md:44   class: omission
  leaf-text: «in an owner-only file never logged (ratified by the founder, 2026-10-19),»
  source: .andromeda/security-plan.md:185 «the cold-agent pipe's live child keeps its checks so in the owner-only `target/cold-agent/record/`, read by the pipe for bools and counts only»
  replace-with: «in an owner-only file never logged (ratified by the founder, 2026-10-19) — the cold-agent pipe's live child keeping its checks so in the owner-only `target/cold-agent/record/`, read by the pipe for bools and counts only, never printed, copied or committed —,»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «and the stand's flight booker and CRUD state all three»
  source: .andromeda/architecture-detail/standard-contracts.md:226 «the stand states no `aria-pressed`, the flight booker's mode a `select#flight-type` whose options state its choice»
  replace-with: «and the stand's flight booker and CRUD state invalid and selected — the stand states no `aria-pressed`, the flight booker's mode a `select#flight-type` whose options state its choice, and `accessibility_states` pins the pressed reading on a fixture»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «every stand screen under the 10,000-byte `SNAPSHOT_TEXT_BUDGET`»
  source: .andromeda/architecture-detail/standard-contracts.md:127 «Cells staying no `LeanTask`: its screen passes the snapshot text's budget and reads in parts»
  replace-with: «every lean stand screen under the 10,000-byte `SNAPSHOT_TEXT_BUDGET` (the Cells screen the stand serves beside them passing it)»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «proven on an over-budget fixture page in both layout modes, in process and through `serve_mcp`»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:24 «the stand's own Cells screen, booted through `stand::boot_cells`: 47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`»
  replace-with: «proven on an over-budget fixture page in both layout modes, in process and through `serve_mcp`, and on the stand's own Cells screen (47,949 bytes in 740 lines, 5 parts) walked in parts through the driver»

- leaf: CLAUDE.md:110   class: omission
  leaf-text: «proven on the stand's Timer and on minimal fixtures in both layout modes»
  source: .andromeda/architecture-detail/standard-contracts.md:197 «The load wait's served witness on the stand is the load page»
  replace-with: «proven on the stand's Timer, on the stand's load page — a `click` on `load-fetch` waiting out its `LoadNet` load inside `LOAD_WAIT` — and on minimal fixtures in both layout modes»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «proven on the four lean tasks by the binary run as a process»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:8 «each of the six served tasks — the four lean tasks, `cells` and `load` — starts, answers and stops»
  replace-with: «proven on the six served tasks by the binary run as a process»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «each lean task reading 0 findings at boot»
  source: .andromeda/layout-templates.md:54 «and the flight booker reads one `overlap`, `flight-type-one-way` × `flight-type-return`»
  replace-with: «the counter, the timer, CRUD and the load page reading 0 findings at boot and the flight booker one `overlap` (`flight-type-one-way` × `flight-type-return`, the snapshot giving each option its select's client rect — the lint's reading of a select's options, owned by a `.andromeda/residuals.md` line)»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «at a pinned viewport with its bundled font and no network»
  source: .andromeda/architecture.md:163 «every boot offline but the load page's, whose one stylesheet fetch the in-process stand-in `LoadNet` answers»
  replace-with: «at a pinned viewport with its bundled font, every boot offline but the load page's, whose one stylesheet fetch the in-process stand-in `LoadNet` answers»
