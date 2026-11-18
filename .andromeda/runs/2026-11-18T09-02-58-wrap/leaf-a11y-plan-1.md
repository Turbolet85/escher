@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=26 class=omission
leaf-text: and other mutations mark it
source: dec-3xjtbjfc «The engine's changed set marks a mutation of an in-document node, a focus or checked change, a text control's input, a select's changed choice and a single select's list opening or closing, a scroll»
replace-with: and other mutations of an in-document node mark it, while node creation, hover, active, any other scroll and layout do not
