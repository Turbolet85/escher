@task_prose part=session
The session added canvas, the sixteenth driver verb and the twelfth at the instance level, placed after layout. It reads the draw commands an app's widget paints into a canvas element as a summary by default or as a full dump, and it is deterministic and diffable.
The verb is on every carrier: the escher-session command line as canvas --id <id> [--full] [--part <part>], the MCP tool canvas placed after layout with help still last, the session socket, and a kept check's tool canvas.
Circle Drawer's canvas is now read in process in both layout modes. Its committed table of 17 rows holds on linux, windows and macOS, and two reads give the same bytes.
Measured: a full dump of 527, 5,915 and 49,266 bytes at 1, 12 and 100 circles; a summary of 166 to 172 bytes; the 100-circle dump in 6 parts of at most 10,000 bytes each; the Cells actions answer from 118,292 to 130,496 bytes for 678 nodes.
Counts moved: verbs 15 to 16, instance verbs 11 to 12, MCP tools 15 to 16, target verbs on each listed actions node 5 to 6, and the cold-agent pipe's verb list 15 to 16 with the counted verbs unchanged. Nothing retired; no new cause, exit status, session error or argument kind.
@task_file n=1 path=packages/escher-driver/src/canvas.rs
what: new; the driver's canvas reading: the closed draw-command kinds (layer, clip, mask, pop, fill, stroke, glyphs, shadow), the brushes, and the in-process Session canvas read of a canvas element by id
@task_file n=2 path=packages/escher-driver/src/json/canvas.rs
what: new; the canvas answer's writer: the summary's counts per command kind and digests, and the full dump's one line per command
@task_file n=3 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: new; the stand check of Circle Drawer's canvas read: the summary and full dump at boot, after a click, at 12 and 100 circles, paging, the read changing nothing, and refusals
@task_file n=4 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: new; the stand check of the canvas verb on the socket, MCP and the kept check, and that no answer holds a glyph id or text
@task_file n=5 path=packages/blitz-paint/src/lib.rs
what: edited; the paint engine, which gains a read of one custom widget's draw commands outside a raster
@task_file n=6 path=packages/blitz-test-harness/src/screenshot.rs
what: edited; the headless harness's screenshot side, which gains a canvas element's scene at the viewport's scale
@task_file n=7 path=packages/escher-driver/Cargo.toml
what: edited; the driver crate's dependencies, which now take anyrender and peniko from the workspace
@task_file n=8 path=Cargo.lock
what: edited; the workspace lock, where only the driver's dependency list moved
@task_file n=9 path=packages/escher-driver/src/schema.rs
what: edited; the driver's verb table, with the CANVAS verb row after LAYOUT, its help, its arguments and its answer fields
@task_file n=10 path=packages/escher-driver/src/schema/spec.rs
what: edited; the verb table's field kinds, which gain length, command-counts, digest, commands and digests
@task_file n=11 path=packages/escher-driver/src/schema_tests.rs
what: edited; the verb table's checks, now at sixteen verbs
@task_file n=12 path=packages/escher-driver/src/command.rs
what: edited; the driver's command form, which gains Canvas with id, full and part
@task_file n=13 path=packages/escher-driver/src/execute.rs
what: edited; the driver's command execution: the canvas summary, dump and part answers, and the cut of a dump into parts within the answer budget
@task_file n=14 path=packages/escher-driver/src/json.rs
what: edited; the driver's answer writer, which writes the canvas answers
@task_file n=15 path=packages/escher-driver/src/explore.rs
what: edited; the explore filter by field kind, which now also leaves out the canvas kinds
@task_file n=16 path=packages/escher-driver/src/actions.rs
what: edited; the actions answer's target verbs, which now list canvas as the sixth with its refusal cause
@task_file n=17 path=packages/escher-driver/src/describe.rs
what: edited; the driver's description of a call, which now describes canvas
@task_file n=18 path=packages/escher-driver/src/mcp.rs
what: edited; the MCP server's tool listing doc, now sixteen tools
@task_file n=19 path=packages/escher-driver/src/mcp_tests.rs
what: edited; the MCP server's tools/list checks, with canvas after layout
@task_file n=20 path=packages/escher-driver/src/cli_tests.rs
what: edited; the escher-session command line's checks, with the canvas command form
@task_file n=21 path=packages/escher-driver/src/recording_tests.rs
what: edited; the session recording checks, where a kept check reads a canvas record
@task_file n=22 path=packages/escher-driver/src/lib.rs
what: edited; the driver crate's root and doc, which export the canvas reading and state sixteen verbs, twelve at the instance level
@task_file n=23 path=tests/blitz-tests/tests/fixtures/mod.rs
what: edited; the stand fixtures, which gain a canvas-commands page with a small and a large canvas
@task_file n=24 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited; the stand sessions' shared calls, which gain canvas, canvas_full and canvas_part
@task_file n=25 path=tests/blitz-tests/tests/stand_actions.rs
what: edited; the stand check of the actions answer, with the six target verbs and the Cells answer bound
@task_file n=26 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: edited; the cross-OS readings table, with its 14 actions digests re-read
@task_file n=27 path=tests/blitz-tests/tests/stand_act_spans.rs
what: edited; the stand check of command spans, now 43 calls per mode
@task_file n=28 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: edited; the stand scroll check, whose verb-count pin now reads 16
@task_file n=29 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: edited; the stand check of MCP tool calls, with a canvas tool call parenting one command
@task_file n=30 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: edited; the stand check that a masked typed value reaches no new answer, now including the canvas answer
@task_file n=31 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: edited; the stand requirement sweep over the verbs
@task_file n=32 path=examples/seven_guis/tests/cli_commands.rs
what: edited; the seven_guis escher-session command line checks, with a canvas host and the canvas command through the binary
@task_file n=33 path=examples/seven_guis/tests/mcp_server.rs
what: edited; the seven_guis MCP server checks, where a canvas reads the same through MCP and the command line
@task_file n=34 path=examples/seven_guis/tests/host_log.rs
what: edited; the escher-session host log checks, with no canvas content in the log at trace
@task_file n=35 path=examples/seven_guis/tests/mcp_log.rs
what: edited; the MCP server log checks, with no canvas content in the log at trace
@task_file n=36 path=examples/seven_guis/tests/common/mod.rs
what: edited; the seven_guis checks' shared helpers, which gain the canvas needles
@task_file n=37 path=scripts/cold-agent.sh
what: edited; the cold-agent.sh run pipe, whose verb list now holds canvas (16)
@task_file n=38 path=.github/scripts/test_cold_agent.py
what: edited; the cold-agent pipe's test, which takes canvas
@task_file n=39 path=docs/capabilities.md
what: edited; the capability statement: sixteen verbs, canvas named, the canvas answer's sizes, and the unpaged-actions limit at 130,496 bytes
@task_file n=40 path=README.md
what: edited; the project readme, which states the sixteen driver commands as tools
@task_item n=1 part=touches
decision/circle-drawer-circles-painted-on-canvas is stale. Circle Drawer's circles stay painted on the canvas, but the driver's canvas verb now reads their draw commands, so they no longer cross as pixels alone (the founder's F1 B and F3 A).
@task_item n=2 part=touches
contract/escher-session-command/form now has sixteen verbs, and the new form is canvas --id <id> [--full] [--part <part>].
@task_item n=3 part=touches
contract/escher-session-mcp/tools: tools/list now lists sixteen tools, canvas after layout and help still last. The canvas inputSchema is id (required), full (boolean) and part (integer).
@task_item n=4 part=touches
contract/escher-session-refusals/fonts-unavailable: canvas is refused fonts-unavailable where the app cannot paint text, the same as screenshot and lint.
@task_item n=5 part=touches
contract/escher-session-actions/answer: each listed node now carries six target verbs, with canvas last and no cause on an app that can paint text. The Cells answer is now 130,496 bytes for 678 nodes, still within the 1,048,576-byte bound.
@task_item n=6 part=touches
contract/escher-session-snapshot/part: the --part paging convention (whole lines within a 10,000-byte budget, parts 1 to 1000, a part past the last empty) now also applies to the canvas full dump.
@task_item n=7 part=touches
contract/escher-session-layout/answer: canvas resolves its id the same way layout does, and its answer and argument rows sit beside layout's.
@task_item n=8 part=touches
contract/escher-session-help/answer: the verb table's help now carries the canvas verb row and its text-field help.
@task_item n=9 part=touches
contract/escher-session-refusals/unknown-verb: canvas is now a known verb. A missing id, an id past the bound, part 0 or 1001, or an untaken argument is refused malformed.
@task_item n=10 part=touches
contract/escher-session-exit-codes/0 and contract/escher-session-exit-codes/1: the canvas command answers exit 0 and refused exit 1. No new exit status.
@task_item n=11 part=touches
contract/escher-session-mcp/result-bound: canvas results stay within the result bound because a full dump over budget is cut into parts.
@task_item n=12 part=touches
contract/stand-element-ids/circle-canvas: this is the id the canvas verb reads on Circle Drawer. The id itself is unchanged.
@task_item n=13 part=touches
decision/session-recording-replayable-check and contract/escher-session-record/check-file: a kept check can now hold tool canvas, which replays green and stops on a changed digest.
@task_item n=14 part=touches
decision/cold-agent-uses-driver-mcp and the cold-agent.sh run pipe rows: the pipe's verb list now holds sixteen including canvas, and its counted verbs are unchanged. The canvas reading reaches the cold agent through MCP alone.
@task_item n=15 part=touches
decision/no-caller-input-echoed: no canvas answer, MCP result or kept check holds text, a glyph id, a glyph position, font bytes or a family name, and a masked typed value reaches no canvas answer.
@task_item n=16 part=touches
decision/headless-stand-pinned-viewport (indirect): a canvas read takes its scene at the pinned viewport's scale with no resolve, tick or pump.
@task_item n=17 part=touches
contract/escher-session-host/log-service-name and the MCP server's end and stdio rows (indirect): the host and MCP logs now carry canvas command spans that record the verb alone, with 0 canvas content needles at trace.
@task_fact n=1 slug=surface-cli
fact: canvas is the sixteenth driver verb and the twelfth at the instance level, after layout. Without --full it answers a summary of counts per command kind (layer, clip, mask, pop, fill, stroke, glyphs, shadow) with 64-bit digests; with --full it answers a dump of one line per draw command.
@task_fact n=2 slug=surface-cli
fact: The canvas answer is closed words, numbers and 64-bit digests only. It never holds text, a glyph id, a glyph position, font bytes or a font family name; glyph ids cross as a digest.
@task_fact n=3 slug=surface-cli
fact: A canvas full dump over the 10,000-byte answer budget is cut into parts of whole lines. The fonts are listed in part 1 alone, the parts joined equal the whole, and a part past the last is empty.
@task_fact n=4 slug=surface-cli
fact: Stated limit: a changed draw command reads as one line removed and one added until virtual nodes give commands an identity, and a layer's filter is written as a flag only.
@task_fact n=5 slug=surface-desktop-native
fact: A canvas read changes nothing in the app: snapshot, both focus readings, diff and virtual clock stay unmoved, and pending Timer ticks stay unapplied.
@task_fact n=6 slug=surface-desktop-native
fact: Circle Drawer's canvas reads deterministically in both layout modes. Two reads give the same bytes, one committed table holds across linux, windows and macOS, a click's line diff is all added, and advancing the clock gives an empty diff.
@task_fact n=7 slug=surface-cli
fact: The MCP tool canvas has inputSchema id (required), full (boolean) and part (integer). It answers the same line as the command line, and its tool_call parents one command span.
@task_item n=1 part=untouched
ShellProvider window chrome controls: canvas reads no window state.
@task_item n=2 part=untouched
Stand stable element ids other than circle-canvas: no id was added, renamed or removed.
@task_item n=3 part=untouched
agent-run.sh test contract rows: that pipe did not move.
@task_item n=4 part=untouched
multi_doc and paint_bench command line rows: their arguments, environment and output are unchanged.
@task_item n=5 part=untouched
escher-session served tasks and the coverage role rows: canvas was proven on test hosts, and no served task was added.
@task_item n=6 part=untouched
escher-session MCP protocol revisions and the screenshot answer and result: their shape is unchanged.
@task_item n=7 part=untouched
Stand layout decisions (fixed document skeleton, keyed controls, accent colour, attributes-only ids and a11y, lints clean at boot, CRUD keyboard region, open list rows, scroll into view, visible region, load page network, order page, flight type select): no stand page or control changed.
@task_item n=8 part=untouched
absence/arbitrary-index-html-templates and the mobile-native and web-spa surfaces: canvas reaches no document template, mobile layout or web layout.
@task_item n=9 part=untouched
The escher-session graph, lint and snapshot answer rows: their answers and sizes did not move.
