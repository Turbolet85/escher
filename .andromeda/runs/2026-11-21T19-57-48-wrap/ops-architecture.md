@row_add n=1 row=decision/single-select-list-box area=established-decisions fact=F1 detector=D-arch-decisions
name: Sized single select as an in-flow list box
rule: A single select whose size parses above 1 is an in-flow list box in every document, display size 1 keeping the drop-down: it opens no list, arrows, Home, End and typeahead choose and scroll the row into view, a row click chooses that row alone whatever modifier, and nothing need be chosen.
why: ratified as a direct engine change for every document, flagged upstreamable
guard: the_display_size_decides_list_box_or_drop_down
basis: fct-3lv5erx6
because: The chunk makes a single select of display size above 1 a list box with its own keys and clicks, a standing decision no row states.
@row_add n=2 row=contract/select-readers/is-list-box area=standard-contracts fact=F2 detector=D-arch-resources
boundary: BaseDocument select readers
item: is_list_box
promise: Public and read-only: true for a select with multiple or a display size above 1; a stale or non-select id reads false.
basis: fct-s4plbaso
because: The chunk lands a new public BaseDocument reader, which the select readers' registry must carry.
@row_add n=3 row=contract/accessibility-tree/select-role area=standard-contracts fact=F3 detector=none
boundary: blitz-dom accessibility tree
item: select role
promise: A select reads ListBox where it is a list box, else ComboBox; a single list box carries no expanded, no active descendant, no author aria-expanded and is not multiselectable.
basis: fct-s4plbaso
because: The tree's role for a select now follows its display size, a contract at the accessibility boundary no row states.
@row_set n=1 row=contract/geometry/get-client-bounding-rect cell=promise seen=1 fact=F4 detector=none
value: get_client_bounding_rect reads an element where its transforms move it, and a node a restyle turned display none keeps the client rect it was last laid out at.
basis: fct-m2ceszre
because: The chunk measured that the public reader answers stale layout for a node turned display none, which a caller must read beside the computed display.
@row_set n=2 row=decision/targeted-restyle-on-mutation cell=except seen=1 fact=F5 detector=D-arch-decisions
value: a size or a multiple attribute write on a select restyles its descendants, since both decide its rows' presentational hints
basis: fct-bwztwsmk
because: A size or multiple write now restyles a select's whole subtree, an exception to the targeted-restyle rule the row does not carry.
@row_set n=3 row=decision/select-size-hint cell=rule seen=1 detector=D-arch-decisions
value: A select's `size` is read in every document: a `multiple` select's, or a single select's above 1, hints its height in rows, and a single select of display size 1 stays a drop-down.
basis: fct-qc4vactb
because: The stated limit that a single select's size stays unread is retired, so the row's second clause is false.
@row_set n=4 row=decision/single-select-list-opening cell=rule seen=1 detector=D-arch-decisions
value: A click, Enter and a Space with no typed text open a drop-down's list, a drop-down being a single select of display size 1, which the engine paints over the page with a group's `label` as a header.
basis: fct-s4plbaso
because: The list now opens on a drop-down alone, a single list box opening none.
@row_set n=5 row=decision/select-list-painted-overlay cell=rule seen=1 detector=D-arch-decisions
value: A drop-down, a single select of display size 1, opens a list the engine paints over the page, by a click, by Enter, or by a Space with no typed text standing.
basis: fct-s4plbaso
because: The open-list gate now reads the drop-down alone, so a single select of display size above 1 opens no list.
@row_set n=6 row=decision/select-keyboard-model cell=rule seen=1 detector=D-arch-decisions
value: A focused closed drop-down, a single select of display size 1, takes arrow keys and Home/End to enabled options and an unmodified character as typeahead reset after 1 s of the document's clock; Enter or a Space with no typed text opens its list, every other key does nothing.
basis: fct-s4plbaso
because: The drop-down key model is now reached by a drop-down alone, a single list box taking its own keys and opening nothing on Enter or Space.
@row_set n=7 row=decision/hit-focus-form-engine-changes cell=rule seen=3 detector=none
value: Visible region through transforms, no hit on non-invertible transforms, focus fixups under inert and after layout, Tab skipping hidden nodes, tree focus falling back to window, native toggled, multiple select, list box, optgroup and checkbox and radio change are engine changes for all documents.
basis: fct-lu2pn6ee
because: The single list box's clicks and keys are a further direct engine change for every document, flagged upstreamable.
@row_set n=8 row=decision/click-focuses-button cell=except seen=1 detector=none
value: a select focuses through its own arm, and a list box's row focuses its select
basis: fct-s4plbaso
because: A single list box's row click now focuses its select as a multiple select's row does.
@row_set n=9 row=decision/closed-select-options-covered cell=rule seen=1 detector=none
value: An option of a closed drop-down, a single select of display size 1, under a group too, takes no click and stays refused `covered`.
basis: fct-s4plbaso
because: closed_select_of now answers None for a list box, so a single list box's options take a click.
@row_set n=10 row=decision/multiple-select-chosen-row-colours cell=rule seen=1 detector=none
value: A list box's chosen row is painted with a #c8c8c8 background and #000000 text, a multiple select's from the default stylesheet and a single list box's as presentational hints.
basis: fct-lu2pn6ee
because: A single list box's chosen row now takes the same chosen pair through its rows' hints.
@row_set n=11 row=contract/accessibility-tree/select-expanded cell=promise seen=1 detector=none
value: A drop-down, a single select of display size 1, writes its native expanded, true while its list stands open, and a single list box writes none; no aria-expanded is read on either, and every other element keeps the aria-expanded read.
basis: fct-s4plbaso
because: expanded is now written on a drop-down alone, a single list box reading neither it nor an author aria-expanded.
@row_set n=12 row=contract/accessibility-tree/aria-expanded cell=promise seen=1 detector=none
value: Read as a boolean on every element but a single select, which reads none; a drop-down writes expanded from whether its list is open, and a single list box writes none.
basis: fct-s4plbaso
because: A single list box no longer writes expanded from an open list.
@row_set n=13 row=contract/dioxus-native-dom/closed-select-of cell=promise seen=1 detector=none
value: Answers None for a select whose list is open and for a select BaseDocument::is_list_box reads true.
basis: fct-s4plbaso
because: closed_select_of now also answers None for a list box.
@row_set n=14 row=contract/driver-forms/closed-select-of cell=promise seen=1 detector=none
value: The public closed_select_of answers None for a select whose list is open and for a select BaseDocument::is_list_box reads true.
basis: fct-s4plbaso
because: closed_select_of now also answers None for a list box.
@row_set n=15 row=contract/driver-forms/select-snapshot-options cell=promise seen=1 detector=none
value: A select's options are its children in the snapshot; a list box's rows and groups read client rects of their own, and an open list's rows and groups read select_list_bounds before the client rect.
basis: fct-3lv5erx6
because: A single list box's rows now read their own bounds in the snapshot, as a multiple select's do.
@row_set n=16 row=contract/snapshot/closed-select-option cell=promise seen=1 detector=none
value: An option of a closed drop-down, under an optgroup too, reads its select's client rect as bounds and is the select's child, named by its label; while the list is open a row reads its own place in the list.
basis: fct-s4plbaso
because: A single list box's option is no longer read as a closed select's option.
@row_set n=17 row=contract/snapshot/multiple-select-bounds cell=promise seen=1 detector=none
value: A list box's rows and groups, a multiple select's or a single select's of display size above 1, read their own client rects as their bounds.
basis: fct-3lv5erx6
because: A single list box's rows now read their own bounds, as a multiple select's do.
@row_set n=18 row=contract/lint-verb/overlap-select-rows cell=promise seen=1 detector=none
value: A closed drop-down's options, read by the element and never the role, and an open list's rows and groups are skipped by `overlap` on either side; a list box's rows are linted as usual.
basis: fct-m2ceszre
because: The lint's flatten reads closed_select_of, which now leaves a single list box's rows to be linted as usual.
@row_set n=19 row=contract/driver-verbs/click cell=promise seen=1 detector=none
value: Takes id; clicks the centre of the element's bounds after the disabled, off-screen and covered checks; a click on a drop-down opens its list, and a click on a list box's row chooses it.
basis: fct-3lv5erx6
because: A click on a select opens a list on a drop-down alone, and a single list box's row is accepted and chosen.
@row_set n=20 row=contract/select-list-default-actions/click-select cell=promise seen=1 detector=none
value: Clicking a drop-down, a single select of display size 1, opens its list, or closes one standing, focusing the select.
basis: fct-s4plbaso
because: click_select now opens a list on a drop-down alone.
@row_set n=21 row=contract/select-list-default-actions/open-keys cell=promise seen=1 detector=none
value: On a drop-down Enter opens the list, and so does a Space with no typed text standing; a released Space does nothing more; on a single list box neither opens anything.
basis: fct-3lv5erx6
because: Enter and a Space open nothing on a single list box.
