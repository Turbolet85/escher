@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=24 class=stale
leaf-text: one `GraphNode` per snapshot node in pre-order but a canvas's virtual nodes, which it excepts, so the actions answer and exploration hold none of them
source: dec-45n3iy3a «The read-only UI graph holds one node per snapshot node, a canvas's virtual nodes among them, each owned by its canvas with no handlers or reads, so the actions answer and exploration hold them too»
replace-with: one `GraphNode` per snapshot node in pre-order, a canvas's virtual nodes among them, each owned by its canvas with no handlers or reads, so the actions answer and exploration hold them too
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=24 class=omission
leaf-text: in the accessibility tree and, from it, the snapshot and its diff, no element's id holding `///`
source: ctr-2peziqew «served in the accessibility tree and, from it, the snapshot and its diff, the UI graph, the actions answer and the layout answer, these three never carrying the node's declared name»
replace-with: in the accessibility tree and, from it, the snapshot and its diff, the UI graph, the actions answer and the layout answer, these three never carrying the node's declared name, no element's id holding `///`
