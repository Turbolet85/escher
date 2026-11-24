@task_prose part=session
The session merged upstream blitz main at 29a439e05 (6 commits, two-parent merge), the next sync's merge base moving from 165d698c3 to 29a439e05; the standing left-out set stays out, recounted 14 commits, not 15.
It took upstream drag and drop: a DataTransfer model, drag UI events and DOM events (dragenter, dragover, dragleave, drop), engine drag dispatch with a sub-document forward, a windowed OS drop source in the shell, and the Dioxus bridge's drag data with a dropped-file reader that reads files with no path restriction.
It took a flex fix (a br after text joins the text's anonymous flex item), one-axis overflow clip painting only the clipped axis, preventDefault ignored on a non-cancelable event, and Taffy's CalcHandle in the calc resolver.
Taffy moved from the 0.15 git pin to 0.16.0 from crates.io: the workspace's one git dependency is retired and the lock holds no git source; Dioxus stays held at 0.7.3.
Two new blitz-tests targets, drag_and_drop and flex_br, each its own target found by autodiscovery; pointer_leave gained a test that preventDefault does not cancel pointerleave; two windowed root examples, dnd and file_drop, were added with no test.
dioxus-native-dom line coverage fell from 91.14 % to 88.59 % (floor 83 % unchanged, the dropped-file reader's lines uncovered); every CI leg green, CI 17 of 17, no leg list, agent-run verb or CI step edited.
@task_file n=1 path=packages/dioxus-native-dom/src/events.rs mark=conflict
what: the Dioxus bridge's event data converters — the one merge conflict, its import blocks resolved; now also the drag data and the dropped-file reader
@task_file n=2 path=packages/blitz-dom/src/layout_reason.rs
what: the layout reason reader — edited by hand to pass Taffy's CalcHandle to the calc resolver
@task_file n=3 path=Cargo.toml mark=merge
what: the workspace manifest — Taffy dependency moved to 0.16.0 from crates.io, the root package's dev features and the new examples
@task_file n=4 path=Cargo.lock mark=merge
what: the workspace lock — Taffy 0.16.0 from crates.io, the last git source gone
@task_file n=5 path=examples/dnd.rs mark=merge
what: a new windowed root example of drag and drop between areas, printing to stdout
@task_file n=6 path=examples/file_drop.rs mark=merge
what: a new windowed root example listing dropped files and data with previews; file-URL image and HTML-file previews refused
@task_file n=7 path=packages/blitz-dom/src/document.rs mark=merge
what: the engine document — holds the current drag target node
@task_file n=8 path=packages/blitz-dom/src/events/driver.rs mark=merge
what: the engine event driver — drag dispatch, and preventDefault ignored on a non-cancelable event
@task_file n=9 path=packages/blitz-dom/src/events/mod.rs mark=merge
what: the engine event entry — forwards a drag into a sub-document
@task_file n=10 path=packages/blitz-dom/src/layout/construct.rs mark=merge
what: layout box construction — a br after text joins the text's anonymous flex item
@task_file n=11 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine layout entry — adapted to Taffy 0.16's calc handle
@task_file n=12 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: replaced-element layout — adapted to Taffy 0.16
@task_file n=13 path=packages/blitz-dom/src/layout/table.rs mark=merge
what: table layout — adapted to Taffy 0.16
@task_file n=14 path=packages/blitz-dom/src/node/custom_widget.rs mark=merge
what: the custom widget (app widget painted from Rust) — accepts a drop only by setting the dragover drop effect
@task_file n=15 path=packages/blitz-paint/src/render.rs mark=merge
what: the scene painter — one-axis overflow clip clips only the clipped axis
@task_file n=16 path=packages/blitz-shell/Cargo.toml mark=merge
what: the windowed shell's manifest — gains the workspace url dependency
@task_file n=17 path=packages/blitz-shell/src/application.rs mark=merge
what: the windowed shell's application loop — wires the OS drop source
@task_file n=18 path=packages/blitz-shell/src/lib.rs mark=merge
what: the windowed shell's module list — the drop source module
@task_file n=19 path=packages/blitz-shell/src/window.rs mark=merge
what: the windowed shell's window — feeds OS drag position, leave and drop to the document
@task_file n=20 path=packages/blitz-shell/src/dnd.rs mark=merge
what: the windowed OS drop source — turns OS drags into drag UI events; accepts only file URIs as paths
@task_file n=21 path=packages/blitz-traits/src/events.rs mark=merge
what: the shared UI and DOM event types — drag UI events, drag DOM events and the drag event
@task_file n=22 path=packages/blitz-traits/src/events/datatransfer.rs mark=merge
what: the DataTransfer model — dragged items, files, allowed and chosen drop effects
@task_file n=23 path=packages/dioxus-native-dom/src/dioxus_document.rs mark=merge
what: the Dioxus document bridge — the drag arm of its event converter
@task_file n=24 path=packages/stylo_taffy/src/convert.rs mark=merge
what: the style-to-Taffy converter — the calc resolver takes Taffy's CalcHandle
@task_file n=25 path=tests/blitz-tests/tests/all.rs mark=merge
what: the blitz-tests shared module list (no test target of its own) — gains the two new modules
@task_file n=26 path=tests/blitz-tests/tests/pointer_leave.rs mark=merge
what: the blitz-tests pointer-leave target — gains a test that preventDefault does not cancel pointerleave
@task_file n=27 path=tests/blitz-tests/tests/drag_and_drop.rs mark=merge
what: a new blitz-tests target — drag events, drop effects, widget drop acceptance and sub-document drags over the engine's UI event entry, with hand-written handlers
@task_file n=28 path=tests/blitz-tests/tests/flex_br.rs mark=merge
what: a new blitz-tests target — br line breaks in flex and grid containers, read through the harness built from HTML
@task_item n=1 part=touches
One blitz-tests target per file and one integration test target per behaviour: drag_and_drop and flex_br each stand as their own target, found by autodiscovery with no manifest edit.
@task_item n=2 part=touches
The blitz-tests manifest rows (one test target, its difference from upstream): the shared module list stays test = false and gains two mod lines; the manifest itself is unedited.
@task_item n=3 part=touches
Per-crate coverage floors: dioxus-native-dom line coverage moved 91.14 % to 88.59 %, its 83 % floor unchanged and met; a row holding the crate's reading or floor is touched.
@task_item n=4 part=touches
Workspace tested in the fast CI leg, and a file joining a leg keeps its legs: the two new targets run in the workspace test leg and agent-run run all, joining no named leg list.
@task_item n=5 part=touches
The upstream mounted-flush test left out and the upstream buffered-render test kept as written: the left-out set stays as the last sync ruled, recounted 14 commits; no row's word on it changes.
@task_item n=6 part=touches
Example fixtures read by a human / carry no automated assertion, and the absence of an automated test that launches a window: two windowed root examples and the windowed OS drop source were added with no test, the build leg compiling them.
@task_item n=7 part=touches
file: URLs loadable only from a file: document: the drop example refuses file-URL image and HTML-file previews, and the OS drop source turns only file URIs into paths.
@task_item n=8 part=touches
Harness-driven engine tests and blitz-dom fixtures built without the HTML parser: drag_and_drop drives the document's UI event entry directly with its own document helper, flex_br reads heights through the harness built from HTML.
@task_item n=9 part=touches
Hand-written stand-ins, no mocking library: drag_and_drop stands in its own event handler, drop widget and child document by hand.
@task_item n=10 part=touches
Canvas shots compared across hosts by digest and pixel tests on a CPU buffer: the painter's one-axis overflow clip changed; stand_canvas_cross_os green on linux, the Circle Drawer digest watch read green on macOS and Windows CI (1 of 3 runs).
@task_item n=11 part=touches
App widget painted from Rust shows in shots: the custom widget gained drop acceptance through the dragover drop effect.
@task_item n=12 part=touches
Layout assertions in both layout modes and every layout reader word read on a fixture: the layout reason reader and box construction changed (CalcHandle, br in a flex item); flex_br asserts heights in one mode.
@task_item n=13 part=touches
Dependency audit, cargo legs locked and minimum supported Rust version: the lock moved (Taffy 0.16.0 from crates.io, no git source left); audit, msrv and locked legs green.
@task_item n=14 part=touches
Regression tests named for and documenting their bug: the upstream tests arrive named by behaviour (br_between_text_breaks_the_line_in_a_flex_container, prevent_default_does_not_cancel_pointerleave).
@task_item n=15 part=touches
Stand checks repeated three rounds with a zero-failure budget, and the native platform CI matrix: stand-repeat 3 of 3 green; CI 17 of 17, the Windows job running drag_and_drop and flex_br.
@task_fact n=1 slug=test-strategy
fact: Drag and drop is tested headlessly through the document's UI event entry (the drag_and_drop target); the windowed OS drop source has no test of its own, unrunnable headless.
@task_fact n=2 slug=quality-gates-coverage-targets
fact: The Dioxus bridge's dropped-file reader has no test: no harness input reaches a dropped file, so its lines read uncovered in dioxus-native-dom's coverage.
@task_fact n=3 slug=ci-integration
fact: The blitz-tests targets drag_and_drop and flex_br run in the workspace test leg and agent-run run all, in no named leg list.
@task_fact n=4 slug=integration-test-strategy
fact: preventDefault on a non-cancelable event is ignored, pinned by upstream's pointer-leave test that preventDefault does not cancel pointerleave.
@task_fact n=5 slug=e2e-test-strategy
fact: The windowed root examples dnd and file_drop are compiled by the build leg and run by no automated test.
@task_fact n=6 slug=test-scope-summary
fact: The blitz-tests target names drag_and_drop and flex_br are occupied, upstream's, each its own target.
@task_item n=1 part=untouched
Session lifecycle, session wire, session host, session errors and session recording contracts: no session verb, wire message or bound changed.
@task_item n=2 part=untouched
The driver's verbs, refusal causes, snapshot, actions and graph answers, and the escher-session command line, exit codes and stderr: unchanged.
@task_item n=3 part=untouched
MCP server and MCP role contracts, tools list and protocol revision: unchanged.
@task_item n=4 part=untouched
Cold-agent runs, their verdict, events and isolation: untouched.
@task_item n=5 part=untouched
Agent-run verbs, selections, events, exit codes and state: no verb or status shape edited.
@task_item n=6 part=untouched
WPT runner, its CLI, outputs, statuses and pinned commit: untouched.
@task_item n=7 part=untouched
Lint defects and findings, handler coverage limits and the coverage line: unchanged.
@task_item n=8 part=untouched
Mutation testing, complexity ceiling, CI action pinning, secrets, permissions, cache and package installs: no CI step edited.
@task_item n=9 part=untouched
Recorded checks, their replay and committed-check content, and the virtual clock and advance rows: untouched.
