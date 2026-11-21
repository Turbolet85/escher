@row_set n=1 row=decision/single-select-engine-overlay-list cell=rule seen=1 detector=none
value: A closed drop-down single select (display size 1) opens an engine overlay list on every document, remote pages included, answered first by hit testing and painted after the root; a single list box opens none.
basis: fct-s4plbaso
because: The list now opens on a drop-down alone, the open gate reading is_drop_down, so a single select of display size above 1 opens no list and the row's every-single-select claim is false.
@row_set n=2 row=contract/aria-property-reads/aria-expanded cell=promise seen=1 detector=none
value: aria-expanded is read as true or false and nothing else on every element but a single select, which reads no author attribute: a drop-down writes its list's open state instead, a single list box writes no expanded.
basis: fct-ap4gd4al
because: A single list box writes no expanded and reads no author aria-expanded, so the row's open-state clause no longer holds for every single select.
@row_set n=3 row=decision/single-select-expanded-from-list cell=rule seen=1 detector=none
value: A drop-down single select is a ComboBox that reads no aria-expanded and writes expanded from its list's open state, its chosen option selected; a single list box is a ListBox writing no expanded.
basis: fct-s4plbaso
because: The tree now reads ListBox where a select is a list box and writes expanded on a drop-down alone, so a single select is no longer always a ComboBox.
@row_set n=4 row=contract/accessibility-tree/single-select cell=promise seen=1 fact=F4 detector=none
value: A drop-down single select reads ComboBox and writes expanded from its list's open state; a single list box reads ListBox, no expanded, active descendant or multiselectable; an author's open or aria-expanded steers neither; its chosen option reads selected and the tree's focus stays on the select.
basis: fct-s4plbaso
because: A single select of display size above 1 now reads ListBox with no expanded, active descendant or multiselectable, so the ComboBox promise holds for a drop-down alone.
@row_set n=5 row=contract/accessibility-tree/select-expanded cell=promise seen=1 detector=none
value: A drop-down single select's expanded comes from engine state, never from an author aria-expanded, and focus stays on the select while its list is open; a single list box writes no expanded.
basis: fct-s4plbaso
because: expanded is now written on a drop-down alone, so a single list box carries none.
@row_set n=6 row=contract/accessibility-tree/select-role cell=promise seen=1 detector=none
value: A select reads ListBox where it is a list box, multiple or of display size above 1, else ComboBox; its options, under an optgroup or display none too, stay as ListBoxOption nodes, each selected true or false and named by its non-empty label attribute, else its text.
basis: fct-s4plbaso
because: The role now follows the list-box reading rather than the multiple attribute alone, so a single select of display size above 1 reads ListBox.
@row_set n=7 row=contract/select-closed-list/enter-space cell=promise seen=1 detector=none
value: Enter, and a Space with no typed text standing, open a focused drop-down's list on the chosen option and fire nothing; a disabled, inert, hidden or empty select, or a single list box, opens nothing.
basis: fct-3lv5erx6
because: A single list box now opens no list on a click, Enter or a Space, so the keys open a drop-down's list alone.
@row_set n=8 row=decision/hidden-content-excluded cell=why seen=2 detector=none
value: Every option of a drop-down single select but its chosen one computes display none.
basis: fct-3lv5erx6
because: A single list box's rows now lay out with boxes, so the reason holds for a drop-down's options alone.
@row_set n=9 row=contract/snapshot/closed-select-option-bounds cell=promise seen=1 fact=F7 detector=none
value: A closed drop-down's options read under it, each with the select's client rect as its bounds; a list box, multiple or of display size above 1, is no closed select and its rows read their own bounds.
basis: fct-s4plbaso
because: closed_select_of now answers none for a list box, so a single list box's rows read their own bounds and the closed-select reading holds for a drop-down alone.
@row_set n=10 row=decision/multiple-select-chosen-row-pair cell=rule seen=1 fact=F8 detector=none
value: A list box's chosen rows, multiple or single, read #000000 on #c8c8c8, Chromium's inactive selected pair, hardcoded in the default sheet.
basis: fct-itfudsnr
because: A single list box's chosen row now paints the same engine pair, 12.56:1 measured, so the pair is a list box's and not a multiple select's alone.
@row_add n=1 row=decision/select-list-box-or-drop-down area=aria-patterns-roles fact=F3 detector=D-a11y-surface
name: Select is a list box or a drop-down by display size
rule: A select's display size is its size attribute parsed above 1, else 1 (absent, 1, 0, negative, empty or unparsable), and a select is a list box when multiple or of display size above 1, a drop-down otherwise.
why: Ratified, following HTML's display size.
guard: the_display_size_decides_list_box_or_drop_down
basis: fct-s4plbaso
because: The chunk makes display size decide a select's role, keys, clicks and list, a classification every later select change must respect.
@row_add n=2 row=contract/single-list-box/keys area=keyboard-navigation fact=F1 detector=D-a11y-surface
boundary: single list box
item: keys
promise: On a focused single list box that is no disabled form control, the arrows, Home and End step and choose, a printable key with no Ctrl, Alt or Meta is typeahead, the reached row is scrolled into view; Enter, a Space with no typed text and every other key do nothing, and no list opens.
basis: fct-s4plbaso
because: A single select of display size above 1 is a new interactive element with its own key handling, and its keyboard coverage under SC 2.1.1 stands in no row.
@row_add n=3 row=contract/single-list-box/row-click area=keyboard-navigation fact=F2 detector=D-a11y-surface
boundary: single list box
item: row click
promise: A click on a single list box's row focuses the select, clears its typeahead and chooses that row alone whatever modifier is held; a disabled select or row changes nothing.
basis: fct-s4plbaso
because: A single list box's row click is a new pointer path apart from a drop-down's and a multiple select's, and no row states it.
@row_add n=4 row=decision/only-drop-down-settles-choice area=keyboard-navigation fact=F5 detector=none
name: Only a drop-down settles to its first enabled option
rule: A single list box may stand with nothing chosen; only a drop-down settles to its first enabled option, a list box turned back into a drop-down by a size write included.
basis: fct-s4plbaso
because: The choice settle now falls back to the first enabled option at display size 1 alone, a selectedness rule later select work must respect.
@row_add n=5 row=decision/size-write-reclassifies-select area=keyboard-navigation fact=F6 detector=none
name: A size or multiple write reclassifies a select
rule: A size or multiple write moves a select between list box and drop-down, restyling its rows and closing an open list.
basis: fct-3lv5erx6
because: A select's kind is now live under attribute writes, so a list and the select's keys follow a write, which no row states.
@row_none n=1 fact=F9 type=4
relied: the list-box page's boot lint check pins its scrolled-out rows as clipped, truncated or off-screen findings with no overlap.
