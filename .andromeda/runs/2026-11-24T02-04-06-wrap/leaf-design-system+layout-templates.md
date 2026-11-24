@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=43
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=27 class=stale
leaf-text: and the load page and the order page read 0
source: dec-emcqlk3f «decision/stand-lints-clean-at-boot · Stand lints clean at boot — At boot the counter, the flight booker, the timer, CRUD, the load page, the order page and Circle Drawer read zero visual-lint findings, in both layout modes.»
replace-with: and the load page, the order page and Circle Drawer read 0
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=27 class=omission
leaf-text: `circle-undo` · `circle-redo` · `circle-canvas`, with
source: dec-tp5vibgi «decision/circle-drawer-circles-painted-on-canvas · Circle Drawer circles painted on a canvas — Circle Drawer's circles are painted by a canvas widget from one data-circles attribute (x,y,d per circle»
replace-with: `circle-undo` · `circle-redo` · `circle-canvas` (Circle Drawer's circles painted by a canvas widget from one `data-circles` attribute, x,y,d per circle, semicolon-joined, malformed text read as no circle, never as DOM nodes, so they reach no snapshot, tree, graph or lint and cross as shot pixels alone; provisional), with
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=13 class=omission
leaf-text: neither a lean task, each in both layout modes
source: dec-eiwpw2n7 «decision/circle-drawer-canvas-colours-in-code · Circle Drawer canvas colours as widget constants — Circle Drawer's canvas colours, the stand accent stroke, the dark label and the accent fill at alpha 38/255»
replace-with: neither a lean task, each in both layout modes; Circle Drawer's canvas colours (the stand accent stroke, the dark label and the accent fill at alpha 38/255) are constants in the sample's widget code, not the cascade
