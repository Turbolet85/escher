@task_prose part=chunk
Takes upstream's seven commits in upstream order onto build/escher-0.3.0: the reversed-scroll-origin change (#1294), its hit-test panic fix for elements with no computed styles yet (#1304), and the five Taffy bumps to upstream's pin a3997d55.
Moves the workspace's Taffy git rev from e90751fd to a3997d55, still a git source pinned at a full commit rev, the Taffy exception of the no-git-dependency invariant kept, the rev its only moving word.
Reconciles upstream's reversed scroll origins with escher's ratified readings: a scrolled box reads its own bounds where it stands, a hit stops at the padding box of every clipping box, absolute position subtracts only containing blocks' offsets, into-view scroll moves every scrolling box that holds the target.
Re-reads the take already measured in the dead run's measurement and staged patch (trial worktree target/tmp/trial-taffy-1372) and does not re-measure; sizes the take at P5.
@task_file n=1 path=Cargo.toml mark=edits
what: the workspace manifest, holding the Taffy git dependency pinned at a full commit rev — edits: the taffy rev line
@task_file n=2 path=Cargo.lock mark=conflict
what: the workspace lockfile, the Taffy git source line pinned at its rev; four bumps conflict here alone
@task_file n=3 path=packages/blitz-dom/src/document.rs mark=conflict
what: the engine's document, its stylo prefs set at init; the conflict resolves to escher's side, no sideways pref
@task_file n=4 path=packages/blitz-dom/src/layout/writing_mode.rs mark=conflict
what: the engine's physical layout geometry for a box and its placers, escher's own-scroll rule (a scrolled box reads its own bounds unshifted); the conflict takes upstream's fourth physical_layout argument, and a private scroll-container check reading false for a node with no computed styles is added at three sites
@task_file n=5 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the engine's node, its hit test (the self match keeping escher's own x/y, the padding-box stop of a clipping box) and its absolute position reading; the content test takes upstream's scrollable-overflow form
@task_file n=6 path=packages/blitz-dom/src/scrolling.rs mark=edits
what: the engine's scrolling, escher's into-view scroll through every scrolling box that holds the target — edits: scroll_into_view reading the scroll state as current, min and max, skipping a box whose min and max are both zero
@task_file n=7 path=packages/blitz-dom/src/debug.rs mark=merge
what: the engine's layout debug output, reached by upstream's scroll-origin pick
@task_file n=8 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: the engine's pointer event handling over hit results and scroll offsets, reached by upstream's picks
@task_file n=9 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: the engine's inline layout, measuring an inline box's scrollable overflow, reached by upstream's picks
@task_file n=10 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: the engine's layout entry over Taffy, its style getters and the Taffy trait implementations the bumps change
@task_file n=11 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: the engine's replaced-element layout over Taffy, reached by the bumps
@task_file n=12 path=packages/blitz-dom/src/mutator.rs mark=merge
what: the engine's document mutator, and upstream's unit test of hit testing before the first resolve
@task_file n=13 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the engine's scrollbar geometry over a box's scroll range, now able to run negative
@task_file n=14 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: the engine's display-contents layout tests, reached by upstream's picks
@task_file n=15 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: upstream's new test of an inline box's scrollable overflow, riding the pick
@task_file n=16 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: the engine's pre-element overflow scroll tests, reached by upstream's picks
@task_file n=17 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test runner, reached by the Taffy bumps
@task_item n=1 part=needs
The reversed scroll origin for every document: a box's content measured against Taffy's scrollable overflow rect, its scroll range able to run negative.
@task_item n=2 part=needs
A scrolled box reads its own bounds where it stands, with no subtraction of its own scroll offset; upstream's scroll-shifted readings do not enter (open list rows reading their own bounds rides on this).
@task_item n=3 part=needs
A hit stops at the padding box of every box that clips by overflow and reads the box's own x and y; the visible region of a target runs through every clipping box that holds it.
@task_item n=4 part=needs
A node's absolute position subtracts only its containing blocks' scroll offsets.
@task_item n=5 part=needs
Into-view scroll brings a target into view through every scrolling box that holds it, now reading each box's current, min and max scroll and skipping a box that cannot scroll.
@task_item n=6 part=needs
Hit testing an element that has no computed styles yet (before the first resolve, or a removed or inert select's open list) does not panic; a node with no computed styles reads as no scroll container.
@task_item n=7 part=needs
The Taffy dependency stays a git source pinned at a full commit rev, no patch section, no checksum, the dependency list unchanged: the no-git-dependency invariant's Taffy exception kept, its rev the only word that moves.
@task_item n=8 part=needs
The driver's answers (snapshot bounds, off-screen refusals, actions resolved by hit) change only where the engine's readings change, and the measurement reads none changed on the stand.
@task_item n=9 part=needs
No new carrier, verb, argument, result field, node key, cause, session error, exit, log field or span.
@task_item n=10 part=needs
The stand's committed cross-OS readings and the committed Counter checks pass unchanged; the stand-repeat and accessibility tallies stay unmoved.
@task_item n=11 part=needs
Only the seven named upstream commits enter, in upstream order; the later upstream sync stays its own chunk.
@task_item n=12 part=needs
The take is read from the earlier run's measurement and staged patch, not re-measured; the founder's ratification of the scroll behaviour change binds this take alone.
@task_item n=1 part=not_needed
The window chrome controls of the desktop shell provider: the take moves no window decoration, drag, maximize, minimize or close behaviour.
@task_item n=2 part=not_needed
The stand's stable element id register and its every-control-keyed rule: no id is added, renamed or removed.
@task_item n=3 part=not_needed
The agent-run and cold-agent test pipes, their usage, exits, run log and Windows pass-through: no script changes.
@task_item n=4 part=not_needed
The multi_doc and paint_bench command lines, their arguments, environment knobs and stdout: no benchmark changes.
@task_item n=5 part=not_needed
The MCP server's transport, protocol revisions, tool list and result bound, and the session exit codes: no protocol or exit changes.
@task_item n=6 part=not_needed
The session coverage role, recording directory and check files, and the graph, lint and help answers: none of their shapes move.
@task_item n=7 part=not_needed
The stand's accent colour, document skeleton and served task list, and the load page's network provider: styling and serving are untouched.
@task_item n=8 part=not_needed
The mobile-native and web-spa surfaces: not yet measured and not reached by an engine dependency take.
