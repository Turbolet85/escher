@leaf_group group=architecture+security-plan+test-plan-1
@leaf_read leaf=.claude/docs/services/escher-driver.md lines=63
last: - The command line, through the real `escher-session` binary
@leaf_edit n=1 leaf=.claude/docs/services/escher-driver.md line=16 class=stale
leaf-text: handlers inside a canvas are not counted — a virtual node a canvas declares carries none, and a click on one counts its canvas's own listener under the canvas's id
source: ctr-rq3jq44s «A virtual node's click is listed under its {owner}///{key} id, owned by the canvas, exercised where a replayed click named it; a canvas click landing inside a node counts the canvas's own listener under the canvas's id alone.»
replace-with: a virtual node's click is listed under its `{owner}///{key}` id, owned by the canvas, and exercised where a replayed click named it, while a canvas click landing inside a node counts the canvas's own listener under the canvas's id alone
@leaf_edit n=2 leaf=.claude/docs/services/escher-driver.md line=62 class=stale
leaf-text: nor of the stand's UI graph or actions answer while the stand declares no virtual node
source: dec-b5ugo5lk «No id of the lint answer holds the reserved `{owner}///{key}` form; the stand's UI graph and actions answer hold it only for the virtual nodes a canvas declares, Circle Drawer's circles.»
replace-with: the stand's UI graph and actions answer holding it only for the virtual nodes a canvas declares, Circle Drawer's circles
@leaf_edit n=3 leaf=.claude/docs/services/escher-driver.md line=32 class=stale
leaf-text: while the graph, actions and layout answers carry the id, never the name
source: dec-eg7d4fr7 «A canvas's virtual node id `{owner}///{key}` and its declared name cross the accessibility tree, snapshot text, diff, MCP results, kept checks and socket and command-line answer line; the graph, actions, layout and coverage answers carry the id, never the name»
replace-with: while the graph, actions, layout and coverage answers carry the id, never the name
@leaf_stands n=1 leaf=.claude/docs/services/escher-driver.md line=63
because: The line names the virtual-node rows' answers only in `stand_masks_on_new_answers`' account of where a typed password and a file path never occur, a masking claim that none of the changed id-form or crossing rows bears on, so it still says what its sources say.
