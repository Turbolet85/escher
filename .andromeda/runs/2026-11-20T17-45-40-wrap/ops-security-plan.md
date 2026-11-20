@row_set n=1 row=decision/select-list-crossing cell=rule seen=2 fact=F1 detector=none
value: The select list crosses the ratified carriers only as a row's own bounds in the snapshot, the diff and a layout answer, as the layout answer's select-list placing word, as pixels in a shot, and as the closed highlighted bool on the open list's one highlighted row; no expanded node key stands.
basis: fct-s4gho2nl, fct-voh3czmv, fct-gvhc5qds
because: Node state now holds the highlighted bool, written in the snapshot and diff on the open list's highlighted row, so «node state holds no field of it» is false.
@row_set n=2 row=decision/snapshot-state-readings cell=rule seen=1 detector=none
value: The snapshot text and the diff hold three state readings per control, selected, pressed and invalid, and a highlighted reading on an open single select's highlighted row, all as closed words and booleans.
basis: fct-voh3czmv, fct-gvhc5qds
because: A fourth closed state, the highlight, now crosses as a snapshot word and a diff reading inside the ratified carriers, so a rule naming only three readings is incomplete.
@row_set n=3 row=decision/cells-grid-keyboard cell=rule seen=1 detector=none
value: The 7GUIs Cells grid takes focus with tabindex 0 and a closed key match and no role: arrows move the current cell, clamped, Enter opens the editor on it, focused, and the editor's Enter commits, stops propagation and returns focus to the grid; the current cell alone states aria-selected true.
basis: fct-voh3czmv, fct-73upmmhj
because: The Cells grid's current data cell now states aria-selected true and an arrow moves it, so the grid's model gains its selected cell.
@row_add n=1 row=contract/diff-node/highlighted area=input-validation detector=none
boundary: Diff node keys
item: highlighted
promise: A boolean written only when true, on the one highlighted row of an open single select's list; a changed node's `differs` can name it.
basis: fct-s4gho2nl, fct-voh3czmv
because: The driver's closed node-key schema gains highlighted in the diff's changed and added nodes, a contract beside the other diff node keys.
@row_add n=2 row=decision/select-active-descendant-from-engine area=input-validation fact=F2 detector=none
name: Open select's active descendant from engine state
rule: While a single select's list is open the accessibility tree names the highlighted option as the select's active descendant from engine state alone; a page's aria-activedescendant stays unread and the tree's focus stays on the select.
why: An engine change for every document, ratified.
guard: an_open_list_names_its_highlighted_option_as_the_selects_active_descendant
basis: fct-voh3czmv, fct-ju67t7bl, fct-g5qothqo
because: The accessibility builder now writes the active descendant for an open single select from engine state while no page attribute is read, a rule no standing row says.
@row_add n=3 row=contract/snapshot-text/highlighted area=input-validation fact=F3 detector=none
boundary: Snapshot text state words
item: highlighted
promise: Written after `clickable` only when the element is the highlighted row of an open single select's list.
basis: fct-voh3czmv, fct-s4gho2nl
because: The snapshot text gains the state word highlighted after clickable, a contract item beside the other state words.
