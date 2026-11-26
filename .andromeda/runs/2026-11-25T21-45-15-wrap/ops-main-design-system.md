@row_add n=1 row=decision/canvas-line-node-tag area=surface detector=none
name: Canvas line node tag
rule: Every canvas line closes with node= and the 1-based place, in declared order, of the canvas's virtual node whose region holds the command's centre, nearest centre on a tie, never its id, or node=- for none; node is a key no other pair of a line holds, so no line holds a key twice.
why: A tag under the key n gave a glyph line two values under one key, since the glyph count already holds n.
guard: no_canvas_line_holds_a_key_twice
basis: fct-yf6akjxu, fct-zozw7znf, fct-lscnn6ng
because: The canvas line form gains a closing node tag, the node's index a number alone on the founder's own F2 B, and the standing line-form row cannot take it within its rule's bound, so the tag stands as a row of its own.
