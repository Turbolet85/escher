@leaf_group group=masters
@leaf_read leaf=.claude/docs/gotchas.md lines=68
last: - Decisions and rationale: `doc:architecture` (the records s
@leaf_read leaf=CLAUDE.md lines=169
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=omission
leaf-text: each taken node a child of its canvas's accessibility node and a snapshot node,
source: ctr-7r44w5kv «A canvas's virtual node takes selected as its declaration states it, none where it states none; a canvas's current node is conveyed by selected alone, the tree's focus staying on the canvas»
replace-with: each taken node a child of its canvas's accessibility node and a snapshot node, its selected as its declaration states it, a canvas's current node conveyed by selected alone with the tree's focus staying on the canvas,
