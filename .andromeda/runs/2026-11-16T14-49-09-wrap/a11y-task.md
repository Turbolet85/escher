@task_prose part=session
The session took seven upstream engine commits, cherry-picked as one measured patch: reversed scroll origins measured against Taffy's scrollable overflow, hit testing of elements that have no computed styles yet, and five Taffy bumps.
The Taffy engine pin moved from e90751fd to a3997d55, version 0.15.0 and features unchanged, still the one git source.
Every document now has a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scroll to negative offsets; scroll width and height answer the CSSOM scroll size; the founder ratified this behaviour change.
The engine's scroll state reads a current, minimum and maximum offset; programmatic scroll clamps between them; scroll into view skips a box whose minimum and maximum are both zero.
The hit test's clip stop on a clipping inline root now answers in content-box coordinates, the frame the final self hit and text hits answer in; which node is hit and whether a hit happens are unchanged; a node with no computed styles is never a scroll container, closing a select popup panic.
Gates green: test tally 1501 to 1511, agent-run run all 845 to 851, the a11y leg unmoved at 14 · 115 · 0, stand-repeat 3 of 3 rounds green, CI green 17/17; no node key, verb, cause, event or log field added.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest that pins the engine's layout dependency, Taffy, by git rev
@task_file n=2 path=Cargo.lock mark=merge
what: the lock that records the Taffy git source the build resolves
@task_file n=3 path=packages/blitz-dom/src/debug.rs mark=merge
what: the engine document's debug dump of node layout
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the engine's pointer event handling, including touch-action pan axes read from a node's scroll width
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline and text layout, the inline root's scrollable overflow
@task_file n=6 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout pass over Taffy, its layout cache
@task_file n=7 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's layout of replaced elements such as images, their min-content contributions
@task_file n=8 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: the engine's writing-mode geometry, the mirrored overflow of reversed boxes and which node is a scroll container
@task_file n=9 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the engine's document mutator and its unit tests of hit testing
@task_file n=10 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the engine's node, its hit testing, its layout data read and its scroll width and height
@task_file n=11 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the engine's scrollbars, the thumb's position over the scroll range and the scrollbar band
@task_file n=12 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the engine's scrolling, programmatic scroll, scroll animation and scroll into view
@task_file n=13 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the engine's integration test of display contents layout
@task_file n=14 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: the engine's integration tests of scrollable overflow and reversed scroll origins
@task_file n=15 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the engine's integration test of overflowing preformatted text scrolling
@task_file n=16 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute checks, its scroll size checks
@task_item n=1 part=touches
A rule over the engine's hit testing (an inert element takes no hit, the hit-testing contract for inert): the hit test's content check now reads the overflow rect's four sides plus the client offset, and the clip stop on a clipping inline root answers in content-box coordinates; the node hit is unchanged.
@task_item n=2 part=touches
The driver click refusal for a covered target and a driver click in a shown scrollbar band: both read the engine's hit test and scrollbar band, whose scroll range is now signed for a reversed box.
@task_item n=3 part=touches
A rule over default scrollbar thumbs (their contrast stroke, a near-white author thumb's visible state change): the thumb now runs over the signed scroll range of a reversed scroller.
@task_item n=4 part=touches
Snapshot bounds of a closed select's options, an open list's rows and a multiple select's rows and groups, and the lint contrast finding's bounds: all read layout geometry that moved with five Taffy bumps and reversed origins; the chunk holds escher's bounds, hit and into-view readings.
@task_item n=5 part=touches
The synthetic click at a node's centre and the driver actions: they read node bounds and the hit test the take changed.
@task_item n=6 part=touches
The single select's engine overlay list: a node with no computed styles is now never a scroll container, the seam that closes a select popup panic under the workspace's features, and an element with no computed styles yet can be hit.
@task_item n=7 part=touches
Open select list rows of one height and the select list without motion: the list's scrolling now clamps between a minimum and a maximum offset, and scroll into view skips a box with no scroll range.
@task_item n=8 part=touches
Hidpi or zoom change reshaping text and the stand reading no lint finding at boot: inline and text layout moved with the Taffy bumps (float insets, zero-height float segments, collapse-through margins, replaced min-content); the a11y and stand legs stayed green.
@task_item n=9 part=touches
The a11y CI leg and the accessibility checks gating CI: the adapter probe re-read on the lock move, its tally 14 · 115 · 0 unmoved; where a row holds that tally it stands.
@task_item n=10 part=touches
Stand checks repeated three rounds: stand-repeat 3 of 3 rounds green, its tally 258 · 1380 · 0 · 24 unmoved.
@task_item n=11 part=touches
The engine dependency the accessibility tree is laid out over: the Taffy pin moved from e90751fd to a3997d55, where an a11y-plan row names the engine pin.
@task_item n=1 part=untouched
ARIA role mapping, accessible name precedence, ARIA state and property reads and hidden-content exclusion: no role, name, state or attribute read changed.
@task_item n=2 part=untouched
Keyboard navigation contracts: range input keys, single and multiple select keys, the Cells grid, the CRUD list, Tab order and keyboard activation are unchanged.
@task_item n=3 part=untouched
Focus rules: what is focusable, focus fixup, focus-visible, inert exclusion from focus and focus on removal are unchanged.
@task_item n=4 part=untouched
The snapshot text form, state tokens, the snapshot diff, the actions answer, MCP tool results, served node ids and the UI graph: no node key, field, cause or verb was added.
@task_item n=5 part=untouched
The AccessKit adapter, its creation order, the platform adapter choice and the accessibility feature defaults: no feature or adapter changed.
@task_item n=6 part=untouched
WCAG contrast thresholds, the contrast ratio helper, the stand accent colour and the stand's contrast assertions: no colour or ratio moved.
@task_item n=7 part=untouched
Cognitive accessibility, password masking and the obs log format: the take adds no log site, span or field.
