@task_prose part=chunk
Builds a read of one canvas's content as data: the draw commands a widget's paint records into its scene (fill, stroke, glyph run, push layer, push clip layer, push mask layer, pop layer, box shadow), for one canvas element named by its stable element id, on the held instance, running no step and changing nothing.
The read answers at two levels: a summary (a count per command kind and the canvas's size, or a closed digest) and a full level (one line or object per command in paint order, layers as nesting).
It is deterministic and hardware-independent (same bytes in process, across processes and on the linux, windows and macOS CI hosts, numbers at a stated rounding) and diffable (a no-op step reads an empty diff).
A driver verb carries it (a sixteenth verb or an argument of an existing one) on the carriers the founder rules; what crosses to a model provider, and each carrier, is a founder fork written to evidence before the phase stops.
It measures the dump of Circle Drawer's canvas on the stand at each level, at boot and after circles are added, in bytes against the snapshot's 10,000-byte budget.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the paint path that builds each custom widget's scene inside the scene paint and drops it after — edits: paint_scene, build_custom_widget_scenes, process_custom_widget_node
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget component — the Widget trait whose paint answers a recorded scene
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs mark=edits
what: the headless test harness surface — a read of one canvas's scene beside the screenshot
@task_file n=4 path=packages/blitz-test-harness/src/screenshot.rs
what: the harness screenshot, the read the new canvas read stands beside
@task_file n=5 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's verb table — the new verb or argument
@task_file n=6 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver's verb specification — the new verb or argument and its result shape
@task_file n=7 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command parsing — the new verb or argument
@task_file n=8 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's verb execution on the held instance — the canvas read
@task_file n=9 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's result writer — the canvas read's summary and full result shapes
@task_file n=10 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's help answer — the new verb or argument
@task_file n=11 path=packages/escher-driver/src/mcp.rs mark=edits
what: the driver's MCP server — the MCP tool carrying the canvas read
@task_file n=12 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording — the kept check of the canvas read
@task_file n=13 path=packages/escher-driver/src/replay.rs mark=edits
what: the recording replay — the kept check of the canvas read
@task_file n=14 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the Circle Drawer task, the stand's one canvas whose dump is measured
@task_file n=15 path=examples/seven_guis/src/stand.rs
what: the seven-task stand that boots Circle Drawer's canvas
@task_file n=16 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables for the stand checks
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement — the new verb where one is added
@task_file n=18 path=README.md mark=edits
what: the README's tool count — where a verb is added
@task_file n=19 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the check holding the README's tool count against the capability statement
@task_item n=1 part=needs
A new driver verb, the verb table's sixteenth, or an argument of an existing verb, answering one canvas's draw commands for a stable element id on the held instance, running no step and changing nothing; its command form, help answer, result shape and MCP tools list entry.
@task_item n=2 part=needs
The read's two levels: a summary level (a count per command kind and the canvas's size, or a closed digest) and a full level (one entry per command in paint order, layers as nesting).
@task_item n=3 part=needs
Determinism of the read: two reads of one state answer the same bytes in process, across processes and on the linux x86_64, windows x86_64 and macOS aarch64 hosts, numbers written at a stated rounding; font and image data written only by reference (a digest or an index), never inline.
@task_item n=4 part=needs
Diffability: two reads before and after a step name exactly what changed, and a step that changes nothing reads an empty diff.
@task_item n=5 part=needs
Size: the dump of Circle Drawer's canvas at each level, at boot and after circles are added, measured in bytes against the snapshot answer's 10,000-byte budget and the MCP server's result bound.
@task_item n=6 part=needs
The widget's recorded scene read without a raster from the paint path where custom widget scenes are built and dropped; the paint path, placement, pointer frame, font hand-off and click synthesis stand as built (no new painting).
@task_item n=7 part=needs
Circle Drawer's circles painted on a canvas: the stand's one canvas, addressed by its stable element id circle-canvas, is the read's target, with circles added through its existing controls.
@task_item n=8 part=needs
Glyph runs carry glyph ids and font data, not text: whether a label crosses as text, as glyph ids or not at all is the founder's fork, against the rule that caller input is never written back and the cold agent's model-provider path.
@task_item n=9 part=needs
The carriers of the read — the command line, the MCP server over local stdio, the session recording's kept check and its replay — each the founder's fork; Circle Drawer is not among the escher-session served tasks, so a carrier reaches no stand canvas until it is served or a fixture serves one.
@task_item n=10 part=needs
The headless stand's pinned viewport and scale, at which the canvas's float coordinates are written and compared across hosts; glyph positions from shaping may differ by host and are measured, not assumed.
@task_item n=11 part=needs
The held instance reads as a fresh boot and session roles share the stand boot: the read at boot equals a fresh boot's read.
@task_item n=12 part=needs
The read's refusals and exit codes: an unknown verb, an id that names no canvas, fonts unavailable, through the escher-session refusals and exit codes.
@task_item n=13 part=needs
Session recording as a replayable check: the canvas read, where carried, recorded as a check file and replayed equal.
@task_item n=14 part=needs
The capability statement and the README's tool count follow a new verb, held by the capability statement check.
@task_item n=15 part=needs
The cross-OS canvas shot check of Circle Drawer's canvas (glyph origins on whole device pixels, one SAMPLE digest on the three hosts) is watched; the read's cross-host equality is proved the same way.
@task_standing n=1
convention: each new driver verb or argument joins the verb table, the help answer, the MCP tools list and the verb specification's kept check together
home: surface-cli
@task_standing n=2
convention: each new answer or log line writes no caller input back
home: surface-cli
@task_standing n=3
convention: each new stand check reaches a stand control by its stable element id, read from attributes alone
home: surface-desktop-native
@task_standing n=4
convention: each new stand check runs on the headless stand at its pinned viewport and boots it as a fresh boot
home: surface-desktop-native
@task_item n=1 part=not_needed
ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): the read changes no window.
@task_item n=2 part=not_needed
The agent-run test contract and the cold-agent run pipe scripts (usage, exits, run log, record dir, Windows pass-through): the chunk adds no script and changes no pipe.
@task_item n=3 part=not_needed
The multi_doc and paint_bench command lines: no benchmark or multi-document run is touched.
@task_item n=4 part=not_needed
The load page's network provider, the order page's controls, the flight type select's boot reading, the CRUD list keyboard region and open list rows' bounds: other stand tasks than the canvas.
@task_item n=5 part=not_needed
Into-view scroll across holding scrollers and the visible region through clipping boxes: the read scrolls nothing and reports commands, not visibility.
@task_item n=6 part=not_needed
Stand accent colour and the fixed document skeleton with its main mount: no stand styling or document structure changes.
@task_item n=7 part=not_needed
Arbitrary index.html templates for a document: no document template is added.
@task_item n=8 part=not_needed
The escher-session graph, layout, lint and coverage answers: the chunk adds no canvas lint and no grouping by virtual node, and changes none of those answers.
@task_item n=9 part=not_needed
The escher-session host log's service name and the MCP protocol revisions and end: unchanged by a new tool.
