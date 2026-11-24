@leaf_group group=a11y-plan-1
@leaf_read leaf=.claude/docs/a11y-summary.md lines=45
last: **Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules:
@leaf_edit n=1 leaf=.claude/docs/a11y-summary.md line=22 class=omission
leaf-text: role and cell semantics stay out, owned by residual `res-wvatzhpe` (re-carried at 0.3.0, target: next).
source: dec-bg6u7ulu «decision/circle-drawer-canvas-pixels-only · Circle Drawer's drawing area as one named canvas node — Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable»
replace-with: role and cell semantics stay out, owned by residual `res-wvatzhpe` (re-carried at 0.3.0, target: next). Circle Drawer's drawing area is one canvas node, role group, named Drawing area, keyed and clickable, and its circles and diameter labels reach no DOM, snapshot, accessibility tree, UI graph or lint, leaving only as shot pixels (standing; guard `stand_circle_drawer_canvas`).
