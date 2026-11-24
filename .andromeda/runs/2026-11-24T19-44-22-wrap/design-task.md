@task_prose part=session
The session added canvas, the driver's sixteenth verb and twelfth instance verb, placed after layout: it reads the draw commands an app's widget paints into a canvas element as text, never as pixels.
By default it answers a summary, which counts the commands of each of eight kinds (Layer, Clip, Mask, Pop, Fill, Stroke, Glyphs, Shadow) and gives a digest. With full it answers a dump of one line per command, cut into parts of whole lines within the 10,000-byte budget, with fonts listed in part 1 alone.
It is carried on every channel: the session socket, the escher-session command line, the MCP tool canvas and a kept check's tool. It is refused with fonts-unavailable where the app cannot paint text, and with not-found or stale for a bad id. The read changes nothing and leaves the virtual clock unmoved.
No answer holds a glyph id, text, font bytes or a family name; glyph ids appear only as a digest. Circle Drawer's canvas is now readable command by command, deterministic across linux, windows and macOS.
Measured: the actions answer lists canvas as the sixth target verb on each node, and Cells' actions answer grew from 118,292 to 130,496 bytes, still within the 1,048,576-byte bound. The MCP tool listing grew from fifteen tools to sixteen, with canvas after layout and help last. Nothing was retired.
@task_file n=1 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas reading: Session::canvas and the closed values of a canvas's draw commands, brushes, fonts and images (new)
@task_file n=2 path=packages/escher-driver/src/canvas_tests.rs
what: the unit checks of the canvas reading: each command and brush kind, font and image indexing, and the glyph-id digest (new)
@task_file n=3 path=packages/escher-driver/src/json/canvas.rs
what: the canvas answer's writer: the summary line and the full dump's one line per command, with numbers, colours and digests (new)
@task_file n=4 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the stand check reading Circle Drawer's canvas in both layout modes: its committed table, parts, no clock move and refusals (new)
@task_file n=5 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: the stand check of the canvas verb over the socket, MCP and a kept check, and of the answer holding no glyph id or text (new)
@task_file n=6 path=packages/blitz-paint/src/lib.rs
what: the engine's paint crate: it gains custom_widget_scene, which reads one custom widget's scene outside a raster (edited)
@task_file n=7 path=packages/blitz-test-harness/src/screenshot.rs
what: the test harness's screenshot module: it gains Harness::canvas_scene, a canvas widget's scene at the viewport's scale (edited)
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the driver's verb table: verbs, their help, arguments and answer fields, and exit statuses; it gains the CANVAS row (edited)
@task_file n=9 path=packages/escher-driver/src/schema/spec.rs
what: the verb table's field kinds, which grew from seventeen to twenty-two with length, command-counts, digest, commands and digests (edited)
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: the verb table's checks: verb count, order and field kinds (edited)
@task_file n=11 path=packages/escher-driver/src/command.rs
what: the driver's command parsing and validation: it gains Command::Canvas with id, full and part (edited)
@task_file n=12 path=packages/escher-driver/src/command_tests.rs
what: the command parsing checks: canvas's malformed forms (edited)
@task_file n=13 path=packages/escher-driver/src/execute.rs
what: the driver's command execution and outcomes: the canvas summary, dump and parts, and the cut within the answer budget (edited)
@task_file n=14 path=packages/escher-driver/src/json.rs
what: the driver's answer writer, which now writes the canvas outcomes (edited)
@task_file n=15 path=packages/escher-driver/src/json_tests.rs
what: the answer writer's checks: every canvas command line and the summary, dump and part forms (edited)
@task_file n=16 path=packages/escher-driver/src/explore.rs
what: the driver's explore pass, whose field-kind filter now also leaves out the canvas kinds (edited)
@task_file n=17 path=packages/escher-driver/src/actions.rs
what: the actions verb's per-node target verbs and checks, which now list canvas (edited)
@task_file n=18 path=packages/escher-driver/src/describe.rs
what: the driver's describe and help text for the verbs (edited)
@task_file n=19 path=packages/escher-driver/src/mcp.rs
what: the escher-session MCP role's tool listing, now sixteen tools (edited)
@task_file n=20 path=packages/escher-driver/src/mcp_tests.rs
what: the MCP tool listing checks: the canvas tool and its inputSchema (edited)
@task_file n=21 path=packages/escher-driver/src/cli_tests.rs
what: the escher-session command line's argument checks for canvas (edited)
@task_file n=22 path=packages/escher-driver/src/recording_tests.rs
what: the kept-check recording checks, which now read a canvas record (edited)
@task_file n=23 path=packages/escher-driver/src/lib.rs
what: the driver crate's root and public exports, including the canvas types and the doc stating sixteen verbs (edited)
@task_file n=24 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the stand checks' test apps, which gain a canvas fixture with small and large canvases (edited)
@task_file n=25 path=tests/blitz-tests/tests/session_common/mod.rs
what: the stand checks' shared session helpers, which gain the canvas call helpers (edited)
@task_file n=26 path=tests/blitz-tests/tests/stand_actions.rs
what: the stand check of the actions answer: target verbs per node and the Cells answer bound (edited)
@task_file n=27 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the stand check of the cross-OS reading digests, with the actions digests re-read (edited)
@task_file n=28 path=tests/blitz-tests/tests/stand_act_spans.rs
what: the stand check of the command spans per verb call (edited)
@task_file n=29 path=tests/blitz-tests/tests/stand_act_scroll.rs
what: the stand check of the scroll verb, whose verb-count pin moved to sixteen (edited)
@task_file n=30 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: the stand check of the MCP tools: the canvas tool call and its span (edited)
@task_file n=31 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: the stand check that new answers hold no masked or typed value; it gains the canvas answer (edited)
@task_file n=32 path=examples/seven_guis/tests/cli_commands.rs
what: the escher-session command line's process checks, which gain a canvas test host (edited)
@task_file n=33 path=examples/seven_guis/tests/mcp_server.rs
what: the escher-session MCP role's process checks: canvas reads the same through MCP and the command line (edited)
@task_file n=34 path=examples/seven_guis/tests/host_log.rs
what: the session host's log checks: no canvas content in the logs at trace (edited)
@task_file n=35 path=examples/seven_guis/tests/mcp_log.rs
what: the MCP role's log checks: no canvas content in the logs at trace (edited)
@task_file n=36 path=examples/seven_guis/tests/common/mod.rs
what: the seven_guis process checks' shared helpers, which gain the canvas needles (edited)
@task_file n=37 path=scripts/cold-agent.sh
what: the cold-agent pipe, whose verb list now has sixteen verbs; the counted verbs are unchanged (edited)
@task_file n=38 path=.github/scripts/test_cold_agent.py
what: the cold-agent pipe's check, which now takes canvas (edited)
@task_file n=39 path=docs/capabilities.md
what: the project's capability statement: sixteen verbs, the canvas answer, its measured sizes and the unpaged-actions limit (edited)
@task_file n=40 path=README.md
what: the project README, which states the sixteen driver commands as tools (edited)
@task_item n=1 part=touches
The escher-session commands rows (help, and the answer and argument rows that stand beside layout's): help lists sixteen verbs with canvas after layout. canvas has argument and answer rows of its own; none stands yet.
@task_item n=2 part=touches
contract/escher-session-commands/snapshot and the graph answer rows: canvas's full dump now pages the same way, in parts of whole lines within the 10,000-byte budget. The paging convention gains a new reader.
@task_item n=3 part=touches
The escher-session MCP role rows (other-results, help-result, stdout): the MCP tool listing grew from fifteen tools to sixteen, with canvas after layout and help last. The canvas tool's result text is the run's line.
@task_item n=4 part=touches
contract/actions-line/content and contract/escher-session-commands/actions-answer: each listed node carries six target verbs instead of five, with canvas last. Cells' actions answer is now 130,496 bytes, within the 1,048,576-byte bound.
@task_item n=5 part=touches
decision/actions-share-the-verbs-checks: canvas gains its own check_target arm (fonts-unavailable, otherwise free), which actions reuses.
@task_item n=6 part=touches
contract/driver-screenshot-refusals/fonts-unavailable and contract/driver-screenshot/fonts-unavailable: the fonts-unavailable cause now also refuses canvas where the app cannot paint text.
@task_item n=7 part=touches
decision/screenshot-never-skips-text and the test-fonts decisions: canvas likewise refuses rather than read a document whose text cannot be painted.
@task_item n=8 part=touches
The driver-verbs-and-virtual-clock rows (advance, click, press, type) and decision/driver-advance-moves-one-clock: a new canvas read leaves the clock unmoved, with Timer ticks unapplied, and advance 100 reads an empty canvas diff.
@task_item n=9 part=touches
decision/load-wait-moves-no-clock: canvas is among the verbs that wait on nothing.
@task_item n=10 part=touches
decision/circle-drawer-canvas-colours-in-code: Circle Drawer's canvas colours are now readable as rgba values in the canvas dump. They stay widget constants.
@task_item n=11 part=touches
decision/escher-session-agent-read-surface and decision/driver-output-for-agents: canvas answers one plain line for an agent, in closed words, numbers and digests, with no colour.
@task_item n=12 part=touches
The escher-session stdout rows (command-line-line, mcp-line): canvas prints its answer line on both roles.
@task_item n=13 part=touches
The escher-session exit status and exit code rows, and decision/exit-statuses-from-schema-table: canvas exits 0 when answered and 1 when refused. No new exit status, cause or session error was added.
@task_item n=14 part=touches
The web-spa surface overview, which describes a canvas: a canvas element's widget commands are now read by the driver. The WASM page's own canvas is not read.
@task_fact n=1 slug=surface
fact: escher-session commands, canvas arguments: canvas --id <id> [--full] [--part <part>]. id is required and part runs from 1 to 1000. It is malformed without an id, past the id byte bound, with part 0 or 1001, with an argument it does not take, or with full twice.
@task_fact n=2 slug=surface
fact: escher-session commands, canvas answer: by default a summary of the commands counted by eight kinds (Layer, Clip, Mask, Pop, Fill, Stroke, Glyphs, Shadow) plus a digest. With full, one line per command. Two reads of one state are the same bytes.
@task_fact n=3 slug=surface
fact: A canvas full dump over the 10,000-byte answer budget is cut into parts of whole lines. Fonts are listed in part 1 alone, the parts joined equal the whole, and a part past the last is empty.
@task_fact n=4 slug=surface
fact: A canvas answer, MCP result or kept check holds no glyph id, glyph position, text, font bytes or family name. Glyph ids cross only as a digest.
@task_fact n=5 slug=surface
fact: The canvas line form's conventions: colours are written as rgba hex; a Custom paint reads as #00000000; blend= is mix,compose; cap= is one word where both caps agree; dash= is the lengths, @, then the offset; a glyph run with no glyph writes at=none.
@task_fact n=6 slug=surface
fact: Stated limit of the canvas read: commands have no identity, so a changed command reads as one line removed and one added. A layer's filter is written as a flag only.
@task_fact n=7 slug=motion-calibrated-to-expression-level-unrecorded
fact: Driver verbs and the virtual clock, canvas: the read changes nothing (snapshot, focus, diff or clock) and applies no pending Timer tick.
@task_fact n=8 slug=surface
fact: escher-session MCP role, canvas tool: its inputSchema has id (required), full (boolean) and part (integer). Its result is the run's line, and its tool_call span parents one command span that records the verb alone.
@task_item n=1 part=untouched
Brand identity, spacing, border radius and the project type-scale absence: the session sets no token, scale or spacing.
@task_item n=2 part=untouched
Iconography and the cursor rows: no cursor or favicon behaviour moved.
@task_item n=3 part=untouched
The select list rows (events, accessibility, colours, placement, gestures, its screenshot and snapshot rows): no select behaviour moved.
@task_item n=4 part=untouched
The WPT runner output and result colour rows: the WPT runner is not reached.
@task_item n=5 part=untouched
The shell provider window rows, the JS window object rows and the dioxus-native configuration rows: none moved.
@task_item n=6 part=untouched
The scroll, animation and scrollbar motion rows (instant driver scroll, smooth scroll, overlay scrollbar fade, select list motion): canvas moves no scroll and no animation.
@task_item n=7 part=untouched
The colour-scheme recascade, the engine colour constants and the color-token absence: no colour token or scheme handling changed.
@task_item n=8 part=untouched
The escher-session tasks rows (cells, counter, crud, flight-booker, timer, load, order, unknown task): no served task was added, removed or reordered.
