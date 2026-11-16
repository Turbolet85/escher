@task_prose part=session
The session took seven upstream engine commits in one patch: reversed scroll origins measured on the layout engine's scrollable overflow, hit testing of nodes that have no computed styles yet, and five bumps of the Taffy layout engine pin from e90751fd to a3997d55.
Behaviour change for every document, remote pages included: a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) now scrolls to negative offsets; scroll width and height answer the CSSOM scroll size, reversed origins included.
The scroll state is now a triple (current, min, max); programmatic scroll clamps between min and max; the into-view scroll reads the triple and skips a box whose min and max are both zero.
A hit stopped at a clipping box now answers in its content-box frame, so only an inline root's answer moves; the node hit, whether it is text and whether a hit happens are unchanged.
Escher's own bounds, hit and into-view readings are held; one public read-only accessor for a node's layout data is added; the test tally moved 1501 to 1511 and the per-package agent run 845 to 851; no IPC method, verb, log field, span or id added; nothing retired.
@task_file n=1 path=Cargo.toml
what: the workspace manifest — the engine dependency pins, here the Taffy layout engine git rev
@task_file n=2 path=Cargo.lock
what: the dependency lock — the Taffy layout engine's one git source line
@task_file n=3 path=packages/blitz-dom/src/debug.rs
what: the DOM engine's layout debug print of a node's box and overflow
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: the DOM engine's pointer events — touch-action pan axes read a node's scroll width against its client width
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs
what: the DOM engine's inline layout — an inline box's scrollable overflow
@task_file n=6 path=packages/blitz-dom/src/layout/mod.rs
what: the DOM engine's layout pass — the layout cache read the engine asks for
@task_file n=7 path=packages/blitz-dom/src/layout/replaced.rs
what: the DOM engine's replaced-element layout — min-content contributions of percentage-width replaced boxes
@task_file n=8 path=packages/blitz-dom/src/layout/writing_mode.rs
what: the DOM engine's writing-mode geometry — mirrored overflow, the scroll-container check and the physical bounds a box reads
@task_file n=9 path=packages/blitz-dom/src/mutator.rs
what: the DOM engine's document mutator and its hit-testing unit tests
@task_file n=10 path=packages/blitz-dom/src/node/node.rs
what: the DOM engine's node — its hit test against clipping boxes, its scroll width and height and its scroll range
@task_file n=11 path=packages/blitz-dom/src/node/scrollbar.rs
what: the DOM engine's scrollbar — the thumb over the signed scroll range
@task_file n=12 path=packages/blitz-dom/src/scrolling.rs
what: the DOM engine's scrolling — the scroll state, the programmatic scroll clamp and the into-view scroll over every holding scroller
@task_file n=13 path=tests/blitz-tests/tests/display_contents.rs
what: the engine integration test for display contents layout
@task_file n=14 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs
what: the engine integration tests for scrollable overflow and reversed scroll origins
@task_file n=15 path=tests/blitz-tests/tests/pre_overflow_scroll.rs
what: the engine integration test for a pre element's overflow scroll
@task_file n=16 path=wpt/runner/src/test_runners/attr_test.rs
what: the web-platform-test runner's attribute checks — scroll size read as the node's CSSOM size
@task_item n=1 part=touches
The into-view scroll that moves every holding scroller: it now reads the scroll state as current, min and max and skips a scroller whose min and max are both zero; a reversed scroller scrolls to negative offsets.
@task_item n=2 part=touches
The visible region through clipping boxes: a clipping box's scrollable overflow is now measured on the layout engine's scrollable overflow with reversed origins, and a hit stopped at a clip answers in the content-box frame for an inline root.
@task_item n=3 part=touches
Open list rows read their own bounds: the bounds body was reworked by the take while escher's own-scroll rule was kept; the reading is held, a row whose bounds or into-view reading it rules over may be confirmed or restated.
@task_item n=4 part=touches
The flight type select read at boot: a node with no computed styles yet is no scroll container and is hit-testable, the seam that kept the select popup from panicking under the workspace's features.
@task_item n=5 part=touches
The off-screen refusal of the session: it rests on the bounds and visible region the engine reports, whose scroll measurement moved for reversed boxes.
@task_item n=6 part=touches
The snapshot and screenshot answers of the session where they carry box bounds or scroll offsets: a reversed scroller's offsets may now be negative.
@task_item n=7 part=touches
Any row naming the Taffy layout engine pin e90751fd: the pin is now a3997d55, version 0.15.0 and features unchanged.
@task_item n=8 part=touches
The headless stand's pinned viewport and the stand boot, indirectly: every layout of the stand now runs on the bumped layout engine, with collapse-through margins no longer double-counted and float insets picked up.
@task_fact n=1 slug=surface-desktop-native
fact: A reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets: scroll width and height answer the CSSOM scroll size, programmatic scroll clamps between the scroll range's min and max, and the into-view scroll skips a box whose range is zero at both ends — taken from upstream and ratified by the founder.
@task_item n=1 part=untouched
The stand stable element ids: no id was added, renamed or removed.
@task_item n=2 part=untouched
The window chrome controls of the shell provider: no window drag, maximize, minimize, close or decoration call moved.
@task_item n=3 part=untouched
The agent-run and cold-agent command lines, their usage, exits, stdout, logs and record directories, and their Windows pass-throughs.
@task_item n=4 part=untouched
The session's command form, exit codes, help, lint, graph, coverage role, actions answer, recording, served tasks, unknown-verb and fonts refusals, host log service name and MCP server tools, stdio, revisions and result bound: no verb, argument, result field, cause or log field was added.
@task_item n=5 part=untouched
The multi_doc and paint_bench command lines, the stand accent colour, the stand's attribute-only names, roles and states, its lint-clean boot, the CRUD keyboard region, the fixed document skeleton, the load page's network provider, held instances and session roles.
@task_item n=6 part=untouched
The absence of arbitrary index.html templates, and the not-yet-measured mobile-native, cli and web-spa surface overviews.
