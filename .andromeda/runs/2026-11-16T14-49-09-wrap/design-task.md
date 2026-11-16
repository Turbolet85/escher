@task_prose part=session
Took seven upstream Blitz picks past #1372: reversed scroll origins on Taffy's scrollable overflow, hit-testing of elements with no computed styles yet, and five Taffy bumps to git a3997d55 (version 0.15.0 unchanged).
Every document now measures a box's content against Taffy's scrollable overflow, so a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets; the founder ratified this behaviour change for the take.
The engine's scroll state became a triple of current, min and max; programmatic scroll clamps between min and max; into-view scroll skips a box whose min and max are both zero; scrollWidth and scrollHeight answer the CSSOM scroll size, reversed origins included.
A hit stopped at an inline root's paint clip now answers in the content-box frame the final self hit and the text hits answer in; the node hit and whether a hit happens are unchanged.
escher's own bounds, hit and into-view readings were held over the take; no verb, argument, result field, event, log field, span or exit was added; nothing retired.
Measured: test leg 242 · 1511 · 0 · 13 (from 1501), agent-run run all 851 passed (from 845), CI green 17 of 17.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest holding the engine dependency pins, the Taffy git rev among them
@task_file n=2 path=Cargo.lock mark=merge
what: the lockfile, the Taffy package's git source line
@task_file n=3 path=packages/blitz-dom/src/debug.rs mark=merge
what: the DOM engine's debug printing of a node's layout
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the DOM engine's pointer event handling, the touch-action pan axes read from scroll width over client width
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the DOM engine's inline layout, inline boxes and their scrollable overflow
@task_file n=6 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the DOM engine's layout pass over Taffy, its layout cache read
@task_file n=7 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the DOM engine's layout of replaced elements, their min-content contributions
@task_file n=8 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: the DOM engine's writing-mode geometry, mirrored overflow and which node is a scroll container, the own-scroll rule of physical geometry
@task_file n=9 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the DOM engine's document mutator and its unit tests of hit-testing
@task_file n=10 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the DOM node, its hit-testing at the paint clip, its layout data and its scroll width, height and range
@task_file n=11 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the overlay scrollbar, its thumb position and drag over the scroll range
@task_file n=12 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the engine's scrolling, the scroll state, programmatic scroll, smooth scroll animation and into-view scroll
@task_file n=13 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the engine tests of display contents
@task_file n=14 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: the engine tests of scrollable overflow, inline boxes and reversed scroll origins
@task_file n=15 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the engine tests of scrolling preformatted overflow
@task_file n=16 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test checks, its scroll size checks
@task_item n=1 part=touches
The decision that hit-testing stops where paint clips (both of its twin rows): the hit stopped at an inline root's paint clip now answers in content-box coordinates, the frame of the self and text hits; the node and whether a hit happens are unchanged.
@task_item n=2 part=touches
The decisions that into-view scroll is instant in nested boxes (both twin rows): into-view scroll now reads a min, max range and skips a box whose min and max are both zero; reversed boxes scroll into view at negative offsets.
@task_item n=3 part=touches
The decision that driver scroll is instant: programmatic scroll now clamps between a scroll range's min and max, negative for a reversed box.
@task_item n=4 part=touches
The decision on smooth scroll-behavior and wheel cancel: the smooth scroll animation reads the current offset of the new scroll state triple, its range now signed for reversed boxes.
@task_item n=5 part=touches
The decision that a shown overlay scrollbar takes a pointerdown as drag, and the overlay scrollbar fade timings: the scrollbar thumb now runs over the signed scroll range.
@task_item n=6 part=touches
The decision that wheel and key scroll the open select list, and the decision that a select list is an engine overlay: a node with no computed styles is never a scroll container, the fix for the select popup panic under the workspace's features.
@task_item n=7 part=touches
The decisions that stacking-context roots group paint and hit-testing and that the open select list answers hits first: the hit's content test now reads the point against the overflow rect's four sides plus the client offset, and elements with no computed styles yet are hit-tested.
@task_item n=8 part=touches
The snapshot contract on a row of an open select list's bounds and the screenshot verb's select list row: bounds readings were held over the take, but the bounds body now sits on Taffy's scrollable overflow.
@task_item n=9 part=touches
The driver verbs click, type and press: they hit-test and scroll into view through the changed hit and scroll paths, results unchanged.
@task_item n=10 part=touches
The decision that a select list is placed inside the viewport and the spacing decisions on margins and table spacing hints: the Taffy bumps stop double-counting collapse-through margins, add float insets and zero-height float segments, and change percentage-width replaced min-content contributions.
@task_item n=11 part=touches
The decision on the logical viewport and the JS window inner sizes: page scrollWidth and scrollHeight now answer the CSSOM scroll size, reversed origins included, while the MountedData scroll size stays on Taffy's extent.
@task_fact n=1 slug=motion
fact: A reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets, its content measured against Taffy's scrollable overflow, for every document including remote pages; ratified by the founder for the take past #1372.
@task_fact n=2 slug=motion
fact: Programmatic scroll clamps to a scroll range of min and max, and into-view scroll skips a box whose min and max are both zero.
@task_fact n=3 slug=motion
fact: The overlay scrollbar thumb runs over the signed scroll range, so a reversed box's thumb starts at its negative extreme.
@task_fact n=4 slug=depth-strategy
fact: A hit stopped at an inline root's paint clip answers in content-box coordinates, the same frame as the final self hit and the text hits.
@task_fact n=5 slug=depth-strategy
fact: Elements with no computed styles yet are hit-tested before the first style resolve, and such a node is never a scroll container.
@task_fact n=6 slug=surface
fact: scrollWidth and scrollHeight answer the CSSOM scroll size, reversed origins included, and touch-action pan axes read scroll width over client width; the exception is MountedData's scroll size, left on Taffy's extent.
@task_item n=1 part=untouched
Color palette, engine colours, select list colours and colour tokens: no colour moved.
@task_item n=2 part=untouched
Typography, fonts, font context and the type scale: no font or text rule moved.
@task_item n=3 part=untouched
Iconography, cursor icons, favicon and the shell provider's window controls.
@task_item n=4 part=untouched
escher-session commands, exit statuses and codes, stderr, stdout, the mcp role and session error: no verb, argument, result field or exit added.
@task_item n=5 part=untouched
The actions, coverage and UI graph lines and the coverage check files.
@task_item n=6 part=untouched
The escher-session tasks (counter, crud, cells, timer, flight-booker, load).
@task_item n=7 part=untouched
The select accessibility tree and the select list events (open, close, choose, highlight move).
@task_item n=8 part=untouched
The virtual clock, animation driven by it, settle and load wait, the advance verb, and reduced-motion handling.
@task_item n=9 part=untouched
WPT runner result colours and its test name hyperlink output.
@task_item n=10 part=untouched
The CLI and web-spa surfaces, the mobile targets, the dioxus-native configuration and MountedData calls during a render.
@task_item n=11 part=untouched
Brand identity, border radius, the anti-patterns, the self-validation protocol and the design decisions log: no recorded intent moved.
