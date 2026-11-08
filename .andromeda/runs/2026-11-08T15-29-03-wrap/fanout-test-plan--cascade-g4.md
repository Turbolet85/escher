@proposal n=1 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-csryo2zo basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 6. E2E Test Strategy
change: `cli_commands` (16) → `cli_commands` (17); «each of the nine instance-level verbs is accepted and answers exactly its fields» → «each of the ten instance-level verbs is accepted and answers exactly its fields»
sidecar: e2e-test-strategy: cli_commands per-OS count 16 → 17 and the instance-level verbs nine → ten, with actions
rationale: the report's Counts moved: cli_commands tests per OS 16 → 17 [cli-tests] and instance verbs 9 → 10 [instance], actions the tenth instance verb after graph
@proposal n=2 detector=D-new-detail severity=warning file=e2e-test-strategy anchor=fct-csryo2zo fact=F10 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-a7grs7bs
section: ## 6. E2E Test Strategy
change: after the `graph_answers_one_line_of_the_ui_graph` clause add: «since 2026-11-08-what-can-i-do-here `actions_answers_one_line_of_the_actionable_nodes` runs `actions` as a process against a started host and reads one JSON line, and an argument to `actions` is refused with status `1` (examples/seven_guis/tests/cli_commands.rs:495-516)»
sidecar: e2e-test-strategy: cli_commands' actions test, one JSON line, an argument refused status 1 (F10)
rationale: the report's Outcome (security) states the argument refused on the command line over the socket, status 1, in cli_commands; New text lists the test at cli_commands.rs:495-516
@proposal n=3 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-gtzxj2kr basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tj4uicby
section: ## 6. E2E Test Strategy
change: `mcp_server` (7) → `mcp_server` (8); «`tools/list` offering the verbs in order — thirteen, `graph` after `lint`» → «`tools/list` offering the verbs in order — fourteen, `graph` after `lint` and `actions` after `graph`»; `stand_mcp_tools` (6, its sixth a `lint` tool call …) → `stand_mcp_tools` (8, its sixth a `lint` tool call …)
sidecar: e2e-test-strategy: mcp_server 7 → 8, tools/list fourteen with actions after graph, stand_mcp_tools → 8
rationale: the report's Counts moved: mcp_server tests 7 → 8 [mcps-tests], stand_mcp_tools 7 → 8 [smt-tests, no master site found by the sweep], VERBS 13 → 14 with actions after graph; Outcome (tests) counts reads tools/list fourteen in table order
@proposal n=4 detector=D-new-detail severity=warning file=e2e-test-strategy anchor=fct-gtzxj2kr fact=F11 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-7rhac4bh
section: ## 6. E2E Test Strategy
change: after the `graph` tool call clause add: «since 2026-11-08-what-can-i-do-here an `actions` tool call answering its line of the actionable nodes, an argument reading `isError` (examples/seven_guis/tests/mcp_server.rs:363-388), and in process `stand_mcp_tools`' `an_actions_tool_call_answers_the_readers_nodes_and_parents_a_command_recording_its_verb_alone` reads `tool_call` parenting one `command` that records `verb` alone, its body the helper `a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone` the `graph` test shares (tests/blitz-tests/tests/stand_mcp_tools.rs:783-792)»
sidecar: e2e-test-strategy: the actions tool call over MCP and in stand_mcp_tools (F11)
rationale: the report's Outcome (security) and (obs) state the MCP isError refusal in mcp_server and stand_mcp_tools and the tool_call parenting one command; Deviations fct-njnsyi3r names the shared helper; New text lists both line ranges
@proposal n=5 detector=D-cascade severity=warning file=test-data-fixtures anchor=fct-qbzx34mx basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-cfi7tfzq
section: ## 7. Test Data & Fixtures
change: after «and `stand_contrast`'s `the_load_page_meets_its_ratios`» add: «, and since 2026-11-08-what-can-i-do-here by `stand_actions`, which puts its fetch in flight with `harness.click("#load-fetch")`, waiting on nothing, to read `load-fetch`'s stated cause»
sidecar: test-data-fixtures: stand_actions joins the load page's readers
rationale: the report's Decisions state stand_actions puts a fetch in flight through harness.click on load-fetch; its Outcome (matrix#v030-06) reads load-fetch with its fetch in flight
@proposal n=6 detector=D-new-detail severity=warning file=test-data-fixtures anchor=fct-a24rqe7v fact=F3 basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-d57gqkg5
section: ## 7. Test Data & Fixtures
change: add a row after it: «| Actionable counts at boot (stand tasks) | `ACTIONABLE_AT_BOOT: [(&str, usize); 6]` in the stand checks' shared module, the measured actionable node counts at boot: counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2 (against the plan's «about 677» Cells forecast); read by `stand_actions` | (tests/blitz-tests/tests/common/mod.rs:93-102) |»
sidecar: test-data-fixtures: new row ACTIONABLE_AT_BOOT, 2 · 7 · 3 · 11 · 678 · 2 (F3)
rationale: the report's Symbols list ACTIONABLE_AT_BOOT at common/mod.rs:93-102 and Decisions record it measured, not assumed, 2 · 7 · 3 · 11 · 678 · 2
@proposal n=7 detector=D-cascade severity=warning file=ci-integration anchor=fct-mrjxzany basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-tayahs5z
section: ## 9. CI Integration
change: «with one `--test` per `tests/blitz-tests/tests/stand_*.rs` — 52 files» → «— 53 files»; after «`stand_act_popup` and `stand_graph`» add «and, since 2026-11-08-what-can-i-do-here, `stand_actions`»
sidecar: ci-integration: stand-repeat's stand_*.rs files 52 → 53, stand_actions joining by its prefix
rationale: the report's Harness surface states stand_actions enters stand-repeat through the existing stand_*.rs glob, the seventeen named targets unchanged; Counts moved: stand_*.rs files 52 → 53 [stand-tally]
@proposal n=8 detector=D-cascade severity=warning file=ci-integration anchor=fct-kjlei6et basis_doc=doc:2026-11-08-what-can-i-do-here/report basis_fact=fct-3wtajwii
section: ## 9. CI Integration
change: after the re-count with upstream's twelve commits add: «Re-counted at 2026-11-08-what-can-i-do-here: 223 result lines, 0 failed (`test result: ok` 223, `FAILED` 0), the passed and ignored totals not read; the moves since 219 not attributed line by line here, this chunk's own one new result line `stand_actions` (8) — as measured at that chunk's `ci-leg.sh fast` on the committed tree (escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md)»
sidecar: ci-integration: local baseline re-counted, 223 result lines, 0 failed
rationale: the report's Outcome basis states ci-leg.sh fast exited 0 on the committed tree with 223 test result ok and 0 FAILED; the record's last re-count reads 219
@remark
Not mine: F2 (Cells actions answer 106,088 bytes, integration-test-strategy) could sit as a data-seed row beside test-data-fixtures#3.8; the untested replay of a kept actions step (F6, D-tests-coverage) is integration-test-strategy's CARRY; actions_needles has no §7 row beside option_needles (no task_fact); #16.1's per-OS tallies were not read from CI#42493052318.
