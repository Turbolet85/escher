@task_prose part=chunk
Builds a read of one canvas element's content as data: the draw commands an app's widget paint records into its scene (fill, stroke, glyph run, push layer, push clip layer, push mask layer, pop layer, box shadow), for one canvas named by its stable element id, on the held instance, running no step and changing nothing.
Two levels: a summary (a count per command kind and the canvas's size, or a closed digest of the scene) and a full level (one entry per command in paint order, layers as nesting).
Deterministic and host-independent (same bytes in and across processes and on linux x86_64, windows x86_64 and macOS aarch64), and diffable before and after a step, a step that changes nothing reading an empty diff.
Carried by a driver verb (a sixteenth verb or an argument of an existing one) on the carriers the founder rules; what crosses (authored text in glyph runs) is the founder's fork, written to the fork evidence and stopped at.
Re-measures the dump's size of Circle Drawer's canvas on the stand at each level, at boot and after circles are added, against the snapshot's 10,000-byte budget.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the paint path that builds each custom widget's recorded scene inside the scene paint and drops it after — edits: paint_scene, build_custom_widget_scenes, process_custom_widget_node
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the custom widget component — the Widget trait whose paint returns a scene
@task_file n=3 path=packages/blitz-test-harness/src/lib.rs mark=edits
what: the test harness's readers — edits: a read of one canvas's scene, beside the screenshot
@task_file n=4 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema, its verb table and result shapes — edits: the new verb or argument and its result shape
@task_file n=5 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver schema's spec of verbs — edits: the new verb or argument
@task_file n=6 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command parsing — edits: the new verb or argument
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's command execution on the held instance — edits: the canvas read's execution
@task_file n=8 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's result writer for the carriers — edits: the canvas read's writer
@task_file n=9 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's help text — edits: help for the new verb or argument
@task_file n=10 path=packages/escher-driver/src/mcp.rs mark=edits
what: the driver's MCP tool results — edits: the MCP tool for the canvas read
@task_file n=11 path=packages/escher-driver/src/recording.rs mark=edits
what: the driver's recording of commands — edits: the new verb in the recording
@task_file n=12 path=packages/escher-driver/src/replay.rs mark=edits
what: the driver's replay of a recording — edits: the new verb in replay and the kept check
@task_file n=13 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: Circle Drawer's drawing area, the stand's one named canvas node
@task_file n=14 path=examples/seven_guis/src/stand.rs
what: the seven_guis stand that hosts the one canvas
@task_file n=15 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared expectation tables — edits: the canvas read's expected values
@task_file n=16 path=docs/capabilities.md mark=edits
what: the capability statement — edits: the new verb, where a verb is added
@task_file n=17 path=README.md mark=edits
what: the README — edits: the tool count, where a verb is added
@task_file n=18 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the check that the README's tool count and the capability statement agree with the driver
@task_item n=1 part=needs
The new read changes nothing: like the screenshot and the actions verb, it runs no step on the held instance and leaves the document and the engine's changed set as they were.
@task_item n=2 part=needs
Circle Drawer's drawing area stays one named canvas node in the accessibility tree and the snapshot; the read reaches its content as data without adding virtual nodes or children to that node.
@task_item n=3 part=needs
The canvas is named by its stable element id as the snapshot serves it (the author id and the served node id form); the read's refusal causes for a missing or non-canvas element follow the driver refusal answers.
@task_item n=4 part=needs
Glyph runs carry the authored text of the canvas's labels in another form (glyph ids, font data): what crosses to a model provider on each carrier is the founder's fork, met against the rules that redact text and values on the carriers and never reveal password text.
@task_item n=5 part=needs
The read's result goes out on the carriers (command line, MCP tool result, recording and replay) in the keyed wire form the other driver results take; the driver schema still defines no role or name set.
@task_item n=6 part=needs
The read's diff before and after a step names exactly what changed and reads empty when nothing changed, alongside the snapshot diff's entries and differs.
@task_item n=7 part=needs
The read is a reader beside the snapshot readers: whether it stands behind the accessibility feature as the snapshot readers do, while the session library always enables accessibility.
@task_item n=8 part=needs
Capability statement and README tool count edited by hand where a verb is added, with accessibility reach left as stated (tier 0).
@task_item n=9 part=needs
Size measured against the snapshot's 10,000-byte budget at boot and after circles are added, at each level.
@task_item n=10 part=needs
Determinism across the three CI hosts: glyph positions from shaping may differ by host (the macOS glyph-origin snap the canvas element shot's cross-OS digest watches), so numbers are written at a stated rounding and measured, not assumed.
@task_item n=11 part=needs
The canvas's existing paint path, placement, pointer frame, font hand-off and click synthesis at the node's centre stay as built; the scene is read without a raster.
@task_item n=12 part=needs
The canvas element shot of the driver's screenshot keeps its digest equal across the three hosts (the folded watch).
@task_standing n=1
convention: each new stand check states its expected values once in the stand's shared expectation tables
home: screen-reader-support · 7-screen-reader-support
@task_standing n=2
convention: each new stand check is repeated three rounds
home: ci-integration · 9-ci-integration
@task_standing n=3
convention: each new accessibility-facing check is a Rust test over the harness, run in the slow-tier accessibility leg that gates CI
home: 3-a11y-assertion-harness-contract
@task_item n=1 part=not_needed
Keyboard navigation: no canvas keyboard path, no Tab order, focus, activation or arrow-key behaviour is added or changed.
@task_item n=2 part=not_needed
Select lists, list boxes, range inputs, the Cells grid and the CRUD list: no control of these kinds is touched.
@task_item n=3 part=not_needed
Contrast and the visual lint: no canvas lint and no contrast measure is built (the next entry builds the canvas lint).
@task_item n=4 part=not_needed
ARIA role mapping, ARIA states and property reads, accessible-name precedence and hidden-content exclusion: no role, state, name or hidden rule changes.
@task_item n=5 part=not_needed
Cognitive accessibility: no invalid-state signalling or timeout is touched.
@task_item n=6 part=not_needed
Drag and drop, pan and zoom: no drag, pan or zoom is added.
