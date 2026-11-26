@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=43
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/l
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=13 class=omission
leaf-text: the accent fill at alpha 38/255)
source: dec-eiwpw2n7 «decision/circle-drawer-canvas-colours-in-code · Circle Drawer canvas colours as widget constants — Circle Drawer's canvas colours, the stand accent stroke, the dark label, the accent fill at alpha 38/255 and the selected circle's fill»
replace-with: the accent fill at alpha 38/255 and the selected circle's fill at alpha 96/255)
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=27 class=omission
leaf-text: lint reads label ink boxes; standing)
source: dec-robagc2z «decision/circle-drawer-click-selects-or-adds · Circle Drawer click selects or adds — A click on Circle Drawer's canvas selects the topmost circle whose disc holds the point and adds none; only a click on no circle adds a circle.»
replace-with: lint reads label ink boxes; standing; a click on the canvas selects the topmost circle whose disc holds the point and adds none, only a click on no circle adding a circle)
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=27 class=omission
leaf-text: with `circle-diameter` · `circle-close` on the open dialog;
source: dec-fltlgmy7 «decision/circle-drawer-dialog-disables-other-circles · Circle Drawer dialog disables other circles — While Circle Drawer's diameter dialog is open, every circle but the one being adjusted declares disabled and is refused disabled.»
replace-with: with `circle-diameter` · `circle-close` on the open dialog (while it is open every circle but the one being adjusted declares disabled and is refused disabled; provisional);
@leaf_edit n=4 leaf=.claude/docs/design-summary.md line=27 class=omission
leaf-text: `circle-canvas` (Circle Drawer's circles painted
source: dec-bvhmgy6h «decision/circle-drawer-canvas-keeps-size · Circle Drawer canvas keeps its size — Circle Drawer's canvas keeps 700 x 400 and never shrinks, so on the 600 px stand view the dialog's Close lies below the fold and is scrolled into view»
replace-with: `circle-canvas` (keeping 700 x 400 and never shrinking, provisional, so on the 600 px stand view the dialog's Close lies below the fold and is scrolled into view before a click; Circle Drawer's circles painted
