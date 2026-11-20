@row_set n=1 row=decision/changed-set-marks cell=rule seen=2 detector=none
value: The engine's changed set marks a node's mutation, a focus or checked change, a text input, a select's changed choice, a single select's list opening or closing, a closing scroll included, and its highlight moving to another row; node creation, hover, active, other scrolls and layout do not.
basis: fct-voh3czmv
because: A highlight move to another row, by key or by pointer, now inserts the list's select into the changed set, so the row's claim that hover and list moves mark nothing is false.
@row_set n=2 row=decision/wcag-contrast-thresholds cell=rule seen=1 detector=none
value: Text contrast is measured by the WCAG 2.x formula, from computed styles or, on an open list's highlighted row, from its painted pair, and must reach 4.5, or 3.0 for large text.
basis: fct-voh3czmv
because: The highlighted row of an open list is now read at its painted engine-constant pair through the one formula, not from computed styles alone.
@row_set n=3 row=decision/lint-contrast-single-formula cell=rule seen=1 detector=none
value: The driver's visual lint reads contrast through the harness's readers, the highlight reader first and the text contrast where it answers nothing, with the stand's exempt rule, never a second formula or threshold.
basis: fct-voh3czmv
because: The lint now reads the highlight reader before the text contrast, so a row naming the text contrast alone is incomplete.
@row_set n=4 row=decision/lint-contrast-shares-one-reader cell=rule seen=1 fact=F3 detector=none
value: The visual lint's contrast reading uses the harness's readers, the highlight reader for an open list's highlighted row and the text contrast elsewhere, through the one contrast formula, its required ratio and the stand check's exempt rule, so a blended text's pair is unread by the lint as well.
basis: fct-voh3czmv
because: The harness gained a highlight reader answering only for an open list's highlighted row through the one formula, and the lint reads it first.
@row_set n=5 row=decision/select-list-colours-engine-constants cell=rule seen=1 fact=F4 detector=none
value: An open select list's highlight, disabled text and border colours are engine constants, in no sheet and not tokens, the highlight pair equal to the default sheet's chosen-row pair, black on #c8c8c8, and the harness asserts the highlight and disabled rows as painted against them.
basis: fct-73upmmhj
because: The highlight pair is now pinned equal to the default sheet's chosen-row pair, which the row does not say.
@row_set n=6 row=decision/select-list-rows-one-height cell=rule seen=1 detector=none
value: An open select list's rows take one height, the tallest row's line with the select's own one-line height as the floor, so a row with a larger authored font is painted whole.
basis: fct-pmqhsgvu
because: The row height is now the tallest row's and the clause that a larger authored font clips was retired.
@row_set n=7 row=contract/snapshot/select-list-rows cell=promise seen=1 detector=none
value: The snapshot holds no expanded key and reads an open list's highlighted row alone as highlighted; an open list's row reads its own bounds in the list.
basis: fct-s4gho2nl
because: The snapshot now carries the highlighted reading on an open list's highlighted row, so the claim that the row is in no snapshot and no tree is false.
@row_set n=8 row=contract/snapshot/open-list-highlighted-row cell=promise seen=1 detector=none
value: While a single select's list is open, the tree names the highlighted option as the select's active descendant and the snapshot reads that row alone highlighted, the tree's focus staying on the select.
basis: fct-voh3czmv
because: The stated limit that the highlighted row is in no snapshot and no tree was replaced by the tree's active descendant and the snapshot's highlighted reading.
@row_set n=9 row=decision/snapshot-reads-the-tree cell=rule seen=1 detector=none
value: The snapshot takes each node's role, name, focus and its selected, pressed, invalid and highlighted readings from the accessibility tree, never from the element, with no second role or name mapping.
basis: fct-voh3czmv
because: The snapshot's new highlighted reading is taken from the tree, so the list of tree-read readings is incomplete.
@row_set n=10 row=decision/snapshot-reads-accessibility-tree cell=rule seen=1 detector=none
value: The snapshot takes each node's role, name, focus, selected, pressed, invalid and highlighted from the accessibility tree, never from the element.
basis: fct-voh3czmv
because: The snapshot's new highlighted reading is taken from the tree, so the list of tree-read readings is incomplete.
@row_retire n=1 row=decision/highlighted-row-contrast-unread seen=1 detector=none
why: The visual lint now reads an open list's highlighted row at its painted pair, so the stated limit no longer holds.
basis: fct-gvhc5qds
because: The lint reads the highlight reader first, retiring the limit that a highlighted row's contrast is unread.
@row_add n=1 row=contract/snapshot-node/highlighted area=screen-reader-support fact=F1 detector=none
boundary: the snapshot node
item: highlighted
promise: highlighted is a closed bool written only when true, true on an open single select's highlighted row alone, read from the tree as the node a tree node names as its active descendant; the diff's differs can name it.
basis: fct-s4gho2nl
because: The driver's node-key schema gained highlighted inside the snapshot and diff carriers, a promise no row states.
@row_add n=2 row=contract/accessibility-tree/select-active-descendant area=screen-reader-support fact=F2 detector=none
boundary: the accessibility tree
item: a single select's active descendant
promise: While a single select's list is open with a current row, the select names that option as its active descendant and names none when closed; focus stays on the select and a page's aria-activedescendant is never read.
basis: fct-voh3czmv
because: The tree builder now names an open single select's highlighted option as its active descendant, a promise no row states.
@row_add n=3 row=contract/cells-grid/current-cell-selected area=5-keyboard-navigation fact=F5 detector=none
boundary: seven_guis Cells grid
item: current cell selected
promise: The Cells grid's current data cell alone states aria-selected true, and an arrow key moves that state to the new current cell.
basis: fct-voh3czmv
because: Cells' current data cell now states aria-selected and an arrow moves it, a promise of the grid no row states.
