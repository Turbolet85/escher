group: test-plan-1
read: .claude/docs/tests-summary.md · 56 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»

- leaf: .claude/docs/tests-summary.md:14   class: omission
  leaf-text: «`seven_guis::stand::boot(LeanTask, options)` / `boot_timer(options)` mount counter, flight booker, timer or CRUD in TaskShell»
  source: .andromeda/test-plan.md:130 «beside the lean tasks it boots Cells through `stand::boot_cells(HarnessOptions)` (`Task::Cells` in TaskShell, no `LeanTask`) and the load page through `stand::boot_load(HarnessOptions)`»
  replace-with: «`seven_guis::stand::boot(LeanTask, options)` / `boot_timer(options)` mount counter, flight booker, timer or CRUD in TaskShell, and beside them `stand::boot_cells(options)` boots Cells and `stand::boot_load(options)` the load page (TaskShell titled "Load"), read by the served-page checks and by `escher-session`'s six tasks»

- leaf: .claude/docs/tests-summary.md:14   class: stale
  leaf-text: «offline (`net_provider: None`)»
  source: .andromeda/test-plan.md:337 «`net_provider: None` — kept for every caller but `stand::boot_load`, which replaces it with `Some(Arc::new(LoadNet))` through `..options`, every other field kept»
  replace-with: «offline (`net_provider: None` — every boot but `stand::boot_load`, which replaces it with the stand's `LoadNet`, answering `http://stand.test/load.css` from a thread of its own after 200 ms)»

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «asking for the four lean apps one at a time»
  source: .andromeda/test-plan.md:182 «The fixed prompt `STAND_PROMPT` asks for six apps one at a time — counter, flight booker (its return booking chosen on its flight type control, plus one screenshot), timer, CRUD, cells»
  replace-with: «asking for six apps one at a time — counter, flight booker (its return booking chosen on its flight type control, plus one screenshot), timer, CRUD, cells (plus one visual lint) and the load page —»

- leaf: .claude/docs/tests-summary.md:16   class: omission
  leaf-text: «the driver's own `escher-session mcp` with its twelve verbs»
  source: .andromeda/test-plan.md:182 «recreates `target/cold-agent/` with its owner-only `record/` (`mkdir -m 700`)»
  replace-with: «the driver's own `escher-session mcp --record` into the owner-only `target/cold-agent/record/` (`mkdir -m 700`, its kept checks never printed, copied or committed) with its twelve verbs»

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «all four lean apps in `tasks_met`»
  source: .andromeda/test-plan.md:184 «all six apps in `tasks_met` (an app is met from an accepted `snapshot` answer»
  replace-with: «all six apps in `tasks_met` (a paged screen read as the latest answer of each of its parts joined)»

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «`wrong_calls` recorded, never deciding.»
  source: .andromeda/test-plan.md:185 «`wrong_calls` = refused + error + foreign `tool_use` blocks + `permission_denials`, deciding the outcome against `wrong_call_bar_pct` (5)»
  replace-with: «`wrong_calls` deciding the outcome against `wrong_call_bar_pct` (5): `passed` also needs `wrong_calls × 100 ≤ 5 × tool_calls`, each of `lint_calls` · `screenshot_calls` · `advance_calls` · `part_reads` above 0 and `recording_kept` (each accepted `start` leaving one kept check), else `wrong-calls-over-bar` · `capability-unused` · `recording-not-kept`.»

- leaf: .claude/docs/tests-summary.md:16   class: stale
  leaf-text: «Proof: `test_cold_agent.py` (33) under a `claude` shim and a fake `target/debug/escher-session` + one recorded live run at 2026-10-15-cold-agent-test (passed, all four apps met, the check passed and able to fail, 43 calls, 0 wrong, 0.3197 USD; before it the retired stub's `counter` run, passed, 5 calls, 0 wrong). The live run is never in CI; the pipe now requires twelve tools with the `lint` verb, its contract tests' fake answering `lint`, and the live run is not re-fired.»
  source: .andromeda/test-plan.md:188 «the contract tests `test_cold_agent.py` (§4, 37) and the live run on the operator host, never part of CI (no CI secret), fired once: `passed`, reasons none»
  replace-with: «Proof: `test_cold_agent.py` (37) under a `claude` shim and a fake `target/debug/escher-session` (answering Cells as a paged JSON line of two parts and the load page, keeping under `--record` one check per accepted start) + the live run, fired once at 2026-10-25-cold-agent-bar (passed, reasons none, all six apps met, `counts_agree` 66 · 66, `lint_calls` 1 · `screenshot_calls` 1 · `advance_calls` 1 · `part_reads` 2, `recorded_sessions` 6 with `recording_kept`, the check passed and able to fail (59 steps, 16 expects), 66 calls, 0 wrong of 66 against the 5 % bar, 0.8244 USD, `claude` 2.1.308). The live run is never in CI.»

- leaf: .claude/docs/tests-summary.md:18   class: stale
  leaf-text: «`run stand` 197 · 0 · 7 over 48 files at 2026-10-24-stand-requirement-sweep (»
  source: .andromeda/test-plan.md:176 «`run stand`'s `run.end` reads passed 205 · failed 0 · ignored 7 over 50 files — +8 passed (`stand_cells_served` 4 and `stand_load_page` 2»
  replace-with: «`run stand` 205 · 0 · 7 over 50 files at 2026-10-25-cold-agent-bar (+8 passed: `stand_cells_served` 4 and `stand_load_page` 2, two new files picked up by their `stand_` prefix, `stand_keyboard` +1 and `stand_snapshot` +1), 197 · 0 · 7 over 48 files at 2026-10-24-stand-requirement-sweep (»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the 15 controls' roles and names»
  source: .andromeda/test-plan-detail/test-scope-summary.md:93 «the 14 controls' roles and names»
  replace-with: «the 14 controls' roles and names»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «exactly the active flight mode reading pressed»
  source: .andromeda/test-plan-detail/test-scope-summary.md:94 «exactly the chosen flight type's option reading selected and the reading swapping with the choice»
  replace-with: «exactly the chosen flight type's option reading selected»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «18 rows, one per `controls(task)` row over the four lean tasks at boot, read through `Harness::text_contrast`; pass 14 · fail 0»
  source: .andromeda/test-plan-detail/test-scope-summary.md:115 «(17 rows, one per `controls(task)` row over the four lean tasks at boot, read through `Harness::text_contrast` from computed styles, `flight-type` `#1a1a1a` on `#ffffff` at 17.40»
  replace-with: «17 rows, one per `controls(task)` row over the four lean tasks at boot, read through `Harness::text_contrast`, `flight-type` `#1a1a1a` on `#ffffff` at 17.40; pass 13 · fail 0»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the five former fails now `#ffffff` on `#3a5ce5` at 5.47»
  source: .andromeda/test-plan-detail/test-scope-summary.md:116 «the five former fails — four of them standing, `counter-increment`, `flight-book`, `timer-reset` and `crud-create` — now `#ffffff` on `#3a5ce5` at 5.47»
  replace-with: «the five former fails — four of them standing, `counter-increment`, `flight-book`, `timer-reset` and `crud-create` — now `#ffffff` on `#3a5ce5` at 5.47»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «within 1 per channel on all 17 measured rows»
  source: .andromeda/test-plan-detail/test-scope-summary.md:116 «within 1 per channel on all 16 measured rows»
  replace-with: «within 1 per channel on all 16 measured rows»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`STATE_ROWS` (19 reached states, 18 pass and 1 exempt»
  source: .andromeda/test-plan-detail/test-scope-summary.md:116 «(`STATE_ROWS`, 18 reached states — return-chosen 2 (`flight-type` and `flight-return-date`, each 17.40), start-unparseable 2, booked 1, row-selected 3, hover 5, active 5 — 17 pass and 1»
  replace-with: «`STATE_ROWS` (18 reached states, 17 pass and 1 exempt»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «`stand_keyboard` (14, 11 until»
  source: .andromeda/test-plan-detail/test-scope-summary.md:116 «`stand_keyboard` 15 (11 until 2026-10-14-stand-a11y-assertions»
  replace-with: «`stand_keyboard` (15, 11 until»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «Enter and Space on a focused stand button reading as its click, 20 rows»
  source: .andromeda/test-plan-detail/test-scope-summary.md:117 «Enter and Space on a focused stand button read as its click (16 rows stated and counted, `counter-increment`, `flight-book` and `crud-create` changing)»
  replace-with: «Enter and Space on a focused stand button reading as its click, 16 rows; the flight type select moving its choice by its arrows»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «the flight booker after `flight-return` (six stops)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:118 «the flight booker after the return flight is chosen on `flight-type` by ArrowDown (five stops»
  replace-with: «the flight booker after the return flight is chosen on `flight-type` by ArrowDown (five stops)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «nine rows v020-01 to v020-09 in order, 46 witnesses»
  source: .andromeda/test-plan-detail/test-scope-summary.md:120 «nine rows v020-01 to v020-09 in order, 49 witnesses, a `Subject::Served` witness naming a check on a page the stand serves beside the lean four»
  replace-with: «nine rows v020-01 to v020-09 in order, 49 witnesses, a `Subject::Served` witness naming a check on a page the stand serves beside the lean four (`stand_load_page` for v020-07, `stand_cells_served` for v020-09)»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «0 on each lean task at boot: a planted page reading exactly its seven findings in document order with their reasons and its negatives nothing, each lean task at boot 0 findings»
  source: .andromeda/test-plan-detail/test-scope-summary.md:124 «the lean tasks at boot read their measured findings, `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`»
  replace-with: «the lean tasks' measured findings at boot: a planted page reading exactly its seven findings in document order with their reasons and its negatives nothing, the lean tasks at boot reading `[0, 1, 0, 0]` — the flight booker's one `overlap`, `flight-type-one-way` × `flight-type-return`, the lint's reading of a select's options, a driver defect owned by a `.andromeda/residuals.md` line»

- leaf: .claude/docs/tests-summary.md:21   class: stale
  leaf-text: «a step returning the node whose pressed, selected or invalid reading it changed)»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:19 «an acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step»
  replace-with: «a step returning the node whose selected or invalid reading it changed)»

- leaf: .claude/docs/tests-summary.md:21   class: omission
  leaf-text: «a screen inside the budget read whole; picked up by `run stand` and the stand-repeat leg by its prefix)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:133 «`stand_cells_served` 4, on the stand's own Cells (`stand::boot_cells`, a screen of 47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`) through `Session::run`»
  replace-with: «a screen inside the budget read whole; picked up by `run stand` and the stand-repeat leg by its prefix) and, since 2026-10-25-cold-agent-bar, two checks on the pages the stand serves beside the lean four, each declaring `mod session_common;` and no `mod fixtures;`, both layout modes, picked up by `run stand` and the stand-repeat leg by their prefix — `stand_cells_served` (4, on the stand's own Cells, `stand::boot_cells`, 47,949 bytes in 740 lines, 5 parts of `SNAPSHOT_TEXT_BUDGET`, through `Session::run`: Cells walked in parts through the driver, committed cells reading their values and a cell past the viewport read in the last part, a click right after a `scroll` landing on the shown overlay scrollbar so the walk takes `advance 5000` before its click, the lint at boot reading its measured findings) and `stand_load_page` (2, on the load page, `stand::boot_load`: a click on `load-fetch` waiting out the load `LoadNet` answers after 200 ms and `load-done` showing "Loaded", the page keyed with no lint finding at boot)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «a click accepted on 70 of the 77 nodes and refused on 7»
  source: .andromeda/test-plan-detail/test-scope-summary.md:104 «a driver `click` naming it is accepted on 68 and refused on the 9 the check states from the measured stand»
  replace-with: «a click accepted on 68 of the 77 nodes and refused on 9»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «a step returning the node whose pressed, selected or invalid reading it changed, as it reads after the step»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:19 «an acting call returns the node it changed with its `selected` or `invalid` reading as it reads after the step»
  replace-with: «a step returning the node whose selected or invalid reading it changed, as it reads after the step»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «and `host_log` (3, unix only;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:15 «`tests/host_log.rs` (4 tests, unix only, the fourth below;»
  replace-with: «and `host_log` (4, unix only, the fourth reading a paged Cells part through the host's own sink;»

- leaf: .claude/docs/tests-summary.md:24   class: retired
  leaf-text: «a paged `part` read through the `escher-session` host's own sink stays unmeasured — the host boots only the four lean tasks and none pages — its measurement owned by a `.andromeda/residuals.md` line (target: next), the in-process capture reading one (`stand_act_spans`)»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:23 «A paged `part` read through the `escher-session` host's own sink is read by `host_log`'s fourth test, which runs `serve cells` at `RUST_LOG=trace`»
  replace-with: «a paged `part` read through the `escher-session` host's own sink is read by `host_log`'s fourth test — `serve cells` at `RUST_LOG=trace`, a sentinel committed into a part-2 cell, then `snapshot --part 2`, 0 id and 0 name needles under both builds — the in-process capture reading one too (`stand_act_spans`)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`host_log` holding 3 tests and `mcp_log` 2»
  source: .andromeda/test-plan-detail/integration-test-strategy.md:23 «`host_log` holds 4 tests and `mcp_log` 2»
  replace-with: «`host_log` holding 4 tests and `mcp_log` 2»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «`occurrences` and the unix-only `recorded_needles` (`mcp_log`'s own until then)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:14 «`found`, `occurrences`, the unix-only `recorded_needles` and `cells_needles(part)`»
  replace-with: «`occurrences`, the unix-only `recorded_needles` (`mcp_log`'s own until then) and `cells_needles(part)` (part `part`'s author keys of the in-process Cells screen and the middle column-A cell it holds)»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «three rounds of the 48 `stand_*.rs` files»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «run the 50 `stand_*.rs` files, the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry»
  replace-with: «three rounds of the 50 `stand_*.rs` files (48 before `stand_cells_served` and `stand_load_page`)»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «(104 CI-script tests in eight files — 100 in seven until»
  source: .andromeda/test-plan-detail/test-scope-summary.md:8 «108 tests in eight test files (§4; 104 with test_cold_agent.py's 33;»
  replace-with: «(108 CI-script tests in eight files — 104 with test_cold_agent.py's 33; 100 in seven until»

- leaf: .claude/docs/tests-summary.md:30   class: stale
  leaf-text: «`cargo test --workspace` 1105 · 0 · 12 over 206 result lines with `stand-repeat`'s ten named targets at 2026-10-24-stand-requirement-sweep (from»
  source: .andromeda/test-plan-detail/ci-integration.md:63 «re-counted with the served Cells and load page: 208 result lines, 1117 passed · 0 failed · 12 ignored — +2 result lines (`stand_cells_served`, `stand_load_page`)»
  replace-with: «`cargo test --workspace` 1117 · 0 · 12 over 208 result lines with the served Cells and load page at 2026-10-25-cold-agent-bar (from 206 · 1105: +2 result lines — `stand_cells_served`, `stand_load_page` — and +12 passed — `stand_cells_served` 4, `stand_load_page` 2, seven_guis' three stand unit tests, `host_log` 1, `stand_keyboard` 1, `stand_snapshot` 1; `run stand` 205 passed · 0 failed · 7 ignored over 50 files, `run all` not re-counted; timings not re-measured); 1105 · 0 · 12 over 206 result lines with `stand-repeat`'s ten named targets at 2026-10-24-stand-requirement-sweep (from»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «read again with the stand checks' named list at 2026-10-24-stand-requirement-sweep escher-driver 97.99 %, dioxus-native-dom 85.57 %, blitz-test-harness 94.15 % (equal to CI#40105170961)»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:16 «read again with the served Cells and load page, the floors and `coverage_floor.py` unchanged: escher-driver 4918/5019 = 97.99 %, dioxus-native-dom 2741/3202 = 85.60 %, blitz-test-harness 952/1008»
  replace-with: «read again with the served Cells and load page at 2026-10-25-cold-agent-bar escher-driver 97.99 %, dioxus-native-dom 85.60 %, blitz-test-harness 94.44 %»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «zero failures since 2026-10-14-quality-gates — the 48 `stand_*.rs` files»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «run the 50 `stand_*.rs` files, the ten named blitz-tests targets of its `--test` list and seven_guis' eight process checks three times with no retry»
  replace-with: «zero failures since 2026-10-14-quality-gates — the 50 `stand_*.rs` files»

- leaf: .claude/docs/tests-summary.md:43   class: omission
  leaf-text: «(with the ten CI#40114977936's»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:22 «with the served Cells and load page the local leg `stand-repeat: 3 of 3 rounds green` and CI#40214713710 green 17/17»
  replace-with: «(with the served Cells and load page the local leg `stand-repeat: 3 of 3 rounds green` and CI#40214713710 green 17/17; with the ten CI#40114977936's»
