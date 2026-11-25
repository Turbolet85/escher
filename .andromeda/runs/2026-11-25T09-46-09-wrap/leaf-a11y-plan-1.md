@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=23 class=omission
leaf-text: a driver `click` on an element under the list refused `covered`
source: ctr-26pvuvyx «A driver click on an element under an open select list is refused `covered`; a virtual node reads `covered` only when an element lies over its region's centre.»
replace-with: a driver `click` on an element under the list refused `covered`, a virtual node reading `covered` only when an element lies over its region's centre
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=24 class=stale
leaf-text: one `GraphNode` per snapshot node in pre-order
source: dec-45n3iy3a «The read-only UI graph holds one node per snapshot node but a canvas's virtual nodes, which it excepts, so the actions answer and exploration hold none of them; names and closed words only.»
replace-with: one `GraphNode` per snapshot node in pre-order but a canvas's virtual nodes, which it excepts, so the actions answer and exploration hold none of them
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=24 class=omission
leaf-text: The snapshot reads `enabled` by presence
source: ctr-3abkoz7e «The snapshot's enabled field reads the presence of the element's disabled attribute, and on a virtual node the declaration's disabled flag, enabled false where declared disabled.»
replace-with: The snapshot reads `enabled` by presence (on a virtual node the declaration's disabled flag, `enabled` false where declared disabled)
@leaf_edit n=4 leaf=.claude/docs/a11y-summary.md line=24 class=omission
leaf-text: which reads one canvas's declared nodes, are no readers of the snapshot model.
source: ctr-2peziqew «The form {owner}///{key} is reserved for a canvas's virtual nodes and served in the accessibility tree and, from it, the snapshot and its diff; no element's id holds ///.»
replace-with: which reads one canvas's declared nodes, are no readers of the snapshot model; a canvas's virtual nodes are served by the reserved id form `{owner}///{key}` in the accessibility tree and, from it, the snapshot and its diff, no element's id holding `///`.
