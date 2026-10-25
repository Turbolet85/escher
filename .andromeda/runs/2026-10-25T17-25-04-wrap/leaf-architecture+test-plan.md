group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:40   class: stale
  leaf-text: «the four lean apps, one at a time, in one isolated `claude -p` session»
  source: .andromeda/test-plan.md:180 «the one task `stand` — six apps, counter, flight booker, timer, CRUD, cells and the load page, in one client session»
  replace-with: «the six served apps — counter, flight booker, timer, CRUD, cells and the load page — one at a time, in one isolated `claude -p` session»

- leaf: .claude/docs/commands.md:40   class: omission
  leaf-text: «given only the escher server's twelve tools (the driver's own `escher-session mcp`;»
  source: .andromeda/architecture-detail/standard-contracts.md:286 «the live server child runs `mcp --record` into the owner-only `target/cold-agent/record/`, its two replay children without `--record`»
  replace-with: «given only the escher server's twelve tools (the driver's own `escher-session mcp`, the live child run with `--record` into the owner-only `target/cold-agent/record/`, its two replay children without it;»

- leaf: .claude/docs/commands.md:58   class: stale
  leaf-text: «the driver's command line over the four lean stand tasks (unix only): `start <counter|flight-booker|timer|crud> [--record <dir>]`»
  source: .andromeda/architecture.md:187 «the driver's command line over six served stand tasks — the four lean ones, Cells and the load page»
  replace-with: «the driver's command line over the six served stand tasks — the four lean ones, Cells and the load page (unix only): `start <counter|flight-booker|timer|crud|cells|load> [--record <dir>]`»

- leaf: .claude/docs/commands.md:59   class: stale
  leaf-text: «`start` naming one of the four tasks»
  source: .andromeda/architecture-detail/standard-contracts.md:249 «an MCP server on the binary's standard streams whose `start` tool boots the same six through the same boot»
  replace-with: «`start` naming one of the six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`)»

- leaf: .claude/docs/commands.md:60   class: stale
  leaf-text: «3 tests, unix only, the third reading a recording session's clients»
  source: .andromeda/test-plan-detail/test-scope-summary.md:15 «`tests/host_log.rs` (4 tests, unix only, the fourth below;»
  replace-with: «4 tests, unix only, the fourth serving `cells` at `RUST_LOG=trace`, committing a sentinel into a part-2 cell and reading `snapshot --part 2` — 0 id and 0 name needles under both builds — the third reading a recording session's clients»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «7 findings on the planted page, 0 on each lean task at boot»
  source: .andromeda/test-plan-detail/test-scope-summary.md:123 «7 findings on the planted page, the lean tasks' measured findings at boot»
  replace-with: «7 findings on the planted page, the lean tasks' measured findings at boot»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «its negatives nothing, each lean task at boot 0 findings,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:124 «the lean tasks at boot read their measured findings, `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`»
  replace-with: «its negatives nothing, the lean tasks at boot their measured findings `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options, a driver defect owned by a `.andromeda/residuals.md` line —,»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_act_select --test stand_snapshot_parts` · `cargo test -p seven_guis --locked --lib stand::`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:133 «`tests/blitz-tests` holds two checks on the pages the stand serves beside the lean four»
  replace-with: «--test stand_act_select --test stand_snapshot_parts --test stand_cells_served --test stand_load_page` · `cargo test -p seven_guis --locked --lib stand::`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «and a screen inside the budget read whole as before; each picked up by `run stand`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:133 «`tests/blitz-tests` holds two checks on the pages the stand serves beside the lean four»
  replace-with: «and a screen inside the budget read whole as before; since 2026-10-25-cold-agent-bar the two served-page checks, both layout modes — `stand_cells_served`, 4 tests, on the stand's own Cells (`stand::boot_cells`, 47,949 bytes in 740 lines, 5 parts): Cells walked in parts through the driver, committed cells reading their values and a cell past the viewport read in the last part, a click right after a `scroll` landing on the shown overlay scrollbar so the walk takes `advance 5000` first, the lint at boot reading its measured findings — and `stand_load_page`, 2 tests, on the load page (`stand::boot_load`): a click on `load-fetch` waiting out the load `LoadNet` answers after 200 ms and `load-done` showing "Loaded", and the page keyed with no lint finding at boot; each picked up by `run stand`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «· the stand's time step (milliseconds onto Timer ticks) ·»
  source: .andromeda/test-plan-detail/test-scope-summary.md:135 «the stand module holds three more unit tests — `the_stand_options_stay_offline`»
  replace-with: «· the stand's unit tests, 5 — the time step (milliseconds onto Timer ticks), `the_stand_options_stay_offline` (every boot but `boot_load` keeps `net_provider: None`), `cells_boots_in_the_shell` and `the_load_page_click_leaves_a_load_in_flight` ·»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «the 18 `controls(task)` rows of the four lean tasks at boot»
  source: .andromeda/architecture-detail/existing-scopes.md:113 «the 17 `controls(task)` rows over the four lean tasks at boot»
  replace-with: «the 17 `controls(task)` rows of the four lean tasks at boot»

- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «(`STATE_ROWS`, 19: 18 pass, 1 exempt)»
  source: .andromeda/architecture-detail/existing-scopes.md:113 «(`STATE_ROWS`, 18: 17 pass, 1 exempt»
  replace-with: «(`STATE_ROWS`, 18: 17 pass, 1 exempt)»

- leaf: .claude/docs/services/blitz-test-harness.md:16   class: omission
  leaf-text: «Consumed by seven_guis' headless stand (`seven_guis::stand::{boot, boot_timer, options}`)»
  source: .andromeda/architecture-detail/standard-contracts.md:127 «- `boot_cells(HarnessOptions) -> Harness<DioxusDocument>` mounts `Task::Cells` in `TaskShell`»
  replace-with: «Consumed by seven_guis' headless stand (`seven_guis::stand::{boot, boot_timer, boot_cells, boot_load, options}` — `boot_load` replacing its options' `net_provider` with the stand's `LoadNet` through `..options`, every other boot offline)»

- leaf: .claude/docs/services/blitz-test-harness.md:21   class: omission
  leaf-text: «an offline harness and a document passed to `wrap` carry none»
  source: .andromeda/architecture-detail/standard-contracts.md:108 «the stand's load page, whose `LoadNet` overrides no `is_noop`, carries the counter, so its one stylesheet fetch reads as a load in flight until it is answered»
  replace-with: «an offline harness (every stand boot but `boot_load`'s) and a document passed to `wrap` carry none; the stand's load page, whose `LoadNet` overrides no `is_noop`, carries the counter, so its one stylesheet fetch reads as a load in flight until it is answered»
