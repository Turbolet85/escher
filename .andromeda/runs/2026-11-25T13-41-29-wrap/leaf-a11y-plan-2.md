@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=27 class=stale
leaf-text: a `UiGraph` of one node per snapshot node but a canvas's virtual nodes, which it excepts, so the actions answer and exploration hold none of them, names and closed words only
source: dec-45n3iy3a «The read-only UI graph holds one node per snapshot node, a canvas's virtual nodes among them, each owned by its canvas with no handlers or reads, so the actions answer and exploration hold them too»
replace-with: a `UiGraph` of one node per snapshot node, a canvas's virtual nodes among them, each owned by its canvas with no handlers or reads, so the actions answer and exploration hold them too, names and closed words only
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: a canvas's virtual nodes are served in this tree, and from it the snapshot and its diff, under the reserved form `{owner}///{key}`, which no element's id holds
source: ctr-2peziqew «served in the accessibility tree and, from it, the snapshot and its diff, the UI graph, the actions answer and the layout answer, these three never carrying the node's declared name; no element's id holds ///.»
replace-with: a canvas's virtual nodes are served in this tree, and from it the snapshot and its diff, the UI graph, the actions answer and the layout answer (these three carrying the id and never the node's declared name), under the reserved form `{owner}///{key}`, which no element's id holds
