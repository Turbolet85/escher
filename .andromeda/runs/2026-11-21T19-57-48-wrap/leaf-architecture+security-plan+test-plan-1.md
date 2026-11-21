@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=17 class=stale
leaf-text: `overlap` leaves out a closed select's options and an open list's rows and groups
source: ctr-s6q6q2ec «A closed drop-down's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a list box's rows are linted as usual.»
replace-with: `overlap` leaves out a closed drop-down's options and an open list's rows and groups
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=17 class=stale
leaf-text: a `multiple` select's rows linted as usual
source: ctr-s6q6q2ec «A closed drop-down's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a list box's rows are linted as usual.»
replace-with: a list box's rows (a `multiple` select's or a single select's of display size above 1) linted as usual
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: an option of a closed single select is not clicked
source: dec-yyacsgv5 «An option of a closed drop-down, a single select of display size 1, under a group too, takes no click and stays refused `covered`.»
replace-with: an option of a closed drop-down (a single select of display size 1) is not clicked
@leaf_edit n=4 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: while a `click` on a `multiple` select's row is accepted and chooses it, a `click` on a single select opens its list or closes one standing
source: ctr-uc5tnflx «Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a drop-down opens its list, and a click on a list box's row chooses it.»
replace-with: while a `click` on a list box's row (a `multiple` select's or a single select's of display size above 1) is accepted and chooses it, a `click` on a drop-down opens its list or closes one standing
@leaf_edit n=5 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: a click landing on a `multiple` select's row at the centre and opening a single select's list
source: ctr-uc5tnflx «Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a drop-down opens its list, and a click on a list box's row chooses it.»
replace-with: a click landing on a list box's row at the centre and opening a drop-down's list
@leaf_edit n=6 leaf=.claude/docs/services/escher-driver.md line=18 class=stale
leaf-text: on a focused single select with its list closed a pressed `enter`, or a `space` with no typed text standing, opens its list
source: ctr-vgjhgept «On a drop-down Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more; on a single list box neither opens anything.»
replace-with: on a focused drop-down (a single select of display size 1) with its list closed a pressed `enter`, or a `space` with no typed text standing, opens its list, while on a single list box neither opens anything
