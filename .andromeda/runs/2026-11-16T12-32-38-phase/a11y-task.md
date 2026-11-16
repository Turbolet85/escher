@task_prose part=chunk
Takes seven upstream commits in upstream order onto the build branch: the reversed-scroll-origin change (#1294), the hit test of elements with no computed styles yet (#1304), and five Taffy bumps that move the workspace's git Taffy pin from e90751fd to upstream's a3997d55.
Reconciles upstream's scroll origins (content measured against Taffy's ScrollableOverflow rect, a scroll range that can run negative) with escher's ratified readings: a scrolled box's own bounds, the padding-box hit stop with own_x/own_y, absolute_position subtracting only containing blocks' offsets, and into-view scroll through every scrolling box.
Adds three seam edits beyond the picks: a dropped stylo_taffy import, scroll_into_view reading scroll_state as a (current, min, max) triple, and a private is_scroll_container guard that reads false for a node with no computed styles.
Re-reads the take already measured in the dead run dir (measurement and take patch, trial worktree target/tmp/trial-taffy-1372) and does not re-measure it; the take is sized at P5.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace manifest, holding the Taffy git dependency, the one exception of the no-git-dependency invariant — edits: the taffy rev line
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lock file, holding the Taffy git source line pinned at a full commit rev
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=conflict
what: the engine's document, holding the stylo prefs set in init_stylo_prefs
@task_file n=4 path=packages/blitz-dom/src/layout/writing_mode.rs mark=conflict
what: the engine's physical layout geometry, holding physical_layout, physical_unrounded_geometry, placer_geometry, escher's own-scroll rule and the new private is_scroll_container guard
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the engine's node, holding hit_inner's hit stop and self match, and Node::absolute_position
@task_file n=6 path=packages/blitz-dom/src/scrolling.rs mark=edits
what: the engine's scrolling, holding escher's into-view scroll and its read of each box's scroll state — edits: scroll_into_view
@task_file n=7 path=packages/blitz-dom/src/debug.rs mark=merge
what: the engine's layout debug dump
@task_file n=8 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the engine's pointer event handling, which hits a point and dispatches pointer and click events
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout, sizing inline boxes and their scrollable overflow
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout bridge to Taffy, its style getters and the scrollable overflow it stores per node
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's replaced-element layout
@task_file n=12 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the engine's document mutator, and its unit test of hit testing before the first resolve
@task_file n=13 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the engine's scrollbar geometry, its shown scrollbar bands and thumbs against a box's scroll range
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the layout test of display: contents boxes
@task_file n=15 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: upstream's new layout test of an inline box's scrollable overflow
@task_file n=16 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the layout test of a pre element's overflow scroll
@task_file n=17 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test runner, reading layout geometry
@task_item n=1 part=needs
Hit testing: a hit still stops at the padding box of every box that clips by overflow and reads own_x/own_y; upstream's content test enters in its new form, the point against the scrollable overflow rect's four sides plus the client offset, and an element with no computed styles yet can now be hit before the first resolve without a panic.
@task_item n=2 part=needs
An inert element still takes no hit: the hit-testing inert promise must hold through the reworked hit path and the self match.
@task_item n=3 part=needs
Element bounds the snapshot and the driver read: a scrolled box reads its own bounds where it stands with no subtraction of its own scroll offset, and absolute_position subtracts only its containing blocks' offsets, so a box's bounds, a closed select's option bounds, an open list's row bounds and a multiple select's row bounds read unchanged on the stand.
@task_item n=4 part=needs
The driver's synthetic click lands at the node's centre computed from those bounds, and its covered refusal reads the hit at that point; both must read as before where the engine's readings do not change.
@task_item n=5 part=needs
Scroll into view: escher's scroll_into_view brings a target into view through every scrolling box that holds it, now reading each box's scroll state as current, min and max and skipping a box whose min and max are both zero; a scroll range may run negative under reversed origins.
@task_item n=6 part=needs
Shown scrollbars: the scrollbar band and thumb geometry moves onto the new scroll range, and a driver click in a shown scrollbar band still drags its thumb; the default thumbs' contrast stroke is unaffected in kind.
@task_item n=7 part=needs
A node with no computed styles: the new scroll-container read must read false rather than panic, at each of its three call sites, so the select list test where a select made inert, disabled, hidden or removed with its list open closes the list passes under the workspace's features.
@task_item n=8 part=needs
The open select list overlay and its rows keep one row height, their engine colours and their bounds through the layout change; the list's close on inert or removal keeps its focus behaviour.
@task_item n=9 part=needs
No new carrier, verb, argument, result field, node key, cause, session error, exit, log field or span: the driver's answers, the snapshot and the accessibility tree change only where the engine's readings change, and the measurement reads none changed.
@task_item n=10 part=needs
Upstream's scroll-shifted readings in physical_unrounded_geometry's second body and in hit_inner's self match must not enter; escher's own-scroll rule, its hit stop and its absolute_position reading are kept.
@task_item n=11 part=needs
The take is read back, not re-measured: every leg measured green in the trial (fast, msrv, audit, doc, build, counter, wasm, a11y, stand-repeat three of three rounds), the a11y and stand-repeat tallies unmoved, the committed cross-OS stand digests and Counter checks passing unchanged; coverage and the WPT suites were not run, and the cross-OS digests read only linux locally.
@task_item n=12 part=needs
The stand reads no lint finding at boot and its contrast assertions hold with the moved layout, since the lint's contrast finding carries bounds read from the same geometry.
@task_item n=13 part=needs
No other upstream commit enters; the later upstream sync stays its own chunk.
@task_item n=1 part=not_needed
Keyboard navigation contracts (range input keys, single-choice and multiple select keys, CRUD list and Cells grid keys, Tab order, keyboard activation): the take changes no key handling.
@task_item n=2 part=not_needed
ARIA property reads, ARIA states, role mapping and accessible name precedence: no attribute read, role or name rule changes.
@task_item n=3 part=not_needed
The platform accessibility adapter, the accessibility feature defaults and the AccessKit adapter's creation order: no feature, dependency or adapter wiring other than the Taffy rev moves.
@task_item n=4 part=not_needed
Hidden content and aria-hidden exclusion from the accessibility tree: the take changes geometry, not which nodes the tree holds.
@task_item n=5 part=not_needed
Cognitive accessibility rows (invalid state beyond colour or class, the shared stand Tab order table): untouched.
@task_item n=6 part=not_needed
UI graph state registration and the Dioxus document's reached-handler record: no handler or state registration changes.
@task_item n=7 part=not_needed
The snapshot text form's parts and state tokens, the actions answer's fields and causes, and served node id separators: no wire form changes.
@task_item n=8 part=not_needed
Password masking, IME enabling and password caret placement: no text input behaviour changes.
