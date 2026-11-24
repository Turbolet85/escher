@task_prose part=chunk
Builds a read of one canvas element's content as data: the draw commands a widget's paint records into its scene (fill, stroke, glyph run, push layer, push clip layer, push mask layer, pop layer, box shadow), for one canvas named by its stable element id, on the held instance, running no step and changing nothing.
Two levels: a summary (a count per command kind and the canvas's size, or a closed digest of the scene) and a full level (one line or object per command in paint order, layers as nesting).
Deterministic, independent of hardware and diffable: one state reads the same bytes in and across processes and on the three CI hosts; a step that changes nothing reads an empty diff.
Carried by a driver verb (a sixteenth verb or an argument of an existing one) on carriers the founder rules; every carrier crossing and what may reach a model provider is written as a founder's fork, and the phase stops there with no default taken.
Measures the dump's size for Circle Drawer's canvas at each level, at boot and after circles are added, against the snapshot's 10,000-byte budget; re-reads CI on the last wrap's commit before P5.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the paint of the document into a scene, where the custom widgets' scenes are built and dropped after the paint — edits: build_custom_widget_scenes, process_custom_widget_node, paint_scene
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the Widget trait, a custom widget's paint recording a scene
@task_file n=3 path=packages/blitz-test-harness/src mark=edits
what: the test harness's reads of a held instance — edits: a read of one canvas's scene, beside screenshot
@task_file n=4 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver's verb table and exit statuses — edits: the verb table entry or argument
@task_file n=5 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver schema's spec of each verb, its arguments and result shape — edits: the verb or argument entry
@task_file n=6 path=packages/escher-driver/src/command.rs mark=edits
what: the driver's command parsing of verbs and arguments — edits: the verb or argument
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver's execution of a verb on the held instance — edits: the canvas read's execution
@task_file n=8 path=packages/escher-driver/src/json.rs mark=edits
what: the driver's JSON writer of verb results — edits: the canvas read's result writer
@task_file n=9 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver's help text for each verb — edits: help
@task_file n=10 path=packages/escher-driver/src/mcp.rs mark=edits
what: the escher-session mcp role, its tools and tool results — edits: the MCP tool
@task_file n=11 path=packages/escher-driver/src/recording.rs mark=edits
what: the driver's recording of a session's verbs into a kept check — edits: the kept check
@task_file n=12 path=packages/escher-driver/src/replay.rs mark=edits
what: the driver's replay of a recorded kept check — edits: the kept check
@task_file n=13 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: Circle Drawer, the stand's one canvas, a custom widget drawing circles and a diameter label
@task_file n=14 path=examples/seven_guis/src/stand.rs
what: the seven GUIs stand that hosts the tasks for the checks
@task_file n=15 path=tests/blitz-tests/tests mark=edits
what: the engine's integration checks — edits: the canvas read's checks
@task_file n=16 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables used by the checks — edits: the stand's shared tables
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement listing the driver's verbs
@task_file n=18 path=README.md mark=edits
what: the project README — edits: the tool count
@task_file n=19 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the check that the README's tool count and capability statement match the driver's verbs
@task_item n=1 part=needs
A new driver read of a canvas's draw commands, as a sixteenth verb or an argument of an existing one, naming one canvas element by its stable element id; it runs no step, moves no virtual clock and changes nothing on the held instance.
@task_item n=2 part=needs
The read's two levels and their result shapes: a summary of counts per command kind and the canvas's size (or a closed digest), and a full level of one entry per command in paint order with layers as nesting.
@task_item n=3 part=needs
Determinism: two reads of one state answer the same bytes in process, across processes and on linux x86_64, windows x86_64 and macOS aarch64; every number written at a stated rounding.
@task_item n=4 part=needs
Diffability: reads before and after a step compare line by line to name what was added, removed or changed; a step that changes nothing reads an empty diff.
@task_item n=5 part=needs
The dump's size for Circle Drawer's canvas at each level, at boot and after circles are added, stated in bytes against the snapshot's 10,000-byte budget.
@task_item n=6 part=needs
Font data and image data in a command cross only by reference (a digest or an index), never inline.
@task_item n=7 part=needs
Glyph runs carry glyph ids and font data, not text: whether a label is written as text, as glyph ids or not at all, and what authored text may reach a model provider, is the founder's fork.
@task_item n=8 part=needs
The read takes a widget's recorded scene without a raster, while the paint itself stays as built: the widget paint path, placement, pointer frame, font hand-off and click synthesis do not change, and no new painting is added.
@task_item n=9 part=needs
The test harness gains a read of one canvas's scene beside its screenshot read, which the checks use.
@task_item n=10 part=needs
The verb's carriers — the command line, the MCP tool, help, the recording and replay of a kept check — each a founder's fork, its crossing written with the rule it meets.
@task_item n=11 part=needs
Circle Drawer is not a task escher-session serves, so a carrier reaches no stand canvas until it is served or a fixture serves one; this dependence is written into the carrier forks.
@task_item n=12 part=needs
The verb's answers and refusals on the command line and MCP follow the driver's existing forms: exit statuses read from the schema's table, the session error object with its remedy, the stdout line and MCP result forms, output read by agents with no colour.
@task_item n=13 part=needs
Where a verb is added, the capability statement and the README's tool count are updated and their check passes.
@task_item n=14 part=needs
Glyph positions from shaping may differ by host (the macOS glyph-origin snap of the canvas label drawn on whole device pixels): cross-host sameness of the dump is measured, not assumed; tests are built with system fonts.
@task_item n=15 part=needs
The watch on Circle Drawer's canvas element shot reading one digest across the three CI hosts in its cross-OS check stays green; CI on the last wrap's commit is re-read before P5.
@task_item n=16 part=needs
Dioxus stays at 0.7.3; no dependency upgrade rides this chunk.
@task_item n=17 part=needs
Every fork is written to the work dir's founder-forks evidence with what crosses, the rule it meets, each option's measured cost, its dependences and the most conservative option that still builds the entry, then the phase stops; no raw log with a temp or home path enters evidence; a probe worktree stands outside target and is never deleted by a plan step; a guard refusal of a write is a stop.
@task_standing n=1
convention: each new check is built with system fonts and never paints or reads with text skipped
home: typography
@task_standing n=2
convention: each new driver verb or argument has its help entry, its MCP tool result form, its exit statuses from the schema's table and its session error with a remedy, with no colour on its output
home: surface
@task_standing n=3
convention: each new command-line or MCP output line follows the escher-session stdout forms and is read by agents
home: surface-cli
@task_item n=1 part=not_needed
The open select list rows: its events, its accessibility entries, its row bounds in the snapshot, its overlay placement and colours; the chunk touches no select.
@task_item n=2 part=not_needed
Scrollbars, smooth scroll, wheel cancel and into-view scroll; the chunk adds no scroll, drag, pan or zoom.
@task_item n=3 part=not_needed
Cursor icons, cursor resolution and favicons.
@task_item n=4 part=not_needed
The WPT runner's output colours and test-name hyperlinks.
@task_item n=5 part=not_needed
The shell provider's window controls: minimize, maximize, decorations toggle and window drag.
@task_item n=6 part=not_needed
The JS window object's sizes and device pixel ratio, and the script runtime's DOM; no Canvas 2D context is given to page scripts.
@task_item n=7 part=not_needed
Presentational hints of body margins, table cell spacing and font elements, the lang attribute and root-relative units; no styling or layout changes.
@task_item n=8 part=not_needed
Mobile targets and the WASM web pages surface; the read runs on the native held instance.
@task_item n=9 part=not_needed
The color-scheme recascade and the engine colour tokens; the chunk adds no colour.
