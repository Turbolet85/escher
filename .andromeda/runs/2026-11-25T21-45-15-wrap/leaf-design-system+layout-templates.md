@leaf_group group=design-system+layout-templates
@leaf_read leaf=.claude/docs/design-summary.md lines=43
last: **Full plans:** `.andromeda/design-system.md` + `.andromeda/la
@leaf_edit n=1 leaf=.claude/docs/design-summary.md line=27 class=stale
leaf-text: x,y,d per circle, semicolon-joined, malformed text read as no circle, never as DOM nodes, so they reach no snapshot, tree or graph, cross as shot pixels and canvas draw commands, and lint reads label ink boxes; standing
source: dec-tp5vibgi «Circle Drawer's circles are painted by a canvas widget from one data-circles attribute (x,y,d,id per circle, semicolon-joined, malformed text read as no circle), never as DOM nodes; each also a canvas»
replace-with: x,y,d,id per circle, semicolon-joined, malformed text read as no circle, never as DOM nodes; each also a canvas virtual node, crossing as shot pixels, draw commands and its node, and lint reads label ink boxes; standing
@leaf_edit n=2 leaf=.claude/docs/design-summary.md line=28 class=retired
leaf-text: its two stated limits: handlers inside a canvas not counted (owner 0.4.0) and a handler on no screen a replay reached not listed (no owner)
source: ctr-g7ohrk4h «a canvas virtual node's click under its {owner}///{key} id, canvas-owned, exercised where a replayed click named it; a failed boot is the session error, exit 3.»
replace-with: a canvas virtual node's click listed under its `{owner}///{key}` id, canvas-owned, exercised where a replayed click named it, a canvas click landing inside a node counted for the canvas's own listener alone; its stated limit: a handler on no screen a replay reached not listed (no owner)
@leaf_edit n=3 leaf=.claude/docs/design-summary.md line=28 class=omission
leaf-text: a glyph run with no glyph as `at=none`
source: dec-etxesrsm «Every canvas line closes with node= and the 1-based place, in declared order, of the canvas's virtual node whose region holds the command's centre, nearest centre on a tie, never its id, or node=- for none»
replace-with: a glyph run with no glyph as `at=none`, and every line closing with `node=` and the 1-based place, in declared order, of the canvas's virtual node whose region holds the command's centre (nearest centre on a tie, never its id), or `node=-` for none
