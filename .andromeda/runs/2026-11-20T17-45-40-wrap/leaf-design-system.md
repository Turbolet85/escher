@leaf_group group=design-system
@leaf_read leaf=.claude/docs/design-summary.md lines=44
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=24 class=stale
leaf-text: an open select list's row height is the select's font on one line, a larger authored font clipping, a standing limit
source: dec-2qmhfbqn «An open select list's rows take one height, the tallest row's line with the select's own one-line height as the floor, so a row with a larger authored font is painted whole.»
replace-with: an open select list's rows take one height, the tallest row's line with the select's own one-line height as the floor, so a row with a larger authored font is painted whole
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=27 class=stale
leaf-text: the highlight in neither the snapshot nor the tree
source: ctr-j6l2vmng «The highlighted row of a single select's open list alone carries the snapshot's closed bool highlighted, a bare token after clickable in the text and true in JSON, written only when true.»
replace-with: the highlighted row alone carrying the snapshot's closed bool `highlighted` (a bare token after `clickable` in the text, `true` in JSON, written only when true) and the tree naming it the select's active descendant, a page's `aria-activedescendant` not read
