@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=25 class=omission
leaf-text: The snapshot takes the three from the tree node
source: dec-3dkxhrjk «The snapshot takes each node's role, name, focus and its selected, pressed, invalid and highlighted readings from the accessibility tree, never from the element, with no second role or name mapping.»
replace-with: The snapshot takes the three, and its `highlighted` reading, from the tree node
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=26 class=stale
leaf-text: the highlighted row is in no snapshot and no tree
source: ctr-2sijrh32 «While a single select's list is open, the tree names the highlighted option as the select's active descendant and the snapshot reads that row alone highlighted, the tree's focus staying on the select.»
replace-with: the tree names the highlighted option as the select's active descendant, its focus staying on the select, and the snapshot reads that row alone `highlighted`
@leaf_edit n=3 leaf=.claude/rules/a11y.md line=27 class=omission
leaf-text: a scroll that closes a select's list marking it, node creation
source: dec-3xjtbjfc «The engine's changed set marks a node's mutation, a focus or checked change, a text input, a select's changed choice, a single select's list opening or closing, a closing scroll included, and its highlight moving to another row; node creation, hover, active, other scrolls and layout do not.»
replace-with: a scroll that closes a select's list and its highlight moving to another row marking it, node creation
@leaf_edit n=4 leaf=.claude/rules/a11y.md line=33 class=stale
leaf-text: while a highlight or a wheel move inside it marks nothing
source: dec-3xjtbjfc «The engine's changed set marks a node's mutation, a focus or checked change, a text input, a select's changed choice, a single select's list opening or closing, a closing scroll included, and its highlight moving to another row; node creation, hover, active, other scrolls and layout do not.»
replace-with: while a highlight moving to another row marks its select and a wheel move inside it marks nothing
@leaf_edit n=5 leaf=.claude/rules/a11y.md line=38 class=retired
leaf-text: a highlighted row's contrast is unread by the lint
source: dec-4cdysvgx «The driver's visual lint reads contrast through the harness's readers, the highlight reader first and the text contrast where it answers nothing, with the stand's exempt rule, never a second formula or threshold.»
replace-with: the lint reads a highlighted row's contrast at its painted pair through the harness's highlight reader first, the text contrast where it answers nothing,
@leaf_edit n=6 leaf=.claude/rules/a11y.md line=38 class=stale
leaf-text: reads the same `None` through the one reader
source: dec-2vbd4kxl «The visual lint's contrast reading uses the harness's readers, the highlight reader for an open list's highlighted row and the text contrast elsewhere, through the one contrast formula, its required ratio and the»
replace-with: reads the same `None` through the text contrast reader, the harness's readers sharing one formula
