@task_prose part=session
The session added a canvas lint: the lint verb now reads, inside each canvas, every label (glyph run) its widget paints and names it truncated when its ink box is not inside its region.
A label's region is the canvas element's content box, cut by each pushed clip, layer or mask path's bounding box and by the element's clip region; the ink box is read from the glyph outlines of the run's own font.
A canvas finding reports four-number boxes alone in truncated's text and visible fields; no label text or glyph id crosses, the lint answer's keys and its six-defect set are unchanged, and no verb, span, endpoint or env var is added.
Circle Drawer was measured to read no canvas finding at boot, after a click, and at 12 and 100 circles, in both layout modes; a planted page of six numbered cases proves the reading.
The driver takes skrifa (already in the graph) for glyph outline bounds; the Circle Drawer test helpers moved into the shared session test helpers; nothing was retired.
Four lint-like canvas checks were left out and unowned: labels overlapping one another, a label's contrast against the fill under it, a command wholly outside its region, and a shape with no area.
@task_file n=1 path=packages/escher-driver/src/canvas_lint.rs
what: new - the driver's canvas label reader: each glyph run a canvas paints, its ink box and the clip box in force over it
@task_file n=2 path=packages/escher-driver/src/canvas_lint_tests.rs
what: new - unit checks of the canvas label reader: clips narrowing and popping, nested pushes, layer and mask paths, an unreadable font
@task_file n=3 path=tests/blitz-tests/tests/stand_canvas_lint.rs
what: new - the stand check proving the canvas lint on a planted page of numbered cases and on Circle Drawer at four states, in both layout modes
@task_file n=4 path=packages/escher-driver/Cargo.toml
what: edited - the driver crate's dependency list, now taking skrifa
@task_file n=5 path=Cargo.lock
what: edited - the workspace lock, one added skrifa edge under the driver
@task_file n=6 path=packages/escher-driver/src/lib.rs
what: edited - the driver crate root: module list and the crate doc's lint paragraph
@task_file n=7 path=packages/escher-driver/src/lint.rs
what: edited - the visual lint: its defect readings, now chaining canvas label truncations after the DOM truncated reading
@task_file n=8 path=packages/escher-driver/src/schema.rs
what: edited - the verb schema: the lint verb's help and its findings field's help served by help and the MCP tool listing
@task_file n=9 path=docs/capabilities.md
what: edited - the capability statement: the visual lint section and where each claim is proven
@task_file n=10 path=tests/blitz-tests/tests/session_common/mod.rs
what: edited - the shared session test helpers, now holding the Circle Drawer boot, drawing and canvas-reading helpers and the circle canvas id constant
@task_file n=11 path=tests/blitz-tests/tests/stand_canvas_commands.rs
what: edited - the stand check of canvas draw commands, now importing the moved Circle Drawer helpers
@task_item n=1 part=touches
decision/circle-drawer-circles-painted-on-canvas: where it says Circle Drawer's painted labels reach no lint, it is now stale - the labels reach the lint as truncated ink boxes, still with no label text.
@task_item n=2 part=touches
decision/stand-lints-clean-at-boot: the stand's clean lint now covers canvas labels; Circle Drawer reads no finding at boot, after a click, and at 12 and 100 circles, in both layout modes.
@task_item n=3 part=touches
decision/visible-region-through-clipping-boxes: the visible-region rule gains a canvas counterpart - a canvas label's region is the content box cut by each pushed clip, layer or mask path's box and by the element's clip region.
@task_item n=4 part=touches
contract/escher-session-lint/answer: a finding can now name a canvas element truncated once per label past its region, in paint order, its text and visible fields carrying the label's ink box and region; keys and the six-defect set are unchanged.
@task_item n=5 part=touches
contract/escher-session-help/answer: the lint verb's help and its findings field's help now name the canvas label and its ink box.
@task_item n=6 part=touches
contract/escher-session-mcp/tools: the lint tool's description in the MCP tool listing carries the same changed help text.
@task_item n=7 part=touches
contract/stand-element-ids/circle-canvas: the circle-canvas id is now the element the canvas lint reads Circle Drawer's labels under, and the shared test helpers name it as a constant.
@task_item n=8 part=touches
decision/no-caller-input-echoed: indirectly, through the lint answer - a canvas finding crosses only four-number boxes, no label text or glyph id, asserted by the stand check.
@task_item n=1 part=untouched
The ShellProvider window chrome contracts: no window control, decoration or maximize behaviour moved.
@task_item n=2 part=untouched
Stand element ids other than circle-canvas: no id was added, renamed or freed, and no stand page or control changed.
@task_item n=3 part=untouched
The stand's look and skeleton rows - accent colour, document skeleton and main mount, pinned headless viewport, attribute-only ids and a11y: no stand markup or style changed.
@task_item n=4 part=untouched
The command-line scripts and binaries - agent-run, cold-agent, multi_doc, paint_bench - and their usage, stdout, exit codes and Windows pass-through: unchanged.
@task_item n=5 part=untouched
escher-session exit codes, refusals, recording, served tasks, the MCP protocol revisions, stdio, result bound, screenshot result and canvas tool: no verb, refusal, tool or bound was added or moved.
@task_item n=6 part=untouched
The snapshot, layout, graph, coverage, screenshot and actions answers: their forms are unchanged; only the lint reads canvas labels.
@task_item n=7 part=untouched
The mobile-native and web-spa surfaces and the Decisions Log: not reached.
@task_item n=8 part=untouched
Scroll-into-view, open list rows, held instance, flight type select, order and load page rows: no behaviour of those stand tasks changed.
