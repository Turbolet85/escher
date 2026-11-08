@task_prose part=session
The session added the driver verb actions: one read of every actionable node on the screen, each with the verbs of the table's instance rows that take an id and the refusal cause each call would meet now.
escher-driver gains the private module actions (Session::actions, ActionNode, VerbReading), Command::Actions, Outcome::Actions written by to_json as {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, schema::ACTIONS and FieldKind::ActionNodes; VERBS 13 to 14, instance verbs 9 to 10, FieldKind 12 to 13, private modules 17 to 18, re-exports 57 to 59.
The per-verb target check became one shared crate-private check_target that the four id-taking arms (click, type, scroll, screenshot) of Session::execute and Session::actions call; screenshot with an id now decides fonts-unavailable before painting.
dioxus-native-dom adds the read-only DioxusDocument::actionable_nodes and ActionableNode over one shared actionable rule; unkeyed_actionable's listener leg moved from the data-dioxus-id mark to the exact current listeners (the mark itself stays for event dispatch).
Tests: new stand_actions (34th session_common reader, a fixtures reader); host_log calls 8 to 9 and mcp_log calls 11 to 12 with the new actions_needles helper; stand_act_spans driven calls 37 to 38; run stand 253 passed over 53 stand files; no span, span field, event, log site, env read, subscriber or dependency was added, escher-driver keeping 2 info_span! sites.
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
The closed verb and tool word sets: VERBS 13 to 14, actions the tenth instance verb after graph; any statement of thirteen verbs, thirteen tools or nine instance verbs (the command span's verb set, the tool_call span's tool set, the instance-verb list at log-coverage line 52) moves to fourteen and ten.
@task_item n=2 part=touches
The command span: an actions call through Session::run opens one command span recording verb alone (cause when refused); the eight fields are unchanged, but a list of the verbs that open the span now ends with actions after graph.
@task_item n=3 part=touches
The tool_call span: an MCP actions tool call is a tool_call parenting one command child, read by stand_mcp_tools (its tests 7 to 8, the graph and actions tool-call tests sharing one helper body).
@task_item n=4 part=touches
The session library's silence statement: the private modules list (command, refusal, schema, execute, describe, image, recording, replay, lint and the rest) gains the eighteenth, actions, which installs no subscriber, reads no env var and prints nothing; a count of seventeen private modules moves to eighteen.
@task_item n=5 part=touches
The Outcome writers: Outcome::Actions joins Shot, Linted, Graphed and Part as an answer written by to_json alone, over MCP its JSON line, kept whole in a recorded check, never logged or fielded; Command and Outcome each gain a variant, and the Debug statement over Command and Outcome now covers that variant.
@task_item n=6 part=touches
The refusal detection that Session::run runs in front of click and type is now the shared check_target, run before all four id-taking verbs (click, type, scroll, screenshot) and by Session::actions; a statement naming the detection as click and type only is stale.
@task_item n=7 part=touches
The session_common reader count: 33 to 34, stand_actions the thirty-fourth; any enumeration of the module's readers gains stand_actions.
@task_item n=8 part=touches
The fixtures shared module's reader count: 31 to 32, its stand files 23 to 24, stand_actions among them; a stated reader count of that module (28 as measured at an earlier chunk) is stale.
@task_item n=9 part=touches
The log checks' needle helpers in the seven_guis shared module: actions_needles joins graph_needles and option_needles (and the Needles, stand_needles, stamped_lines, found, occurrences, recorded_needles readers); host_log and mcp_log read 0 actions needles at trace under both builds.
@task_item n=10 part=touches
host_log hosted calls 8 to 9 and mcp_log tool calls 11 to 12, one actions call each; any count of their calls or of tool_call lines per call moves.
@task_item n=11 part=touches
The cold-agent pipe's verb list joins actions after graph, uncounted: it is not added to the counted verbs (lint, screenshot, advance) nor to any verdict field.
@task_item n=12 part=touches
stand_act_spans' driven calls 37 to 38 with the instance-row pin 9 to 10; its eight span fields and its two re-run children are unchanged.
@task_item n=13 part=touches
The stand tally: run stand 244 to 253 passed, 0 failed, 7 ignored, over 52 to 53 stand files; stand_actions enters the stand-repeat leg by the existing glob, no named target added.
@task_item n=14 part=touches
dioxus-native-dom's accessibility readings: actionable_nodes and ActionableNode are read-only with no log site; unkeyed_actionable's listener leg reads the exact current listeners instead of the data-dioxus-id mark; the mark is still written and still routes events.
@task_item n=15 part=touches
The answer's content class: the actions answer carries ids, owner ids, verb words, closed cause words and booleans only, never a value, typed text, mask or NodeId; a typed sentinel and MASKED_VALUE read 0 times; it joins the sets of answers that cross the socket, MCP and a kept check.
@task_item n=16 part=touches
The unpaged answer residual res-7cc5bi4k widened to name the actions answer beside the unpaged graph; the Cells actions answer measures 106,088 bytes for 678 nodes, within the 1,048,576-byte bound.
@task_item n=17 part=touches
Unit test counts: escher-driver 100 to 102, dioxus-native-dom 100 to 104 (actionable tests 4 to 8); cli_commands per OS 16 to 17; mcp_server 7 to 8.
@task_fact n=1 slug=log-coverage
fact: The actions check is silent: stand_actions (a session_common and fixtures reader, both layout modes, on the six served screens and its own fixtures) installs no subscriber, reads no env var and prints nothing, its readings going into assertions only; escher-driver's private module actions and dioxus-native-dom's actionable_nodes add no log site, span, field, log field or env read, escher-driver keeping its 2 info_span! sites.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: Outcome::Actions is written by to_json alone as {"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}, owner and cause left out when absent: ids, owner ids, the table's verb words, closed cause words and booleans, never a value, typed text, mask or NodeId; a command's answer, over MCP its JSON line, kept whole in a recorded check, never logged or fielded.
@task_fact n=3 slug=span-trace-coverage
fact: An actions call opens one command span that records verb alone, and cause only when refused; over MCP it is a tool_call with tool=actions parenting one command child, as stand_act_spans and stand_mcp_tools read.
@task_fact n=4 slug=log-coverage
fact: Session::actions reads what each actionable node can take without acting: one snapshot and one element_ids map per call, every call validated then put through the shared check_target, with no pump, tick, dispatch, scroll_into_view or focus call; the read changes nothing in the document.
@task_item n=1 part=untouched
escher-telemetry's sink: its install sites, EnvFilter default, line format, closed-span record class, target allowlist and content-named redaction are unchanged; no binary, subscriber or init call moved.
@task_item n=2 part=untouched
The agent-run harness log: its events, fields and state area are unchanged; the stand tally moving is not a change to its event shape.
@task_item n=3 part=untouched
Metric coverage: no metric, counter, timer or exporter was added; in-process timing and the WPT figures are unchanged.
@task_item n=4 part=untouched
Error capture: the panic hooks, the error classes of the engine crates, blitz-vibey-script's JS error surface and document_may_load are unchanged.
@task_item n=5 part=untouched
CI integration: no CI step, artifact, leg or verdict shape changed; CI stays 17 checks.
@task_item n=6 part=untouched
Engine and upstream emitters (blitz-dom, blitz-net, blitz-html, blitz-shell, blitz-paint, blitz-vibey-script, dioxus-native, the browser app, wpt runner): no log site, field or feature wiring moved, and blitz-dom was diffed unchanged.
@task_item n=7 part=untouched
Telemetry strategy and feature wiring: no dependency, cargo feature or tracing macro use changed.
@task_item n=8 part=untouched
Values logged as-is by the engine (URLs, attributes, outer HTML, CSS values, text-node contents): unchanged, and the data-dioxus-id mark stays written by the bridge.
@task_item n=9 part=untouched
Local state and committed kept checks: the two committed Counter checks are byte-identical; session and recording directory modes are unchanged.
