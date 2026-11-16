@task_prose part=chunk
Takes seven upstream commits onto the build branch in upstream order: the reversed-scroll-origin change (#1294), its fix for hit testing elements with no computed styles yet (#1304), and five Taffy bumps.
Moves the workspace's Taffy git pin from e90751fd to upstream's pin a3997d55. The rev is the only word that changes.
Reconciles upstream's scroll-origin change with escher's ratified scrolled-box bounds, hit and absolute-position readings. Adds three seam edits so the engine compiles and does not panic.
Brings in upstream's tests that ride the picks: an inline-box scrollable-overflow test target and a blitz-dom unit test for hit testing before the first resolve.
The take was already measured in a trial worktree and is not re-measured. Every leg was green there except coverage and the WPT suites, which were not run. The cross-OS answer digests and the Counter checks passed unchanged.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace manifest, home of the Taffy dependency pinned at a full git commit rev — edits: the taffy rev line
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lockfile, read by the locked cargo legs — its taffy source line, conflicted in four of the Taffy bumps
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=conflict
what: the engine document, its stylo prefs set at init
@task_file n=4 path=packages/blitz-dom/src/layout/writing_mode.rs mark=conflict
what: the layout geometry readers: a box's physical layout and a placer's geometry, with the own-scroll rule that a scrolled box reads where it stands, and a private scroll-container test that reads false for a node with no computed styles
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the engine node: hit testing (the padding-box hit stop of a clipping box, the self match on own x/y, the content test against the scrollable overflow rect) and the node's absolute position
@task_file n=6 path=packages/blitz-dom/src/scrolling.rs mark=edits
what: the engine's scrolling and its into-view scroll, which brings a target into view through every scrolling box that holds it — edits: scroll_into_view, reading the scroll state as current, min and max
@task_file n=7 path=packages/blitz-dom/src/debug.rs mark=merge
what: the engine's debug printing of the node tree and layout
@task_file n=8 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the engine's pointer event handling, which hit tests the point under the pointer
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: inline layout, which measures inline boxes and their scrollable overflow
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout bridge to Taffy
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: layout of replaced elements
@task_file n=12 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the document mutator, with its inline unit tests, which gain upstream's test of hit testing before the first resolve
@task_file n=13 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the node's overlay scrollbar geometry over its scroll range
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the blitz-tests test target for display: contents layout
@task_file n=15 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: upstream's new blitz-tests test target for an inline box's scrollable overflow
@task_file n=16 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the blitz-tests test target for the scroll of overflowing preformatted text
@task_file n=17 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test runner
@task_item n=1 part=needs
The Taffy dependency moves to upstream's pin a3997d55. It stays a git source pinned at a full commit rev, with no patch section and no checksum, and the dependency list does not change. The Taffy exception to the no-git-dependency rule is kept as it stands; only the rev changes.
@task_item n=2 part=needs
Reversed scroll origins for every document: a box's content is measured against Taffy's scrollable overflow rect, and its scroll range may run negative. This is a behaviour change, ratified for this take.
@task_item n=3 part=needs
A scrolled box reads its own bounds where it stands, with none of its own scroll offset subtracted. Upstream's scroll-shifted reading in the box geometry reader does not enter. A placer's geometry still subtracts each placer's offset, and the box's physical layout takes upstream's fourth argument.
@task_item n=4 part=needs
A hit stops at the padding box of every box that clips by overflow, and the self match reads the box's own x/y. Upstream's scroll-shifted self match does not enter. The content test takes upstream's form: the point is tested against the scrollable overflow rect's four sides plus the client offset.
@task_item n=5 part=needs
A node's absolute position subtracts only its containing blocks' scroll offsets.
@task_item n=6 part=needs
The into-view scroll brings a target into view through every scrolling box that holds it. It reads each box's scroll state as current, min and max, and skips a box whose min and max are both zero. Its reach is a harness input helper, scroll_into_view.
@task_item n=7 part=needs
Hit testing an element that has no computed styles yet does not panic. A node with no computed styles is not a scroll container. This keeps the select popup's close-on-inert/disabled/hidden/removed check from panicking under the workspace's features.
@task_item n=8 part=needs
Pointer clicks, the harness's click on a clipped element and the harness centre and layout-rect readings all hit test and read positions through the engine. They must keep the readings they had: the driver's answers change only where the engine's readings change, and the measurement found none changed on the stand.
@task_item n=9 part=needs
The driver surface stays as it is: no new carrier, verb, argument, result field, node key, cause, session error, exit, log field or span.
@task_item n=10 part=needs
Upstream's tests ride the picks: a new blitz-tests target for inline-box scrollable overflow, and an inline blitz-dom unit test of hit testing before the first resolve. The workspace test tally moves from 1501 to 1510, a rise of nine, each attributed by test name. The a11y and stand-repeat tallies do not move.
@task_item n=11 part=needs
The cross-OS answer digests of the served tasks and the committed Counter checks pass unchanged. Linux x86_64 was read in the trial; windows and macOS are read only in CI.
@task_item n=12 part=needs
The stand checks stay green over three stand-repeat rounds, with a zero-failure budget, in both layout modes.
@task_item n=13 part=needs
The CI legs read green: fast, msrv (Rust 1.91, verified by build), audit (dependency advisories and licenses over the new Taffy rev), doc, build, counter, wasm, a11y and stand-repeat. Clippy runs over the workspace and upstream blitz-dom. The cargo legs run locked against the updated lockfile.
@task_item n=14 part=needs
Only the seven named upstream commits enter, taken by cherry-pick in upstream order. No other upstream commit enters; the later upstream sync stays a chunk of its own.
@task_item n=15 part=needs
Overlay scrollbar geometry follows the new scroll range. A container that has never been scrolled paints no scrollbar thumb.
@task_standing n=1
convention: each blitz-tests test file is its own test target, and the blitz-tests manifest keeps its stated difference from upstream
home: test-strategy · unit-test-strategy
@task_standing n=2
convention: unit tests sit in an inline test module of their source file
home: 2-test-strategy · test-strategy
@task_standing n=3
convention: blitz-dom fixtures are built without the HTML parser, and test data is stated in code
home: 2-test-strategy · test-data-fixtures
@task_standing n=4
convention: a layout assertion runs in both layout modes, and every layout reader word is read on a fixture
home: integration-test-strategy
@task_standing n=5
convention: a move in a test tally is attributed by test name
home: ci-integration
@task_item n=1 part=not_needed
The cold-agent pipe: its run, prompt, events, verdict, exit codes and state. The take adds no task and no tool.
@task_item n=2 part=not_needed
The agent-run harness verbs, selections, events and exit codes. The take changes no harness command.
@task_item n=3 part=not_needed
The MCP server's protocol revision, tools list, tool results and stdout. The take adds no tool and no protocol line.
@task_item n=4 part=not_needed
Handler coverage: the coverage line, the coverage role and its limits. The take adds no listener and no handler.
@task_item n=5 part=not_needed
Lint defects and their finding order. The take changes no lint rule.
@task_item n=6 part=not_needed
Text contrast and the WCAG ratio readings. The take changes no colour or paint.
@task_item n=7 part=not_needed
The session wire, session host lifecycle, idle expiry and owner-only session state. The take changes no socket, host or state directory.
@task_item n=8 part=not_needed
The session recording and its committed-check file format. The take records nothing new.
@task_item n=9 part=not_needed
Typed-password masking and the snapshot text byte budget. The take changes no text entry and no snapshot.
@task_item n=10 part=not_needed
The virtual clock, advance and timer ticks. The take changes no time source.
@task_item n=11 part=not_needed
The exit codes of the apt-install and coverage-floor scripts, and CI package-install bounds. The take changes no CI script.
@task_item n=12 part=not_needed
The mocking-library absence and the network stand-ins. The take adds no stand-in and no load provider.
@task_item n=13 part=not_needed
CI action SHA pinning, read-only permissions, secrets and upstream-only publishing jobs. The take changes no workflow file.
