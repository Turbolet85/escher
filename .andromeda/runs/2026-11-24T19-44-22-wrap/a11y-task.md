@task_prose part=session
The session added canvas, a sixteenth driver verb at the instance level after layout, which reads the draw commands an app's widget paints into a canvas element: a summary of counts per command kind, or a full dump of one line per command with digests, paged in parts past the answer budget.
It reaches every carrier: the session socket, the escher-session command line, the MCP tool canvas (after layout, help last) and a kept check; in process it is Session::canvas, and Circle Drawer's canvas is read in both layout modes with a committed table of 17 rows.
No answer holds text, a glyph id, a glyph position, font bytes or a family name; glyph runs cross as counts, sizes, brushes and 64-bit digests; the read changes nothing (snapshot, focus, diff, clock) and is refused fonts-unavailable where the app cannot paint text.
The actions answer now lists canvas as a sixth target verb per node; the verb table reads sixteen verbs, twelve at the instance level; the Cells actions answer grew to 130,496 bytes; the founder's own answers let the canvas-crosses-as-pixels-alone rows be re-stated.
Nothing was retired; no new refusal cause, session error, exit status, node key or argument kind; CI green on linux, windows and macOS.
@task_file n=1 path=packages/escher-driver/src/canvas.rs
what: the driver's canvas reading — the draw commands of one canvas element as closed values, and the in-process canvas read resolved like the layout read
@task_file n=2 path=packages/escher-driver/src/canvas_tests.rs
what: unit checks of the canvas reading — each command and brush kind read as its closed value, fonts and images indexed, glyph ids kept only as a digest
@task_file n=3 path=packages/escher-driver/src/json/canvas.rs
what: the writer of the canvas answer — the summary and the full dump's lines, numbers and digests, with no text and no glyph id
@task_file n=4 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: stand check of Circle Drawer's canvas read in process — its expectation table, the read changing nothing, paged dumps and the refusals
@task_file n=5 path=tests/blitz-tests/tests/stand_canvas_carriers.rs
what: stand check of the canvas verb on the socket, MCP and a kept check, and that no answer holds a glyph id or text
@task_file n=6 path=packages/blitz-paint/src/lib.rs
what: the engine's paint — the scene builder, now also building one custom widget's scene outside a raster
@task_file n=7 path=packages/blitz-test-harness/src/screenshot.rs
what: the harness's screenshot reader, now also handing out one canvas widget's scene at the viewport's scale
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the driver schema — the verb table with its help, arguments and answer fields, now holding the canvas verb
@task_file n=9 path=packages/escher-driver/src/schema/spec.rs
what: the driver schema's field kinds, now with length, command-counts, digest, commands and digests
@task_file n=10 path=packages/escher-driver/src/schema_tests.rs
what: checks of the driver schema and its verb and field-kind counts
@task_file n=11 path=packages/escher-driver/src/command.rs
what: the driver's command parser and validation, now taking the canvas command with id, full and part
@task_file n=12 path=packages/escher-driver/src/command_tests.rs
what: checks of command parsing and its malformed refusals
@task_file n=13 path=packages/escher-driver/src/execute.rs
what: the driver's command execution and its answers — now the canvas summary, whole dump and paged parts, and the target check for canvas
@task_file n=14 path=packages/escher-driver/src/json.rs
what: the driver's answer writer for every verb, now dispatching the canvas answers
@task_file n=15 path=packages/escher-driver/src/json_tests.rs
what: checks of the answer writer, now the canvas lines, summary, dump and parts
@task_file n=16 path=packages/escher-driver/src/explore.rs
what: the driver's explore of verbs by field kind, now leaving out the canvas commands and counts
@task_file n=17 path=packages/escher-driver/src/explore_tests.rs
what: checks of the explore filter by field kind
@task_file n=18 path=packages/escher-driver/src/actions.rs
what: the actions verb — the actionable-node listing with each node's target verbs and causes, now listing canvas
@task_file n=19 path=packages/escher-driver/src/describe.rs
what: the driver's describe of verbs and answers, now describing canvas
@task_file n=20 path=packages/escher-driver/src/mcp.rs
what: the MCP server's tool listing, now sixteen tools with canvas after layout
@task_file n=21 path=packages/escher-driver/src/mcp_tests.rs
what: checks of the MCP tool listing and the canvas tool's input schema
@task_file n=22 path=packages/escher-driver/src/cli_tests.rs
what: checks of the escher-session command line, now the canvas command form
@task_file n=23 path=packages/escher-driver/src/recording_tests.rs
what: checks of kept checks, now reading a canvas record
@task_file n=24 path=packages/escher-driver/src/lib.rs
what: the session library's crate surface and doc, now exporting the canvas reading and naming sixteen verbs
@task_file n=25 path=tests/blitz-tests/tests/fixtures/mod.rs
what: the stand fixtures, now a test page painting a small and a large canvas
@task_file n=26 path=tests/blitz-tests/tests/session_common/mod.rs
what: shared stand helpers building driver calls, now the canvas calls
@task_file n=27 path=tests/blitz-tests/tests/stand_actions.rs
what: stand check of the actions answer, its target verbs and causes and the Cells answer bound
@task_file n=28 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: stand check of readings equal across hosts, its actions digests re-read
@task_file n=29 path=tests/blitz-tests/tests/stand_act_spans.rs
what: stand check of the command and tool-call spans per verb
@task_file n=30 path=tests/blitz-tests/tests/stand_mcp_tools.rs
what: stand check of the MCP tools, now the canvas tool call and its span
@task_file n=31 path=tests/blitz-tests/tests/stand_masks_on_new_answers.rs
what: stand check that new answers hold neither a typed sentinel nor a password mask, now the canvas answer
@task_file n=32 path=tests/blitz-tests/tests/stand_requirement_sweep.rs
what: stand sweep of each verb's requirements, now canvas
@task_file n=33 path=examples/seven_guis/tests/cli_commands.rs
what: seven_guis process check of the command line through the binary, now a canvas host and its one-line answer
@task_file n=34 path=examples/seven_guis/tests/mcp_server.rs
what: seven_guis process check of the MCP server, now a canvas read the same through MCP and the command line
@task_file n=35 path=examples/seven_guis/tests/host_log.rs
what: seven_guis check that the host's log holds no screen content, now no canvas needle
@task_file n=36 path=examples/seven_guis/tests/mcp_log.rs
what: seven_guis check that the MCP server's log holds no screen content, now no canvas needle
@task_file n=37 path=examples/seven_guis/tests/common/mod.rs
what: seven_guis shared test helpers, now the canvas needles
@task_file n=38 path=scripts/cold-agent.sh
what: the cold-agent pipe, its verb list now sixteen
@task_file n=39 path=docs/capabilities.md
what: the capability statement — sixteen verbs, the canvas answer and its size, the unpaged-actions limit
@task_file n=40 path=README.md
what: the project's front page, the sixteen driver commands as tools
@task_item n=1 part=touches
decision/circle-drawer-canvas-pixels-only is stale: Circle Drawer's canvas no longer crosses as pixels alone; its draw commands cross through the canvas verb on every carrier as counts, closed words and digests, never text or glyph ids, on the founder's own answers.
@task_item n=2 part=touches
decision/readers-of-one-snapshot-model must note the canvas read as a reader that is no snapshot reader: it reads the painted scene of one canvas node, not the snapshot model.
@task_item n=3 part=touches
decision/snapshot-model-single-source and decision/snapshot-reads-the-tree: the canvas answer is a second driver reading taken from the paint scene, not from the accessibility tree; check whether the single-source wording still holds.
@task_item n=4 part=touches
contract/actions-answer/causes and contract/actions-answer/fields: each listed node's target verbs grow from five to six with canvas last, carrying cause fonts-unavailable where the app cannot paint text, no cause otherwise.
@task_item n=5 part=touches
contract/actionable-nodes/listing: the listing's per-node verb set now includes canvas; the Cells answer grew to 130,496 bytes for 678 nodes, within its bound.
@task_item n=6 part=touches
decision/actions-verb-changes-nothing and decision/screenshot-changes-nothing gain a sibling: the canvas read changes nothing either (snapshot, both focus readings, diff, clock, Timer ticks unapplied).
@task_item n=7 part=touches
decision/driver-schema-has-no-role-set: the schema gained a verb and five field kinds (seventeen to twenty-two) and still defines no role or name set.
@task_item n=8 part=touches
contract/mcp-result/snapshot and the MCP tool listing: the tool listing is sixteen tools, canvas after layout, help last; a canvas tool result is the run's line.
@task_item n=9 part=touches
decision/password-never-revealed and decision/password-value-masked: the canvas answer was checked to hold neither a typed sentinel nor the mask; the rule now spans the canvas answer.
@task_item n=10 part=touches
decision/stand-expectation-tables and decision/role-name-tables-stated-once: Circle Drawer's canvas expectations are a table of 17 rows stated once and read in both layout modes.
@task_item n=11 part=touches
decision/ui-graph-carries-no-values: the explore filter by field kind now leaves out the canvas commands and counts; explored verbs stay click, type and scroll.
@task_item n=12 part=touches
absence/assistive-technology-reach and decision/accessibility-reach-tier-0: canvas content now reaches an agent through the driver but still no accessibility tree or assistive technology; a reach statement must not read the canvas read as accessibility.
@task_item n=13 part=touches
decision/capability-statement-reach-by-hand: the capability statement was edited by hand for sixteen verbs and the canvas answer's size; check its reach wording is unmoved.
@task_item n=14 part=touches
contract/node-id/reserved-separator: the canvas verb resolves its id like layout and refuses past the id byte limit, not-found and stale as other targeted verbs.
@task_fact n=1 slug=screen-reader-support
fact: The canvas verb reads one canvas element's draw commands, a summary of counts per kind or a full dump of lines, on the socket, command line, MCP and kept checks; it adds nothing to the accessibility tree or the snapshot, where the canvas stays one named node.
@task_fact n=2 slug=screen-reader-support
fact: No canvas answer, MCP result or kept check holds text, a glyph id, a glyph position, font bytes or a family name; a glyph run crosses as count, size, brush and 64-bit digests.
@task_fact n=3 slug=screen-reader-support
fact: The canvas read is refused fonts-unavailable where the app cannot paint text, and the actions answer lists canvas with no cause otherwise.
@task_fact n=4 slug=screen-reader-support
fact: Stated limit: draw commands carry no identity, so a changed command reads as one line removed and one added, and Circle Drawer's circles are no nodes of the snapshot or the tree, until virtual nodes give them one.
@task_item n=1 part=untouched
Keyboard navigation rows (Tab order, focus moves, select and range keys, keyboard activation, Cells and CRUD key contracts): the canvas read moves no focus and fires no event.
@task_item n=2 part=untouched
ARIA role mapping, accessibility tree states, ARIA property reads and hidden-content rows: no role, name, state or hidden rule changed.
@task_item n=3 part=untouched
AccessKit adapter and platform adapter rows, the accessibility feature defaults and tier 0: no adapter or feature changed.
@task_item n=4 part=untouched
Contrast and visual lint rows (contrast thresholds, ratio helper, lint findings, stand accent colour, select list colours): no contrast reading or lint finding changed.
@task_item n=5 part=untouched
decision/option-labels-leave-only-as-pixels and the open select list's row contracts: option labels still leave an open list only as pixels; the canvas read does not reach them.
@task_item n=6 part=untouched
Snapshot node keys, snapshot text form and snapshot diff contracts: no node key, state token or diff entry was added.
@task_item n=7 part=untouched
Cognitive accessibility rows and the a11y CI leg and slow-tier a11y test leg: no gate was added to them.
