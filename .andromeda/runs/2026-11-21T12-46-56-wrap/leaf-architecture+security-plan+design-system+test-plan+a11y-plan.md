@leaf_group group=architecture+security-plan+design-system+test-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=166
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=.claude/docs/gotchas.md line=27 class=retired
leaf-text: a focused node made inert gives focus up at the flush with no blur or focusout, while one made disabled or hidden keeps focus and takes keys as the tree's focus names the window (a named SC 2.4.3 gap).
source: dec-fptknbzy «A focused node gives focus up with no DOM blur or focusout, in every document: at the flush when it becomes inert, and after each layout when it no longer shows; a display: contents control and one beside a hidden sibling keep focus.»
replace-with: a focused node gives focus up with no blur or focusout, at the flush when made inert and after the next layout when it no longer shows, its key acting only once re-focused; a `display: contents` control and one beside a hidden sibling keep focus.
@leaf_edit n=2 leaf=.claude/docs/gotchas.md line=31 class=omission
leaf-text: while a move to the same row or a wheel move inside the list marks nothing.
source: dec-ut2clwr7 «a page scroll or a move that closes an open select list marks its select at the next resolve; a focus given up after a layout marks its node»
replace-with: while a move to the same row or a wheel move inside the list marks nothing, and a focus given up after a layout marks its node.
@leaf_edit n=3 leaf=CLAUDE.md line=122 class=stale
leaf-text: a seed reaching a run only from in-process code that builds its VirtualDom — the absence ratified
source: dec-uemnwvfz «a seed reaches a run only from in-process code building its VirtualDom or script document.»
replace-with: a seed reaching a run only from in-process code that builds its VirtualDom or script document (`ScriptDocument::with_seed`, seeding its `Math.random`) — the absence ratified
