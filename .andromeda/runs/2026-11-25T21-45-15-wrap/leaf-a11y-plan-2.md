@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=27 class=stale
leaf-text: the actions answer and the layout answer (these three carrying the id and never the node's declared name)
source: ctr-2peziqew «The form {owner}///{key} is reserved for a canvas's virtual nodes and served in the accessibility tree and, from it, the snapshot and its diff, the UI graph, the actions, layout and handler coverage answers, these four never carrying the node's declared name»
replace-with: the actions, layout and handler coverage answers (these four carrying the id and never the node's declared name)
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=31 class=omission
leaf-text: keep that hook when adding focus-affecting attributes.
source: abs-ddi3svwu «A keyboard route to a canvas's virtual nodes; Circle Drawer's circles are reached by click, tree and snapshot alone»
replace-with: keep that hook when adding focus-affecting attributes. A canvas's virtual nodes have no keyboard route: Circle Drawer's circles are reached by click, tree and snapshot alone.
