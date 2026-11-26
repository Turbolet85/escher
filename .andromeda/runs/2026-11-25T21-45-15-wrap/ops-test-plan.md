@row_set n=1 row=contract/handler-coverage-limits/canvas-handler cell=promise seen=1 fact=F3 detector=none
value: A canvas's virtual node's click is listed under its `{owner}///{key}` id, owned by its canvas, and exercised where a replayed click named it; a canvas click landing inside a node is counted for the canvas listener alone.
basis: fct-yf6akjxu (the coverage line carries a virtual node's `{owner}///{key}` id with owner and event `click`); fct-5wc4mrm4 (arch · CARRY 6, met)
because: Handler coverage now lists a virtual node's click instead of counting nothing inside a canvas, so the limit row must state what is and is not counted.
@row_set n=2 row=contract/stand-boot/boot-circle-drawer cell=promise seen=1 fact=F5 detector=none
value: stand::boot_circle_drawer(HarnessOptions) boots Circle Drawer in process alone, served by no escher-session role; each circle is declared a virtual node `circle-canvas///c{id}`, role `Image`, its bounds its cut region, its id app-given and kept across undo and redo.
basis: fct-5wc4mrm4 (a11y · stand: each circle reads `circle-canvas///c{id}`, `Image`, its bounds its cut region; security: app-given id, stable across undo and redo); fct-yf6akjxu (`CircleCanvas` overrides `Widget::virtual_nodes`)
because: The stand's Circle Drawer now declares its circles as virtual nodes, a contract the checks booting it rely on.
@row_set n=3 row=decision/reserved-graph-id-form cell=rule seen=4 detector=none
value: No id of the lint answer holds the reserved `{owner}///{key}` form; the stand's UI graph and actions answer hold it only for the virtual nodes a canvas declares, Circle Drawer's circles.
basis: fct-lscnn6ng (test-plan · decision/reserved-graph-id-form — carried: the stand now declares virtual nodes); fct-5wc4mrm4 (arch: graph, actions, layout and exploration read the circles by id)
because: The row's condition «while the stand declares no virtual node» no longer holds, since the stand's Circle Drawer now declares its circles.
@row_retire n=1 row=decision/handler-coverage-canvas-uncounted seen=1 heir=contract/handler-coverage-limits/canvas-handler detector=none
why: Handler coverage now lists a canvas's virtual node's click, so a handler inside a canvas is no longer uncounted as a whole.
basis: fct-yf6akjxu (the coverage line carries a virtual node's id and `click`); fct-5wc4mrm4 (arch · CARRY 6, met)
because: The decision's name and rule state the opposite of what coverage now does, and the limit contract states the new reading.
@row_add n=1 row=contract/driver-json/canvas-line-keys area=unit-test-strategy fact=F2 detector=none
boundary: Driver JSON answers
item: canvas line keys
promise: No line of the canvas answer holds a key twice, grouped or not: the closing node tag ` node={k|-}` uses a key no command's own pairs write.
basis: fct-zozw7znf (the node tag shares no key with a field a line already holds; `no_canvas_line_holds_a_key_twice` reads every command kind's line); fct-yf6akjxu (the tag under the key `node`, which no command's own pairs write)
because: The canvas answer's line form gained a closing tag whose key is promised unique on its line, held by a unit row.
@row_add n=2 row=contract/lint-findings/virtual-nodes area=unit-test-strategy fact=F4 detector=none
boundary: Lint findings
item: virtual nodes
promise: The lint reads no virtual node: `off-screen`, `clipped` and `zero-size` are not read on a virtual node, and `overlap` is skipped where either side is one.
basis: fct-vufwaraf (F6 B not built here: `off-screen`, `clipped` and `zero-size` on virtual nodes, `overlap` skipped on either side, a later CARRY); fct-5wc4mrm4 (limits: F6 A witnessed)
because: With the stand now declaring virtual nodes, the lint's standing limit over them is a promise a planner must respect.
@row_add n=3 row=decision/circles-apart-written-through-attribute area=test-data-fixtures fact=F1 detector=none
name: Circles apart written through the attribute
rule: A Circle Drawer check that needs circles apart writes `data-circles` through the mutator rather than clicking.
why: Driver clicks on Circle Drawer all land at the canvas's centre.
basis: fct-ayjha4lf (driver clicks all land at the canvas's centre, so any check needing circles apart writes `data-circles` through the mutator)
because: The chunk sets a fixture convention for Circle Drawer checks that future checks must follow.
@row_add n=4 row=decision/canvas-node-index-proven-apart area=integration-test-strategy fact=F6 detector=none
name: Canvas node index proven on circles apart
rule: A canvas command's node index is positional: circles sharing one region tie and read the first index, so a separated circle's index is proven only on circles written apart.
basis: fct-ol6fjtim (through driver clicks the tie reads c3 at index 1; the positional reading holds for circles apart); fct-wjolgvom (the index of a separated circle is proven on circles written into `data-circles`)
because: The chunk kept the positional index as a limit, which bounds what an integration check can prove through driver clicks.
