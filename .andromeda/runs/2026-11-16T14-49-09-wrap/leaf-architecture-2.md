@leaf_group group=architecture-2
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/stack.md
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/servic
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/servic
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=37
last: - `.claude/rules/verification-harness.md` · `.claude/docs/tests-
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/servic
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=12 class=retired
leaf-text: pinned to one revision before #1372 since 2026-11-15-taffy-from-upstream-git
source: dec-vu7h54n4 «Deferred Stylo, Taffy and Dioxus bumps — Stylo stays "0.22.0", Taffy upstream's git source pinned before #1372 and Dioxus "0.7.3": git Stylo 0.23-pre and Dioxus 0.8.0-alpha.3 wait for escher's next version»
replace-with: pinned to one revision, from #1372 on at a3997d55 since 2026-11-16-taffy-past-1372
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: scrolls every scrolling box that holds its target, innermost first, then the viewport
source: ctr-zara73ad «scroll_into_view scrolls every scrolling box on the target's chain, skipping a box whose scroll min and max are both zero, nested ones instantly with no scroll event, then the viewport»
replace-with: scrolls every scrolling box that holds its target, innermost first, skipping a box whose scroll min and max are both zero, then the viewport
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: stops at the padding box of every box paint clips
source: ctr-zalih6mp «A hit stops at the padding box of a box paint clips, and a hit stopped there answers in that box's content-box frame, the frame of the final self hit and the text hits.»
replace-with: stops at the padding box of every box paint clips, a hit stopped there answering in that box's content-box frame, the frame of the final self hit and the text hits,
@leaf_edit n=4 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: and skips a node dropped since the last resolve
source: dec-7nlzozlo «Elements without computed styles are hit tested — Elements with no computed styles yet are hit tested, and such a node is never a scroll container.»
replace-with: hit tests an element with no computed styles yet, never a scroll container, and skips a node dropped since the last resolve
@leaf_edit n=5 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: `Node::absolute_position` (only
source: ctr-gr7trxoq «Node::try_layout_data — Public and read-only: a node's layout data, None before the first layout.»
replace-with: `Node::try_layout_data` (public and read-only, a node's layout data, `None` before the first layout), `Node::absolute_position` (only
@leaf_edit n=6 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: scroll API (
source: dec-a7dzvu6k «A box's content is measured against Taffy's scrollable overflow for every document, so a reversed box (RTL, row-reverse, column-reverse, wrap-reverse) scrolls to negative offsets.»
replace-with: scroll API (a box's content measured against Taffy's scrollable overflow, so a reversed box — RTL, row-reverse, column-reverse, wrap-reverse — scrolls to negative offsets, `scroll_to` and `scroll_by` clamping between its scroll minimum and maximum; 
@leaf_edit n=7 leaf=.claude/docs/services/blitz-dom.md line=15 class=omission
leaf-text: CSSOM (the resolved
source: dec-3xbvryey «scrollWidth and scrollHeight answer the CSSOM scroll size, reversed origins included, while the Dioxus mounted-data scroll size stays on Taffy's extent.»
replace-with: CSSOM (`scrollWidth` and `scrollHeight` answer the CSSOM scroll size, reversed origins included, while the Dioxus mounted-data scroll size stays on Taffy's extent; the resolved
@leaf_edit n=8 leaf=.claude/docs/services/blitz-test-harness.md line=19 class=omission
leaf-text: every scrolling box that holds the node, then the viewport
source: ctr-zara73ad «scroll_into_view scrolls every scrolling box on the target's chain, skipping a box whose scroll min and max are both zero, nested ones instantly with no scroll event, then the viewport»
replace-with: every scrolling box that holds the node, a box whose scroll min and max are both zero skipped, then the viewport
