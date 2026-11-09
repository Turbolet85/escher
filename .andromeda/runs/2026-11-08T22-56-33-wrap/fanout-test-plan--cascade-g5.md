@proposal n=1 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-csattoyz fact=F1 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-nu73ltec
section: ## 5. Integration Test Strategy
change: add after it: - **Session ↔ held instance, handler coverage:** `stand_handler_coverage` (8 tests, each in both layout modes) holds `escher_driver::handler_coverage` on the six served tasks — a snapshot check lists exactly `HANDLERS_AT_BOOT`, equal to the graph at boot; each task's drives leave its `UNEXERCISED_AFTER_DRIVES` row; two checks list only what neither exercised; the load page reads its click exercised; a refused step exercises nothing and a keyboard activation its click; a handler whose step removes its element reads exercised; a typed sentinel never reaches the coverage line; a check of another app contributes nothing and does not hold (tests/blitz-tests/tests/stand_handler_coverage.rs:212-423) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: integration-test-strategy: stand_handler_coverage's eight checks of handler_coverage added beside the UI graph entity (F1)
rationale: the report's Outcome (matrix#v030-07 MET) and Harness surface name stand_handler_coverage, 8 tests, a new stand target no master states (shc control unfired, 0 sites)
@proposal n=2 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-csattoyz fact=F2 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-rhs2cert
section: ## 5. Integration Test Strategy
change: add after it: - **Handler coverage's screens, PROVISIONAL:** handler coverage reads every screen the replays reach — the graph at boot and after each replayed step — the overseer's delegate call on the screens (`inputs#I4`, Q2), discharged at the epoch boundary (`rul-3ndrhilc`) — as stated at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: integration-test-strategy: the screens-spanned decision added as a PROVISIONAL line beside the UI graph entity (F2, E10)
rationale: the report's Decisions record the overseer's delegate call riding test-plan §5 PROVISIONAL (E10, `provisional` test-plan 0, new)
@proposal n=3 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-csattoyz fact=F3 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-wbvne53q
section: ## 5. Integration Test Strategy
change: add after it: - **Handler coverage, its stated limits:** handlers inside a canvas are not counted (deferred to 0.4.0, the incubator's canvas plans); a handler on no screen a replay reached is not listed (by construction, no owner); a bubbling dispatch counts every ancestor listener it passed, `stopPropagation` unread (a new residual row, target next) (docs/capabilities.md:273-281) — as stated at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: integration-test-strategy: handler coverage's three stated limits added beside the UI graph entity (F3, E7)
rationale: the report's I5 rows (P5 card 2, operator «yes») and New text by line (capabilities.md 273-281) state the three limits; E7 places them in test-plan §5
@proposal n=4 detector=D-cascade severity=warning file=integration-test-strategy anchor=fct-kzdrls5d basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-npklafdi
section: ## 5. Integration Test Strategy
change: - **A kept actions step's replay, discharged:** the replay of a kept `actions` step is no longer owed — `stand_replay`'s `a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes` measures it, discharging the CARRY on «Handler coverage» (`cry-sxrivdk3`) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: integration-test-strategy: the owed actions-step replay test marked discharged by stand_replay's new test
rationale: the report's Outcome matrix#v030-06 MET names the test and states it discharges CARRY cry-sxrivdk3; the owner-title disposition lists fct-kzdrls5d as changed
@proposal n=5 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-kzdrls5d fact=F4 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-npklafdi
section: ## 5. Integration Test Strategy
change: add after it: - **A kept actions step's replay, measured:** `stand_replay`'s `a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes` replays a recorded Counter session with two `actions` steps on a fresh Counter, all 4 steps running, and the copy with one `keyed` flipped stops at step 3, keys exactly `["nodes"]`, in both layout modes (tests/blitz-tests/tests/stand_replay.rs:263-329) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: integration-test-strategy: stand_replay's kept-actions-step test added beside the discharged owed item (F4)
rationale: the report's Outcome matrix#v030-06 and New text by line (stand_replay.rs 263-329) state the test and its readings
@proposal n=6 detector=D-cascade severity=warning file=integration-test-strategy anchor=fct-adkwyvky basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## 5. Integration Test Strategy
change: in «`host_log` holds 5 tests and `mcp_log` 3», read «`host_log` holds 6 tests and `mcp_log` 3», the rest of the entity as it stands
sidecar: integration-test-strategy: host_log's test total 5 → 6 with the coverage run's test
rationale: the report's Counts state seven_guis host_log tests 5 → 6 (pattern host-log), the new test a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace
@proposal n=7 detector=D-new-detail severity=warning file=integration-test-strategy anchor=fct-adkwyvky fact=F6 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-n5joryov
section: ## 5. Integration Test Strategy
change: add after it: - **Session host ↔ what it writes, a coverage run:** seven_guis' `host_log`'s `a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace` runs `coverage crud` over a CRUD session kept through the binary with a typed sentinel and a row click, and reads 0 id, name, coverage, sentinel, path and recorded needles on stderr at `trace`, the coverage needles from `common::coverage_needles` (the CRUD screen's), under the `-p seven_guis` and the workspace build (examples/seven_guis/tests/host_log.rs:141-226; examples/seven_guis/tests/common/mod.rs:454-479) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: integration-test-strategy: host_log's coverage-run test added beside the host's log entity (F6)
rationale: the report's Outcome (obs) and deviation 3 state the run, its CRUD needles and both builds green (entries 7 and 14)
@proposal n=8 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-csryo2zo basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-xgydedcz
section: ## 6. E2E Test Strategy
change: in «`cli_commands` (17)», read «`cli_commands` (18)», the rest of the entity as it stands
sidecar: e2e-test-strategy: cli_commands' linux test count 17 → 18 with the coverage test
rationale: the report's Counts state cli_commands tests on linux 17 → 18 (pattern cli-cmds), naming fct-csryo2zo «`cli_commands` (17)» a changed site
@proposal n=9 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-csryo2zo dependent_of=D-cascade basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-zcn6rmkk
section: ## 6. E2E Test Strategy
change: in «`cli_commands` reads the usage line naming `--record` on `start`, the host role and the `mcp` role», read «`cli_commands` reads the usage line naming `--record` on `start`, the host role, the `mcp` role and, since 2026-11-08-handler-coverage, the `coverage` role (`coverage <app> <check>...`), `coverage nowhere <dir>` a usage error, status `2`», the rest as it stands
sidecar: e2e-test-strategy: cli_commands' usage-line reading gains the coverage role clause and its usage row
rationale: the report's Symbols state the usage line gains «; coverage role: coverage <app> <check>...;» and Counts add the usage row `coverage nowhere <dir>` plus one usage fragment in cli_commands
@proposal n=10 detector=D-new-detail severity=warning file=e2e-test-strategy anchor=fct-csryo2zo fact=F5 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-nu73ltec
section: ## 6. E2E Test Strategy
change: add after it: - **Command line, handler coverage:** `cli_commands`' `coverage_replays_a_kept_check_and_lists_the_handlers_it_left` runs the binary's `coverage` role over a Counter check the binary kept and reads the coverage line, exit 0; the role's command-line shape, `coverage <app> <check>...`, is a standing check of the command-line surface (examples/seven_guis/tests/cli_commands.rs:522-571) — as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md
sidecar: e2e-test-strategy: cli_commands' coverage-role test added beside the command-line entity (F5)
rationale: the report's Outcome matrix#v030-07 names the test through the binary, and deviation 3 states the Counter run stands in cli_commands
@proposal n=11 detector=D-cascade severity=warning file=test-data-fixtures anchor=fct-a24rqe7v basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-tsl356di
section: ## 7. Test Data & Fixtures
change: in «read by `stand_graph`», read «read by `stand_graph` and, since 2026-11-08-handler-coverage, `stand_handler_coverage`, whose drives check covers its 7 tasks», the rest of the row as it stands
sidecar: test-data-fixtures: GRAPH_DRIVES' readers gain stand_handler_coverage
rationale: the report's deviation 2 states UNEXERCISED_AFTER_DRIVES holds all 7 GRAPH_DRIVES tasks, and Outcome names a_check_of_each_tasks_drives_lists_its_unexercised_row over the 7 GRAPH_DRIVES tasks
@proposal n=12 detector=D-new-detail severity=warning file=test-data-fixtures anchor=fct-5d6i6kkv fact=F9 basis_doc=doc:2026-11-08-handler-coverage/report basis_fact=fct-tsl356di
section: ## 7. Test Data & Fixtures
change: add after it: | Handlers at boot and unexercised after drives (stand tasks) | `HANDLERS_AT_BOOT: [(&str, &[(&str, &str)]); 6]` and `UNEXERCISED_AFTER_DRIVES: [(&str, &[(&str, &str)]); 7]` in the stand checks' shared module, partly generated: Cells' boot screen reads 678 handlers, 676 grid-cell clicks generated row by row by `grid_cells()`, with `CELLS_CLICKED_BY_DRIVES` naming the two the drives click; the named handlers stated by hand and held equal to the graph's own reading on a fresh boot, as measured at escher-0.3.0/chunks/2026-11-08-handler-coverage/report.md; read by `stand_handler_coverage` | (tests/blitz-tests/tests/common/mod.rs:336-380; tests/blitz-tests/tests/common/mod.rs:382-401; tests/blitz-tests/tests/common/mod.rs:406-412) |
sidecar: test-data-fixtures: the handler-coverage tables row added beside ACTIONABLE_AT_BOOT (F9)
rationale: the report's deviation 2 and New text by line (common/mod.rs 336-380, 382-401, 406-412) state the two tables, grid_cells() and the 678 handlers
@remark
Outside group 5: §3 fct-glytb3w3 and fct-6bs5asmb (run stand 244·0·7 over 52 files → 262·0·7 over 54), fct-og7dxjok and fct-7m7lxsmy (mod common; 23 → 24), fct-prxcxiqp (session_common readers) and fct-i47d76zr (bridge_tests gains the_reached_record_holds_on_a_page_with_no_registered_state) are harness/helpers facts; F7, F8 are unit-test-strategy; coverage_needles() as a §7 needle row (beside fct-3ywgnxn6) and the §5 body line fct-gq2zcc44 naming stand_handler_coverage are left to the new-detail pass; the in-prose `dioxus_document.rs:319`/`:318` in fct-kgita7a3 sit below the added line 10, a citation-sweep matter.
