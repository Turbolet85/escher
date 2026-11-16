@task_prose part=chunk
Takes seven upstream commits onto the build branch by cherry-pick, in upstream order: the reversed-scroll-origin change (Taffy's ScrollableOverflow), its fix for hit testing elements with no computed styles yet, and five Taffy bumps.
Moves the workspace's Taffy git pin from e90751fd to upstream's a3997d55. The rev is the only thing that changes: it stays a git source pinned at a full commit.
Reconciles the scroll-origin change with escher's ratified readings (own-scroll bounds, hit stop, absolute position, into-view scroll) and adds three seam edits so the engine compiles and does not panic.
Re-reads the take measured earlier (its measurement and staged patch, with the trial worktree standing for it) and does not re-measure it; the take's size is settled at P5.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest, where dependencies are declared once for the workspace; it holds the Taffy git dependency and its pinned rev
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lockfile that the locked cargo legs build against; it holds Taffy's git source line
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=conflict
what: the blitz-dom document core (BaseDocument), where the Stylo prefs are set once, by Stylo 0.22 names, in init_stylo_prefs
@task_file n=4 path=packages/blitz-dom/src/layout/writing_mode.rs mark=conflict
what: blitz-dom layout geometry: physical_layout, the own-scroll rule, placer_geometry and the private scroll-container check for a node with no computed styles
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the blitz-dom Node: the hit walk (hit_inner and its self match) and Node::absolute_position
@task_file n=6 path=packages/blitz-dom/src/scrolling.rs mark=edits
what: BaseDocument scrolling, the one scrolling primitive, including the into-view scroll through the containing-block chain — edits: scroll_into_view (its scroll_state reading)
@task_file n=7 path=packages/blitz-dom/src/debug.rs mark=merge
what: blitz-dom's debug printing of a node's layout
@task_file n=8 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: blitz-dom pointer event dispatch, which hit tests to find the event target
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom inline layout, the deferred inline layout of inline boxes and their scrollable overflow
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: blitz-dom layout, the Taffy traversal and layout traits and the stylo_taffy exports it imports
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: blitz-dom layout of the replaced elements
@task_file n=12 path=packages/blitz-dom/src/mutator.rs mark=merge
what: DocumentMutator, through which DOM mutations go; it holds the unit test for hit testing before the first resolve
@task_file n=13 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: scrollbar geometry shared by paint and hit test, read from a scrolling box's scroll range
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: a blitz-tests integration test target for display contents layout
@task_file n=15 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: a new blitz-tests integration test target, upstream's, for the scrollable overflow of inline boxes
@task_file n=16 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: a blitz-tests integration test target for the overflow scrolling of pre text
@task_file n=17 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test runner, which checks checkLayout attributes against layout geometry
@task_item n=1 part=needs
Upstream changes are taken by cherry-pick: exactly these seven commits, in upstream order. No other upstream commit enters, and the later upstream sync stays a chunk of its own.
@task_item n=2 part=needs
Taffy moves to a3997d55 under the no-git-dependencies invariant. Its Taffy exception stays as it is: still a git source pinned at a full commit rev, with no patch section, no checksum and no change to the dependency list. Cargo legs stay locked against the lockfile.
@task_item n=3 part=needs
This take does the deferred Taffy bump past the ScrollableOverflow change, which used to leave blitz-dom uncompiled. The Stylo and Dioxus bumps stay deferred.
@task_item n=4 part=needs
Upstream's behaviour change is ratified per take. The reversed-scroll-origin change applies to every document: a box's content is measured against Taffy's ScrollableOverflow rect, and its scroll range can run negative. It is taken as an engine behaviour change, on the founder's ratification for this take.
@task_item n=5 part=needs
The scrolled-box bounds rule is kept: a scrolled box's client rect and bounds read the box where it stands, with no subtraction of its own scroll offset. Upstream's scroll-shifted reading in the physical geometry does not enter.
@task_item n=6 part=needs
The hit stop at clipping padding boxes is kept: a hit stops at the padding box of every box that clips by overflow and reads that box's own offsets. Only the content test takes upstream's form: the point is checked against the scrollable overflow rect's four sides plus the client offset.
@task_item n=7 part=needs
The absolute-position rule is kept: Node::absolute_position subtracts only its containing blocks' scroll offsets and never its own.
@task_item n=8 part=needs
The into-view scroll still writes the whole containing-block chain and still writes nested boxes instantly. It now reads each box's scroll state as current, min and max, and skips a box whose min and max are both zero.
@task_item n=9 part=needs
Upstream's fix enters: an element with no computed styles yet can be hit tested, including before the first resolve. Escher's own scroll-container check reads false for a node with no computed styles, so no overflow read panics when a select list is closed by making its select inert, disabled, hidden or removed.
@task_item n=10 part=needs
Physical layout takes upstream's fourth argument, and placer geometry still subtracts each placer's scroll offset. The stylo_taffy exports and the Taffy traversal and layout traits follow the new Taffy: the CopyStyleGetters import is dropped.
@task_item n=11 part=needs
The Stylo prefs stay as set once by Stylo 0.22 names. The document conflict resolves to escher's side, so upstream's sideways pref does not enter.
@task_item n=12 part=needs
Scrollbar geometry shared by paint and hit test follows the new scroll range, a range that can run negative.
@task_item n=13 part=needs
The driver boundary does not widen: no new carrier, verb, argument, result field, node key, refusal cause, session error, exit code, log field or span. The driver's answers change only where the engine's readings change, and the measured stand read none changed.
@task_item n=14 part=needs
Stand readings must stay identical on every CI OS. The committed cross-OS digests and the committed stand Counter checks must pass unchanged.
@task_item n=15 part=needs
The escher-side seam edits in blitz-dom are direct engine changes and are flagged upstreamable: the scroll-container check, the into-view scroll-state reading and the dropped import.
@task_item n=16 part=needs
The take re-reads the earlier measurement and does not re-measure. The measurement found every leg green except coverage and the WPT suites, which were not run. The test tally rose from 1501 to 1510 with upstream's nine tests, and the a11y and stand-repeat tallies did not move. The stand-repeat budget is zero failures.
@task_item n=17 part=needs
The WPT attribute tests read checkLayout attributes from layout geometry. That geometry now uses reversed scroll origins.
@task_item n=18 part=needs
Upstream's tests ride the picks: a new blitz-tests integration target for inline-box scrollable overflow, and a blitz-dom unit test for hit testing before the first resolve.
@task_standing n=1
convention: Integration tests live in blitz-tests, with one test target per file and one behaviour per file
home: design-philosophy · conventions
@task_standing n=2
convention: A regression test taken from upstream cites upstream
home: conventions
@task_standing n=3
convention: Unit tests stay at the end of their source file, and test names are descriptive
home: conventions
@task_item n=1 part=not_needed
The browser app: its history, bookmarks, about pages, toolbar and persistence. The take does not reach the browser chrome.
@task_item n=2 part=not_needed
The driver session's command line, MCP role, session socket wire, session host, recording directory and kept check files. No carrier changes.
@task_item n=3 part=not_needed
Telemetry: its bootstrap, scrubbing, target filter and log line format. The take adds no log line and no span.
@task_item n=4 part=not_needed
The accessibility tree and its ARIA state contracts, and the AccessKit adapter. The take changes no node role, name or state.
@task_item n=5 part=not_needed
The JavaScript runtime, script globals, script fetcher and Boa settings. The take does not reach scripting.
@task_item n=6 part=not_needed
The keyboard models of select lists and range inputs and their default actions. The take changes no key handling.
@task_item n=7 part=not_needed
The cold-agent and agent-run tooling: their events, exit codes and verdicts. Not touched.
@task_item n=8 part=not_needed
The WPT diff-to-PR script, its CI artifacts and the PR body results section. The take changes no WPT reporting.
@task_item n=9 part=not_needed
Paint commands for text decoration, borders, gradients, masks and layers. The take changes layout and hit readings, not painting.
@task_item n=10 part=not_needed
The register of crate, binary and other names. The take adds no crate, binary or named variable.
@task_item n=11 part=not_needed
Stable element ids, snapshot text, the UI graph and the actionable-key check. These read the engine and are not changed by the take.
