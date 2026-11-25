@task_prose part=session
The session added a canvas lint: the lint verb now reads a canvas label whose ink box is not inside its region as the truncated defect, once per label, in paint order.
A label's region is the element's content box cut by each pushed clip, layer or mask path's bounding box and by the element's clip region; the ink box is read from the glyph outlines of the run's own font.
The defect set stays six and the lint answer's form and keys are unchanged; a canvas finding carries the label's ink box and the visible box as numbers and no label text.
A new stand test target proves it on a planted page of six numbered cases and on Circle Drawer at boot, after a click and at 12 and 100 circles, in both layout modes; six unit cases cover the region reader.
Circle Drawer's shared stand helpers moved into the shared session-check module; the driver takes skrifa, already in the graph, as one lock edge.
A measured premise: a 20 px label in DejaVu Sans reads an ink box about 102 px wide, so a planted case was placed at x 90, not x 110; CI read green 17/17 and the Circle Drawer canvas digest watch met its retire condition.
@task_file n=1 path=packages/escher-driver/src/canvas_lint.rs
what: added — the canvas label reader: each glyph run's ink box and the clip region it is painted under, read from a canvas widget's recorded paint commands
@task_file n=2 path=packages/escher-driver/src/canvas_lint_tests.rs
what: added — the canvas label reader's unit tests, a child test module in its own file: clip push and pop, nested pushes, a push's transform, layer and mask paths, a pop on an empty stack, an unreadable font
@task_file n=3 path=tests/blitz-tests/tests/stand_canvas_lint.rs
what: added — the canvas lint's integration test target: a planted page of numbered label cases with a literal findings table, the lint verb's answer, and Circle Drawer reading no canvas finding at every state, in both layout modes
@task_file n=4 path=packages/escher-driver/Cargo.toml
what: edited — the driver crate's manifest, its dependency list now taking skrifa from the workspace
@task_file n=5 path=Cargo.lock
what: edited — the workspace lock file, one line added to the driver's dependency list
@task_file n=6 path=packages/escher-driver/src/lib.rs
what: edited — the driver crate root: the crate doc's lint paragraph and the private canvas lint module declaration
@task_file n=7 path=packages/escher-driver/src/lint.rs
what: edited — the lint reader behind Session::lint: the six defects, the truncated reading now chaining canvas label truncations after the DOM reading, the defect and reason doc comments
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: edited — the driver verb schema: the lint verb's help and its findings field's help text, served by help and the MCP tool listing
@task_file n=9 path=docs/capabilities.md
what: edited — the capability statement: the visual lint section and where each claim is proven, gaining the canvas lint witness
@task_file n=10 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited — the shared session-check module, no test target: now holding Circle Drawer's shared stand helpers and the circle canvas id
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: edited — the canvas commands stand test target, now importing Circle Drawer's helpers and canvas id from the shared session-check module
@task_item n=1 part=touches
The lint defects contract's truncated item: truncated is now also read for a canvas label whose ink box is not inside its region, once per label, with the ink box and the visible box as its numbers.
@task_item n=2 part=touches
The lint findings order contract: a canvas element's truncated findings come once per label in paint order, after the DOM truncated reading and before contrast.
@task_item n=3 part=touches
The lint defect set and its six-word pin stay unchanged; a row holding the six defects or their order is read, not moved.
@task_item n=4 part=touches
The escher-session command line lint item and the driver verb table: the lint verb's help text now names the canvas label and its ink box.
@task_item n=5 part=touches
Rules that the lint and session write nothing of the screen and that answers hold fixed words and numbers only: a canvas finding crosses as four numbers per box, no label text or glyph id, asserted by the new target.
@task_item n=6 part=touches
One integration test target per file or behaviour: a new stand target joins the blitz-tests targets, picked up by the existing glob.
@task_item n=7 part=touches
Shared stand tables and session checks in one module, and shared test modules being no target: Circle Drawer's helpers moved there from the canvas commands target, bodies unchanged.
@task_item n=8 part=touches
Unit tests in an inline module in their source file: the canvas label reader's unit tests sit in a child test module in a separate file, a convention exception unless a split-module row already allows it.
@task_item n=9 part=touches
Stand checks and layout assertions in both layout modes: each new stand test runs in both modes.
@task_item n=10 part=touches
Stand checks on the bundled real font, never skipping: the canvas ink box is measured from DejaVu Sans glyph outlines at 20 px.
@task_item n=11 part=touches
Expected value pinned as a literal and table-driven tests counting their rows: the planted page's findings table holds five rows, and Circle Drawer's glyph runs are pinned at 0, 1, 12 and 100.
@task_item n=12 part=touches
Tests controlled red-before-green: six controls held for the new target, the clipped-box case moved to x 90 so its control reads red.
@task_item n=13 part=touches
Capability claims naming their checks and the capability statement held to the schema: the visual lint section gains the new witness.
@task_item n=14 part=touches
Every requirement witnessed inside stand-repeat and its three rounds: the new target runs inside stand-repeat by the glob, its named-target count of seventeen unchanged.
@task_item n=15 part=touches
Canvas shots compared across hosts by digest: the Circle Drawer canvas digest watch read its third green run on Windows and macOS and met its retire condition.
@task_item n=16 part=touches
The canvas commands table across hosts: the canvas commands target now reads its canvas id and helpers from the shared module, its assertions unchanged.
@task_item n=17 part=touches
Dependency audit of advisories and licenses: the driver takes skrifa, already in the graph, one lock edge, audit green.
@task_item n=18 part=touches
Doc leg gating public doc comment edits: the truncated defect, its reason text, Session::lint and the crate doc's lint paragraph changed, the doc leg green.
@task_item n=19 part=touches
Per-crate coverage floors: the driver crate gains a new private module with its unit tests.
@task_item n=20 part=touches
Handler coverage leaving canvas handlers uncounted: unchanged, but a row describing what the lint or coverage reads of a canvas may need to sit beside the canvas lint.
@task_fact n=1 slug=integration-test-strategy
fact: The canvas lint is proven on a planted page of numbered label cases, each past an edge, a clip, the ascent, inside, inside a clipped box or after a pop, and on Circle Drawer reading no canvas finding at boot, after a click and at 12 and 100 circles, in both layout modes (PROVISIONAL, delegate's answer).
@task_item n=1 part=untouched
WPT conformance, its runner, statuses, expectations and pinned commit.
@task_item n=2 part=untouched
The cold-agent pipe and the agent-run contract, their events, verbs, exit codes and verdicts.
@task_item n=3 part=untouched
The session wire, socket, state directory, recording and check replay: no IPC method, endpoint, field or bound moved.
@task_item n=4 part=untouched
Driver refusal causes, session errors and exit codes of escher-session: none added or reordered.
@task_item n=5 part=untouched
The MCP protocol revision, protocol errors and the refused and screenshot tool result forms.
@task_item n=6 part=untouched
The virtual clock, advance, settle and timer driving, and the hand-written stand-ins with no mocking library.
@task_item n=7 part=untouched
CI job structure, the leg script, triggers, caches, pinned actions and package installs: the new target joins existing legs by glob.
@task_item n=8 part=untouched
Snapshot text budget, screenshot and shot PNG rows, and committed recorded checks.
@task_item n=9 part=untouched
Mutation testing, complexity ceilings and oversize-file rows.
