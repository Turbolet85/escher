@task_prose part=session
Added canvas as the driver's sixteenth verb, the twelfth at the instance level, after layout: it reads the draw commands an app's widget paints into a canvas element, as a summary by default, a full dump with full, or one part of a paged dump with part.
The verb reaches every carrier: the socket call, the escher-session command line canvas --id <id> [--full] [--part <part>], the MCP tool canvas listed after layout with help last, and a kept check's tool canvas; no new refusal cause, session error, exit status, node key, argument kind or span field.
The engine gains a read of one widget's commands outside a raster and the harness a canvas scene read at the viewport's scale; the scene painter is unedited.
Measured: Circle Drawer's canvas read in both layout modes as one committed table of 17 rows, the same on linux, windows x86_64 and macOS aarch64; two reads the same bytes; 100 circles dump in 6 parts each within 10,000 bytes; the Cells actions answer grows to 130,496 bytes.
No canvas answer, MCP result, kept check or trace log holds a glyph id, glyph position, text, font bytes or family name; the actions answer lists canvas as a sixth target verb; the cold-agent pipe takes sixteen tools with its counted verbs unchanged.
Nothing retired.
@task_file n=1 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas reading - the canvas verb's in-process session read, its closed command and brush kinds
@task_file n=2 path=packages/escher-driver/src/canvas_tests.rs
what: unit tests of the canvas reading - each command and brush kind read as its closed value, glyph ids held only as a digest
@task_file n=3 path=packages/escher-driver/src/json/canvas.rs
what: the driver's JSON writer for the canvas answer - summary, full dump line form and parts
@task_file n=4 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: stand check of Circle Drawer's canvas commands in both layout modes - the committed table, same-bytes reads, paging, no-change read, refusals
@task_file n=5 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: stand check of the canvas verb on its carriers - socket, MCP, kept check replay and stop, no glyph id or text in answers
@task_file n=6 path=packages/blitz-paint/src/lib.rs
what: the engine's scene painter - gains a read of one custom widget's scene outside a raster, paint_scene unedited
@task_file n=7 path=packages/blitz-test-harness/src/screenshot.rs
what: the harness's screenshot module - gains the canvas scene read at the viewport's scale
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the driver's verb table - sixteen verbs, the canvas row after layout, its arguments and answer fields
@task_file n=9 path=packages/escher-driver/src/schema/spec.rs
what: the driver schema's field kinds - five new kinds for the canvas answer
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: unit tests of the driver verb table and its counts
@task_file n=11 path=packages/escher-driver/src/command.rs
what: the driver's command grammar - gains the canvas command with id, full and part
@task_file n=12 path=packages/escher-driver/src/command_tests.rs
what: unit tests of the command grammar - canvas validation, malformed id, part 0 or 1001, full twice
@task_file n=13 path=packages/escher-driver/src/execute.rs
what: the driver's executor - canvas outcomes, the whole-or-paged decision and the cut into parts within the text budget
@task_file n=14 path=packages/escher-driver/src/json.rs
what: the driver's JSON answers - writes the canvas outcomes
@task_file n=15 path=packages/escher-driver/src/json_tests.rs
what: unit tests of the driver's JSON answers - every canvas command and brush form, summary, dump and part
@task_file n=16 path=packages/escher-driver/src/explore.rs
what: the driver's exploration filter by field kind - leaves out the canvas commands and counts
@task_file n=17 path=packages/escher-driver/src/explore_tests.rs
what: unit tests of the exploration filter - explored verbs stay click, type and scroll
@task_file n=18 path=packages/escher-driver/src/actions.rs
what: the actions verb's target verbs per listed node - gains canvas
@task_file n=19 path=packages/escher-driver/src/describe.rs
what: the driver's describe of the verb table - the canvas verb described
@task_file n=20 path=packages/escher-driver/src/mcp.rs
what: the MCP server's tools listing - sixteen tools
@task_file n=21 path=packages/escher-driver/src/mcp_tests.rs
what: unit tests of the MCP tools listing - canvas after layout, its input schema
@task_file n=22 path=packages/escher-driver/src/cli_tests.rs
what: unit tests of the escher-session command line - the canvas command form
@task_file n=23 path=packages/escher-driver/src/recording_tests.rs
what: unit tests of session recording - a kept check reads a canvas record
@task_file n=24 path=packages/escher-driver/src/lib.rs
what: the driver crate's public surface and doc - exports the canvas reading, sixteen verbs, twelve instance verbs
@task_file n=25 path=tests/blitz-tests/tests/fixtures/mod.rs
what: shared fixture pages - gains a page of a small and a large canvas
@task_file n=26 path=tests/blitz-tests/tests/session_common/mod.rs
what: the session checks' shared module - canvas call helpers
@task_file n=27 path=tests/blitz-tests/tests/stand_actions.rs
what: stand check of the actions verb - canvas as a target verb, the Cells answer bound
@task_file n=28 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: cross-OS readings of the served tasks - its 14 actions digests re-read
@task_file n=29 path=tests/blitz-tests/tests/stand_act_spans.rs
what: stand check of command spans per acting call - calls per mode 41 to 43
@task_file n=30 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: stand check of scroll - its literal verb count pin moved to 16
@task_file n=31 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: stand check of the MCP tools - a canvas tool call parents a command recording its verb alone
@task_file n=32 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: stand check that typed passwords and masks reach no new answer - the canvas answer
@task_file n=33 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: the requirement sweep witnessed inside stand-repeat - canvas requirements
@task_file n=34 path=examples/seven_guis/tests/cli_commands.rs
what: command-line checks through the binary - a canvas host and its one-line answer, verb count pin 16
@task_file n=35 path=examples/seven_guis/tests/mcp_server.rs
what: MCP role checks as a child - a canvas reads the same through MCP and the command line
@task_file n=36 path=examples/seven_guis/tests/host_log.rs
what: host stderr log silence at trace - no canvas needle
@task_file n=37 path=examples/seven_guis/tests/mcp_log.rs
what: mcp role stderr log silence at trace - no canvas needle
@task_file n=38 path=examples/seven_guis/tests/common/mod.rs
what: seven_guis checks' shared module - the canvas needles
@task_file n=39 path=scripts/cold-agent.sh
what: the cold-agent pipe - its verb list takes canvas, sixteen tools
@task_file n=40 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's offline unit tests - canvas among the verbs
@task_item n=1 part=touches
contract/driver-verbs/verb-table: the verb table reads sixteen verbs, canvas after layout at the instance level.
@task_item n=2 part=touches
contract/mcp-server/tools-list and contract/mcp-tools-list/order: sixteen tools, canvas after layout, help last.
@task_item n=3 part=touches
contract/escher-session-command-line/command-form and usage-line: gain canvas --id <id> [--full] [--part <part>], exit 0 answered and 1 refused.
@task_item n=4 part=touches
contract/driver-session/reading-verbs: canvas joins the reading verbs, changing nothing; contract/session-layout/changes-nothing gains a sibling that resolves its id as layout does.
@task_item n=5 part=touches
decision/socket-carries-commands and contract/session-host/call: the socket now carries the canvas readings.
@task_item n=6 part=touches
contract/cold-agent-check/step and contract/cold-agent-verdict/isolated: the pipe reads sixteen tools; the counted verbs are unchanged.
@task_item n=7 part=touches
contract/session-recording/check-read and record-fields: a kept check reads a canvas record, tool canvas, twelve instance verbs; contract/check-replay/stop: a flipped canvas digest stops the replay naming digest.
@task_item n=8 part=touches
contract/driver-refusal-causes/fonts-unavailable, contract/refusal-cause/fonts-unavailable and contract/driver-refusals/fonts-unavailable: canvas is refused fonts-unavailable where the app cannot paint text; the cause set and its order stay eight.
@task_item n=9 part=touches
contract/driver-session/run-validation and contract/driver-argument-kinds/part: canvas is malformed without an id, past the id byte bound, with part 0 or 1001 or full twice; part reuses the existing argument kind.
@task_item n=10 part=touches
contract/command-span/verb and contract/stand-log/command-span: the verb words read sixteen; canvas records verb alone, refused verb and cause; contract/tool-call-span/ending: a canvas tool_call parents one command.
@task_item n=11 part=touches
contract/driver-verbs/actions and decision/actions-answer-holds-no-value: actions lists canvas as a sixth target verb with no cause on an app that can paint text; the Cells answer stays within the answer bound at 130,496 bytes.
@task_item n=12 part=touches
decision/snapshot-text-budget, contract/cli-snapshot/part and contract/session-wire/answer-bound: the canvas full dump is whole within the text budget, else cut in parts of whole lines each within 10,000 bytes.
@task_item n=13 part=touches
decision/session-writes-no-screen-content, decision/host-stderr-no-screen-content, decision/session-logs-no-screen-content and decision/log-silence-read-on-a-sink-host: the canvas answer joins them; host and mcp logs hold 0 canvas needles at trace.
@task_item n=14 part=touches
decision/typed-password-never-returned and contract/driver-session/password: the canvas answer holds neither the typed sentinel nor its mask.
@task_item n=15 part=touches
contract/committed-checks/excluded-content and decision/checks-hold-no-private-data: a kept canvas check holds no glyph id, text, font bytes or family name.
@task_item n=16 part=touches
decision/cross-os-readings-by-answer-digest: its 14 actions digests are re-read on three hosts, the 40 snapshot and lint digests unmoved; decision/canvas-shots-cross-os-by-digest gains a canvas command read beside the shot.
@task_item n=17 part=touches
decision/app-widget-shows-in-shots and decision/canvas-text-on-whole-device-pixels: an app widget's commands are now read outside a raster, beside the shot; the scene painter is unedited.
@task_item n=18 part=touches
contract/harness-screenshot/screenshot and contract/harness-screenshot/fonts-unavailable: the harness gains a canvas scene read at the viewport's scale with no resolve, tick or pump.
@task_item n=19 part=touches
decision/count-and-rename-sweeps: a verb-count sweep on the words alone missed two literal VERBS.len() pins; the sweep must also search VERBS.len().
@task_item n=20 part=touches
decision/no-change-read-control-needs-pending-work: the canvas read's no-change check holds Timer ticks pending and reads snapshot, focus, diff and clock unmoved.
@task_item n=21 part=touches
contract/session-recording/exploration-checks: exploration leaves out the canvas command and count fields; explored verbs stay click, type and scroll.
@task_item n=22 part=touches
decision/mcp-tested-as-child-and-in-process and decision/cli-checks-through-own-binary: canvas is checked through the binary and as an MCP child, the same answer on both.
@task_item n=23 part=touches
decision/shared-fixture-pages and decision/session-checks-share-one-module: a canvas fixture page and canvas call helpers join the shared modules.
@task_fact n=1 slug=test-strategy
fact: Circle Drawer's canvas commands are read in process in both layout modes as one committed table of 17 rows (boot, after a click, 12 and 100 circles; summary, full and parts), held equal by size and digest on linux, windows x86_64 and macOS aarch64.
@task_fact n=2 slug=session-lifecycle
fact: The canvas verb answers a summary by default (command counts per kind, size, digest), the whole dump with full, or one part with part 1 to 1000; an element holding no canvas reads canvas false; an unread or deleted id is refused not-found or stale, changing nothing.
@task_fact n=3 slug=integration-test-strategy
fact: A canvas answer holds closed words, numbers and 64-bit digests only - never a glyph id, glyph position, text, font bytes or family name - on every carrier and in kept checks.
@task_fact n=4 slug=integration-test-strategy
fact: A canvas read is deterministic: two reads give the same bytes, a click's line diff is all added and none removed, and advance with nothing drawn reads an empty diff.
@task_fact n=5 slug=session-lifecycle
fact: A canvas dump over the text budget is cut into parts of whole lines each within 10,000 bytes, fonts and images in part 1 alone, the parts joined equal to the whole, a part past the last empty.
@task_fact n=6 slug=integration-test-strategy
fact: A canvas command has no identity: a changed command reads as one line removed and one added until virtual nodes give commands an identity, and a layer's filter is written as a flag only.
@task_fact n=7 slug=unit-test-strategy
fact: A control whose case lies in the later of two --test targets of one entry with no --no-fail-fast needs an edit that leaves the earlier target green, since cargo runs targets in name order and stops at the first red one.
@task_item n=1 part=untouched
WPT conformance: the runner, its CLI, status set, expectations, pinned commit and upstream-only workflow.
@task_item n=2 part=untouched
agent-run: its verbs, events, exit codes, selections and state; the cold-agent events, exit codes, state directory and wrong-call bar.
@task_item n=3 part=untouched
CI integration: apt installs, artifacts, caches, legs, MSRV, clippy, dependency audit (no package added) and the per-crate coverage floors, which hold.
@task_item n=4 part=untouched
Lint defects and findings, the contrast reader and its formula.
@task_item n=5 part=untouched
Session wire versions, hello, request and read bounds, the session error set and exit statuses: no new session error or exit status.
@task_item n=6 part=untouched
Harness construction, input helpers and settle; the advance verb and the virtual clock contracts, which a canvas read leaves unmoved.
@task_item n=7 part=untouched
Handler coverage, including canvas handlers left uncounted.
@task_item n=8 part=untouched
Mutation testing, the complexity ceiling of 15 and the fifteen session errors: their numbers are no verb count.
@task_item n=9 part=untouched
Mocking discipline and network stand-ins; the graph and screenshot verbs' own answers.
