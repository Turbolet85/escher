@task_prose part=session
The session took upstream Blitz past #1372 by seven cherry-picks: reversed scroll origins on Taffy's ScrollableOverflow, hit testing of elements with no computed styles yet, and five Taffy bumps.
Taffy moved from git rev e90751fd to upstream main rev a3997d55, version 0.15.0 and features unchanged; it is still the workspace's one git source.
Behaviour change for every document, remote pages included: a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) now scrolls to negative offsets; the founder ratified it for this take.
The scroll state became a current with a min and a max; the programmatic scroll clamps between them; scroll into view skips a box whose min and max are both zero; scrollWidth and scrollHeight answer the CSSOM scroll size.
The hit walk's clip stop now answers in the content-box frame, as the final self hit and the text hits do, resolving the textual conflict between upstream's hit hunk and the non-invertible-transform answer; Node gained a public read-only layout-data accessor.
Escher's own-scroll rule for client geometry was kept against upstream's scroll-shifted form; the Dioxus mounted-data scroll size stays on Taffy's extent; no verb, carrier, log field, span or name was added.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest, where dependencies are declared once, holding the Taffy engine pin as a git revision
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lock file, holding the one git dependency source, Taffy's
@task_file n=3 path=packages/blitz-dom/src/debug.rs mark=merge
what: blitz-dom's debug printing of nodes and their layout data
@task_file n=4 path=packages/blitz-dom/src/events/pointer.rs mark=merge
what: blitz-dom's pointer event handling, including touch-action pan axes read from a box's scroll size
@task_file n=5 path=packages/blitz-dom/src/layout/inline/layout.rs mark=merge
what: blitz-dom's inline layout of inline roots and their inline boxes, floats and scrollable overflow
@task_file n=6 path=packages/blitz-dom/src/layout/mod.rs mark=merge
what: blitz-dom's layout pass, the implementer of Taffy's traversal, layout and cache traits
@task_file n=7 path=packages/blitz-dom/src/layout/replaced.rs mark=merge
what: layout of the replaced elements, their intrinsic and min-content size contributions
@task_file n=8 path=packages/blitz-dom/src/layout/writing_mode.rs mark=merge
what: writing-mode physical geometry, the overflow mirroring of a box and the own-scroll rule of client geometry
@task_file n=9 path=packages/blitz-dom/src/mutator.rs mark=merge
what: DocumentMutator, through which DOM mutations go, and its unit tests of hit testing
@task_file n=10 path=packages/blitz-dom/src/node/node.rs mark=conflict
what: the Node, its hit test walk and clip stop, its scroll width and height, scroll range and layout-data accessor
@task_file n=11 path=packages/blitz-dom/src/node/scrollbar.rs mark=merge
what: the scrollbar geometry shared by paint and hit testing, its thumb over the scroll range
@task_file n=12 path=packages/blitz-dom/src/scrolling.rs mark=merge
what: the one scrolling primitive, the programmatic scroll clamp, scroll into view and the smooth scroll animation
@task_file n=13 path=tests/blitz-tests/tests/display_contents.rs mark=merge
what: a blitz-tests integration test target for display contents
@task_file n=14 path=tests/blitz-tests/tests/inline_box_scrollable_overflow.rs mark=merge
what: a blitz-tests integration test target for scrollable overflow and scroll dimensions, now with reversed-origin cases
@task_file n=15 path=tests/blitz-tests/tests/pre_overflow_scroll.rs mark=merge
what: a blitz-tests integration test target for overflow scrolling of preformatted text
@task_file n=16 path=wpt/runner/src/test_runners/attr_test.rs mark=merge
what: the WPT runner's attribute test runner, the checkLayout attribute checks including scroll sizes
@task_item n=1 part=touches
decision/deferred-upstream-dependency-bumps: the Taffy deferral ended; Taffy is taken from #1372 on at a3997d55, so the row's Taffy clause is stale (Stylo and Dioxus untouched).
@task_item n=2 part=touches
decision/upstream-take-engine-changes: this take adds an engine behaviour change, reversed scroll origins scrolling to negative offsets, plus hit testing of unstyled elements.
@task_item n=3 part=touches
decision/upstream-behaviour-changes-ratified-per-take: the reversed-origin change was ratified for this take by the founder.
@task_item n=4 part=touches
decision/upstream-taken-by-cherry-pick and decision/upstream-sync-merge-base: seven upstream commits taken by cherry-pick with their trailers; a carried hit-hunk conflict is now resolved.
@task_item n=5 part=touches
decision/no-git-dependencies and decision/taffy-layout: Taffy stays the one git source, its pinned revision moved e90751fd to a3997d55; any row holding that pin value is stale.
@task_item n=6 part=touches
decision/hit-stops-at-paint-clip and decision/hit-stops-at-clip-padding-box: the clip stop's early answer now reads in the content-box frame; only an inline root's answer moves.
@task_item n=7 part=touches
contract/scroll-hit/hit-clip, contract/scroll-hit/hit-with-scrollbar and contract/scroll-hit/hit-dropped-node: the hit walk's content test takes upstream's form over the overflow rect's four sides; elements with no computed styles yet are hit tested.
@task_item n=8 part=touches
decision/non-invertible-transform-takes-no-hit and contract/scroll-hit/hit-degenerate-transform: the upstream hit hunk that conflicted with this answer is resolved with the answer kept.
@task_item n=9 part=touches
contract/scroll-hit/scroll-into-view and decision/scroll-into-view-whole-chain: scroll into view reads the scroll state's current, min and max and skips a box whose min and max are both zero.
@task_item n=10 part=touches
decision/programmatic-scroll-one-box and decision/one-scrolling-primitive: the programmatic scroll clamps between a min and a max, the min negative for a reversed box.
@task_item n=11 part=touches
decision/shared-scrollbar-geometry: the scrollbar thumb runs over the signed scroll range.
@task_item n=12 part=touches
decision/overflow-auto-as-scroll and decision/overflow-hidden-programmatic-only: the scroll container test now answers false for a node with no computed styles; the scrollable extent comes from Taffy's scrollable overflow.
@task_item n=13 part=touches
decision/client-rect-ignores-own-scroll, decision/scrolled-box-bounds and decision/absolute-position-containing-blocks: escher's own-scroll rule kept against upstream's scroll-shifted geometry body and self match.
@task_item n=14 part=touches
decision/writing-mode-off-by-default and decision/writing-mode-opt-in: the writing-mode geometry gained overflow mirroring and a scroll container argument to physical layout.
@task_item n=15 part=touches
contract/layout/taffy-traits: the layout pass's cache get now takes a shared reference, as Taffy's cache trait asks.
@task_item n=16 part=touches
decision/crate-internal-visibility and decision/restricted-crate-internals: blitz-dom's public API widened by one read-only Node layout-data accessor; the scroll range stays crate-internal.
@task_item n=17 part=touches
decision/both-layout-modes and decision/scenarios-both-layout-modes: upstream's six reversed-origin integration tests are kept as written in one layout mode, an exception.
@task_item n=18 part=touches
decision/replaced-elements: percentage-width replaced elements' min-content contributions follow Taffy's fix.
@task_item n=19 part=touches
contract/wpt-attribute-tests/checklayout-attributes: the WPT runner's scroll size checks read the node's CSSOM scroll size.
@task_item n=20 part=touches
decision/user-scroll-chaining and contract/dom-events/pointer-events: touch-action pan axes and user scroll read the scroll width against the client width and the signed range.
@task_item n=21 part=touches
decision/resolve-before-js-geometry and decision/cssom-geometry-snapping: a page's scrollWidth and scrollHeight now answer the CSSOM scroll size, reversed origins included.
@task_item n=22 part=touches
decision/layout-modes-identical and decision/deferred-inline-layout: inline layout picks up Taffy's float insets, zero-height float segments and no double counting of collapse-through margins.
@task_fact n=1 slug=established-decisions
fact: A box's content is measured against Taffy's scrollable overflow for every document, so a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets; ratified per take by the founder.
@task_fact n=2 slug=established-decisions
fact: A scroll state is a current with a min and a max; the programmatic scroll clamps between them and the scrollbar thumb runs over that signed range.
@task_fact n=3 slug=established-decisions
fact: The hit walk's clip stop answers in the content-box frame, the frame of the final self hit and the text hits; only an inline root's answer differs from the border frame.
@task_fact n=4 slug=established-decisions
fact: Elements with no computed styles yet are hit tested, and such a node is never a scroll container.
@task_fact n=5 slug=established-decisions
fact: scrollWidth and scrollHeight answer the CSSOM scroll size, reversed origins included, while the Dioxus mounted-data scroll size stays on Taffy's extent.
@task_fact n=6 slug=established-decisions
fact: Escher keeps its own-scroll rule for client geometry and self hits, and keeps out upstream's scroll-shifted geometry body and scroll-shifted self match.
@task_fact n=7 slug=standard-contracts
fact: Scroll into view skips a box whose scroll min and max are both zero.
@task_fact n=8 slug=standard-contracts
fact: Node exposes a public read-only layout-data accessor that answers none before the first layout.
@task_fact n=9 slug=conventions
fact: Upstream's taken integration tests are kept as written, one layout mode, an exception to scenarios in both layout modes.
@task_item n=1 part=untouched
The driver's verbs, arguments, refusal causes, node keys, answer JSON, session wire, MCP role and command line: no verb, argument, result field, cause or exit was added.
@task_item n=2 part=untouched
Telemetry: no log site, span, field, target or scrub rule was added or moved.
@task_item n=3 part=untouched
The accessibility tree and AccessKit adapter contracts: the a11y leg stayed green and unmoved.
@task_item n=4 part=untouched
The open select list, select readers, form controls and their default actions.
@task_item n=5 part=untouched
CI gates, legs, coverage floors, stand-repeat budget and the harness surface: no gate or threshold moved; test tallies are held by no row.
@task_item n=6 part=untouched
Stylo, Dioxus, Parley, anyrender and every other dependency or toolchain pin: only Taffy moved.
@task_item n=7 part=untouched
The names sheet and occupied resources: no binary, crate, env var, listener, socket or directory was taken or freed.
@task_item n=8 part=untouched
Stable element ids, the snapshot, its diff, lint, the UI graph and kept checks.
@task_item n=9 part=untouched
The browser app, its history and persistence, networking, the script engine's globals and fetchers.
@task_item n=10 part=untouched
Paint scene emission, text decoration, paint layer and gradient caps.
