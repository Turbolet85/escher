@task_prose part=session
The session added canvas lint: the lint verb now reads a label a canvas widget paints and names it truncated when its ink box is not inside its region.
A label's ink box is read from each glyph's outline bounds in the run's own font; its region is the widget's content box cut by each pushed clip, layer or mask path's bounding box and by the element's clip region.
The lint answer's written form, its keys and the six-defect set are unchanged; a canvas finding carries the ink box and the visible region as four numbers each, with no label text.
escher-driver takes skrifa from the workspace, already in the graph; the lock moves by one line, no package added; no public symbol, verb, IPC method, port, env var or span is added.
A new stand target proves it on a planted page of numbered cases and on Circle Drawer at four states, in both layout modes; shared Circle Drawer test helpers moved into the shared blitz-tests module.
Measured: Wide label at 20 px in DejaVu Sans reads an ink box 102.02 px wide, so a planted case moved from x 110 to x 90; CI read green 17/17 on three OSes.
@task_file n=1 path=packages/escher-driver/src/canvas_lint.rs
what: new crate-private module of the driver that reads a canvas widget's draw commands into labels, each a glyph run's ink box and the clip region it is painted under
@task_file n=2 path=packages/escher-driver/src/canvas_lint_tests.rs
what: new unit tests of the canvas label reader, a path child test module of the driver
@task_file n=3 path=tests/blitz-tests/tests/stand_canvas_lint.rs
what: new blitz-tests stand target proving the canvas lint on a planted page and on Circle Drawer, in both layout modes
@task_file n=4 path=packages/escher-driver/Cargo.toml
what: escher-driver's manifest, its dependency list; edited to take skrifa from the workspace
@task_file n=5 path=Cargo.lock
what: the workspace lock file; edited by one dependency edge of escher-driver on skrifa
@task_file n=6 path=packages/escher-driver/src/lib.rs
what: the driver crate root and its crate doc; edited to declare the canvas lint module and restate the lint paragraph
@task_file n=7 path=packages/escher-driver/src/lint.rs
what: the lint verb's reader, its defects, reasons and findings behind Session::lint; edited to chain the canvas truncations after the DOM truncated reading
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: the driver's command schema, the verbs' help text served by help and the MCP tool listing; edited for the lint verb's help
@task_file n=9 path=docs/capabilities.md
what: the capability statement; edited in the visual lint section and its witnesses
@task_file n=10 path=tests/blitz-tests/tests/session_common/mod.rs
what: the shared blitz-tests session helpers module; edited to take the Circle Drawer canvas helpers and CIRCLE_CANVAS
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: the canvas draw commands stand target; edited to import the moved Circle Drawer helpers
@task_item n=1 part=touches
contract/lint-verb/truncated: a canvas element now reads truncated once per label whose ink box is not inside its region, beside the DOM text reading.
@task_item n=2 part=touches
contract/lint-verb/finding: one finding per defect read, a canvas's truncated once per label, in paint order after the DOM truncated reading and before contrast.
@task_item n=3 part=touches
contract/driver-verbs/lint and contract/escher-session/lint: the lint verb's help and its findings field's help now name the canvas label and its ink box.
@task_item n=4 part=touches
contract/driver-session/lint: Session::lint's doc names the canvas label and its region under truncated and states four limits beside Geometry only.
@task_item n=5 part=touches
contract/lint-reason-keys/truncation: the truncated reason's text and visible keys now also carry a canvas label's ink box and its region; keys unchanged.
@task_item n=6 part=touches
contract/answer-json/lint-finding and contract/driver-lint/answer: written form and keys unchanged, but a truncated finding may now name a canvas element with its label's box.
@task_item n=7 part=touches
contract/lint-verb/defects: the defect set stays six, unchanged; Truncated's doc and reason text now include a canvas label.
@task_item n=8 part=touches
decision/canvas-crossing: canvas content crossed as draw commands and pixels; a canvas label's ink box now also crosses as four numbers inside a lint finding, with no label text.
@task_item n=9 part=touches
decision/lint-findings-on-carriers and decision/fixed-program-text-on-carriers: the new findings and the changed help ride the existing carriers, no new carrier.
@task_item n=10 part=touches
decision/lint-own-bounds-type: canvas boxes are converted into the lint's own rectangle type in CSS px.
@task_item n=11 part=touches
decision/custom-widget-content-box-paint: a canvas label's region starts from the widget's content box, the box it is painted at.
@task_item n=12 part=touches
contract/test-harness/canvas-scene, contract/driver-session/canvas and contract/canvas-verb/command-kinds: the lint reads the same canvas scene and command kinds (clip, layer and mask pushes and pops, glyph runs) the canvas verb reads.
@task_item n=13 part=touches
decision/driver-spans-only, decision/driver-logs-no-content and decision/escher-driver-spans-ungated: the canvas reading opens no span and logs nothing.
@task_item n=14 part=touches
decision/skrifa-matches-parley and decision/parley-text-layout: escher-driver now takes skrifa directly, the workspace's 0.49, to read glyph outline bounds.
@task_item n=15 part=touches
decision/workspace-dependencies, decision/workspace-shared-dependencies and decision/dependency-and-licence-audit: the new edge is declared from the workspace; audit reads advisories and licences ok.
@task_item n=16 part=touches
name/crate/escher-driver and decision/crate-internal-visibility: the driver gains crate-private modules canvas_lint and its tests; no public symbol added.
@task_item n=17 part=touches
decision/driver-unit-tests-in-path-children and decision/split-test-module-as-child: the canvas lint's unit tests are a path child module.
@task_item n=18 part=touches
decision/blitz-tests-one-target-per-file and decision/stand-repeat-zero-failure-budget: a new stand target joins test and stand-repeat by the glob; stand-repeat's named-target count, seventeen, is unchanged.
@task_item n=19 part=touches
decision/stand-check-shared-helpers and decision/shared-fixtures-stated-once: the Circle Drawer canvas helpers moved into the shared blitz-tests module, renamed constant CIRCLE_CANVAS.
@task_item n=20 part=touches
decision/numbered-fixture-cases, decision/feature-fixture-numbered-cases and decision/scenarios-both-layout-modes: the planted page holds six numbered cases, run in both layout modes.
@task_item n=21 part=touches
decision/capability-statement and contract/capability-statement/sections: the visual lint section and its witnesses gain the canvas lint target.
@task_item n=22 part=touches
decision/measured-limits-stated-in-the-tool and decision/visibility-from-geometry-not-hit: the lint stays geometry only and states its canvas limits in the tool's doc.
@task_item n=23 part=touches
decision/stand-typography-dejavu-sans and decision/stand-readings-same-on-every-ci-os: the canvas label readings are measured in DejaVu Sans and read the same on the three CI OSes.
@task_fact n=1 slug=established-decisions
fact: A canvas label's region is the widget's content box cut by the bounding box of each pushed clip, layer or mask path, mapped through its transform, and by the element's clip region; its ink box is the union of its glyphs' outline bounds; a label reads truncated when its ink box is not inside that region (PROVISIONAL, a delegate's answer).
@task_fact n=2 slug=stack-and-technologies
fact: escher-driver takes skrifa, the workspace's version already in the graph, to read a glyph's outline bounds for the canvas lint (PROVISIONAL, a delegate's answer).
@task_fact n=3 slug=standard-contracts
fact: The lint reads no other check inside a canvas: labels overlapping one another, a label's contrast against the fill under it, a command wholly outside its region and a shape with no area are not read (an absence, PROVISIONAL).
@task_fact n=4 slug=established-decisions
fact: A canvas label's region is never the shape or the virtual node under it, only the content box and the clips; that reading is deferred to Circle Drawer's circles as virtual nodes (PROVISIONAL).
@task_fact n=5 slug=standard-contracts
fact: A glyph run whose font skrifa cannot read adds no canvas label and no finding, and never panics.
@task_item n=1 part=untouched
The session socket's wire, its versions and bounds, and the MCP server's protocol, handshake and revisions are unchanged.
@task_item n=2 part=untouched
Refusal causes and faults, validation order and the per-verb target checks are unchanged; lint takes no argument.
@task_item n=3 part=untouched
The snapshot, diff, actions, graph, layout, screenshot and acting verbs and their answers are unchanged.
@task_item n=4 part=untouched
Engine rows (style, layout, paint, hit testing, select list, accessibility tree, DOM events) are not reached; the lint only reads the scene a widget already paints.
@task_item n=5 part=untouched
Occupied resources: no port, listener, socket, env var, state or recording directory is added.
@task_item n=6 part=untouched
Telemetry install, scrub and line format, and the command and tool_call span fields are unchanged.
@task_item n=7 part=untouched
The browser app, WPT runner, cold-agent and agent-run scripts, and the CI leg set and workflows are unchanged.
@task_item n=8 part=untouched
Project intent, upstream sync and versioning rows are not reached.
