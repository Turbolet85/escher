@task_prose part=chunk
Takes seven upstream commits in upstream order onto build/escher-0.3.0: the reversed-scroll-origin change (0980cce5f), its hit-test panic fix for elements with no computed styles (ebacd68bd) and five Taffy bumps, moving the workspace's git Taffy from e90751fd to upstream's pin a3997d55.
Resolves the three conflicts of 0980cce5f by keeping escher's ratified readings: the own-scroll rule, the hit stop at the padding box of a clipping box, and absolute_position subtracting only containing blocks' offsets.
Adds three seam edits: the dropped CopyStyleGetters import, scroll_into_view reading the scroll state as a (current, min, max) triple, and a private scroll-container test that reads false for a node with no computed styles.
Does not re-measure: the take was measured in the trial worktree, its staged diff and its legs read back as they stand; the committed cross-OS digests and Counter checks are re-read and must pass unchanged.
@task_file n=1 path=Cargo.toml
what: the workspace manifest — the git Taffy dependency pinned at a full commit rev, the one exception of the no-git-dependency invariant
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lockfile — the taffy source line, still a git source at a full rev with no checksum and no patch section
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=conflict
what: the engine document — the stylo prefs set at init_stylo_prefs, escher's side kept
@task_file n=4 path=packages/blitz-dom/src/layout/writing_mode.rs mark=conflict
what: the engine's layout geometry read — physical_layout taking upstream's fourth argument, escher's own-scroll rule, placer_geometry subtracting each placer's offset, and the new private scroll-container test used at three sites
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the engine node — hit_inner's self match on own_x/own_y, its content test against the scrollable overflow rect plus the client offset, and absolute_position
@task_file n=6 path=packages/blitz-dom/src/scrolling.rs mark=edits
what: the engine's scrolling — escher's into-view scroll through every scrolling box that holds the target — edits: scroll_into_view
@task_file n=7 path=packages/blitz-dom/src/debug.rs
what: the engine's debug printing of node layout
@task_file n=8 path=packages/blitz-dom/src/events/pointer.rs
what: the engine's pointer event handling, reading hits and scroll offsets
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs
what: the engine's inline layout, the inline box's scrollable overflow
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs
what: the engine's layout tree glue to Taffy
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs
what: the engine's replaced-element layout
@task_file n=12 path=packages/blitz-dom/src/mutator.rs
what: the engine's document mutator, with upstream's unit test of hit testing before the first resolve
@task_file n=13 path=packages/blitz-dom/src/node/scrollbar.rs
what: the engine's scrollbar geometry over a box's scroll range
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs
what: the engine's layout tests for display contents
@task_file n=15 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs
what: upstream's new layout test of an inline box's scrollable overflow
@task_file n=16 path=tests/blitz-tests/tests/pre_overflow_scroll.rs
what: the engine's layout test of overflow scroll on preformatted text
@task_file n=17 path=wpt/runner/src/test_runners/attr_test.rs
what: the WPT runner's attribute test runner
@task_item n=1 part=needs
The reversed-scroll-origin change for every document: a box's content measured against Taffy's scrollable overflow rect and its scroll range able to run negative — a changed engine behaviour the take brings, ratified by the founder for this take.
@task_item n=2 part=needs
A scrolled box reads its own bounds where it stands, with no subtraction of its own scroll offset; upstream's scroll-shifted readings in the geometry read and in the hit's self match do not enter.
@task_item n=3 part=needs
A hit stops at the padding box of every box that clips by overflow and reads own_x/own_y; hit testing an element that has no computed styles yet answers without a panic.
@task_item n=4 part=needs
The node's absolute position subtracts only its containing blocks' scroll offsets; the into-view scroll brings a target into view through every scrolling box that holds it, skipping a box whose scroll range is zero at both ends.
@task_item n=5 part=needs
The engine's overflow read must not call the computed-style accessor on a node with no computed styles: the scroll-container test reads false there, so the open select list closing on inert, disabled, hidden or removed raises no panic.
@task_item n=6 part=needs
No new carrier, verb, argument, result field, node key, cause, session error, exit, log field or span: the driver's answers and its command and tool_call span fields change only where the engine's readings change, and the measurement reads none changed on the stand.
@task_item n=7 part=needs
The layout and geometry read the take changes opens no span and emits no event of its own; the new scroll-container test and the into-view scroll stay silent.
@task_item n=8 part=needs
The engine's tracing stays behind its cargo feature: the Taffy bump brings no ungated tracing into the engine or upstream crates.
@task_item n=9 part=needs
The dependency list is unchanged by the bump: Taffy stays a git source at a full commit rev, and no telemetry backend, metrics exporter or error-reporting client enters with it.
@task_item n=10 part=needs
The snapshot and diff readings of bounds — open list row bounds of a scrolled select list among them — keep their committed values; the committed cross-OS stand digests and Counter checks are re-read and pass unchanged.
@task_item n=11 part=needs
The take is not re-measured: every leg read green in the trial (fast, msrv, audit, doc, build, counter, wasm, a11y, stand-repeat), test count 1501 to 1510 from upstream's nine tests; coverage and the WPT suites were not run.
@task_standing n=1
convention: each new or taken test runs silent — no print, no log line on a passing run — and a check that asserts an empty stderr unsets RUST_LOG
home: 3-observability-harness-contract
@task_item n=1 part=not_needed
The cold-agent pipe and agent-run harness: their events, verdict fields, reasons, counted verbs, kept checks and transcripts — the take touches no harness.
@task_item n=2 part=not_needed
The session recording file, recording and state directory privacy and the session lifecycle — no recording or session state changes.
@task_item n=3 part=not_needed
The page script error surface: window error event fields, onerror arguments and script error source labels — the take changes layout and hits, not script errors.
@task_item n=4 part=not_needed
Metric coverage and WPT score publication — no metric is added and nothing is published.
@task_item n=5 part=not_needed
CI artifact uploads and the package-install diagnostic line — no CI workflow or job changes.
