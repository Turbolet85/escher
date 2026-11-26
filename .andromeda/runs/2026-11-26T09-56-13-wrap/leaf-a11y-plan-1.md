@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: 
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=23 class=stale
leaf-text: a virtual node reading `covered` only when an element lies over its region's centre
source: ctr-26pvuvyx «A driver click on an element under an open select list is refused `covered`; a virtual node reads `covered` when an element lies over its region's centre, or when a node its canvas declares later has a region holding that centre.»
replace-with: a virtual node reading `covered` when an element lies over its region's centre, or when a node its canvas declares later has a region holding that centre
@leaf_edit n=2 leaf=.claude/docs/a11y-summary.md line=26 class=retired
leaf-text: exists, nor a keyboard route to a canvas's virtual nodes, Circle Drawer's circles reached by click, tree and snapshot alone, a drag
source: ctr-vndyno3y «The Circle Drawer canvas circle-canvas is one Tab stop and keeps focus while its keys act.»
replace-with: exists, a drag
@leaf_edit n=3 leaf=.claude/docs/a11y-summary.md line=22 class=omission
leaf-text: owned by residual `res-wvatzhpe` (re-carried at 0.3.0, target: next).
source: ctr-sqt43a4q «The arrows move the selected circle in declared circle order, clamped, and the first arrow with none selected selects the first circle.»
replace-with: owned by residual `res-wvatzhpe` (re-carried at 0.3.0, target: next). Circle Drawer: the canvas `circle-canvas` is one Tab stop and keeps focus while its keys act; the arrows move the selected circle in declared circle order, clamped, the first arrow with none selected selecting the first circle; Enter opens the adjust dialog on the selected circle, and with the dialog open a key on the canvas changes nothing; no circle is the tree's focus, so an assistive technology learns the current circle from its `selected` state alone, a stated residual beside the CRUD list's unread active descendant.
@leaf_edit n=4 leaf=.claude/docs/a11y-summary.md line=14 class=omission
leaf-text: and walks the load page's `LOAD_TAB_ORDER` and Cells' `CELLS_TAB_ORDER` forward and back in both layout modes
source: dec-4cqzojua «Each lean task, the load page, Cells and Circle Drawer state their Tab order, walked forward and back in both layout modes with the engine's focus, the snapshot's focused node and the tree's focus agreeing at every step»
replace-with: and walks the load page's `LOAD_TAB_ORDER`, Cells' `CELLS_TAB_ORDER` and Circle Drawer's stated Tab order forward and back in both layout modes
