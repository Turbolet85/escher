@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=31 class=retired
leaf-text: A canvas's virtual nodes have no keyboard route: Circle Drawer's circles are reached by click, tree and snapshot alone.
source: abs-ddi3svwu «absence/keyboard-virtual-nodes — A keyboard route to a canvas's virtual nodes; Circle Drawer's circles are reached by click, tree and snapshot alone»
replace-with: A canvas's virtual nodes have a keyboard route on Circle Drawer: the canvas `circle-canvas` is one Tab stop and keeps focus while its keys act, the arrows moving the selected circle in declared circle order, clamped (the first arrow with none selected selecting the first circle), Enter opening the adjust dialog on the selected circle, and a key changing nothing while that dialog is open; no circle is the tree's focus, so an assistive technology learns the current circle from its selected state alone (a stated residual, provisional).
@leaf_edit n=2 leaf=.claude/rules/a11y.md line=35 class=omission
leaf-text: walks the shared tables `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, beside
source: dec-4cqzojua «decision/stated-tab-order-walked · Stated Tab order walked — Each lean task, the load page, Cells and Circle Drawer state their Tab order, walked forward and back in both layout modes with the engine's focus»
replace-with: walks the shared tables `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`, and Circle Drawer's stated Tab order, beside
