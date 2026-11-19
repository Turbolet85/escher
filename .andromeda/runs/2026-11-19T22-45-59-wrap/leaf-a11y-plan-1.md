@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=20 class=omission
leaf-text: Hidden: `hidden` / `display:none` / `visibility:hidden` exclude the subtree
source: dec-3r272kl2 «An aria-hidden true node is kept but marked hidden, while hidden, inert, display none, visibility hidden and visibility collapse exclude the node and its children from the accessibility tree.»
replace-with: Hidden: `hidden` / `display:none` / `visibility:hidden` / `visibility:collapse` exclude the subtree
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=26 class=stale
leaf-text: rebuilt on a poll that reported work when the document's changed set, taken by that poll, was non-empty
source: dec-3ytgccnd «The accessibility tree is built from the document when first requested and pushed again in a redraw, after style resolve, only when the changed set is non-empty, the set then cleared; a poll pushes no tree.»
replace-with: pushed again in a redraw, after style resolve, only when the document's changed set, taken and cleared by that redraw, was non-empty, a poll pushing no tree
