@leaf_group group=a11y-plan-2
@leaf_read leaf=.claude/rules/a11y.md lines=46
last: - 2026-10-06: On the dev host the AT-SPI bus reads `IsEnable
@leaf_edit n=1 leaf=.claude/rules/a11y.md line=28 class=omission
leaf-text: Circle Drawer with its dialog open included
source: dec-bg6u7ulu «Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable, and its circles and diameter labels reach no DOM, snapshot, accessibility tree, UI graph or lint, leaving only as shot pixels.»
replace-with: Circle Drawer with its dialog open included, its drawing area one canvas node, role group, named Drawing area, keyed and clickable, its circles and diameter labels reaching no DOM, snapshot, accessibility tree, UI graph or lint and leaving only as shot pixels (`stand_circle_drawer_canvas` its guard)
