@row_set n=1 row=decision/single-select-selectedness cell=except seen=1 detector=none
value: A single select of display size above 1, a list box, falls back to no option and may stand with nothing chosen; an option of a closed drop-down has no click arm and stays refused covered.
basis: fct-s4plbaso
because: Settling now falls back to the first enabled option only at display size 1 and a single select's size is read, so the stated exception that its size is not read no longer holds.
@row_set n=2 row=decision/multiple-select-size-rows cell=rule seen=1 detector=none
value: A multiple select's size, a whole number above zero capped at 65534, makes it that many rows tall and a key scrolls the next row in; zero or a non-numeric value gives no hint.
basis: fct-qc4vactb
because: The stated limit that a single select's size is unread is retired, so the clause saying so leaves the rule.
@row_set n=3 row=decision/select-size-hint-cap cell=rule seen=1 detector=none
value: The `size` attribute on a select is read as a hint capped at 65534, with `0` or a non-numeric value giving no hint.
basis: fct-prneohlv
because: A single select's size is now read in every document through the same capped hint, so the hint is no longer a multiple select's alone.
@row_set n=4 row=decision/select-list-open-gate cell=rule seen=1 detector=none
value: Only a drop-down, a single select of display size 1, opens a list, and a disabled, inert or hidden one, one with no row to show, and one under a non-invertible transform open none.
basis: fct-s4plbaso
because: The open gate now reads the drop-down reading in place of the single-select reading, so a sized single select opens no list.
@row_set n=5 row=decision/select-list-keys-and-painted-labels cell=rule seen=1 detector=none
value: Enter and Space open a drop-down's engine-painted list, Space only with no typed text standing, and an option's and a group's label are painted in the open list.
basis: fct-3lv5erx6
because: A single select of size above 1 opens no list on Enter or Space, so the keys open the list of a drop-down alone.
@row_set n=6 row=decision/select-expanded-from-engine-state cell=rule seen=1 detector=none
value: An author's `open` on a select and `aria-expanded` on a single select steer nothing: a drop-down's expanded state is the engine's own closed state, a single list box states none, and `open` on a select does not match `:open`.
basis: fct-s4plbaso
because: The tree now writes the expanded state on a drop-down alone, and a single list box reads neither its own nor an author's.
@row_set n=7 row=decision/select-readers-resolve-node-closed-select-none cell=rule seen=1 detector=none
value: Each public select reader resolves its node so a stale id reads none, and the closed-select reader answers none for a select whose list is open or that is a list box.
basis: fct-s4plbaso
because: The closed-select reader now also answers none for a select the list-box reading holds true.
@row_set n=8 row=decision/select-list-closes-when-select-unfit cell=rule seen=1 detector=none
value: A select removed, disabled, hidden, made inert or made a list box with its list open closes the list with no panic, and a dropped option is no row.
basis: fct-3lv5erx6
because: A size write that turns a drop-down into a list box now closes its open list.
@row_add n=1 row=decision/single-select-list-box area=input-validation fact=F1 detector=none
name: Single select sized as a list box
rule: In every document, remote pages included, a single select whose size parses above 1 is an in-flow list box of its rows that opens no popup list and may stand with nothing chosen; size 1, 0, negative, non-numeric, empty or absent keeps the drop-down.
why: Ratified as an engine change for every document.
guard: select_interaction, select_popup, stand_act_select
basis: fct-3lv5erx6
because: A single select's size is now read in every document and decides list box or drop-down, a decision no standing row states.
@row_add n=2 row=decision/single-list-box-keys-and-clicks area=input-validation fact=F2 detector=none
name: Single list box keys and clicks
rule: A single list box takes a closed key set, the arrows, Home, End and plain-character typeahead, and a row click chooses that row alone whatever modifier is held; Enter, Space without typed text and every other key do nothing, and no multi-choice by Ctrl+click or Ctrl+A is offered.
why: Ratified as an engine change for every document.
guard: select_interaction, stand_act_select
basis: fct-s4plbaso
because: The single list box gains its own closed key match and row click, which no standing row states.
