@task_prose part=session
Chunk 2026-11-08-handler-coverage adds handler coverage: escher-session gains a third command-line role, coverage <app> <check>..., over the new escher_driver::handler_coverage, listing the UI graph handlers no kept check exercised as one JSON line.
dioxus-native-dom gains an ungated reached-handler record (ReachedHandler, record_reached, reached_handlers) written on DioxusEventHandler::handle_event's path, read through a new crate-private element_listener_ids.
A new stand target stand_handler_coverage (8 tests, both layout modes) lands, and stand_replay gains the owed test that a kept actions step replays, discharging the CARRY on Handler coverage.
Measured: run stand 262 passed, 0 failed, 7 ignored over 54 files (was 244 over 52); CI green 17/17 at the pre-CI commit; no dependency, verb, workflow, leg list or agent-run script moved; nothing retired.
@task_file n=1 path=packages/dioxus-native-dom/src/reached.rs
@task_file n=2 path=packages/escher-driver/src/coverage.rs
@task_file n=3 path=tests/blitz-tests/tests/stand_handler_coverage.rs
@task_file n=4 path=packages/dioxus-native-dom/src/dioxus_document.rs
@task_file n=5 path=packages/dioxus-native-dom/src/dioxus_document_tests.rs
@task_file n=6 path=packages/dioxus-native-dom/src/element_id.rs
@task_file n=7 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=8 path=packages/escher-driver/src/replay.rs
@task_file n=9 path=packages/escher-driver/src/lib.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/cli.rs
@task_file n=12 path=packages/escher-driver/src/cli_tests.rs
@task_file n=13 path=examples/seven_guis/src/session_host.rs
@task_file n=14 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=15 path=examples/seven_guis/tests/common/mod.rs
@task_file n=16 path=examples/seven_guis/tests/host_log.rs
@task_file n=17 path=tests/blitz-tests/tests/stand_replay.rs
@task_file n=18 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=19 path=docs/capabilities.md
@task_item n=1 part=touches
The newest run stand reading (fct-glytb3w3) moves from passed 244, failed 0, ignored 7 over 52 files to 262, 0, 7 over 54 files, stand_handler_coverage (8) a new file its stand_ prefix picks up with no script change; the same 244 over 52 figure in the test-data bootstrap line (fct-6bs5asmb) moves too, and the stand_* file count goes 53 to 54.
@task_item n=2 part=touches
The blitz-tests mod common; declarer count moves 23 to 24 (fct-7m7lxsmy, and the list of shared-module readers in the stand-checks helper text), stand_handler_coverage the new declarer; seven_guis' 8 declarers are unchanged.
@task_item n=3 part=touches
The session_common reader count moves 34 to 35, stand_handler_coverage the new reader; any test-plan text listing or counting the readers of the session module is touched.
@task_item n=4 part=touches
stand_replay's test count moves 3 to 4 (fct-z2ynupl3): a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes.
@task_item n=5 part=touches
seven_guis cli_commands' linux test count moves 17 to 18 (file #[test] 18 to 19, one test per OS) at fct-csryo2zo, fct-klxgwgcy and fct-k5y3m6cu; its usage-error table gains a tenth row, coverage nowhere <dir>, and one usage fragment.
@task_item n=6 part=touches
escher-driver's command-line line table a_line_is_read_by_its_rules_in_order moves 102 to 109 rows and cli_tests.rs 5 to 6 tests (fct-nks3g3e7); json.rs moves 7 to 8 tests.
@task_item n=7 part=touches
The CARRY on Handler coverage (fct-kzdrls5d) naming this entry as owner of the owed actions replay test is discharged: the test now passes in both layout modes (CARRY cry-sxrivdk3).
@task_item n=8 part=touches
seven_guis' common needle readers gain coverage_needles() (the CRUD screen's ids and names from the graph), called by host_log alone (fct-k5y3m6cu and any line listing actions_needles and its siblings); host_log's tests move 5 to 6.
@task_item n=9 part=touches
The escher-driver unit-test coverage list (what its unit tests pin) gains the coverage line's JSON form and the coverage role's command-line reading.
@task_item n=10 part=touches
The dioxus-native-dom scope and unit-test descriptions (bridge and its dispatch record, the actionable and graph readers) gain the reached-handler record: reached::tests 4 new tests, bridge_tests 8 to 9.
@task_item n=11 part=touches
The bridge-test description (fct-i47d76zr, bridge_tests in its own child file calling handle_event) gains a ninth test on the reached record; the engine trait and the cfg(test) dispatch shim are unchanged.
@task_item n=12 part=touches
The command-line driver surface (E2E, cli_commands, the two shell flows): escher-session now has a third role, coverage, beside the host serve role and the mcp role; its shape is coverage <app> <check>..., refusals malformed status 1, usage status 2, boot failure status 3.
@task_item n=13 part=touches
Recorded-check replay: Check::replay now runs through a crate-private replay_each with a no-op per-step hook, behaviour unchanged; handler_coverage is a second caller reading the graph after each replayed step, and a kept check is now also read from the command line through Check::from_bytes under a bounded read.
@task_item n=14 part=touches
Session-host-writes and log-format lines: host_log now runs a coverage run over a kept CRUD check at trace and reads 0 id, name, coverage, sentinel, path and recorded needles; the coverage role opens no new span, each replayed step leaving its own command span; escher-driver info_span! sites stay at 2.
@task_item n=15 part=touches
The Session-held-instance integration line (fct-gq2zcc44, which names stand_graph and stand_actions over the served screens) is where handler coverage over the six served tasks in both layout modes joins.
@task_item n=16 part=touches
The stand-repeat leg and the flakiness budget pick up stand_handler_coverage by its stand_ prefix; the seventeen named non-prefix targets and the leg lists are unchanged.
@task_item n=17 part=touches
The blitz-tests shared common module gains the stated tables HANDLERS_AT_BOOT (6 tasks), UNEXERCISED_AFTER_DRIVES (7 GRAPH_DRIVES tasks), CELLS_CLICKED_BY_DRIVES and the generator grid_cells(); the module's list of tables and helpers is touched.
@task_item n=18 part=touches
escher-driver crate-root re-exports move 59 to 62 (Coverage, HandlerRef, handler_coverage) and dioxus-native-dom 35 to 36 (ReachedHandler), new modules escher_driver::coverage and dioxus_native_dom::reached; any test-plan line counting or listing these re-exports or modules is touched.
@task_fact n=1 slug=integration-test-strategy
fact: stand_handler_coverage (8 tests, each in both layout modes) holds handler_coverage on the six served tasks: a snapshot check lists exactly HANDLERS_AT_BOOT, equal to the graph at boot; each task's drives leave its UNEXERCISED_AFTER_DRIVES row; two checks list only what neither exercised; the load page reads its click exercised; a refused step exercises nothing and a keyboard activation its click; a handler whose step removes its element reads exercised; a typed sentinel never reaches the coverage line; a check of another app contributes nothing and does not hold.
@task_fact n=2 slug=integration-test-strategy
fact: PROVISIONAL — handler coverage reads every screen the replays reach (boot and after each replayed step), the overseer's delegate call on the screens (inputs I4, Q2), discharged at the epoch boundary (rul-3ndrhilc).
@task_fact n=3 slug=integration-test-strategy
fact: Handler coverage stated limits: handlers inside a canvas are not counted (deferred to 0.4.0, the incubator's canvas plans); a handler on no screen a replay reached is not listed (by construction, no owner); a bubbling dispatch counts every ancestor listener it passed with stopPropagation unread (a new residual row, target next).
@task_fact n=4 slug=integration-test-strategy
fact: stand_replay's a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes replays a recorded Counter session with two actions steps on a fresh Counter, all 4 steps running, and the copy with one keyed flipped stops at step 3, keys exactly nodes, in both layout modes.
@task_fact n=5 slug=e2e-test-strategy
fact: cli_commands' coverage_replays_a_kept_check_and_lists_the_handlers_it_left runs the binary's coverage role over a Counter check the binary kept and reads the coverage line, exit 0; the coverage role's command-line shape is a standing check of the command-line surface.
@task_fact n=6 slug=integration-test-strategy
fact: host_log's a_coverage_run_logs_no_id_no_name_and_nothing_of_its_check_at_trace runs coverage crud over a CRUD session kept through the binary with a typed sentinel and a row click, and reads 0 id, name, coverage, sentinel, path and recorded needles on stderr at trace under both builds.
@task_fact n=7 slug=unit-test-strategy
fact: dioxus-native-dom's reached::tests (4) pin the reached record: a click recorded with the record on and nothing with it off, a bubbling dispatch recording its ancestor's listener of the same event, a handler whose run removes its element recorded by its id, and the record's Debug printing its count alone; bridge_tests' the_reached_record_holds_on_a_page_with_no_registered_state pins it where no state is registered.
@task_fact n=8 slug=unit-test-strategy
fact: escher-driver's json.rs a_coverage_line_keys_each_handler_and_leaves_out_an_absent_owner pins the coverage line's form, and cli_tests.rs the_coverage_role_reads_its_checks_refusing_any_it_cannot_read pins a missing file, a non-check, one past the bound and another app's check refused malformed, a failing boot, and an accepted two-check run, its scratch files under the workspace target/tmp.
@task_fact n=9 slug=test-data-fixtures
fact: Cells' rows in HANDLERS_AT_BOOT and UNEXERCISED_AFTER_DRIVES are partly generated: its boot screen reads 678 handlers, 676 grid-cell clicks generated row by row by grid_cells(), with CELLS_CLICKED_BY_DRIVES naming the two the drives click; the named handlers are stated by hand and held equal to the graph's own reading on a fresh boot.
@task_item n=1 part=untouched
The agent-run 5-command contract, its exit grammar, events, state and test_agent_run.py are unchanged; only the run stand count moved.
@task_item n=2 part=untouched
The cold-agent pipe, its verdict fields, judge, fourteen-verb MCP surface and test_cold_agent.py are unchanged; no MCP, socket or cold-agent surface moved.
@task_item n=3 part=untouched
CI workflows, ci-leg.sh legs, caches, package installs, fast/slow split and the local pre-push gate are unchanged; no new job or leg.
@task_item n=4 part=untouched
Coverage floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90) held and were not re-read per crate; the floor script and mutation tier are unchanged.
@task_item n=5 part=untouched
The shared fixtures module (fixtures::boot, hold, HeldNet, the 32 declarers) and the stand-ins (LoadNet, LateNet, TimerTicks) are unchanged.
@task_item n=6 part=untouched
The committed Counter checks under tests/blitz-tests/checks and record_checks.sh are byte-identical.
@task_item n=7 part=untouched
blitz-dom, blitz-paint, blitz-test-harness, blitz-net, blitz-vibey-script, apps/browser, WPT runner and the CI Python script tests are not reached.
@task_item n=8 part=untouched
Dependencies and third-party test frameworks are unchanged: no Cargo manifest or lock moved, no new dev-dependency.
@task_item n=9 part=untouched
run all's newest reading (743 over the package) was not re-read and stays as written.
