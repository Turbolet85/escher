@task_prose part=session
The session added canvas, the driver's sixteenth verb and twelfth instance verb, after layout: a read of the draw commands an app's widget paints into a canvas element, as a summary (command counts by kind and a digest) or a full dump of one closed line per command, paged in parts within the answer budget.
The verb rides every carrier: the session socket, the escher-session command line (canvas --id, --full, --part), the MCP tool canvas listed after layout with help last, and a kept check whose tool is canvas; Session::canvas reads in process too and opens no span of its own.
No new refusal cause, session error, exit status, node key, argument kind or span field; the command span records verb alone for canvas and, refused, verb and cause; the tool_call span records tool and ending and parents one command.
No canvas answer, MCP result or kept check holds a glyph id, a glyph position, text, a font's bytes or a family name; host_log and mcp_log read 0 canvas needles at trace.
Measured: the Cells actions answer 118,292 to 130,496 bytes for 678 nodes; target verbs per listed actions node five to six; stand_act_spans calls per mode 41 to 43; a 100-circle dump in 6 parts each within 10,000 bytes; CI green on linux, windows and macOS.
The engine gained a read of one custom widget's scene outside a raster, and the cold-agent pipe's verb list takes canvas while its counted verbs stay unchanged.
@task_file n=1 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas reading — the in-process canvas read, its closed command and brush values, and its refusals (not-found, stale, fonts-unavailable)
@task_file n=2 path=packages/escher-driver/src/canvas_tests.rs
what: unit checks of the canvas reading's command and brush kinds and its glyph-id digest
@task_file n=3 path=packages/escher-driver/src/json/canvas.rs
what: the canvas answer's writer — the summary, the full dump's line form and the part
@task_file n=4 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: stand check of Circle Drawer's canvas reading — table, parts, the read changing nothing, refusals
@task_file n=5 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: stand check of the canvas verb over MCP, the socket and a kept check, and that its answers hold no glyph id and no text
@task_file n=6 path=packages/blitz-paint/src/lib.rs
what: the engine's paint scene builder — gains one custom widget's scene read outside a raster
@task_file n=7 path=packages/blitz-test-harness/src/screenshot.rs
what: the test harness's screenshot and scene reads — gains a canvas node's scene
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the driver's verb table — sixteen verbs, the canvas verb's arguments and answer fields
@task_file n=9 path=packages/escher-driver/src/schema/spec.rs
what: the verb table's field kinds — gains length, command-counts, digest, commands, digests
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: checks of the verb table — its order, counts and field kinds
@task_file n=11 path=packages/escher-driver/src/command.rs
what: the driver command's parse and validation — gains the canvas command
@task_file n=12 path=packages/escher-driver/src/command_tests.rs
what: checks of the driver command's parse and its malformed refusals
@task_file n=13 path=packages/escher-driver/src/execute.rs
what: the driver's command run and the command span — the canvas outcomes and the cut into parts
@task_file n=14 path=packages/escher-driver/src/json.rs
what: the driver command answer's writer
@task_file n=15 path=packages/escher-driver/src/json_tests.rs
what: checks of the command answer's written form
@task_file n=16 path=packages/escher-driver/src/explore.rs
what: the auto-exploration of verbs by field kind — leaves out the canvas kinds
@task_file n=17 path=packages/escher-driver/src/explore_tests.rs
what: checks of the auto-exploration's verb filter
@task_file n=18 path=packages/escher-driver/src/actions.rs
what: the actions answer's target verbs per node — gains canvas
@task_file n=19 path=packages/escher-driver/src/describe.rs
what: the driver's help and verb descriptions
@task_file n=20 path=packages/escher-driver/src/mcp.rs
what: the MCP server's tool listing and the tool_call span
@task_file n=21 path=packages/escher-driver/src/mcp_tests.rs
what: checks of the MCP tool listing and its order
@task_file n=22 path=packages/escher-driver/src/cli_tests.rs
what: checks of the escher-session command line's verbs and usage
@task_file n=23 path=packages/escher-driver/src/recording_tests.rs
what: checks of the kept check file's read — a canvas record
@task_file n=24 path=packages/escher-driver/src/lib.rs
what: the driver library's crate doc and public surface — sixteen verbs, the canvas types
@task_file n=25 path=tests/blitz-tests/tests/fixtures/mod.rs
what: stand fixtures — a page with a small and a large canvas
@task_file n=26 path=tests/blitz-tests/tests/session_common/mod.rs
what: stand checks' shared session calls — the canvas calls
@task_file n=27 path=tests/blitz-tests/tests/stand_actions.rs
what: stand check of the actions answer — target verbs and the answer bound on Cells
@task_file n=28 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: stand check of the cross-OS reading digests
@task_file n=29 path=tests/blitz-tests/tests/stand_act_spans.rs
what: stand check of the command and tool_call spans' fields per verb
@task_file n=30 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: stand check of scroll, holding a verb-count pin
@task_file n=31 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: stand check of MCP tool calls and their tool_call span parenting a command span
@task_file n=32 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: stand check that new answers hold neither typed text nor a masked value
@task_file n=33 path=examples/seven_guis/tests/cli_commands.rs
what: process check of the escher-session command line's commands and exit codes
@task_file n=34 path=examples/seven_guis/tests/mcp_server.rs
what: process check of the escher-session mcp role's tools
@task_file n=35 path=examples/seven_guis/tests/host_log.rs
what: the sink-installing host's log read at trace — no content needles
@task_file n=36 path=examples/seven_guis/tests/mcp_log.rs
what: the mcp role's log read at trace — no content needles
@task_file n=37 path=examples/seven_guis/tests/common/mod.rs
what: process checks' shared helpers — the canvas needles
@task_file n=38 path=scripts/cold-agent.sh
what: the cold-agent pipe — its verb list takes canvas, its counted verbs unchanged
@task_file n=39 path=.github/scripts/test_cold_agent.py
what: checks of the cold-agent pipe's verb list
@task_file n=40 path=docs/capabilities.md
what: the capability statement — sixteen verbs, canvas named, the canvas answer's sizes and the unpaged-actions limit
@task_item n=1 part=touches
contract/command-span/verb and contract/escher-driver-command-span/verb: the verb word set moves fifteen to sixteen, canvas added after layout.
@task_item n=2 part=touches
contract/tool-call-span/tool and contract/escher-driver-tool-call-span/tool: the tool word set moves to sixteen with canvas.
@task_item n=3 part=touches
contract/command-span/settled and contract/escher-driver-command-span/settled: the verb-alone list gains canvas (records verb alone; refused, verb and cause).
@task_item n=4 part=touches
contract/command-span/cause and contract/refusal-cause/fonts-unavailable: canvas refuses fonts-unavailable where the app cannot paint text, else not-found, stale, malformed; no new cause name.
@task_item n=5 part=touches
decision/layout-read-opens-no-span and decision/one-span-per-command: the in-process canvas read opens no span of its own; the canvas command opens one command span.
@task_item n=6 part=touches
decision/driver-spans-carry-no-content and decision/command-span-no-call-content: the canvas span holds no id, full flag or part number.
@task_item n=7 part=touches
decision/session-logs-carry-no-page-content, decision/graph-answer-never-logged, decision/debug-types-never-logged and decision/part-answer-never-logged: the canvas answer, summary, dump and part join what is never logged.
@task_item n=8 part=touches
decision/sink-host-read-at-trace: the host and mcp log reads at trace now count canvas needles, reading 0.
@task_item n=9 part=touches
contract/layout-answer/content and contract/layout-answer/logging: the canvas answer is their new sibling with no row of its own yet.
@task_item n=10 part=touches
contract/command-span/layout and the per-verb command-span rows (actions, graph, lint, screenshot, snapshot-part): canvas is a new per-verb call with no row; its span is read by the stand span check at 43 calls per mode.
@task_item n=11 part=touches
contract/escher-session-commands/help and contract/tool-call-span/help: the MCP tool listing has canvas after layout and help still last, sixteen tools.
@task_item n=12 part=touches
contract/mcp-role-stdout/json-rpc and contract/escher-session-stdout/command-answer: the tools listing and the command answer carry the canvas answer's one line.
@task_item n=13 part=touches
contract/escher-session-exit-codes/0 and contract/escher-session-exit-codes/1: the canvas command line exits 0 answered and 1 refused; no new exit status.
@task_item n=14 part=touches
contract/kept-check/answer, contract/kept-check/answer-digest and contract/kept-check/arguments: a kept check may record tool canvas with id, full and part, and stops naming digest when the answer moves.
@task_item n=15 part=touches
contract/actions-answer/cause and contract/actions-answer/shape: each listed node's target verbs move five to six, canvas last with no cause on an app that can paint text; the Cells answer is 130,496 bytes.
@task_item n=16 part=touches
decision/exploration-adds-no-span and decision/actions-read-changes-nothing: auto-exploration also leaves out the commands and command-counts field kinds, explored verbs stay click, type and scroll; the canvas read changes nothing in the document or the clock.
@task_item n=17 part=touches
contract/command-answer/shot, decision/shot-image-leaves-only-in-answer and decision/open-list-shot-carries-labels: canvas content no longer crosses as shot pixels alone; it crosses as closed words, numbers and 64-bit digests.
@task_item n=18 part=touches
contract/command-span/snapshot-part: a canvas full dump over the answer budget now reads in parts (part 1 to 1000, each within 10,000 bytes), beside the snapshot and graph parts.
@task_item n=19 part=touches
contract/cold-agent-counted-verbs rows and the cold-agent verdict's per-verb call counters: the pipe's verb list takes canvas but the counted verbs stay unchanged; no canvas counter is added.
@task_item n=20 part=touches
decision/escher-crates-tracing-ungated and decision/driver-emits-no-event: escher-driver now takes the paint recording crates without a feature; still no event and no subscriber from the driver.
@task_fact n=1 slug=log-coverage
fact: The canvas command's span records verb alone; refused, verb and cause; no field holds the id, the full flag or the part.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: The canvas answer (summary, full dump and part) is never logged and holds only closed words, numbers and 64-bit digests: no glyph id, glyph position, text, font bytes or family name.
@task_fact n=3 slug=pii-scrubbing-compliance
fact: Canvas content now crosses the socket, the command line, MCP and a kept check as the canvas answer, not as shot pixels alone.
@task_fact n=4 slug=4-span-trace-coverage
fact: The in-process canvas read opens no span of its own, as the layout read does not.
@task_fact n=5 slug=3-observability-harness-contract
fact: The sink-installing host and the mcp role read at trace hold no canvas needle.
@task_fact n=6 slug=log-coverage
fact: The cold-agent pipe lists canvas among its verbs but does not count it; the verdict holds no canvas counter until a served app holds a canvas.
@task_item n=1 part=untouched
Telemetry backends, OTel export, metrics exporters and metric emission: none added.
@task_item n=2 part=untouched
The sink's log line format, escaping, scrub set, target allowlist, RUST_LOG and output stream: no new target or field.
@task_item n=3 part=untouched
Error capture: the panic hook, script error source labels, window error events, window.onerror and the WPT report.
@task_item n=4 part=untouched
escher-telemetry's startup and panic events and the sink's service.name and service.version keys.
@task_item n=5 part=untouched
The agent-run harness's events, its log and its cleanup.
@task_item n=6 part=untouched
CI workflow artifacts, the build-directory sweep line and the package-install diagnostic line.
@task_item n=7 part=untouched
Select, optgroup, multiple-select form entries, page script events and the snapshot node and text rows.
@task_item n=8 part=untouched
The host wire version, session errors, exit codes 2 and 3 and the coverage role's line: no new session error or exit status.
