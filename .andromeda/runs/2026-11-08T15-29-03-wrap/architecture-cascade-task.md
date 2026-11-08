@task_prose part=session
Chunk 2026-11-08-what-can-i-do-here adds the driver verb actions: one read-only reading of every actionable node on the screen, each with the verbs from the schema table that take an id (click, type, scroll, screenshot) and the refusal cause each would meet now.
escher-driver gains the private module actions (Session::actions, ActionNode, VerbReading), Command::Actions, Outcome::Actions, schema::ACTIONS, FieldKind::ActionNodes (action-nodes) and one shared per-verb target check, check_target, which the four id-taking arms of Session::execute and Session::actions all call.
dioxus-native-dom gains ActionableNode and DioxusDocument::actionable_nodes. The crate-private Readings is now the one actionable rule, and unkeyed_actionable and actionable_nodes both read it.
Retired: unkeyed_actionable no longer reads the data-dioxus-id mark as its listener leg. It reads the exact current listeners (an event other than mounted) instead. The mark stays and event dispatch still reads it.
Counts moved: VERBS 13 to 14, instance verbs 9 to 10, FieldKind 12 to 13, escher-driver private modules 17 to 18 and re-exports 57 to 59, run stand 244 to 253 over 52 to 53 stand files, escher-driver unit tests 100 to 102, dioxus-native-dom unit tests 100 to 104.
Measured: ACTIONABLE_AT_BOOT is 2, 7, 3, 11, 678, 2. The Cells actions answer is 106,088 bytes for 678 nodes. CI is green 17/17. One residual is open: the replay of a kept actions step is owed a test (a CARRY), and v030-06 is verified only once that test passes.
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
Driver session contract: the command and refusal schema has fourteen verbs, not thirteen. actions is the tenth instance verb, after graph. The eight closed causes are unchanged.
@task_item n=2 part=touches
Any count of instance verbs (nine becomes ten), of driver commands or MCP tools (thirteen becomes fourteen), or of FieldKinds (twelve becomes thirteen, adding ActionNodes or action-nodes) in the driver, CLI or MCP contracts.
@task_item n=3 part=touches
escher-driver crate shape: seventeen private modules become eighteen (actions), and 57 re-exports at the crate root become 59 (ActionNode, VerbReading). Command and Outcome each gain one variant, Command::Actions and Outcome::Actions.
@task_item n=4 part=touches
The per-verb target check of the executor. click and type met action_point alone in their own arms. Now one shared check_target (with a Cleared result) serves the four id-taking verbs, and no arm keeps a check of its own. screenshot with an id now meets in_view, then an empty crop, then can_paint_text false (fonts-unavailable) before it paints. Target became pub(crate), and locate now ends in target_of.
@task_item n=5 part=touches
Dioxus DOM bridge, actionable-key check: unkeyed_actionable's listener leg moved from the data-dioxus-id mark to the element's exact current listeners holding an event other than mounted. Any text that says the check reads the mark as its listener leg is stale. Its domain, order, fields and remedy are unchanged.
@task_item n=6 part=touches
Dioxus DOM bridge, read-only readers under accessibility: actionable_nodes joins the snapshot, its diff and the UI graph as one more reader that holds no NodeId. ActionableNode is re-exported from the crate root beside UnkeyedActionable.
@task_item n=7 part=touches
Established decision on the driver session's boundary. The actions answer crosses every ratified carrier: socket call, command line, MCP and kept check. It carries ids, owner ids, the verb words of the table, closed cause words and booleans (keyed), never a value, typed text or NodeId. This widens the list of what crosses, by the founder's word.
@task_item n=8 part=touches
The hand-written json module gains the writer action_node, of the form nodes with id, owner, keyed and verbs (verb, cause). owner and cause are left out when absent. This holds the rule that every answer is written by hand, and widens what it writes.
@task_item n=9 part=touches
Recording and kept checks: a kept actions step keeps its answer whole (its nodes, causes kept, an absent cause left out). Its replay through Check::replay is exercised by no test yet. That test is owed as a CARRY.
@task_item n=10 part=touches
CLI contract of escher-session: its verbs now include actions, which takes no argument (an argument is refused malformed, exit 1) and answers one line of JSON. The served stand tasks are unchanged at six.
@task_item n=11 part=touches
Cold-agent pipe contract: the pipe's verb list joins actions after graph, and the fake's comment now says fourteen tools.
@task_item n=12 part=touches
Front page and capability statement: README and the capability statement now count fourteen driver commands an MCP client gets as tools. The capability statement adds two stated limits, Accepted is not acting and An unpaged actions answer.
@task_item n=13 part=touches
Stand and test tallies: run stand moves from 244 to 253 passing, over 53 stand files instead of 52. session_common declarers move from 33 to 34. fixtures declarers move from 31 to 32 (its stand files from 23 to 24). stand_act_spans driven calls move from 37 to 38. The two schema and MCP test names now say fourteen.
@task_item n=14 part=touches
Unit-test counts: escher-driver moves from 100 to 102 (actions.rs, a seventeenth file in the per-file list), dioxus-native-dom actionable from 4 to 8, the crate from 100 to 104. cli_commands tests per OS move from 16 to 17, mcp_server from 7 to 8, stand_mcp_tools from 7 to 8.
@task_item n=15 part=touches
The log checks' needle helpers gain actions_needles beside graph_needles and option_needles. host_log's hosted calls move from 8 to 9 and mcp_log's tool calls from 11 to 12.
@task_item n=16 part=touches
The command span: an actions call records the verb field alone (cause when refused), and its MCP tool_call parents one command. The span's eight fields are unchanged.
@task_item n=17 part=touches
The stated limit of the unpaged answer: residual res-7cc5bi4k (target next), beside An unpaged graph, is widened to own the unpaged actions answer as well.
@task_fact n=1 slug=standard-contracts
fact: Session::actions reads one snapshot and one element_ids map per call and records every id it reads. For each listed node and each Level::Instance verb with an ArgKind::Id argument, in table order (today click, type, scroll, screenshot; no verb name written in the code), it builds the call call_naming spells, runs it through validate and then check_target, and reads stale for a node that resolves to no element.
@task_fact n=2 slug=standard-contracts
fact: check_target, crate-private in execute.rs, is the one per-verb target check. click and type meet action_point (disabled, then off-screen, then covered), scroll meets none, and an id-taking screenshot meets in_view, then an empty crop, then can_paint_text false (fonts-unavailable). Its only callers are the four id-taking arms of Session::execute and Session::actions.
@task_fact n=3 slug=standard-contracts
fact: The actions read changes nothing. After it, has_changes is false, and the snapshot, both focus readings and the diff are unchanged. An open select list stays open. actions.rs makes no pump, tick, dispatch, scroll_into_view, close_select_list or focus_select call.
@task_fact n=4 slug=standard-contracts
fact: actions answers whether a call would be accepted, not what it would do (Accepted is not acting). It takes no argument and answers the whole screen at once, unpaged. The Cells answer is 106,088 bytes for 678 nodes, within the 1,048,576-byte answer bound.
@task_fact n=5 slug=standard-contracts
fact: DioxusDocument::actionable_nodes, read-only under accessibility, returns ActionableNode records (id, owner, keyed) over the UI graph's nodes in snapshot pre-order. It keeps each node the actionable rule holds for, or whose graph owner is Some, and keyed is true where the id holds no slash. keyed is the first wire form of the actionable-key check, PROVISIONAL until the founder's own word.
@task_fact n=6 slug=standard-contracts
fact: The crate-private Readings in dioxus-native-dom is the one actionable rule: Node::is_focussable, or one of the 27 interactive roles, or current listeners (from element_facts, the exact reading of the vdom walk) holding an event other than mounted. unkeyed_actionable and actionable_nodes both read it and share the private fn roles.
@task_fact n=7 slug=established-decisions
fact: The actions answer crossing every ratified carrier of the driver session was a boundary widening on pattern rul-izni57qv. The founder's word ratified it, relayed verbatim by the overseer on 2026-11-08 (inputs I4, relayed again as I6).
@task_item n=1 part=untouched
blitz-dom engine behaviour (hit testing, visible_region, the select model, scrolling, focus, inert, painting) is unchanged. No engine file changed, and can_paint_text is only read.
@task_item n=2 part=untouched
Event routing through the data-dioxus-id mark is unchanged: the bridge still writes the mark and event dispatch still reads it. Only the actionable-key check stopped reading it.
@task_item n=3 part=untouched
Occupied resources: no port, socket, env var, IPC method, filesystem area, thread or process was added. The session socket, state directory and recording directory are unchanged.
@task_item n=4 part=untouched
Stack, dependencies and pinning: no dependency, manifest, lockfile or deny config moved, and no crate was added or removed.
@task_item n=5 part=untouched
CI workflow, jobs, legs, caches, coverage floors and the seventeen named targets: no CI step or gate changed. stand_actions enters stand-repeat through the existing glob.
@task_item n=6 part=untouched
escher-telemetry, its sink, SAFE_FIELDS and redaction, and the headless stand's boots, LoadNet and the six served tasks are unchanged.
@task_item n=7 part=untouched
Upstream sync and project intent: the merge base, the fifty-seven upstream commits and the Stylo, Taffy and Dioxus waits are unchanged. That fifty-seven is not the re-export count.
@task_item n=8 part=untouched
Browser app, rdme, WPT runner, JavaScript runtime, examples other than the seven_guis tests, and the HTML fixtures are not reached.
@task_item n=9 part=untouched
NODE_FIELDS (14), the eight causes, the fifteen session errors, the four exit statuses and the seven ArgKinds are unchanged.
