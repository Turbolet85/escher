@row_set n=1 row=contract/select-accessibility/highlighted-row cell=promise seen=1 fact=F2 detector=none
value: The highlighted row of a single select's open list alone carries the snapshot's closed bool highlighted, a bare token after clickable in the text and true in JSON, written only when true.
basis: fct-voh3czmv, fct-s4gho2nl, fct-73upmmhj
because: The highlighted row is now read in the snapshot and the tree, so the promise that it is in neither is false.
@row_set n=2 row=contract/select-accessibility/open-select cell=promise seen=1 fact=F1 detector=none
value: The tree reads an open select as ComboBox and expanded with focus kept on the select; while a single select's list stands open the tree names the highlighted option as the select's active descendant, and a page's aria-activedescendant is not read.
basis: fct-voh3czmv, fct-73upmmhj
because: The tree of an open single select now names its highlighted option as the active descendant, which the open-select promise says less of.
@row_set n=3 row=contract/select-list-events/highlight-move cell=promise seen=1 fact=F3 detector=none
value: Keys and the pointer move the highlight over enabled rows and choose nothing; a move to another row marks its select changed, while a move to the same row or a wheel move inside the list marks nothing.
basis: fct-voh3czmv, fct-gvhc5qds
because: A highlight move to another row now marks its select changed, ratified, and the highlight-move promise must say which moves mark.
@row_set n=4 row=decision/select-list-colours-engine-constants cell=rule seen=1 fact=F5 detector=D-design-tokens
value: The open select list's colours are engine constants, not design tokens and not in the default stylesheet; its highlight shares the default sheet's chosen row pair, and the accessibility lint reads the highlighted row at that painted pair and plain rows at their own computed pair.
basis: fct-voh3czmv, fct-73upmmhj, fct-g5qothqo
because: The highlight pair is now pinned to the default sheet's chosen row and the lint reads it as painted, which the rule must carry.
@row_add n=1 row=decision/select-list-rows-one-height area=spacing fact=F4 detector=none
name: Open select list rows take one height
rule: Every row of an open select list takes one height, the tallest row's line with headers at weight 700 included and the select's own one-line height as the floor, plus the row padding.
why: So no authored font is clipped inside its row; ratified.
guard: every_row_takes_the_tallest_rows_line_so_a_larger_font_is_not_cut
basis: fct-voh3czmv, fct-pmqhsgvu, fct-73upmmhj
because: The row height is now the tallest row's line instead of clipping a larger authored font, a ratified decision no spacing row states.
