@leaf_group group=architecture+test-plan
@leaf_read leaf=.claude/docs/commands.md lines=83
last: - Fonts missing in headless tests → `system-fonts` feature / f
@leaf_read leaf=.claude/docs/services/blitz-test-harness.md lines=38
last: - `.claude/rules/verification-harness.md` · `.claude/docs/tests
@leaf_edit n=1 leaf=.claude/docs/commands.md line=58 class=stale
leaf-text: a click on a select opens its list and a second closes it; an option of a closed select lies under it and is refused `covered`
source: ctr-uc5tnflx «Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a drop-down opens its list, and a click on a list box's row chooses it.»
replace-with: a click on a drop-down, a single select of display size 1, opens its list and a second closes it, and a click on a list box's row chooses it; an option of a closed drop-down lies under it and is refused `covered`
@leaf_edit n=2 leaf=.claude/docs/services/blitz-test-harness.md line=26 class=omission
leaf-text: so a check that a click clears focus asserts the root element.
source: dec-qygu7ld5 «Box presence read with computed display — A check asking whether a node has a box reads the node's computed display beside its client rect. · why: A node a restyle turns display none keeps the client rect it last laid out at.»
replace-with: so a check that a click clears focus asserts the root element. A node a restyle turns `display: none` keeps the client rect it last laid out at, so a check asking whether a node has a box reads its computed display beside its client rect.
