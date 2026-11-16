@task_prose part=session
The session took seven upstream blitz commits: reversed scroll origins measured on Taffy's ScrollableOverflow, a hit test of elements with no computed styles yet, and five Taffy bumps that move the engine's Taffy git pin from e90751fd to a3997d55.
Every document's box, remote pages included, now measures its content against Taffy's scrollable overflow, so a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets; the founder ratified this for this take.
escher's own readings were held: the scroll state now reads current, min and max, the programmatic scroll clamps between min and max, scroll-into-view skips a box whose min and max are both zero, and scrollWidth and scrollHeight answer the CSSOM scroll size.
The hit test's clip stop now answers in the content-box frame, which moves only an inline root's answer; blitz-dom gains one public read-only layout-data accessor.
No log site, span, field, event, cause, exit, endpoint or env var was added or removed; the test leg tally moved 1501 to 1511 passed and agent-run run all 845 to 851, and CI went green 17 of 17.
@task_file n=1 path=Cargo.toml
what: the workspace manifest, holding the engine's Taffy git dependency pin
@task_file n=2 path=Cargo.lock
what: the workspace lock file, holding the Taffy package's git source line, the one git source
@task_file n=3 path=packages/blitz-dom/src/debug.rs
what: blitz-dom's layout debug printing of a node's layout data
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs
what: blitz-dom's pointer event handling, its touch-action pan axes read off the scroll size
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs
what: blitz-dom's inline layout, an inline box's scrollable overflow fed to Taffy
@task_file n=6 path=packages/blitz-dom/src/layout/mod.rs
what: blitz-dom's layout pass over Taffy, its layout cache read and the scrollable overflow it stores
@task_file n=7 path=packages/blitz-dom/src/layout/replaced.rs
what: blitz-dom's replaced-element sizing, its min-content contribution
@task_file n=8 path=packages/blitz-dom/src/layout/writing_mode.rs
what: blitz-dom's writing-mode geometry, mirroring overflow for reversed boxes and deciding a scroll container, false for a node with no computed styles
@task_file n=9 path=packages/blitz-dom/src/mutator.rs
what: blitz-dom's document mutator and its unit tests of hit testing before the first resolve and of a clipping inline root's border hit
@task_file n=10 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: blitz-dom's node, its hit test, its layout-data accessor and its CSSOM scroll width and height, the scroll size a page reads
@task_file n=11 path=packages/blitz-dom/src/node/scrollbar.rs
what: blitz-dom's scrollbar, its thumb over the signed scroll range
@task_file n=12 path=packages/blitz-dom/src/scrolling.rs
what: blitz-dom's scrolling, the scroll state, the programmatic scroll clamp and escher's scroll-into-view
@task_file n=13 path=tests/blitz-tests/tests/display_contents.rs
what: the blitz integration test of display contents layout
@task_file n=14 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs
what: the blitz integration tests of scrollable overflow, reversed scroll origins and negative offsets
@task_file n=15 path=tests/blitz-tests/tests/pre_overflow_scroll.rs
what: the blitz integration test of a pre element's overflow scroll
@task_file n=16 path=wpt/runner/src/test_runners/attr_test.rs
what: the WPT runner's attribute test, its scroll size checks
@task_item n=1 part=touches
A rule on engine tracing behind the tracing cargo feature or set by the build's invocation: blitz-dom's layout, scrolling and hit test changed and the take added no tracing call, so such a rule should hold unchanged.
@task_item n=2 part=touches
The rule that the layout read opens no span: the scroll size, scroll state and layout-data readings changed in body and still open no span.
@task_item n=3 part=touches
Snapshot and diff contracts on bounds, and the open list row bounds: escher's bounds reading was held while the scroll origin may now be negative for a reversed box.
@task_item n=4 part=touches
The snapshot node and snapshot text clickable contracts: the hit test now hits elements with no computed styles yet and answers in the content-box frame at the clip stop for an inline root.
@task_item n=5 part=touches
Rules on the open select list and its shot (open list shot carries labels, select list row label never logged): a node with no computed styles is no scroll container, the seam edit for the select popup panic.
@task_item n=6 part=touches
WPT rows (WPT scores published from upstream only, WPT results published upstream only, the WPT report message): the WPT runner's scroll size checks now read the node's CSSOM size.
@task_item n=7 part=touches
Rows on the panic event and the panic hook, and exploration catching a panicking call: the take removes a select popup panic under the workspace's features, with no new panic path or event.
@task_item n=8 part=touches
Rows naming the engine dependency or its Taffy pin, if any holds the rev e90751fd: it moved to a3997d55.
@task_item n=9 part=touches
The Log Coverage area's standing claim that no log site, span or field was added by an engine take: this take adds none, nothing moves.
@task_item n=10 part=touches
Rows on a page's scrollWidth and scrollHeight or the scroll-into-view action, where a row's cell holds their value: they now answer the CSSOM scroll size, reversed origins included, and scroll-into-view skips a box whose min and max are both zero.
@task_item n=1 part=untouched
The escher-driver command span and tool_call span, their fields, endings and the refusal cause names: no span, field or cause was added or renamed.
@task_item n=2 part=untouched
escher's telemetry sink, its log line format, the target allowlist, the scrub set and RUST_LOG handling.
@task_item n=3 part=untouched
The agent-run and cold-agent harness events, the cold-agent verdict and its counted verbs, and the kept check files.
@task_item n=4 part=untouched
escher-session's commands, exit codes, stdout, stderr, error object, host wire, coverage role and mcp role.
@task_item n=5 part=untouched
The absence of metrics exporters, metric emission, a telemetry backend and an error-reporting service: none added.
@task_item n=6 part=untouched
CI integration rows: the CI artifacts, log upload on failure, coverage upload, and the recording and session state directories.
@task_item n=7 part=untouched
Script error surfaces, the window error event and window.onerror, and the embedder drain of script diagnostics.
@task_item n=8 part=untouched
PII rows on the actions answer, graph answer, coverage line, page events and session logs carrying no page content.
