@row_set n=1 row=decision/select-chosen-row-chromium-pair cell=rule seen=1 fact=F5 detector=none
value: A list box's chosen row, of a multiple select or of a single select sized above 1, is painted with Chromium's inactive selected pair, a #c8c8c8 background with #000000 text, 12.56:1.
basis: fct-itfudsnr
because: A single list box's chosen row now takes the same pair as a multiple select's, so the rule naming the multiple select alone says it less.
@row_set n=2 row=decision/single-select-box-shows-chosen-label cell=rule seen=1 detector=none
value: A single select of display size 1, a drop-down, lays out as a box holding its chosen option's label, from engine defaults with no design token.
basis: fct-3lv5erx6
because: A single select whose size parses above 1 now lays out as a list box of its rows, so the box holding the chosen label is the drop-down's alone.
@row_set n=3 row=decision/single-select-open-gestures cell=rule seen=1 detector=none
value: A drop-down single select's list opens on a click, Enter, or a Space with no typed text standing; a single select sized above 1 opens no list on any of them.
basis: fct-3lv5erx6
because: A single list box now opens no list on a click, Enter or a Space, so the opening gestures hold for the drop-down alone.
@row_set n=4 row=decision/selects-that-open-no-list cell=rule seen=1 detector=none
value: A disabled, inert or hidden select, a select with no row to show, a multiple select and a single select sized above 1 open no list.
basis: fct-s4plbaso
because: The open gate now reads the drop-down alone, so a single list box joins the selects that open no list.
@row_set n=5 row=contract/select-list-events/close-on-select-change cell=promise seen=1 detector=none
value: Removing, disabling, hiding or making its select inert, or a size write that makes its select a list box, closes the list.
basis: fct-3lv5erx6
because: A size write moving a select between drop-down and list box now closes an open list, a new way the list closes on its select's change.
@row_add n=1 row=decision/single-select-display-size area=surface fact=F1 detector=none
name: Single select display size decides list box or drop-down
rule: A single select whose size parses above 1 is an in-flow list box of that many rows, the rest scrolling, opening no list; display size 1, from size absent, 0, 1, empty, negative or non-numeric, keeps the drop-down, and a size write moves a select between the two.
why: Ratified.
guard: the_display_size_decides_list_box_or_drop_down
basis: fct-3lv5erx6
because: The engine now reads a single select's size, which before stayed unread, and no row states which display size gives which control.
@row_add n=2 row=decision/single-list-box-may-stand-unchosen area=surface fact=F2 detector=none
name: Single list box may stand with nothing chosen
rule: A single select sized above 1 may stand with no option chosen; only a drop-down falls back to its first enabled option.
why: Ratified.
guard: a_sized_single_select_with_nothing_chosen_reads_no_value
basis: fct-s4plbaso
because: The settle of a select's choice now falls back to the first enabled option at display size 1 alone, a decision no row states.
@row_add n=3 row=decision/single-list-box-keys-and-clicks area=surface fact=F3 detector=none
name: Single list box keys and row clicks choose a row
rule: A single list box's arrows, Home, End and typeahead choose a row and bring it into view, a row click chooses that row alone whatever modifier is held, and Enter, Space without typed text and other keys do nothing; a disabled select or row changes nothing.
why: Ratified.
guard: the_keys_of_a_sized_single_select_choose_and_scroll
basis: fct-s4plbaso
because: A single list box gained its own key and click handling, distinct from the drop-down's list events, and no row states it.
@row_add n=4 row=contract/select-accessibility/single-list-box area=surface fact=F4 detector=none
boundary: The snapshot and accessibility tree of a select
item: single list box
promise: A single select sized above 1 reads role ListBox in the snapshot and tree, with no expanded state, no active descendant and no multiselectable, and an author aria-expanded is not read.
basis: fct-s4plbaso
because: The tree now reads a single list box as ListBox without the drop-down's expanded state and active descendant, a promise no row of the select's tree makes.
@row_none n=1 fact=F6 type=4
relied: The driver lint reads the rows of any scrolling box scrolled out of it as clipped, truncated or off-screen findings, never overlap; the chunk measured this unchanged lint behaviour on a list box and pinned it.
