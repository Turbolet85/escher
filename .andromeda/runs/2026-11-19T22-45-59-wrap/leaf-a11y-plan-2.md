@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=24 class=stale
leaf-text: `hidden`, `display: none`, `visibility: hidden` exclude a node and its children
source: dec-3r272kl2 «An aria-hidden true node is kept but marked hidden, while hidden, inert, display none, visibility hidden and visibility collapse exclude the node and its children from the accessibility tree.»
replace-with: `hidden`, `display: none`, `visibility: hidden`, `visibility: collapse` exclude a node and its children
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=27 class=stale
leaf-text: on InitialTreeRequested and, on a poll that reported work, when the document's changed set taken by that poll was non-empty
source: dec-3ytgccnd «The accessibility tree is built from the document when first requested and pushed again in a redraw, after style resolve, only when the changed set is non-empty, the set then cleared; a poll pushes no tree.»
replace-with: on InitialTreeRequested and, in a redraw after style resolve, when the document's changed set taken there was non-empty, a poll pushing no tree
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=27 class=stale
leaf-text: before 2026-10-07 the poll-time refresh never ran
source: dec-4hym5ykk «The accessibility tree is built from the document on the initial tree request and rebuilt in a redraw after style resolve when the changed set is non-empty, both paths going through the document trait's accessibility-tree method.»
replace-with: before 2026-10-07 the refresh on change never ran
