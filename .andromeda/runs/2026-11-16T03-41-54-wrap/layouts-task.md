@task_prose part=session
The session added a read-only layout read: the engine now answers, for any element, why it sits where it does at its size — what placed it (Block, Flex, Grid, Table, Inline, Float, Absolute, Fixed, Root, Select, SelectList), any scroll or transform shift, and the cause of each size (Style, Min, Max, FlexGrow, FlexShrink, Stretch, Grid, Table, Replaced, Content, Layout).
The driver session gained an in-process layout read by stable element id, answering string ids, closed words and numbers; it refuses an unread id not-found and a deleted row's id stale, and runs no step, pump, tick or settle, opens no span and emits no event.
The layout read reaches no carrier: no verb, MCP tool, socket call, command-line argument or kept check was added; the verb list still holds fourteen, and this absence of every carrier is PROVISIONAL pending the founder's word.
An element the engine cannot explain reads as placed by Block with both causes Layout in process; a closed word of its own is owed before the read crosses any carrier. A reserved app placer (a canvas name) is spelled but built by unit tests alone.
Measured on the stand's six served screens in both layout modes: every snapshot node answers with its snapshot bounds and sits at its container plus its offset to 1/64 px inclusive; cell-A0 reads Grid with height Style and width Grid; crud-person-0 reads Flex with width Stretch; after 12 creates and a scroll the last crud row reads a scroll shift held by crud-list; counter-value reads width cause Min; a fixed box reads container html.
Nothing in any document's behaviour changed; escher-driver line coverage measured 97.99 %.
@task_file n=1 path=packages/blitz-dom/src/layout_reason.rs
what: the engine's layout reader on every document — why an element is placed where it is and why it has its size, with its closed placing and size-cause words
@task_file n=2 path=packages/blitz-dom/src/lib.rs
what: the engine crate's root and its public re-exports
@task_file n=3 path=packages/dioxus-native-dom/src/lib.rs
what: the native DOM crate's root, re-exporting the engine's node id, bounding rect and layout-reason types
@task_file n=4 path=packages/escher-driver/src/execute.rs
what: the driver's run path — the verb arms and the stable-id resolution that refuses stale and not-found ids
@task_file n=5 path=packages/escher-driver/src/layout.rs
what: the driver session's in-process layout read by stable element id and its answer types
@task_file n=6 path=packages/escher-driver/src/layout_tests.rs
what: the unit tests of the driver session's layout read
@task_file n=7 path=packages/escher-driver/src/lib.rs
what: the driver crate's root and its public surface
@task_file n=8 path=tests/blitz-tests/tests/layout_reason.rs
what: the engine check that reads every placing word and every size cause on a fixture, in both layout modes
@task_file n=9 path=tests/blitz-tests/tests/stand_layout.rs
what: the stand check that reads the layout of every snapshot node on the six served screens, in both layout modes, and proves the read changes nothing
@task_file n=10 path=docs/capabilities.md
what: the capability statement — what the session can and cannot do, its verbs and its absences
@task_item n=1 part=touches
The escher-session MCP server's tool list: the layout read adds no MCP tool; a row on the tools list is unmoved but now stands beside a capability deliberately left off it (PROVISIONAL).
@task_item n=2 part=touches
The escher-session help answer and command form: the verb list still holds fourteen and gains no layout verb; any row naming the verb set or its count stands, beside a new in-process read no verb reaches.
@task_item n=3 part=touches
The cold agent drives through the driver's MCP server: the cold-agent bar cannot count the layout read while it stays in process.
@task_item n=4 part=touches
Session recording as a replayable check: no kept check key reaches the layout read; a recorded check cannot replay it.
@task_item n=5 part=touches
Stand stable element ids — cell-{A..Z}{0..25}, cells-grid, crud-list, crud-person-{person.id}, counter-value: the new stand check pins layout readings on these ids (cell-A0 Grid, crud-person-0 Flex with width Stretch, the last crud row's scroll shift held by crud-list, counter-value width cause Min from its min-width 60px).
@task_item n=6 part=touches
Stand ids as attributes alone and every stand control keyed: the layout read resolves an element by its stable id and answers holders and containers by stable id, reading none where a container has no stable id.
@task_item n=7 part=touches
Open list rows read their own bounds: the layout read leaves an open list open, and an open list's rows are placed by SelectList.
@task_item n=8 part=touches
Flight type select reads its two options at boot: a closed select's options, hidden by the default sheet, read as placed by Select at their select's box; a closed select's optgroup reads no answer.
@task_item n=9 part=touches
Into-view scroll moves every holding scroller and visible region through clipping boxes: the layout read reports a scroll shift per holding scroller and a transform shift, and reports bounds unmoved by its own holder's scroll.
@task_item n=10 part=touches
Headless stand pinned viewport and session roles share the stand boot: the new stand check boots the six served screens through the shared stand boot at the pinned viewport, in both layout modes.
@task_item n=11 part=touches
escher-session served tasks (cells, counter, crud, flight-booker, load, timer): each served screen's every snapshot node now answers a layout read with its snapshot bounds, its offset chain holding to 1/64 px inclusive.
@task_item n=12 part=touches
Held instance reads as a fresh boot: the layout read changes nothing — no pending change, the snapshot, focus, diff and clock unchanged, delivered timer ticks left unapplied.
@task_item n=13 part=touches
Caller input never written back: the layout read answers ids, closed words and numbers only, no text or value.
@task_item n=14 part=touches
Fixed document skeleton with main mount: a fixed box reads its container as html, and the root reads Root.
@task_fact n=1 slug=surface-cli
fact: The layout read is in process only: no verb, MCP tool, socket call, command-line argument or kept check carries it — a PROVISIONAL absence, pending the founder's word.
@task_fact n=2 slug=surface-desktop-native
fact: A closed select's options read as placed by Select at their select's box, and an open list's rows as placed by SelectList; a closed select's optgroup reads no layout answer.
@task_fact n=3 slug=surface-desktop-native
fact: An element the engine cannot explain reads in process as placed by Block with both size causes Layout; it owes a closed word of its own before the read crosses any carrier.
@task_item n=1 part=untouched
ShellProvider window chrome controls: no window drag, close, decoration, maximize or minimize behaviour moved.
@task_item n=2 part=untouched
The agent-run.sh and cold-agent.sh command contracts — usage, stdout, exit codes, log and record directories, Windows pass-through — are unmoved.
@task_item n=3 part=untouched
The multi_doc and paint_bench command lines — arguments, environment switches, backend, stdout — are unmoved.
@task_item n=4 part=untouched
escher-session exit codes, refusals fonts-unavailable, off-screen and unknown-verb, and the coverage role, graph, lint, screenshot, snapshot and actions answers are unmoved: no answer field, cause or exit code was added.
@task_item n=5 part=untouched
escher-session recording (check files, record-dir-not-private, recording-failed) and the host log's service name are unmoved.
@task_item n=6 part=untouched
Stand accent colour, stand names, roles and states by attributes alone, stand lints clean at boot, and the load page's network provider are unmoved: no stand markup or style changed.
@task_item n=7 part=untouched
The absence of arbitrary index.html templates for a document stands.
@task_item n=8 part=untouched
The mobile-native and web-spa surfaces and the decisions log are not reached.
