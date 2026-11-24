@task_prose part=chunk
Builds a read of one canvas's content as data: the draw commands a custom widget's paint records into its anyrender Scene, for one canvas element named by its stable element id, on the held instance, running no step and changing nothing.
The read answers at two levels: a summary (a count per command kind and the canvas's size, or a closed digest) and a full level (one line or object per command in paint order, layers as nesting).
It is deterministic and host-independent (same bytes in process, across processes and on the three CI hosts, numbers at a stated rounding), diffable before and after a step, and its size is measured on Circle Drawer's stand canvas against the snapshot's 10,000-byte budget.
A driver verb carries it (a sixteenth verb or an argument of an existing one) on the carriers the founder rules; what crosses each carrier, glyph-run text above all, is written as the founder's forks and the phase stops there.
It re-reads the cross-OS watch on Circle Drawer's canvas shot digest and paints nothing new.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: blitz-paint's paint_scene, where custom widget scenes are built and dropped after the paint — edits: build_custom_widget_scenes, process_custom_widget_node
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the Widget trait of blitz-dom's nodes and widgets, whose paint returns an anyrender Scene
@task_file n=3 path=packages/blitz-test-harness/src/screenshot.rs mark=edits
what: the test harness's screenshot reader, beside which a read of one canvas's scene stands
@task_file n=4 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's command and refusal schema and verb table — edits: the verb or argument
@task_file n=5 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver verb specs of the schema — edits: the verb or argument spec
@task_file n=6 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's validated command form — edits: the verb or argument
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's command execution on the session — edits: the read's execution
@task_file n=8 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's own hand-written JSON answer writer — edits: the read's result shape and its writer
@task_file n=9 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's one self-description for command line and MCP — edits: help
@task_file n=10 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP server's tool listing and tool calls — edits: the MCP tool
@task_file n=11 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording and kept check file — edits: the kept check
@task_file n=12 path=packages/escher-driver/src/replay.rs mark=edits
what: the kept check replay — edits: the kept check
@task_file n=13 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: Circle Drawer's task, the stand's one canvas
@task_file n=14 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand that boots the tasks
@task_file n=15 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables of the blitz-tests shared test module — edits: the stand's shared tables
@task_file n=16 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs mark=re-reads
what: the cross-OS check that Circle Drawer's canvas element shot reads its SAMPLE digest equal on linux x86_64, windows x86_64 and macOS aarch64
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement, where a verb is added
@task_file n=18 path=README.md mark=edits
what: the root README — edits: the count of driver commands an MCP client gets as tools
@task_file n=19 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the check holding the capability statement and README tool count to the driver schema
@task_item n=1 part=needs
The paint seam: custom widget scenes are built inside paint_scene and dropped after the paint; the read needs one widget's recorded Scene without a raster, and the widget paint path, content-box placement, pointer frame, font hand-off and click synthesis stay as they are (no new painting).
@task_item n=2 part=needs
The Widget trait's paint into an anyrender Scene and the commands it records: fill, stroke, glyph run, push layer, push clip layer, push mask layer, pop layer, box shadow.
@task_item n=3 part=needs
A change in the engine's paint crate follows the fork's rule for engine changes: escher's additions live in functions of its own and a direct engine change is flagged upstreamable.
@task_item n=4 part=needs
The read names its canvas by a stable element id, on the held instance, runs no step, moves no clock and changes nothing; an id that is not found, stale or not a canvas is refused by the existing refusal causes and call validation order.
@task_item n=5 part=needs
Two levels of the answer: a summary (a count per command kind and the canvas's size, or a closed digest of the scene) and a full level (one line or object per command in paint order, layers as nesting).
@task_item n=6 part=needs
Determinism: two reads of one state answer the same bytes in process and across processes, and stand readings are identical on every CI OS (linux x86_64, windows x86_64, macOS aarch64); numbers are written at a stated rounding, with no process-local value crossing; glyph positions from shaping are measured across hosts, not assumed.
@task_item n=7 part=needs
Diffability: two reads before and after a step name exactly the lines added, removed or changed, and a step that changes nothing reads an empty diff; the dump's diff is its own, separate from the engine's changed set and the snapshot diff.
@task_item n=8 part=needs
Size: the dump of Circle Drawer's canvas at each level, at boot and after circles are added, stated in bytes against the snapshot text budget of 10,000 bytes; a measured limit is stated where an agent reads the tool, and the answer stays within the MCP result bound and the session socket's answer bound.
@task_item n=9 part=needs
The carrying verb: a sixteenth verb in the driver verb table or an argument of an existing verb, its argument kinds, and the widening of the validated verb set only by ratification.
@task_item n=10 part=needs
The answer is written by the driver's own hand-written JSON writer with its string escaping; serde_json stays a reader only and no serde_json type enters the driver's public API.
@task_item n=11 part=needs
The carriers the founder rules: the command line, the MCP tool listing and tool calls, the session socket's call, and a kept check; a new carrier widens the session boundary, help and the MCP tool share one self-description, and the kept check's kept verbs, bounds and digest govern a kept read.
@task_item n=12 part=needs
What crosses: canvas content today crosses only as pixels; a read of the canvas as data, and what it may carry to a model provider (authored text in glyph runs), is the founder's fork, measured per option.
@task_item n=13 part=needs
Glyph runs carry glyph ids and font data, not text: a dump writing a label as text takes it from elsewhere, and font and image data cross only by reference (a digest or an index), never inline.
@task_item n=14 part=needs
Circle Drawer is not one of the tasks escher-session serves: a verb on the command line, MCP or a kept check reaches no stand canvas until it is served or a fixture serves one, and a fixture state on the carriers is a standing absence; the carrier fork depends on it.
@task_item n=15 part=needs
The harness's read of one canvas's scene beside its screenshot reader, on the harness that turns on blitz-paint's custom-widget feature, with no raster for the read.
@task_item n=16 part=needs
The anyrender serde form: RenderCommand's serde derives behind anyrender's feature and the workspace's anyrender_serialize; a dependency or feature it pulls into the driver's graph is declared once in the workspace, passes the dependency and licence audit, and adds no platform adapter.
@task_item n=17 part=needs
The driver logs no content: the command span carries the verb and its fields but never the dump's commands, text or glyphs, and the driver opens spans and emits no event.
@task_item n=18 part=needs
The capability statement and the README count of driver commands an MCP client gets as tools stay held to the driver schema where a verb is added.
@task_item n=19 part=needs
The stand's one canvas, Circle Drawer, at the stand's pinned viewport, scale, environment and DejaVu Sans typography, offline.
@task_item n=20 part=needs
The cross-OS watch is re-read: Circle Drawer's canvas element shot, its diameter label drawn with glyph origins on whole device pixels, reads its SAMPLE digest equal on the three CI hosts.
@task_standing n=1
convention: each new integration test lives in blitz-tests as one test target per file, one behaviour per file, with a descriptive name, asserting its fixture first and taking shared stand tables stated once in the shared module
home: conventions
@task_standing n=2
convention: each new stand check reads the same on every CI OS, under the stand's pinned environment and the stand-repeat zero-failure budget
home: standard-contracts
@task_standing n=3
convention: each new driver unit test sits in a path child of its module
home: existing-scopes
@task_standing n=4
convention: each new public item and module is documented, with rustdoc warnings as errors, crate internals restricted, and unwrap or expect justified
home: conventions
@task_standing n=5
convention: each change passes the fmt and clippy gates and the cognitive complexity ceiling
home: stack-and-technologies
@task_standing n=6
convention: each touched crate keeps its per-crate line coverage floor
home: inherited-defaults
@task_item n=1 part=not_needed
Canvas lint and the lint verb's defect set, reason keys and findings: the next entry builds canvas lint.
@task_item n=2 part=not_needed
Virtual nodes and the grouping of canvas commands by virtual node, drag, pan, zoom and a canvas keyboard path: later entries build them.
@task_item n=3 part=not_needed
A Canvas 2D context for page scripts, Boa, script globals and the JS runtime: page scripts get no canvas.
@task_item n=4 part=not_needed
The select list, form controls, form submission and password masking: the chunk reads a canvas only.
@task_item n=5 part=not_needed
Layout, style and cascade: Taffy, Stylo, incremental layout, damage bits and the layout reason readers are untouched.
@task_item n=6 part=not_needed
Networking and fetching: blitz-net, the HTTP cache, script fetchers and per-host request caps.
@task_item n=7 part=not_needed
The WPT runner, its result kinds, outputs and CI artifacts.
@task_item n=8 part=not_needed
The browser app's chrome, history, persistence and about pages, and rdme.
@task_item n=9 part=not_needed
The accessibility tree, AccessKit adapters and ARIA state readings: the read adds no node to the snapshot.
@task_item n=10 part=not_needed
The Dioxus version and its 0.8 move, and the upstream sync merge rules: Dioxus stays at 0.7.3 and no take happens here.
@task_item n=11 part=not_needed
Windowed shells and platform entries: winit, Android, iOS, wasm and the window renderer choice.
@task_item n=12 part=not_needed
The agent-run.sh JSON-line events, exit codes and state directory.
