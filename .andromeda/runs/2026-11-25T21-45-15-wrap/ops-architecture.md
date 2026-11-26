@row_set n=1 row=contract/handler-coverage/canvas cell=promise seen=2 fact=F3 detector=none
value: A virtual node's click is listed under its {owner}///{key} id, owned by the canvas, exercised where a replayed click named it; a canvas click landing inside a node counts the canvas's own listener under the canvas's id alone.
basis: fct-5wc4mrm4
because: The coverage line now lists a virtual node's click under its id, so the row's claim that handlers inside a canvas are not counted is false and must follow.
@row_add n=1 row=decision/canvas-command-grouping area=established-decisions fact=F1 detector=D-arch-decisions
name: Canvas commands grouped under virtual nodes
rule: A canvas reading groups each draw command under the canvas's virtual node whose region holds the command's centre, the region whose centre lies nearest first, by its 1-based positional index, or none; a glyph run's bounds are its ink box, its glyph-origins box the fallback.
why: A glyph run's origins sit on its baseline and group some labels under another node, while its ink box groups every label under its own.
basis: fct-g4glg3se
because: The chunk lands the grouping of draw commands under virtual nodes, a new standing decision no row of the canvas reading holds.
@row_add n=2 row=contract/canvas-verb/node-tag area=standard-contracts fact=F2 detector=D-arch-resources
boundary: The canvas verb
item: node
promise: Every line of the full answer and of each part closes with node={k}, the index of the virtual node grouping the command, or node=-, under a key no other pair of a line holds, so no canvas line holds a key twice; the summary keeps its shape, its digest taken of the tagged text.
basis: fct-yf6akjxu
because: The canvas answer's line form gains a closing node tag, a promise at the verb's boundary that no canvas-verb row states.
@row_add n=3 row=contract/blitz-dom-crate-root/virtual-node-role area=standard-contracts fact=F5 detector=D-arch-resources
boundary: blitz-dom crate root
item: VirtualNodeRole
promise: Behind custom-widget, the crate root re-exports AccessKit's Role as VirtualNodeRole, the type a virtual node declaration's role takes, so an app reaches it without naming accesskit.
basis: fct-yf6akjxu
because: blitz-dom's public surface gains a re-export, which the crate-root rows must register beside its other public re-exports.
@row_add n=4 row=absence/virtual-node-keyboard-path area=standard-contracts fact=F7 detector=none
capability: A keyboard path to a canvas's virtual nodes
basis: fct-l3p3ykbb
because: The chunk's circle nodes read in the tree and snapshot but take no keyboard path, a whole capability left to a later entry that no row states absent.
@row_none n=1 fact=F4 type=4
relied: Circle Drawer's circles read as virtual nodes circle-canvas///c{id}, role Image, their ids app-given and stable across undo and redo, which the stand checks rely on; the app's code and its data-circles attribute hold it.
@row_none n=2 fact=F6 type=4
relied: A sibling virtual node is not refused covered and driver clicks land at the canvas's centre, which contract/refusal-causes/covered and decision/canvas-placement-at-click-centre already hold; the positional index proven on circles written apart is a test's reach.
