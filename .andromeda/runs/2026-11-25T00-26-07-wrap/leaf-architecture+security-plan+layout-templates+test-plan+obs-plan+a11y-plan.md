@leaf_group group=architecture+security-plan+layout-templates+test-plan+obs-plan+a11y-plan
@leaf_read leaf=.claude/docs/gotchas.md lines=69
last: - Decisions and rationale: `doc:architecture` (the records st
@leaf_read leaf=CLAUDE.md lines=169
last: <!-- USER:session-learnings end -->
@leaf_edit n=1 leaf=CLAUDE.md line=122 class=omission
leaf-text: pinned by `stand_lint` in both layout modes
source: ctr-s5ximkln «a node clipped or off-screen that holds text also reads `truncated`; a canvas element reads `truncated` once per label whose ink box does not lie inside its region.»
replace-with: pinned by `stand_lint` in both layout modes, and a canvas element reading `truncated` once per label whose ink box does not lie inside its region, in paint order
@leaf_edit n=2 leaf=CLAUDE.md line=122 class=omission
leaf-text: the load page and the order page reading 0 findings at boot
source: dec-emcqlk3f «At boot the counter, the flight booker, the timer, CRUD, the load page, the order page and Circle Drawer read zero visual-lint findings, in both layout modes, and Circle Drawer reads no canvas finding»
replace-with: the load page, the order page and Circle Drawer reading 0 findings at boot, Circle Drawer no canvas finding after a click or at 12 and 100 circles either
@leaf_edit n=3 leaf=CLAUDE.md line=122 class=omission
leaf-text: as the `canvas` verb's answer of draw commands beside a screenshot's pixels
source: dec-lyyw3eam «A canvas label its widget paints crosses the lint's ratified carriers only as numbers, its ink box and its visible region inside a truncated finding, never the label's text, a glyph id or font bytes.»
replace-with: as the `canvas` verb's answer of draw commands beside a screenshot's pixels, and a label's ink box and visible region as numbers inside a lint `truncated` finding, never its text, a glyph id or font bytes
