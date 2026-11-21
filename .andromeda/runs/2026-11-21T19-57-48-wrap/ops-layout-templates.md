@row_add n=1 row=decision/single-select-sized-list-box area=surface-desktop-native fact=F1 detector=none
name: Sized single select as an in-flow list box
rule: A single select whose size parses above 1 is an in-flow list box: its rows laid out in the page with their own bounds, no popup list, role ListBox with no expanded, keys and row clicks choosing one row, nothing chosen allowed; any other size keeps the drop-down.
why: Ratified; size on a single select is read in every document, a drop-down being a select of display size 1.
guard: the_display_size_decides_list_box_or_drop_down, a_sized_single_select_opens_no_list
basis: fct-3lv5erx6, fct-s4plbaso, fct-prneohlv, fct-nebdrcee
because: A single select's size was unread and every single select a drop-down; now size above 1 makes it a list box whose rows read their own bounds, a choice future layout and driver work must respect.
@row_none n=1 fact=F2 type=4
relied: On a single list box the driver's click on a row is accepted and chooses it alone, press arrows, home and end choose with no list open, type chooses by the text's start, actions lists the rows with no cause; the verb set, key names, fields, causes and exits are unchanged.
@row_none n=2 fact=F3 type=4
relied: The boot lint reads a list box's rows scrolled out of its box as clipped, truncated or off-screen, never overlap, as it reads any scrolling box's hidden rows; the stand's own pages still lint clean.
