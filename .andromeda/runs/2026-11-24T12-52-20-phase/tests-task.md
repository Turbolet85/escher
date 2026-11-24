@task_prose part=chunk
Builds a read of one canvas's content as data: the draw commands an app widget's paint records into its scene (fill, stroke, glyph run, push layer, push clip layer, push mask layer, pop layer, box shadow), for one canvas element named by its stable element id, on the held instance, running no step and changing nothing.
The read answers at two levels, a summary (a count per command kind and the canvas's size, or a closed digest) and a full level (one entry per command in paint order, layers as nesting), deterministic across processes and the three CI hosts, and diffable before and after a step.
A driver verb carries it (a sixteenth verb of the verb table or an argument of an existing one), on the carriers the founder rules; what may cross to a model provider is the founder's fork.
It measures the dump's size for Circle Drawer's canvas on the stand at each level, at boot and after circles are added, against the snapshot's 10,000-byte budget.
It adds no new painting, no canvas lint and no virtual nodes; Dioxus stays at 0.7.3.
@task_file n=1 path=packages/blitz-paint/src/lib.rs mark=edits
what: the scene painter, which builds each app widget's scene inside the paint of the scene and drops it after the paint — edits: build_custom_widget_scenes, process_custom_widget_node
@task_file n=2 path=packages/blitz-dom/src/node/custom_widget.rs
what: the app widget trait, whose paint returns a recorded scene
@task_file n=3 path=packages/blitz-test-harness/src/ mark=edits
what: the headless test harness — edits: a read of one canvas's scene, beside the harness screenshot
@task_file n=4 path=packages/escher-driver/src/schema.rs mark=edits
what: the driver schema, the verb table — edits: the new verb or argument
@task_file n=5 path=packages/escher-driver/src/schema/spec.rs mark=edits
what: the driver verb specs, each verb's arguments and level — edits: the new verb's or argument's spec
@task_file n=6 path=packages/escher-driver/src/command.rs mark=edits
what: the driver command, a parsed verb and its arguments — edits: the new verb or argument
@task_file n=7 path=packages/escher-driver/src/execute.rs mark=edits
what: the driver executor, running a verb against the held instance — edits: the canvas read
@task_file n=8 path=packages/escher-driver/src/json.rs mark=edits
what: the driver JSON answers, the writer of each verb's result — edits: the canvas read's result shape and its writer
@task_file n=9 path=packages/escher-driver/src/describe.rs mark=edits
what: the driver help text — edits: help for the new verb or argument
@task_file n=10 path=packages/escher-driver/src/mcp.rs mark=edits
what: the MCP server role, its tools list and tool results — edits: the MCP tool for the canvas read
@task_file n=11 path=packages/escher-driver/src/recording.rs mark=edits
what: the session recording, the kept check written from a session — edits: the canvas read in a kept check
@task_file n=12 path=packages/escher-driver/src/replay.rs mark=edits
what: check replay, a kept check replayed through the ordinary call path — edits: the canvas read in replay
@task_file n=13 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: Circle Drawer, the stand's one canvas, its widget painted from Rust
@task_file n=14 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand, its boot of Circle Drawer
@task_file n=15 path=tests/blitz-tests/tests/ mark=edits
what: the blitz-tests integration test targets — edits: the canvas read's checks
@task_file n=16 path=tests/blitz-tests/tests/common/mod.rs mark=edits
what: the stand's shared tables, the one shared test module — edits: the stand's shared tables
@task_file n=17 path=docs/capabilities.md mark=edits
what: the capability statement pinned to the driver schema — edits: the claim of the new verb where a verb is added
@task_file n=18 path=README.md mark=edits
what: the README's tool count — edits: the tool count where a verb is added
@task_file n=19 path=.github/scripts/test_capability_statement.py mark=re-reads
what: the check holding the README's tool count and the capability statement to the driver schema
@task_item n=1 part=needs
The canvas read: one canvas element by its stable element id, on the held instance, answering the eight draw command kinds its widget recorded; it runs no step, settles nothing and changes nothing of the instance.
@task_item n=2 part=needs
The two levels of the answer: a summary (a count per command kind and the canvas's size, or a closed digest of the scene) and a full level (one line or object per command in paint order, layers as nesting).
@task_item n=3 part=needs
Determinism: two reads of one state answer the same bytes in process and across processes, and on the linux x86_64, windows x86_64 and macOS aarch64 CI hosts; numbers are written at a stated rounding; the cross-host comparison is a byte or digest comparison holding no shot.
@task_item n=4 part=needs
Diffability: a read before and after a step names exactly the lines added, removed or changed, and a step that changes nothing reads an empty diff; the no-change control must carry pending work to be a control.
@task_item n=5 part=needs
Size measured: the dump of Circle Drawer's canvas on the stand, at each level, at boot and after circles are added, stated in bytes against the snapshot's 10,000-byte text budget.
@task_item n=6 part=needs
The driver verb: a sixteenth verb in the verb table or an argument of an existing verb, with its lookup, level check, result shape, JSON writer, help text, MCP tool in the tools list and its order, and its place in a recorded kept check and its replay.
@task_item n=7 part=needs
The carriers of the read (the session command line, the MCP role, a recording) are the founder's fork; Circle Drawer is not yet served by the session binary, so a carrier reaches the stand canvas only once it is served or a fixture serves one.
@task_item n=8 part=needs
What crosses: a glyph run carries glyph ids and font data, not text, and a dump of either still carries authored text in some form; what the read may carry to a model provider is the founder's fork, so the session's rules that it writes and logs nothing of the screen and returns no typed value bind the answer and its log lines.
@task_item n=9 part=needs
Font data and image data in a command cross only by reference (a digest or an index), never inline, so the answer stays host-independent and within the size budget.
@task_item n=10 part=needs
The widget's recorded scene is read without a raster: the painter builds widget scenes during the paint and drops them after, so the read needs the recorded scene with no screenshot taken and no font-unavailable refusal from a raster it never makes.
@task_item n=11 part=needs
The harness read of one canvas's scene stands beside the harness screenshot, on a harness-built instance; it is held to the harness contract's construction and inspection helpers.
@task_item n=12 part=needs
Refusals of the read: an id that names no element or a stale one, a malformed argument, and an element that is no canvas each refuse with a cause from the fixed cause set in its order, change nothing and echo no input.
@task_item n=13 part=needs
Glyph positions from text shaping may differ by host (the macOS glyph-origin snap); the read's cross-host sameness is measured, not assumed, and the canvas label's glyph origins on whole device pixels are what the cross-OS canvas shot check already watches.
@task_item n=14 part=needs
The capability statement and the README's tool count are held to the driver schema: a new verb adds its claim, names its checks and moves the count and claim section names that the capability check reads.
@task_item n=15 part=needs
The stand's boot of Circle Drawer and its shared stand tables are what the read's checks drive, adding circles by the stand's existing click path; the widget paint path, placement, pointer frame, font hand-off and click synthesis stand unchanged.
@task_item n=16 part=needs
Evidence of the chunk's forks holds no raw log with a temporary or home path, and a probe worktree stands outside the build directory and is not deleted by a plan step.
@task_standing n=1
convention: each new integration check file is its own test target in the blitz-tests manifest, and a shared module is no target
home: test-strategy · 2-test-strategy
@task_standing n=2
convention: each new stand check runs on the bundled font with no skip, in both layout modes where it reads layout, and witnesses its requirement inside stand-repeat
home: test-strategy · 2-test-strategy
@task_standing n=3
convention: each new fixture or stand table is test data stated in code in the one shared module, nothing seeded
home: 3-test-harness-contract
@task_standing n=4
convention: each new test pins its expected value as a literal, prints no argument value on failure, counts its rows where it is table-driven, and is controlled red before green
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=5
convention: each new unit test sits in its source file's inline test module
home: unit-test-strategy · 4-unit-test-strategy
@task_standing n=6
convention: each new session or driver check reads no wall clock and sleeps not, and owns any host it starts
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=7
convention: each new log line of the driver holds fixed words and counts only, no ids, names or screen content
home: integration-test-strategy · 5-integration-test-strategy
@task_standing n=8
convention: each new stand check is repeated three rounds under the zero-failure budget
home: ci-integration · 9-ci-integration
@task_standing n=9
convention: each new driver or harness unit stays within the per-crate coverage floor and the cognitive-complexity ceiling
home: quality-gates-coverage-targets · 10-quality-gates-coverage-targets
@task_item n=1 part=not_needed
WPT conformance: the runner, its suites, statuses, expectations and pinned commit; the chunk changes no CSS or layout behaviour.
@task_item n=2 part=not_needed
The cold-agent pipe and the agent-run harness: their verbs, events, exit codes, verdict and live run; the chunk adds no cold-agent run.
@task_item n=3 part=not_needed
Lint defects and lint findings order: canvas lint is the next entry, not this chunk.
@task_item n=4 part=not_needed
Virtual element ids and grouping of commands by virtual node: a later entry builds them.
@task_item n=5 part=not_needed
The virtual clock, advance and timer ticks: the chunk changes no time behaviour.
@task_item n=6 part=not_needed
Handler coverage and its limits: the chunk adds no handler and changes no counting of handlers.
@task_item n=7 part=not_needed
Typed password masking, select list colours and painting, scrollbar thumbs and fade, box-shadow expectations from Chromium: the chunk adds no new painting.
@task_item n=8 part=not_needed
Network stand-ins, load waits, failed loads and the load page stand-in: the read loads nothing.
@task_item n=9 part=not_needed
CI package installs, the skia archive, Rust caches and SHA-pinned actions: the chunk changes no CI infrastructure.
@task_item n=10 part=not_needed
The Dioxus bridge's buffered render and the upstream tests kept or left out with it: Dioxus stays at 0.7.3.
@task_item n=11 part=not_needed
The accessibility graph verb, its owner keys and task-state registration: the read neither changes nor answers the UI graph.
