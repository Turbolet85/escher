@task_prose part=session
The session put the in-process layout read on every ratified carrier: a fifteenth driver verb, layout --id, on the session socket, the command line, the MCP role's tools and a kept check, answering one element's bounds, laid-out box, placement, shifts and width and height reasons as ids, closed words and numbers, changing nothing.
The verb table grew from fourteen to fifteen verbs and its instance verbs from ten to eleven; layout sits after actions at the instance level with one required id argument and seven always-present result fields.
The schema's field kinds grew from thirteen to seventeen (rect, placement, shifts, size); exploration now leaves out a verb by those kinds as it does by the image kind, so the explored verbs stay click, type and scroll.
The driver's placer gained an unexplained member for an element the snapshot lists but the engine gives no layout reason; the actions answer lists layout as a fifth per-node verb with no cause, and the Cells actions answer at boot measured 118,292 bytes, up from 106,088.
No IPC method, socket, port, env var, crate, refusal cause, session error, exit status, node key or span field was added; the cold-agent pipe's verb list stays at fourteen by the founder-fork's conservative option.
The twelve actions digests of the cross-OS readings check were re-read and held on linux, windows and macOS; the snapshot and lint digests did not move.
@task_file n=1 path=packages/escher-driver/src/layout.rs
what: the driver's Session::layout read and its Placer, the layout explanation of one element by id
@task_file n=2 path=packages/escher-driver/src/layout_tests.rs
what: unit tests of the driver's layout read and its unexplained placer
@task_file n=3 path=packages/escher-driver/src/schema.rs
what: the driver verb table VERBS, each verb's arguments, result fields and level
@task_file n=4 path=packages/escher-driver/src/schema/spec.rs
what: the verb table's spec types, the argument kinds and result field kinds
@task_file n=5 path=packages/escher-driver/src/schema_tests.rs
what: unit tests holding the verb table's order, levels and counts
@task_file n=6 path=packages/escher-driver/src/command.rs
what: the driver's Command set and call validation against the verb table
@task_file n=7 path=packages/escher-driver/src/execute.rs
what: the session's one run path, its outcomes, the per-verb target check and what a kept check records per call
@task_file n=8 path=packages/escher-driver/src/json.rs
what: the driver's own hand-written JSON writer for every answer
@task_file n=9 path=packages/escher-driver/src/explore.rs
what: escher-driver explore and the rule choosing its explored verbs
@task_file n=10 path=packages/escher-driver/src/explore_tests.rs
what: unit tests of the explored-verbs rule by field kind
@task_file n=11 path=packages/escher-driver/src/actions.rs
what: the actions verb's answer and the target verbs it lists per node
@task_file n=12 path=packages/escher-driver/src/describe.rs
what: the one self-description document shared by the command line and MCP
@task_file n=13 path=packages/escher-driver/src/mcp.rs
what: the MCP role, its tool listing and tool calls
@task_file n=14 path=packages/escher-driver/src/mcp_tests.rs
what: unit tests of the MCP role's tool listing and order
@task_file n=15 path=packages/escher-driver/src/cli_tests.rs
what: unit tests of the escher-session command line, its usage line and verb flags
@task_file n=16 path=packages/escher-driver/src/recording_tests.rs
what: unit tests of the kept check file and its replay
@task_file n=17 path=packages/escher-driver/src/lib.rs
what: the escher-driver crate root and its crate documentation naming the verbs
@task_file n=18 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check of the session's layout read, now also through the verb in both layout modes
@task_file n=19 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions verb's answer, its per-node verbs and the Cells answer bound
@task_file n=20 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the cross-OS readings check, a table of answer digests identical on every CI OS
@task_file n=21 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check of the command span each call opens, its verb and cause fields
@task_file n=22 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of MCP tool calls and their tool_call span
@task_file n=23 path=tests/blitz-tests/tests/stand_replay.rs
what: the stand check of kept check replay and its first difference
@task_file n=24 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared stand-check helpers module of blitz-tests
@task_file n=25 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of the scroll verb, pinning the verb table's length
@task_file n=26 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the stand check that every 0.2.0 requirement names its witness inside stand-repeat
@task_file n=27 path=examples/seven_guis/tests/cli_commands.rs
what: the seven_guis check of the escher-session command line's verbs and answers
@task_file n=28 path=examples/seven_guis/tests/mcp_server.rs
what: the seven_guis check of the MCP server's tool listing and tool results
@task_file n=29 path=examples/seven_guis/tests/mcp_log.rs
what: the seven_guis check that the MCP role's log holds no content at trace
@task_file n=30 path=examples/seven_guis/tests/host_log.rs
what: the seven_guis check that the session host's log holds no content at trace
@task_file n=31 path=docs/capabilities.md
what: the capability statement, held to the driver schema, naming the verbs and measured limits
@task_file n=32 path=README.md
what: the root README, escher's own front page, with the count of driver commands an MCP client gets as tools
@task_item n=1 part=touches
The decision that the layout read stays in process is broken: the read is now reached by a verb on the session socket, the command line, MCP and a kept check.
@task_item n=2 part=touches
The Session::layout contract: the read is now also reached through the session's run path by the layout verb, inside that call's command span, opening no span of its own and changing nothing.
@task_item n=3 part=touches
The driver verb table's order and count: fifteen verbs, layout after actions and before help, eleven at the instance level.
@task_item n=4 part=touches
The per-verb driver-verbs contract rows hold no layout row; the new verb belongs beside them.
@task_item n=5 part=touches
The kept check file's kept verbs: eleven instance verbs, layout among them, its answer kept at rest and replayed, a flipped placed value stopping the replay.
@task_item n=6 part=touches
The explored-verbs contract of escher-driver explore: a verb is left out by a result field of image or layout-reading kind, not by its name; the explored verbs stay click, type and scroll.
@task_item n=7 part=touches
The actions verb's target verbs and the actions answer's node fields: each listed node now carries layout as a fifth verb with no cause.
@task_item n=8 part=touches
A measured limit stated in the tool or a bound on the actions answer: the Cells actions answer at boot is now 118,292 bytes, from 106,088.
@task_item n=9 part=touches
The MCP server's tools/list and the mcp role's tool listing: fifteen tools, layout after actions, help last.
@task_item n=10 part=touches
The root README's count of driver commands an MCP client gets as tools: fifteen.
@task_item n=11 part=touches
The capability statement and the decision holding it to the driver schema: fifteen verbs, layout named.
@task_item n=12 part=touches
The escher-session command line: its usage line gains layout --id after actions; a missing id is refused malformed with exit status 1.
@task_item n=13 part=touches
The command span's verb field and the tool_call span's tool field: their word sets gain layout; an accepted layout call records verb alone, a refused one verb and cause.
@task_item n=14 part=touches
The decision that the validated verb set widens only by ratification and the decision that a new carrier widens the session boundary: layout was ratified and widens what crosses the carriers.
@task_item n=15 part=touches
The answer JSON contracts and the decision that answers are written by the driver's own JSON writer: a new layout answer shape is written by hand.
@task_item n=16 part=touches
The one self-description for command line and MCP: the describe document's verbs grow from fourteen to fifteen.
@task_item n=17 part=touches
The per-verb target check: layout checks nothing beyond the id read itself, and a call refused not-found or stale changes nothing.
@task_item n=18 part=touches
The decision that stand readings are identical on every CI OS: the twelve actions digests were re-read and held on all three OSes.
@task_item n=19 part=touches
The decisions on the socket carrying screen readings and session calls: a layout answer now crosses the session socket as ids, closed words and numbers.
@task_item n=20 part=touches
The cold-agent rows that count the pipe's verbs: the pipe stays at fourteen, layout left out, provisionally on the founder's fork.
@task_item n=21 part=touches
The blitz-dom and dioxus-native-dom layout reason types and the placed-by contract: the driver maps them to closed words and adds its own unexplained placer where the engine gives no reason.
@task_item n=22 part=touches
The shared stand-check helpers convention: two layout helpers added to the shared module, and the layout stand check now reads the shared fixtures module.
@task_fact n=1 slug=standard-contracts
fact: The layout verb takes one required id argument, sits at the instance level after actions, and answers seven always-present fields: id, bounds, laid_out, placed, shifts, width and height.
@task_fact n=2 slug=standard-contracts
fact: The verb table's result field kinds include rect, placement, shifts and size, seventeen kinds in all.
@task_fact n=3 slug=standard-contracts
fact: The layout answer is written with kebab-case closed words for placing (block, flex, grid, table, inline, float, absolute, fixed, root, select, select-list, unexplained, app), shift (scroll, transform), size cause (style, min, max, flex-grow, flex-shrink, stretch, grid, table, replaced, content, layout) and length (auto, none, min-content, max-content, fit-content, stretch, px, percent, calc), with fallback words unexplained, transform, layout and auto for a later member of an engine set, never a panic.
@task_fact n=4 slug=standard-contracts
fact: An element the snapshot lists but the engine gives no layout reason, such as a display contents group, answers placed by unexplained at its snapshot bounds.
@task_fact n=5 slug=standard-contracts
fact: The escher-session command line carries layout --id with --session, answering the run's one JSON line, exit 0 on an answer and 1 on a refusal.
@task_fact n=6 slug=standard-contracts
fact: The cold-agent pipe's verb list stays at fourteen without layout, a provisional founder-fork choice, until the founder widens it.
@task_item n=1 part=untouched
The engine's layout itself: Taffy, Stylo, the layout modes, anonymous blocks and damage are not changed; only read.
@task_item n=2 part=untouched
The session wire's version, hello, call and stop forms, and the request and reply bounds: no new IPC method or version.
@task_item n=3 part=untouched
The refusal causes closed at eight, the refusal faults, session errors and exit status codes: none added.
@task_item n=4 part=untouched
The node keys and the snapshot, its text form, diff and stable element id tiers.
@task_item n=5 part=untouched
Telemetry init, scrubbing, line format and engine targets: no span field or log content added.
@task_item n=6 part=untouched
CI legs, the leg runner, coverage floors, the stand-repeat budget and the workflow contract: no leg or script changed.
@task_item n=7 part=untouched
The lint verb, the graph verb, the screenshot verb and their answers.
@task_item n=8 part=untouched
The occupied names, crates, binaries, env vars, ports and listeners: none taken or freed.
@task_item n=9 part=untouched
Browser, WPT runner, paint, script and the upstream Blitz design rows.
