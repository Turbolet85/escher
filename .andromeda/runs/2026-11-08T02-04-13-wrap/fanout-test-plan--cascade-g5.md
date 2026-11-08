@proposal n=1 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-csryo2zo basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-yr4375o3
section: ## 6. E2E Test Strategy
change: «each of the eight instance-level verbs is accepted and answers exactly its fields» becomes «each of the nine instance-level verbs (eight until 2026-11-07-ui-graph added `graph`) is accepted and answers exactly its fields».
sidecar: e2e-test-strategy — the command line's instance-level verbs 8 → 9 (`graph`), 2026-11-07-ui-graph.
rationale: The report's Schema / config moves the instance verbs from 8 to 9 (fct-yr4375o3), and escher-session answers `graph` on the command line (pattern inst-verbs, stale).
@proposal n=2 detector=D-new-detail severity=warning file=e2e-test-strategy anchor=fct-csryo2zo fact=F10 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-e44heebj
section: ## 6. E2E Test Strategy
change: After the hosted `lint` clause add «since 2026-11-07-ui-graph `graph_answers_one_line_of_the_ui_graph` runs the graph verb as a process and reads one line of the UI graph (examples/seven_guis/tests/cli_commands.rs:467-492)».
sidecar: e2e-test-strategy — cli_commands' graph verb run as a process (F10), 2026-11-07-ui-graph.
rationale: The Outcome names cli_commands::graph_answers_one_line_of_the_ui_graph as the command line's witness of `graph` (fct-e44heebj), and New text lists it at cli_commands.rs:467-492.
@proposal n=3 detector=D-cascade severity=warning file=e2e-test-strategy anchor=fct-gtzxj2kr basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-brwfx6hx
section: ## 6. E2E Test Strategy
change: «`tools/list` offering the verbs in order — twelve —» becomes «`tools/list` offering the verbs in order — thirteen since 2026-11-07-ui-graph, `graph` after `lint` —».
sidecar: e2e-test-strategy — the MCP listing 12 → 13 tools (`graph` after `lint`), 2026-11-07-ui-graph.
rationale: The Outcome states that the MCP listing holds 13 tools, `graph` after `lint` (fct-brwfx6hx), and `VERBS` moves from 12 to 13 (fct-yr4375o3, pattern verbs, stale).
@proposal n=4 detector=D-new-detail severity=warning file=e2e-test-strategy anchor=fct-gtzxj2kr fact=F10 dependent_of=D-new-detail basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-e44heebj
section: ## 6. E2E Test Strategy
change: After the `lint` tool call clause add «since 2026-11-07-ui-graph a `graph` tool call over the mcp child answering its line of the UI graph (`a_graph_tool_call_answers_its_line_of_the_ui_graph`)».
sidecar: e2e-test-strategy — mcp_server's graph tool call (F10), 2026-11-07-ui-graph.
rationale: The Outcome names mcp_server::a_graph_tool_call_answers_its_line_of_the_ui_graph as the MCP child's witness (fct-e44heebj). This is F10's MCP half, beside the MCP entity.
@proposal n=5 detector=D-cascade severity=warning file=test-data-fixtures anchor=fct-6hjdlvz3 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-qyqo7z6u
section: ## 7. Test Data & Fixtures
change: «`counter-cli.json` (8 lines, 2,929 B) and `counter-mcp.json` (11 lines, 3,720 B)» becomes «`counter-cli.json` (8 lines) and `counter-mcp.json` (11 lines), re-recorded through `record_checks.sh` at 2026-11-07-ui-graph (+4/−4 and +6/−6 lines, their nodes carrying `clickable`; 2,929 B and 3,720 B before it), still replayed green by `stand_replay`».
sidecar: test-data-fixtures — the committed checks re-recorded at 2026-11-07-ui-graph, so their pre-chunk byte sizes are now dated.
rationale: Expected amendments states that the committed checks' sizes moved (+4/−4 and +6/−6 lines) and that stand_replay is green (fct-qyqo7z6u). The Outcome states that they were re-recorded and replay (fct-brwfx6hx).
@proposal n=6 detector=D-cascade severity=warning file=test-data-fixtures anchor=fct-iee65fcu basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ubggv3nh
section: ## 7. Test Data & Fixtures
change: «47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary, as measured at escher-0.3.0/chunks/2026-11-01-upstream-sync-ui-graph/evidence/counts.md» becomes «53,859 bytes in 740 lines and 6 parts of `SNAPSHOT_TEXT_BUDGET` (47,089 B in 5 parts until 2026-11-07-ui-graph added the ` clickable` token), read through the binary, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md».
sidecar: test-data-fixtures — the stand Cells screen 47,089 → 53,859 B, 5 → 6 parts, 2026-11-07-ui-graph.
rationale: Counts moved, pattern cells-parts: 47,089 → 53,859 bytes, 740 lines, 5 → 6 parts, with test-data-fixtures.md:28 listed stale (fct-ubggv3nh). stand_cells_served PARTS is now 6.
@proposal n=7 detector=D-new-detail severity=warning file=test-data-fixtures anchor=fct-iee65fcu fact=F9 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-reiinlg2
section: ## 7. Test Data & Fixtures
change: Add the row «| Stand graph answer (Cells) | `graph` on a fresh `start cells`: 293,869 bytes (547,685 at first build, 522,692 after the readers narrowing); `graph` takes no argument and answers the whole screen at once, the stated limit «An unpaged graph» owned by residual `res-7cc5bi4k`, target next; held within the answer bound by `stand_graph`'s `the_whole_cells_answer_is_within_the_answer_bound`, as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md | (tests/blitz-tests/tests/stand_graph.rs:381-394) |».
sidecar: test-data-fixtures — the Cells graph answer's measured size and its stated limit (F9), 2026-11-07-ui-graph.
rationale: Insufficient fixes records 547,685 → 522,692 → 293,869 B and the residual res-7cc5bi4k (fct-reiinlg2). Counts moved has the figure with no master site, for P2 to place (fct-ubggv3nh).
@proposal n=8 detector=D-new-detail severity=warning file=test-data-fixtures anchor=fct-qbzx34mx fact=F7 basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-ej52fywd
section: ## 7. Test Data & Fixtures
change: Add the row «| Graph drives (stand tasks) | `GRAPH_DRIVES` in the stand shared table module, seven rows of `Drive` steps, drives each stand task's conditional writes once: Cells clicks A0, C0, A0 and leaves its keyed editor once by Enter and once by Tab, so no stand write reads `unconfirmed` or `undeclared`; read by `stand_graph` | (tests/blitz-tests/tests/common/mod.rs:243-323) |».
sidecar: test-data-fixtures — the stand's GRAPH_DRIVES seed table (F7), 2026-11-07-ui-graph.
rationale: The Decision records GRAPH_DRIVES as the drive table that drives each conditional write once (fct-ej52fywd), and New text lists it at common/mod.rs:243-323 (fct-haatublt).
@proposal n=9 detector=D-cascade severity=warning file=ci-integration anchor=fct-mrjxzany basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-rsklenkh
section: ## 9. CI Integration
change: «— 51 files, `stand_session_recording` joining by its prefix» becomes «— 52 files, `stand_session_recording` joining by its prefix», and after «`stand_act_popup`» add «and, since 2026-11-07-ui-graph, `stand_graph`».
sidecar: ci-integration — the stand-repeat glob 51 → 52 files (`stand_graph`), 2026-11-07-ui-graph.
rationale: Harness / gate surface states that the stand-repeat leg's glob takes the new `stand_graph` (fct-rsklenkh). Counts moved lists ci-integration.md:8 stale, 51 → 52 (fct-ubggv3nh).
@proposal n=10 detector=D-cascade severity=warning file=quality-gates-coverage-targets anchor=fct-3ctmcebn basis_doc=doc:2026-11-07-ui-graph/report basis_fact=fct-rsklenkh
section: ## 10. Quality Gates & Coverage Targets
change: «run the 51 `stand_*.rs` files» becomes «run the 52 `stand_*.rs` files (51 until 2026-11-07-ui-graph added `stand_graph`)».
sidecar: quality-gates-coverage-targets — stand-repeat's stand files 51 → 52, 2026-11-07-ui-graph.
rationale: The new stand target stand_graph is a `stand_*` file that the stand-repeat glob takes (fct-rsklenkh). The stand file count moves 51 → 52 (fct-ubggv3nh, pattern stand-files).
@remark
F11, F12 (integration-test-strategy) and F13 (test-scope-summary) belong to other categories and are not placed here. The new seven_guis graph_needles helper (common/mod.rs:425-452) has no task_fact. The test counts cli_commands (15), mcp_server (6) and stand_mcp_tools (6) are not restated by the report, so they are left for re-count. mcp_tests' `the_listing_is_the_twelve_verbs_of_the_table`, named in quality-gates#6.1's dated reading, may have been renamed, which the report does not show.
