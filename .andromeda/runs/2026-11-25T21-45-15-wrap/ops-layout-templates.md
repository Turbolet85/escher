@row_set n=1 row=decision/circle-drawer-circles-painted-on-canvas cell=rule seen=4 fact=F2 detector=none
value: Circle Drawer's circles are painted by a canvas widget from one data-circles attribute (x,y,d,id per circle, semicolon-joined, malformed text read as no circle), never as DOM nodes; each also a canvas virtual node, crossing as shot pixels, draw commands and its node, and lint reads label ink boxes.
basis: fct-xedf6lma, fct-yf6akjxu, fct-lscnn6ng
because: The circles are now declared virtual nodes of the canvas and data-circles reads x,y,d,id per circle, so the row's x,y,d form and its claim that circles reach no snapshot, tree or graph are false.
@row_set n=2 row=contract/escher-session-coverage/answer cell=promise seen=1 fact=F6 detector=none
value: Each check replayed on a fresh boot; one uncoloured line {"unexercised":[{id,owner,event}],"checks","held"}, owner left out when absent; a canvas virtual node's click under its {owner}///{key} id, canvas-owned, exercised where a replayed click named it; a failed boot is the session error, exit 3.
basis: fct-yf6akjxu, fct-5wc4mrm4
because: The coverage line now carries a canvas virtual node's click under its {owner}///{key} id, owned by the canvas, so the answer's promise must say what it lists of virtual nodes.
@row_add n=1 row=contract/stand-element-ids/circle-canvas-circle-node area=surface-desktop-native fact=F1 detector=none
boundary: Stand stable element ids
item: circle-canvas///c{id}
promise: Stable id of each Circle Drawer circle's virtual node, {id} an app-given whole number from 1 stable across undo and redo; the node reads role Image, name Circle {id}, {d} px, its bounds the circle's region cut to the canvas content box.
basis: fct-5wc4mrm4, fct-yf6akjxu
because: Each Circle Drawer circle is now a declared virtual node of the canvas with a stable app-given id, a new stand id no row holds.
@row_add n=2 row=contract/escher-session-canvas/node-tag area=surface-cli fact=F5 detector=none
boundary: escher-session canvas
item: node={k}
promise: The full answer and every part close each command line with node={k} or node=-, k the 1-based index of the canvas virtual node whose region holds the command's centre, nearest centre first, under a key no other pair of the line holds; the index is positional.
basis: fct-yf6akjxu, fct-zozw7znf
because: The canvas verb's answer lines gained a closing node tag, a promise at the CLI boundary no row states.
@row_add n=3 row=name/other/canvas-line-key-node area=surface-cli fact=F7 detector=none
class: other
name: canvas line key node
basis: fct-zozw7znf, fct-qqllch62
because: The canvas answer's node tag occupies the key node, the spelling n= refused because the glyph line already holds n.
@row_none n=1 fact=F3 type=4
relied: Circle Drawer's circle nodes take their bounds cut to this content box.
@row_none n=2 fact=F4 type=4
relied: A check needing Circle Drawer circles apart writes data-circles directly.
