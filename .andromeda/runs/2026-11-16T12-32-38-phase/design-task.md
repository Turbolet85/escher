@task_prose part=chunk
Takes seven upstream commits, in upstream order, onto the build branch: the reversed scroll origins change (#1294), its hit test fix for elements with no computed styles yet (#1304), and five Taffy bumps toward upstream's Taffy pin a3997d55.
Moves the workspace's Taffy from git e90751fd to a3997d55, still a git source pinned at a full rev, its rev the only word that moves.
Reconciles upstream's scroll-origin change with escher's ratified readings: a scrolled box's own bounds, the hit stop at the padding box of a clipping box, absolute position and the into-view scroll.
Adds three seam edits so the take compiles and the select popup test stops panicking; upstream's tests ride in with the picks.
The take is already measured in a standing trial worktree and is not re-measured; it is sized at P5.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace manifest, holding the Taffy git dependency pinned at a full commit rev (the Taffy exception of the no-git-dependency invariant) — edits: the taffy rev line
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lockfile, holding Taffy's git source line pinned at its rev
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=conflict
what: the engine's document, holding its stylo preferences set up in init_stylo_prefs
@task_file n=4 path=packages/blitz-dom/src/layout/writing_mode.rs mark=conflict
what: the engine's physical layout reading of a box's geometry, holding escher's own-scroll rule and the placers' geometry, and the new private is_scroll_container read of a node with no computed styles
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the engine's node, holding the hit test (hit_inner and its self match against the padding box and the scrollable overflow rect) and Node::absolute_position
@task_file n=6 path=packages/blitz-dom/src/scrolling.rs mark=edits
what: the engine's scrolling, holding escher's into-view scroll through every scrolling box that holds a target — edits: scroll_into_view, its read of the scroll state as current, min and max
@task_file n=7 path=packages/blitz-dom/src/debug.rs mark=merge
what: the engine's debug printing of a node's layout
@task_file n=8 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the engine's pointer event handling, reading hits and scroll offsets
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout, measuring an inline box's scrollable overflow
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout bridge to Taffy, its tree traits and its cache
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's layout of replaced elements
@task_file n=12 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the engine's DOM mutator, with its unit test of hit testing before the first resolve
@task_file n=13 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the engine's overlay scrollbar geometry over a scrolling box's scroll range
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the engine's layout tests of display contents
@task_file n=15 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: upstream's new engine test of an inline box's scrollable overflow
@task_file n=16 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the engine's tests of a pre element's overflow scroll
@task_file n=17 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test runner, reading the engine's layout
@task_item n=1 part=needs
A scrolled box reads its own bounds where it stands, with no subtraction of its own scroll offset, while upstream's content is measured against Taffy's scrollable overflow rect and its scroll range may run negative; the snapshot's bounds of a node are what this reading feeds.
@task_item n=2 part=needs
Hit-testing stops where paint clips: a hit stops at the padding box of every box that clips by overflow and reads that box's own position; the content test takes upstream's form, the point against the scrollable overflow rect's four sides plus the client offset.
@task_item n=3 part=needs
A node's absolute position subtracts only its containing blocks' scroll offsets, never its own.
@task_item n=4 part=needs
Into-view scroll brings a target into view through every scrolling box that holds it, instantly in nested boxes; it now reads each box's scroll state as current, min and max and skips a box whose min and max are both zero.
@task_item n=5 part=needs
Driver scroll stays instant and the driver's answers change only where the engine's readings change; the measurement reads none changed on the stand.
@task_item n=6 part=needs
Hit testing an element that has no computed styles yet answers without panicking, both upstream's fix and escher's own read that treats such a node as no scroll container.
@task_item n=7 part=needs
The open select list closes when its select is made inert, disabled, hidden or removed with its list open, without the new overflow read panicking on a node with no computed styles.
@task_item n=8 part=needs
Overlay scrollbars read their geometry from a scrolling box's scroll range, which may now run negative under reversed scroll origins.
@task_item n=9 part=needs
Taffy stays the only git dependency, pinned at a full commit rev, with no patch section and the dependency list unchanged.
@task_item n=10 part=needs
The take adds no new carrier, verb, argument, result field, node key, cause, session error, exit, log field or span.
@task_item n=11 part=needs
The committed cross-OS stand digests and the Counter task's committed checks pass unchanged after the take.
@task_item n=12 part=needs
Stacking-context roots still group paint and hit-testing, and an open select list still answers hits first, through the reconciled hit test.
@task_standing n=1
convention: each engine test the take brings in is built with system fonts, as the engine's other tests are
home: typography
@task_item n=1 part=not_needed
Colour rows: the engine's hardcoded colours, colour-scheme recascade and select list colours; the take changes no colour.
@task_item n=2 part=not_needed
Spacing rows: body margins, table cell spacing hints and the select list's placement in the viewport; no spacing value moves.
@task_item n=3 part=not_needed
Iconography rows: cursor icons, cursor resolution and favicons.
@task_item n=4 part=not_needed
The escher-session command, exit status, stdout, stderr, mcp role and tasks contracts, and the actions, coverage and UI graph lines; the take adds no verb, argument, field or exit.
@task_item n=5 part=not_needed
WPT runner output and result colour rows; the runner's printing is untouched.
@task_item n=6 part=not_needed
The shell provider's window controls, the JS window object's sizes and the dioxus-native configuration rows.
@task_item n=7 part=not_needed
Select list events and the select's accessibility rows: how a select opens, chooses and announces is unchanged.
@task_item n=8 part=not_needed
Brand identity, border radius, the anti-patterns, self-validation and decisions-log sections, and the web-spa surface; nothing of them is recorded or reached.
