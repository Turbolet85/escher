@task_prose part=session
The session took seven upstream Blitz commits as one measured patch: reversed scroll origins read from Taffy's ScrollableOverflow, hit testing of nodes with no computed styles yet, and five Taffy bumps that move the Taffy git pin from e90751fd to a3997d55.
Every document now measures a box's content against Taffy's scrollable overflow, so a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets; the scroll state answers current, min and max, and scroll-into-view skips a box whose min and max are both zero.
The clip stop of hit testing on a clipping inline root now answers in content-box coordinates, the frame the self and text hits answer in; a new unit test checks it in both layout modes, held by a red-before-green control.
Upstream's six reversed-origin integration tests join the inline-box scrollable-overflow target (1 test to 7) as written, in one layout mode; two writing-mode unit tests and the hit-before-first-resolve unit test join too.
The test leg tally moved 242 · 1501 · 0 · 13 to 242 · 1511 · 0 · 13, attributed by test name; agent-run run all moved 845 to 851 passed; embolden, a11y and stand-repeat unmoved; CI green 17 of 17.
No IPC method, verb, argument, event, log field, span, exit or session error was added; no file was added or retired.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest, holding the Taffy engine dependency's git pin
@task_file n=2 path=Cargo.lock mark=merge
what: the lockfile, the one git source (Taffy) and its source line, read by the locked cargo legs
@task_file n=3 path=packages/blitz-dom/src/debug.rs mark=merge
what: the blitz-dom engine's debug printing of a node's layout and scroll readings
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the blitz-dom engine's pointer event handling, the touch-action pan axes read from a node's scroll width
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the blitz-dom engine's inline layout, an inline root's box and its scrollable overflow
@task_file n=6 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the blitz-dom engine's layout pass over Taffy, its layout cache read
@task_file n=7 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the blitz-dom engine's layout of replaced elements, their min-content contributions
@task_file n=8 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: the blitz-dom engine's writing-mode mapping and the bounds reader's bodies (physical geometry, own scroll, mirrored overflow), with its inline unit tests
@task_file n=9 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the blitz-dom engine's document mutator and its inline unit tests, among them the hit-testing tests
@task_file n=10 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the blitz-dom engine's node, its hit testing (the clip stop, the self and text hits), its scroll width, height and range, and its layout data read
@task_file n=11 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the blitz-dom engine's overlay scrollbar, the thumb over the scroll range
@task_file n=12 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the blitz-dom engine's scrolling, the scroll state, programmatic scroll clamping, scroll-into-view and the scroll animation
@task_file n=13 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the blitz-tests integration target for display contents
@task_file n=14 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: the blitz-tests integration target for inline-box scrollable overflow and reversed scroll origins
@task_file n=15 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the blitz-tests integration target for scrolling of overflowing pre text
@task_file n=16 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT conformance runner's attribute test runner, its scroll size checks
@task_item n=1 part=touches
Layout assertions in both layout modes and stand checks in both layout modes: upstream's six reversed-origin integration tests were kept as written in one layout mode, an exception to the convention.
@task_item n=2 part=touches
Upstream tests kept as written (the buffered-render precedent): the reversed-origin tests are a second upstream test kept unaltered.
@task_item n=3 part=touches
Each bounds-reader body holds a check: the bounds reader's bodies changed (mirrored overflow, the own-scroll rule kept in the second body, a fourth argument to the physical layout); both readers of the bounds body ran under fast and agent-run run all.
@task_item n=4 part=touches
A scrolled box reads where it stands: escher's own-scroll rule kept while upstream's scroll-shifted body was left out; reversed boxes now stand at negative scroll offsets.
@task_item n=5 part=touches
Harness input helpers, scroll_into_view: it now reads the scroll state's current, min and max and skips a box whose min and max are both zero.
@task_item n=6 part=touches
Harness input helpers, click on a clipped element and pointer coordinates: the clip stop of hit testing on a clipping inline root now answers content-box coordinates; the node hit and whether a hit happens are unchanged.
@task_item n=7 part=touches
escher_driver Session scroll: a programmatic scroll now clamps between the scroll range's min and max, negative for reversed boxes.
@task_item n=8 part=touches
Never-scrolled containers paint no scrollbar thumbs and the overlay scrollbar fade: the scrollbar thumb now runs over the signed scroll range.
@task_item n=9 part=touches
Tests controlled red-before-green: the clip-stop-frame control held (removed red, the case failed).
@task_item n=10 part=touches
Tally moves attributed by test name and the ANSI strip for CI job-log tallies: the test leg moved 1501 to 1511 passed, +10 named (six reversed-origin integration tests, two writing-mode unit tests, two mutator unit tests).
@task_item n=11 part=touches
Per-package green is not the workspace green: agent-run run all moved 845 to 851 passed beside the fast leg's workspace tally.
@task_item n=12 part=touches
Cargo legs run locked and the dependency audit: the Taffy git pin moved to a3997d55, still the one git source, built locked; msrv and audit green.
@task_item n=13 part=touches
One test target per file and one integration target per behaviour: six tests joined the existing inline-box scrollable-overflow target, no new target.
@task_item n=14 part=touches
Unit tests in inline test modules: two writing-mode unit tests and two mutator unit tests added inline.
@task_item n=15 part=touches
WPT conformance runner: its scroll size checks now read the node's CSSOM scroll size, reversed origins included.
@task_item n=16 part=touches
Incremental layout differential oracle: the layout cache read now takes a shared borrow, as Taffy's cache tree asks.
@task_item n=17 part=touches
Regression tests document and are named for their bug: the hit-before-first-resolve test and the clipping-inline-root border-hit test.
@task_fact n=1 slug=integration-test-strategy
fact: Upstream's reversed-scroll-origin integration tests are kept as written, in one layout mode, an exception to layout assertions in both layout modes.
@task_fact n=2 slug=3-test-harness-contract
fact: The WPT runner's scroll size checks read the CSSOM scroll size, reversed origins included, not Taffy's content extent.
@task_item n=1 part=untouched
The session wire, MCP role, CLI, exit codes, refusal causes and session errors: no verb, argument, cause, exit or error was added or moved.
@task_item n=2 part=untouched
agent-run and cold-agent verbs, events, exit codes, state and verdicts: unchanged; only the passed count moved.
@task_item n=3 part=untouched
Session recording, committed checks and handler coverage: no recording, check or coverage line changed.
@task_item n=4 part=untouched
CI workflow, actions pinning, permissions, secrets, caches and package installs: no workflow or script changed.
@task_item n=5 part=untouched
Coverage floors, mutation testing, the complexity ceiling and the zero-failure stand budget: unmoved; stand-repeat 3 of 3 green.
@task_item n=6 part=untouched
Mocking discipline, the virtual clock, advance and settle: no stand-in, clock or settle rule changed.
@task_item n=7 part=untouched
Lint defects, snapshot nodes, contrast readings and screenshots: no reading changed; a11y and embolden legs unmoved.
@task_item n=8 part=untouched
Log, span and stderr contracts: the take adds no log site, span or field.
