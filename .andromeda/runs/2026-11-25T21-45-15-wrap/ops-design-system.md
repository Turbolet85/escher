@row_add n=1 row=decision/canvas-line-node-tag area=surface fact=F1 detector=none
name: Canvas line node tag
rule: Every canvas line closes with node= and the 1-based place, in declared order, of the canvas's virtual node whose region holds the command's centre, nearest centre on a tie, never its id, or node=- for none; node is a key no other pair of a line holds, so no line holds a key twice.
why: A tag under the key n gave a glyph line two values under one key, since the glyph count already holds n.
guard: no_canvas_line_holds_a_key_twice
state: provisional
basis: fct-yf6akjxu, fct-zozw7znf, fct-qqllch62, fct-lscnn6ng, fct-vufwaraf
because: The canvas line form gains a closing node tag whose index is positional and whose key no other pair holds, and the standing line-form row cannot take it within its rule's bound, so the tag stands as a row of its own, provisional as a delegate's word.
@row_none n=1 fact=F2 type=4
relied: A glyph run's node tag reads the region its ink box lies in, its glyph origins' box the fallback, and a shape or fill its own bounds; the grouping is built in the driver's canvas reading, its owner architecture's canvas reading row.
