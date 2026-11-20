@leaf_group group=test-plan-1
@leaf_read leaf=.claude/docs/tests-summary.md lines=56
last: **Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/tests-summary.md line=13 class=omission
leaf-text: reading and changing nothing (no mutation, input, pump or settle)
source: ctr-wcvnmydf «highlight_contrast(NodeId) -> Option<TextContrast> is a public read-only reader answering only for the highlighted row of an open select list, with the list's highlight text and background through the one contrast formula and the option's own font size and weight, and None for any other node.»
replace-with: reading and changing nothing (no mutation, input, pump or settle), beside it `highlight_contrast(node_id) -> Option<TextContrast>`, as read-only, answering only for the highlighted row of an open select list with the list's highlight text and background through the one contrast formula and the option's own font size and weight, `None` for any other node
@leaf_edit n=2 leaf=.claude/docs/tests-summary.md line=13 class=stale
leaf-text: reading the same with no second formula
source: dec-6q3k2ofk «Session lint's contrast reads through the harness's contrast readers, with no second formula.»
replace-with: reading through the harness's contrast readers with no second formula
@leaf_edit n=3 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: the engine default chosen-row pair `#c8c8c8` on `#000000` reading 12.56:1
source: dec-aguzntwh «A chosen select row is painted with the engine default #000000 on #c8c8c8, and an open list's highlighted row shares that pair.»
replace-with: the engine default chosen-row pair `#000000` on `#c8c8c8` reading 12.56:1, an open list's highlighted row sharing that pair
@leaf_edit n=4 leaf=.claude/docs/tests-summary.md line=21 class=omission
leaf-text: a step returning the node whose selected or invalid reading it changed), since
source: ctr-dcrupvqm «An acting call returns the node it changed with its `selected`, `invalid` or `highlighted` reading as it reads after the step.»
replace-with: a step returning the node whose selected, invalid or highlighted reading it changed), since
@leaf_edit n=5 leaf=.claude/docs/tests-summary.md line=21 class=stale
leaf-text: and the thumb not draggable, a highlighted row's contrast unread by the lint, a larger authored font clipping and the highlight in no snapshot and no tree its four stated limits
source: dec-6q3k2ofk «Session lint's contrast reads through the harness's contrast readers, with no second formula.»
replace-with: and the thumb not draggable its one stated limit
@leaf_edit n=6 leaf=.claude/docs/tests-summary.md line=24 class=omission
leaf-text: a step returning the node whose selected or invalid reading it changed, as it reads after the step
source: ctr-dcrupvqm «An acting call returns the node it changed with its `selected`, `invalid` or `highlighted` reading as it reads after the step.»
replace-with: a step returning the node whose selected, invalid or highlighted reading it changed, as it reads after the step
@leaf_edit n=7 leaf=.claude/docs/tests-summary.md line=44 class=omission
leaf-text: its expected value pinned as the measured literal and never derived from the constant its mutation control changes
source: dec-a7xrcpzo «A check never derives its expected value from the constant its mutation control changes; it pins the measured value as a literal.»
replace-with: its expected value pinned as the measured literal and never derived from the constant its mutation control changes (a pixel-row pin that font-embolden moves pinned as a stated band with its reason in a comment where the test crate cannot see the feature, its wholly-inside assertions kept exact)
