group: architecture+security-plan+test-plan-1
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:26   class: stale
  leaf-text: «On the stand a click is accepted on 70 of the 77 snapshot nodes at boot and refused on 7: the three disabled controls, and each task's `task-header-spacer` as `covered` (a box 0 high; the hit at its centre answers the header).»
  source: .andromeda/architecture-detail/standard-contracts.md:171 «On the stand a driver `click` is accepted on 68 of the 77 snapshot nodes of the four lean tasks at boot, in each layout mode, and refused on 9»
  replace-with: «On the stand a click is accepted on 68 of the 77 snapshot nodes at boot and refused on 9: the three disabled controls, each task's `task-header-spacer` as `covered` (a box 0 high; the hit at its centre answers the header), and the flight booker's two options `flight-type-one-way` · `flight-type-return` as `covered`, each read at its select's client rect.»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «a click accepted on 70 nodes and refused on the 7 stated;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:104 «a driver `click` naming it is accepted on 68 and refused on the 9 the check states from the measured stand, each with its cause»
  replace-with: «a click accepted on 68 nodes and refused on the 9 stated;»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «`stand_act_states` (the step that changes a control's pressed, selected or invalid reading returns that node as it reads after the step)»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:19 «an acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step — a flight-type choice the select `flight-type` with its `value` and its two options»
  replace-with: «`stand_act_states` (the step that changes a control's selected or invalid reading returns that node as it reads after the step — a flight-type choice the select `flight-type` with its `value` and its two options with their `selected` reading, the stand stating no `aria-pressed`, `pressed` pinned on a fixture by `accessibility_states`)»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «`WITNESSES_020`, the 0.2.0 record, nine rows and 46 witnesses,»
  source: .andromeda/test-plan-detail/test-scope-summary.md:120 «`WITNESSES_020`, the 0.2.0 record: nine rows v020-01 to v020-09 in order, 49 witnesses, a `Subject::Served` witness naming a check on a page the stand serves beside the lean four»
  replace-with: «`WITNESSES_020`, the 0.2.0 record, nine rows and 49 witnesses (a `Subject::Served` witness naming a check on a page the stand serves beside the lean four — `stand_load_page` for v020-07, `stand_cells_served` for v020-09),»

- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «7 findings on the planted page, 0 on each lean task at boot (`the_lint_verb_answers_what_the_reader_reads`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:123 «7 findings on the planted page, the lean tasks' measured findings at boot (`the_lint_verb_answers_what_the_reader_reads`»
  replace-with: «7 findings on the planted page, the lean tasks' measured findings at boot (`the_lint_verb_answers_what_the_reader_reads`)»

- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «each lean task at boot reads 0 findings;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:124 «the lean tasks at boot read their measured findings, `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options»
  replace-with: «the lean tasks at boot read their measured findings, `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint reading a select's options at its client rect as overlapping, an open driver defect in `lint.rs`'s `overlaps` owned by a `.andromeda/residuals.md` line (`the_stand_at_boot_reads_its_measured_findings`);»

- leaf: .claude/docs/services/escher-driver.md:62   class: omission
  leaf-text: «a screen inside the budget read whole; picked up by `run stand` and the stand-repeat leg by its prefix).»
  source: .andromeda/test-plan-detail/test-scope-summary.md:133 «`tests/blitz-tests` holds two checks on the pages the stand serves beside the lean four, each declaring `mod session_common;` and no `mod fixtures;`, each test in both layout modes»
  replace-with: «a screen inside the budget read whole; picked up by `run stand` and the stand-repeat leg by its prefix). Since 2026-10-25-cold-agent-bar two checks on the pages the stand serves beside the lean four, each declaring `mod session_common;` and no `mod fixtures;`, both layout modes, picked up by their prefix: `stand_cells_served` (4, on `stand::boot_cells`, a screen of 5 parts: Cells walked in parts through the driver, committed cells reading their values and a cell past the viewport read in the last part, a click right after a `scroll` landing on the shown overlay scrollbar so the walk takes `advance 5000` first, the lint at boot reading its measured findings) and `stand_load_page` (2, on `stand::boot_load`: a click on `load-fetch` waiting out the load `LoadNet` answers after 200 ms inside `LOAD_WAIT` and `load-done` showing "Loaded" — the load wait's served witness — and the page keyed with no lint finding at boot).»

- leaf: .claude/docs/services/escher-driver.md:63   class: stale
  leaf-text: «a paged `part` read through the host's own sink stays unmeasured, the host booting only the four lean tasks and none paging»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:23 «A paged `part` read through the `escher-session` host's own sink is read by `host_log`'s fourth test, which runs `serve cells` at `RUST_LOG=trace`, commits a sentinel into a part-2 cell»
  replace-with: «and a fourth reading a paged `part` through the host's own sink — `serve cells` at `RUST_LOG=trace`, a sentinel committed into a part-2 cell, then `snapshot --part 2` — 0 id and 0 name needles under both builds, its needles from `common::cells_needles(part)`»

- leaf: .claude/docs/services/escher-driver.md:63   class: stale
  leaf-text: «`cli_commands` (every driver command as a process on the four tasks:»
  source: .andromeda/test-plan-detail/e2e-test-strategy.md:8 «`cli_commands` (14): each of the six served tasks — the four lean tasks, `cells` and `load` — starts, answers and stops»
  replace-with: «`cli_commands` (every driver command as a process on the six served tasks — the four lean tasks, `cells` and `load`:»

- leaf: .claude/docs/services/escher-driver.md:27   class: omission
  leaf-text: «the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one;»
  source: .andromeda/architecture-detail/standard-contracts.md:177 «the screen text's quotes escaped inside that line, so a reader decodes the line before it matches the screen's own text»
  replace-with: «the screen text itself for a whole screen and its JSON line `{"text","part","parts"}` for a paged one — the screen text's quotes escaped inside that line, so a reader decodes the line before it matches the screen's own text;»

- leaf: .claude/docs/services/escher-driver.md:29   class: omission
  leaf-text: «its `main` hands argv and the stand's boot to `command_line`, the Timer's session carrying the stand's time step»
  source: .andromeda/architecture-detail/standard-contracts.md:249 «over the stand's six served tasks — the four lean ones, Cells and the load page: `main` installs the telemetry sink first, then hands its argv, the six slugs»
  replace-with: «its `main` hands argv, the six served slugs `counter` · `flight-booker` · `timer` · `crud` · `cells` · `load` (one private `TASKS` table, from which the usage line's apps and the MCP `start` tool's `enum` are written) and the stand's boot to `command_line`, the Timer's session carrying the stand's time step»

- leaf: .claude/docs/services/escher-driver.md:29   class: omission
  leaf-text: «the agent's JSON check replayed on fresh `escher-session mcp` children)»
  source: .andromeda/architecture-detail/standard-contracts.md:286 «the live server child runs `mcp --record` into the owner-only `target/cold-agent/record/`, its two replay children without `--record`»
  replace-with: «the live server child run as `mcp --record` into the owner-only `target/cold-agent/record/` over the six served apps, the agent's JSON check replayed on fresh `escher-session mcp` children without `--record`)»
