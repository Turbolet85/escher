@task_prose part=session
The session added the driver verb actions: one read of what each actionable node can take now, its verbs drawn from the schema's table (click, type, scroll, screenshot) and the refusal each would meet.
It added DioxusDocument::actionable_nodes and ActionableNode in dioxus-native-dom, one crate-private actionable rule (Readings) shared with unkeyed_actionable, and Session::actions, ActionNode, VerbReading, Command::Actions, Outcome::Actions and FieldKind::ActionNodes in escher-driver.
It moved the four id-taking verbs onto one shared per-verb target check, check_target, which the four executor arms and Session::actions call; a screenshot by id now reads fonts-unavailable through can_paint_text before it paints.
It retired unkeyed_actionable's reading of the data-dioxus-id mark as its listener leg; that leg now reads the exact current listeners. The mark itself stays.
It added the stand_actions target (8 tests, both modes) and actions coverage in the CLI, MCP, log and recording checks. It measured run stand at 253 passed, 0 failed, 7 ignored over 53 files, the Cells actions answer at 106,088 bytes for 678 nodes, and CI green 17/17.
A kept actions step's replay is exercised by no test; that test is owed as a CARRY on Handler coverage.
@task_file n=1 path=packages/escher-driver/src/actions.rs
@task_file n=2 path=tests/blitz-tests/tests/stand_actions.rs
@task_file n=3 path=packages/dioxus-native-dom/src/actionable.rs
@task_file n=4 path=packages/dioxus-native-dom/src/lib.rs
@task_file n=5 path=packages/escher-driver/src/execute.rs
@task_file n=6 path=packages/escher-driver/src/schema.rs
@task_file n=7 path=packages/escher-driver/src/schema/spec.rs
@task_file n=8 path=packages/escher-driver/src/schema_tests.rs
@task_file n=9 path=packages/escher-driver/src/command.rs
@task_file n=10 path=packages/escher-driver/src/json.rs
@task_file n=11 path=packages/escher-driver/src/describe.rs
@task_file n=12 path=packages/escher-driver/src/lib.rs
@task_file n=13 path=packages/escher-driver/src/mcp.rs
@task_file n=14 path=packages/escher-driver/src/mcp_tests.rs
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
@task_file n=17 path=tests/blitz-tests/tests/session_common/mod.rs
@task_file n=18 path=tests/blitz-tests/tests/common/mod.rs
@task_file n=19 path=tests/blitz-tests/tests/stand_act_spans.rs
@task_file n=20 path=tests/blitz-tests/tests/stand_act_scroll.rs
@task_file n=21 path=tests/blitz-tests/tests/stand_mcp_tools.rs
@task_file n=22 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
@task_file n=23 path=examples/seven_guis/tests/cli_commands.rs
@task_file n=24 path=examples/seven_guis/tests/mcp_server.rs
@task_file n=25 path=examples/seven_guis/tests/host_log.rs
@task_file n=26 path=examples/seven_guis/tests/mcp_log.rs
@task_file n=27 path=examples/seven_guis/tests/common/mod.rs
@task_file n=28 path=scripts/cold-agent.sh
@task_file n=29 path=.github/scripts/test_cold_agent.py
@task_file n=30 path=docs/capabilities.md
@task_file n=31 path=README.md
@task_item n=1 part=touches
The verb count moved: VERBS 13 to 14. Every statement of thirteen verbs, tools or driver commands now reads fourteen, including the cold-agent server's thirteen verbs, valid_step's admitted tools, and isolated's exactly-thirteen mcp__escher tools.
@task_item n=2 part=touches
Instance verbs 9 to 10: actions is the tenth instance verb, after graph. Any nine-instance-verbs statement or instance-row pin moves.
@task_item n=3 part=touches
FieldKind 12 to 13, adding ActionNodes (action-nodes). Any field-kind count moves.
@task_item n=4 part=touches
describe's count row moved from 13,14,12,3,8,4,15 to 14,14,12,3,8,4,15.
@task_item n=5 part=touches
The run stand tally moved from 244 passed, 0 failed, 7 ignored over 52 files to 253, 0, 7 over 53. The stand_*.rs files went 52 to 53, stand_actions picked up by its stand_ prefix with no script change. This reaches the agent-run Proof's newest reading and the test-data bootstrap's run stand line.
@task_item n=6 part=touches
The session_common module gains the actions() builder and the actioned() reader, and act's match lists Outcome::Actions. Its declarers went 33 to 34, stand_actions the thirty-fourth. This reaches the session-checks helper enumeration and its reader count.
@task_item n=7 part=touches
The fixtures module's declarers went 31 to 32, and its stand_* readers 23 to 24 (stand_actions reads password_labelled). This reaches the fixtures reader count and the test-data bootstrap's 23 stand_* readers.
@task_item n=8 part=touches
The blitz-tests common module gains the table ACTIONABLE_AT_BOOT. This reaches the stand checks' shared-tables enumeration.
@task_item n=9 part=touches
The log checks' needle helpers gain actions_needles beside graph_needles and option_needles. host_log and mcp_log read 0 id, name and actions needles at trace under both builds. actions joins the closed verb and tool sets of the command and tool_call span lines, with no span field added. This reaches the stand log format's per-verb enumeration.
@task_item n=10 part=touches
host_log's hosted calls went 8 to 9, and mcp_log's tool calls 11 to 12, one actions call each.
@task_item n=11 part=touches
escher-driver unit tests went 100 to 102: actions.rs holds 2 and is a seventeenth file in the per-file list. The dioxus-native-dom actionable unit tests went 4 to 8, and the crate 100 to 104. This reaches unit-count statements and the escher-driver and dioxus-native-dom unit-coverage enumerations.
@task_item n=12 part=touches
The cli_commands tests per OS went 16 to 17 (#[test] 17 to 18, two of them linux and non-linux alternates). The mcp_server tests went 7 to 8. The stand_mcp_tools tests went 7 to 8.
@task_item n=13 part=touches
stand_act_spans' driven calls went 37 to 38 (run.calls), and its instance-row pin 9 to 10. It reads the actions command line recording verb alone.
@task_item n=14 part=touches
The actionable-key check (unkeyed_actionable) now reads the element's exact current listeners holding an event other than mounted, not the data-dioxus-id mark. Its domain, order, fields and remedy are unchanged. stand_actionable_keys keeps it empty on every task under the new leg.
@task_item n=15 part=touches
The executor's refusals for click, type, scroll and screenshot by id are now decided by one shared check, check_target. click and type meet action_point; scroll meets none; a screenshot by id meets in_view, then an empty crop, then can_paint_text. Any statement that action_point alone decides click and type, or that each arm checks on its own, moves. The nine stand_act_* targets stay green.
@task_item n=16 part=touches
The residual res-7cc5bi4k (the unpaged graph) was widened to own the unpaged actions answer too. This reaches any test-plan site of that residual or its stated limit.
@task_item n=17 part=touches
Two test names were renamed from thirteen to fourteen: schema_tests' the_fourteen_verbs_are_named_in_order_each_at_its_level and mcp_tests' the_listing_is_the_fourteen_verbs_of_the_table. stand_requirement_sweep's WITNESSES_020 row v020-01 cites the first. Any citation of the old names moves.
@task_item n=18 part=touches
The cold-agent pipe's VERBS joins actions after graph. The cold-agent contract test's fake now answers fourteen tools, where it said twelve.
@task_item n=19 part=touches
The capability statement in docs/capabilities.md and README now states fourteen driver commands and tools, with the capability-statement pin green. docs/capabilities.md adds two stated limits: accepted is not acting, and an unpaged actions answer.
@task_item n=20 part=touches
Session to held instance boundary: the actions verb is driven through a held session on all six served screens in both layout modes, and the read changes nothing (no has_changes, same snapshot, focus and diff, an open list stays open).
@task_item n=21 part=touches
Command line and MCP drivers per surface: cli_commands, mcp_server, mcp_log and stand_mcp_tools now drive actions, and an argument is refused malformed on each surface.
@task_item n=22 part=touches
stand_mcp_tools' graph tool-call test body became the shared helper a_read_tool_call_answers_its_reader_and_parents_a_command_recording_its_verb_alone. The graph test keeps its name, and the new actions test runs through the same helper.
@task_item n=23 part=touches
Recording: recording_tests reads a kept actions step's answer back whole. The two committed Counter checks stay byte-identical and replay green.
@task_item n=24 part=touches
escher-driver's private modules went 17 to 18 (actions), and its re-exports 57 to 59 (ActionNode, VerbReading). Command and Outcome each gain one variant.
@task_fact n=1 slug=integration-test-strategy
fact: stand_actions holds 8 tests over the six served screens, in both layout modes. It checks the listed actionable nodes, their verbs and counts; every listed cause on the four lean tasks equal to what Session::run returns, with 3 disabled controls; the stated Cells and load-page causes; the flight booker's options reading covered with owner flight-type until the list opens; crud-person-14 reading off-screen until a scroll; the fixtures reading covered, unkeyed and fonts-unavailable; that the read changes nothing and holds no typed text or mask; and that the whole Cells answer stays within the answer bound.
@task_fact n=2 slug=integration-test-strategy
fact: The Cells actions answer measures 106,088 bytes for 678 nodes through the binary. 461 cells read off-screen for click, type and screenshot, and all 678 take scroll. stand_actions asserts the answer exceeds 100,000 bytes and stays within 1,048,576, in both modes.
@task_fact n=3 slug=test-data-fixtures
fact: The blitz-tests common module's ACTIONABLE_AT_BOOT table states the measured actionable node counts at boot: counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2.
@task_fact n=4 slug=integration-test-strategy
fact: stand_actions puts a fetch in flight with harness.click on load-fetch, which waits on nothing, where a driver click would wait out LOAD_WAIT.
@task_fact n=5 slug=integration-test-strategy
fact: stand_actions types a sentinel into fixtures::password_labelled and reads it 0 times in the actions answer, with MASKED_VALUE also 0 times and no NodeId field written. The answer carries only ids, owner ids, the table's verb words, cause words and booleans.
@task_fact n=6 slug=integration-test-strategy
fact: No test exercises the replay of a kept actions step; Check::replay runs it through Session::run by construction, unmeasured. The replay test is owed as a CARRY on Handler coverage, and v030-06 reads verified only once that test passes.
@task_fact n=7 slug=unit-test-strategy
fact: The dioxus-native-dom actionable unit tests add four: a listener removed on a re-render is read by neither reader (it also asserts that data-dioxus-id outlives the listener, the in-tree witness that the old mark leg would still count it); a select's options and group are listed with the select as owner; a positional id reads unkeyed and an author key keyed; and an inert or hidden element is not listed.
@task_fact n=8 slug=unit-test-strategy
fact: escher-driver's actions.rs holds two unit tests: every call named for a target verb validates, and an id past its bound reads malformed.
@task_fact n=9 slug=unit-test-strategy
fact: schema_tests' field-kind table is keyed per verb (graph.nodes, actions.nodes), because the two verbs' nodes fields differ in kind. The actions help is pinned word for word among the amended rows, which went 11 to 12.
@task_fact n=10 slug=e2e-test-strategy
fact: cli_commands' actions_answers_one_line_of_the_actionable_nodes runs actions as a process against a started host and reads one JSON line. An argument to actions is refused with status 1.
@task_fact n=11 slug=e2e-test-strategy
fact: Over MCP, mcp_server calls the actions tool, and an argument reads isError. stand_mcp_tools' an_actions_tool_call_answers_the_readers_nodes_and_parents_a_command_recording_its_verb_alone reads tool_call parenting one command that records verb alone.
@task_item n=1 part=untouched
WPT conformance: the runner, its CLI, outputs, status set, scoring and the upstream-only WPT workflows are unchanged.
@task_item n=2 part=untouched
apps/browser tests and helpers are unchanged.
@task_item n=3 part=untouched
blitz-dom, blitz-paint, blitz-html, blitz-net and blitz-vibey-script tests and their coverage statements are unchanged.
@task_item n=4 part=untouched
The blitz-test-harness API, settle semantics, screenshot and screenshot_rect, text_contrast, the input and inspection helpers and HarnessOptions are unchanged. The whole-screen screenshot path and its raster error mapping are unchanged.
@task_item n=5 part=untouched
CI pipeline facts are unchanged: the legs, the fast and slow split, caches, failure logs, package installs, the local pre-push gate, and the stand-repeat leg's seventeen named targets. stand_actions enters stand-repeat through the existing stand_ glob. No CI step or verdict shape changed.
@task_item n=6 part=untouched
Coverage floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90), the flakiness budget and the mutation tier are unchanged.
@task_item n=7 part=untouched
The agent-run contract is unchanged: its invocation, exit grammar, verbs, events and state. The script was not changed.
@task_item n=8 part=untouched
The two committed checks under tests/blitz-tests/checks are byte-identical.
@task_item n=9 part=untouched
Mocking stand-ins are unchanged: HeldNet, LateNet, LoadNet, TimerTicks and the pinned bundled font.
@task_item n=10 part=untouched
Telemetry: the escher-telemetry tests, the target allowlist and the command span's eight fields are unchanged. No span field was added.
@task_item n=11 part=untouched
Event routing through the data-dioxus-id mark: the bridge still writes the mark and event dispatch reads it. Only the actionable-key check stopped reading it.
@task_item n=12 part=untouched
The schema's NODE_FIELDS (14), the eight causes, the fifteen session errors, the four exit statuses and the seven ArgKinds are unchanged.
@task_item n=13 part=untouched
No dependency, dev-dependency or third-party test framework changed (test-that unchanged).
