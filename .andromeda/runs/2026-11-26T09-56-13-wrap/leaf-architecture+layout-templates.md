@leaf_group group=architecture+layout-templates
@leaf_read leaf=.claude/docs/services/seven_guis.md lines=45
last: - `.claude/rules/a11y.md` (the stand is in its paths) · `.cl
@leaf_edit n=1 leaf=.claude/docs/services/seven_guis.md line=32 class=omission
leaf-text: Circle drawer clamps diameter 5–100.
source: dec-robagc2z «A click on Circle Drawer's canvas selects the topmost circle whose disc holds the point and adds none; only a click on no circle adds a circle.»
replace-with: Circle drawer clamps diameter 5–100; a click on its canvas selects the topmost circle whose disc holds the point and adds none, only a click on no circle adding a circle.
@leaf_stands n=1 leaf=.claude/docs/services/seven_guis.md line=30
because: Line 30's selected readings concern the CRUD rows' aria-selected and Cells' unstated selected cell, never a virtual node, so the row's move of selected out of a virtual node's absent keys leaves it true.
