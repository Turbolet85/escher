@task_prose part=session
Circle Drawer's circles and their diameter labels are now painted from Rust on one canvas widget element (an object element, role group, named Drawing area) in place of the div circles and their CSS, which are retired; the circles cross to checks only through a data-circles attribute and the canvas's shot pixels, leaving the DOM, snapshot, tree, UI graph and lint.
An engine change in blitz-dom behind custom-widget: a click on a custom-widget element now reaches the app's onclick, and two clicks with no advance its ondoubleclick, the widget still hearing every pointer event first; guarded by the new check stand_canvas_click in both layout modes.
A new in-process stand boot for Circle Drawer, served by no escher-session role, and a new check stand_circle_drawer_canvas (boot role, name, keyed, lint clean; click paints a circle and its diameter; undo and redo repaint; two clicks open the dialog), both layout modes.
The cross-OS canvas digest check gains a second table SAMPLE (2 rows, both layout modes) for Circle Drawer's canvas element shot, 5 to 6 tests; a macOS red measured that the CPU renderer snaps glyph origins differently on Apple hosts, fixed by drawing the label with glyph origins on whole device pixels, digest 0x5d5664142f9a5201 equal on linux, windows and macOS (CI green 17/17).
Measured: the first centre click reads 348,198,40, not the predicted 350,200,40, because the canvas's client rect is its border box and the widget paints in its content box (2 px border subtracted); the label's dark-pixel literal re-pinned 40 to 41; the capability statement gains Circle Drawer, the widget click, two witnesses and two stated limits, no pinned count moved.
The overseer's answers (role group, the diameter as canvas text, data-circles, the blitz-dom dependency, the glyph snap, the stated limit Canvas text across hosts) are provisional; the engine click synthesis is the founder's word.
@task_file n=1 path=tests/blitz-tests/tests/stand_canvas_click.rs
what: new stand check, its own test target: a click on a custom-widget canvas element reaches the app's click and double-click handlers in both layout modes, the widget hearing the pointer events first
@task_file n=2 path=tests/blitz-tests/tests/stand_circle_drawer_canvas.rs
what: new stand check, its own test target: Circle Drawer's canvas boots named, keyed and lint clean, a click paints a circle and its diameter, undo and redo repaint, two clicks open the dialog; reads PNG pixels of the canvas shot, a dark-pixel literal pinned at 41
@task_file n=3 path=packages/blitz-dom/src/events/pointer.rs
what: blitz-dom pointer event handling, the engine: now synthesises click, context menu and double click on a custom-widget target after the widget handles the event
@task_file n=4 path=packages/blitz-dom/src/events/mod.rs
what: blitz-dom DOM event dispatch, its custom-widget arm now calls the click synthesis
@task_file n=5 path=examples/seven_guis/src/tasks/circle_drawer.rs
what: the seven_guis Circle Drawer task, a stand sample app: its canvas widget painting circles and diameter labels with glyph origins on whole device pixels, the data-circles attribute and its parse, click points read against the canvas content box, with an inline unit test module
@task_file n=6 path=examples/seven_guis/Cargo.toml
what: the seven_guis sample's manifest, its dependencies gaining four workspace crates
@task_file n=7 path=Cargo.lock
what: the workspace lock file, the seven_guis entry gaining four dependency lines, no package added
@task_file n=8 path=examples/seven_guis/src/stand.rs
what: the seven_guis headless stand and its boots, gaining the Circle Drawer boot and its in-shell unit test
@task_file n=9 path=tests/blitz-tests/tests/stand_canvas_cross_os.rs
what: the cross-OS canvas shot check comparing canvas element shots by digest across hosts, gaining the SAMPLE table for Circle Drawer's canvas, 6 tests
@task_file n=10 path=tests/blitz-tests/tests/stand_non_lean_tasks.rs
what: the stand check of the non-lean tasks' measured and clickable readings per task, Circle Drawer re-pinned 1 to 0 for each
@task_file n=11 path=docs/capabilities.md
what: the capability statement, its claims naming their checks and its stated limits, gaining Circle Drawer's canvas, the widget click, two witnesses and two stated limits
@task_file n=12 path=.andromeda/db/log/2026-11-23T17-32-06-2026-11-23-circle-drawer-on-a-canvas.ndjson
what: the process journal of this chunk, harness metadata of the workflow, one line committed
@task_item n=1 part=touches
decision/canvas-shots-cross-os-by-digest: its table gains Circle Drawer's canvas element shot (SAMPLE, 2 rows, both layout modes) read by digest, one value on linux, windows and macOS; the expected amendment of this chunk.
@task_item n=2 part=touches
The rows making canvas shots the one exception to no shot in cross-OS byte comparisons (cross-os-checks-hold-no-shot, no-shot-in-cross-os-byte-checks, no-screenshot-in-cross-os-checks): the exception now holds a canvas painting text, byte-equal only because the text's glyph origins sit on whole device pixels.
@task_item n=3 part=touches
decision/cross-os-readings-by-answer-digest and decision/shot-png-deterministic-per-host: a canvas shot with text differed on macOS until the glyph-origin snap; determinism across hosts now rests on that snap.
@task_item n=4 part=touches
decision/app-widget-shows-in-shots: a second app widget painted from Rust (Circle Drawer's canvas, circles and diameter labels) now shows in shots.
@task_item n=5 part=touches
The seven_guis headless stand boot contracts (contract/stand-boot/boot and its siblings): a new in-process boot for Circle Drawer beside them, served by no escher-session role.
@task_item n=6 part=touches
decision/two-clicks-without-advance-double-click: now holds on a custom-widget element too, through the engine click synthesis, guarded by stand_canvas_click.
@task_item n=7 part=touches
Handler coverage's canvas limit (decision/handler-coverage-canvas-uncounted, contract/handler-coverage-limits/canvas-handler): Circle Drawer's click and double-click handlers now stand on a canvas element.
@task_item n=8 part=touches
contract/snapshot-node/clickable and the one actionable rule (decision/one-actionable-rule): the canvas object with role group reads clickable at boot while the non-lean clickable reading for Circle Drawer moved 1 to 0.
@task_item n=9 part=touches
The capability statement rows (capability-claims-name-their-checks, capability-check-names-held-to-schema, capabilities-doc-pinned-to-schema, capability-statement-pinned, capability-statement-from-schema): two new witnesses and two stated limits added, no pinned count moved.
@task_item n=10 part=touches
One test target per file or behaviour (blitz-tests-one-target-per-file, one-test-target-per-file, one-test-target-per-behaviour) and blitz-tests-single-test-target: two new stand check files, each its own target.
@task_item n=11 part=touches
Shared test modules (session-checks-share-one-module, shared-fixtures-in-one-module, shared-test-modules-not-targets): both new stand files declare the session module and not the fixtures module.
@task_item n=12 part=touches
Stand checks in both layout modes (stand-checks-in-both-layout-modes, layout-tests-in-both-layout-modes): each new check runs both modes.
@task_item n=13 part=touches
decision/requirement-witness-inside-stand-repeat and decision/stand-repeat-three-rounds: requirement v040-09 is witnessed by the new canvas checks inside stand-repeat, 3 of 3 rounds green.
@task_item n=14 part=touches
decision/expected-value-pinned-as-literal: the SAMPLE digest and the label's dark-pixel count (40 to 41) re-pinned as literals from the linux reading.
@task_item n=15 part=touches
Pixel reading in checks (pixel-tests-on-cpu-buffer, pixel-tests-cpu-buffer, png-decoding-in-checks): the Circle Drawer canvas check decodes the shot and reads ring, disc and label pixels.
@task_item n=16 part=touches
Assertion failure text (call-assertions-carry-row-index, assertions-print-no-argument-value): the engine check's assertions carry a mode and an index only.
@task_item n=17 part=touches
decision/mutation-controlled-tests: red taken before green for the engine click check and the held-back data-circles carriage.
@task_item n=18 part=touches
Stand checks on the bundled font (stand-checks-bundled-font, stand-checks-bundled-font-no-skip, stand-pinned-real-font, font-metric-tests-never-skip): the canvas label is now text painted by the widget with the handed fonts.
@task_item n=19 part=touches
decision/graph-drives-confirm-every-write and decision/stable-tree-serialization: Circle Drawer's circles leave the graph, tree and snapshot, readable only through the data-circles attribute and shot pixels.
@task_item n=20 part=touches
Pointer placement in the harness (contract/harness-inspect/center-of, contract/harness-inspect/layout-rect, contract/harness-input/pointer-coordinates): a click at an element's centre is read against the client rect, the border box, while a widget paints in its content box, off by border and padding.
@task_item n=21 part=touches
decision/dependency-audit-advisories-and-licenses and decision/cargo-legs-locked: the seven_guis sample gains four workspace dependencies in the lock, audit and locked legs green.
@task_item n=22 part=touches
decision/recorded-dispatch-bypasses-document: the document's custom-widget dispatch arm changed, now synthesising clicks for widget targets.
@task_fact n=1 slug=test-strategy
fact: Text an app paints on a canvas is byte-equal across hosts only with its glyph origins on whole device pixels, the CPU renderer snapping glyph origins differently on Apple hosts; the cross-OS canvas table so holds Circle Drawer's canvas with its diameter label, one digest on linux, windows and macOS in both layout modes (provisional, the overseer's word).
@task_fact n=2 slug=session-lifecycle
fact: A click on a custom-widget element reaches the app's click handler and two clicks with no advance its double-click handler, in both layout modes, the widget hearing every pointer event first; guard stand_canvas_click.
@task_fact n=3 slug=3-test-harness-contract
fact: The seven_guis headless stand boots Circle Drawer in process alone, served by no escher-session role.
@task_fact n=4 slug=integration-test-strategy
fact: Circle Drawer's circles are no DOM nodes: checks read them through the canvas's data-circles attribute (x,y,d per circle, semicolon-joined, malformed text read as no circle) and its shot pixels, never through snapshot, tree, graph or lint (provisional).
@task_fact n=5 slug=integration-test-strategy
fact: A canvas check reads a click point against the canvas's content box, not its client rect: the first centre click on Circle Drawer's 700 by 400 border-box canvas reads 348,198,40.
@task_item n=1 part=untouched
Driver verbs, refusal causes and their order, session errors, exit codes and the session wire: no verb, argument, carrier, field, cause, session error or exit was added.
@task_item n=2 part=untouched
MCP server and role contracts, tools list and tool results: unchanged.
@task_item n=3 part=untouched
Agent-run and cold-agent contracts, events, verdicts and isolation: unchanged.
@task_item n=4 part=untouched
Logging and telemetry rows (stand log, command and tool_call spans, stderr lines): no log field, span or log site added.
@task_item n=5 part=untouched
Session recording, recorded checks and their replay, session state directory and idle expiry: unchanged.
@task_item n=6 part=untouched
Virtual clock, advance, timer ticks and settle rows: unchanged.
@task_item n=7 part=untouched
WPT runner, its suites, expectations and statuses: unchanged.
@task_item n=8 part=untouched
CI workflow configuration (actions pinning, permissions, caches, package installs, triggers, leg membership): no harness or gate surface changed.
@task_item n=9 part=untouched
Coverage floors, mutation testing and complexity ceilings: no threshold moved.
@task_item n=10 part=untouched
Typed password handling and select list painting: unchanged.
