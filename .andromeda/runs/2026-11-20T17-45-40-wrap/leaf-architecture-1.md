@leaf_group group=architecture-1
@leaf_read leaf=.claude/docs/conventions.md lines=61
last: - Architecture: `doc:architecture` (the records store) · dir
@leaf_read leaf=.claude/docs/services/blitz-dom.md lines=44
last: - `doc:architecture` (the records store) · `.claude/docs/sta
@leaf_read leaf=.claude/docs/services/blitz-paint.md lines=35
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-shell.md lines=34
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_read leaf=.claude/docs/services/blitz-traits.md lines=33
last: - `doc:architecture` (the records store) · `.claude/docs/ser
@leaf_edit n=1 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: Its limits: the thumb is painted and not draggable, a highlighted row's contrast is unread by the lint,
source: ctr-s5m2lrhi «an open list's highlighted row is read at its painted pair, the engine's highlight colours»
replace-with: Its limit: the thumb is painted and not draggable; the lint reads an open list's highlighted row at its painted pair, the engine's highlight colours,
@leaf_edit n=2 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: and the highlight is absent from the snapshot and the tree.
source: ctr-2uf3nocb «While a single select's list stands open with a current option, the tree names that option as the select's active descendant, and none while closed; the tree's focus stays on the select»
replace-with: and while a single select's list stands open with a current option the tree names that option as the select's active descendant, and none while closed, the tree's focus staying on the select and a page's `aria-activedescendant` never read.
@leaf_edit n=3 leaf=.claude/docs/services/blitz-dom.md line=19 class=stale
leaf-text: and a highlight or a wheel move inside the list marking nothing
source: dec-vvnxavu4 «a highlight move to another row, by key or pointer, marks it; a move to the same row and a wheel move inside the list mark nothing.»
replace-with: a highlight move to another row, by key or pointer, marking it, and a move to the same row or a wheel move inside the list marking nothing
@leaf_edit n=4 leaf=.claude/docs/services/blitz-dom.md line=15 class=stale
leaf-text: a larger authored font clips in the one row height,
source: dec-2qmhfbqn «An open select list's rows take one height, the tallest row's line with the select's own one-line height as the floor, so a row with a larger authored font is painted whole.»
replace-with: its rows take one height, the tallest row's line with the select's own one-line height as the floor, so a row with a larger authored font is painted whole,
