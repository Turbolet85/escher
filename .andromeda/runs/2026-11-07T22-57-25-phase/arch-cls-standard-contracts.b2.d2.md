@answer n=1 entity=standard-contracts#6.6 v=no
why: the style-data filter in TElement::borrow_data is blitz-dom engine state the graph read does not touch
@answer n=2 entity=standard-contracts#7.1 v=no
why: the layout pass types and Taffy trait wiring are engine internals, and the graph takes bounds from built nodes
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode stacking-context arm is a paint-tree rule the graph read never reaches
@answer n=4 entity=standard-contracts#9.1 v=yes
why: the graph reader and is_masked_input match on element kinds, and SpecialElementData gains RangeInput, a variant an exhaustive external match must now cover
@answer n=5 entity=standard-contracts#10.1 v=yes
why: is_disabled gates the pointer and key paths, and the scope leaves open whether a disabled control with a click handler reads as reacting to a click, a fork over that reading
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled is the crate-private select list's reading of a disabled row, and the graph does not read select list rows
@answer n=7 entity=standard-contracts#10.3 v=no
why: the crate-private select rules and typeahead are neither named by the scope nor read by the graph
@answer n=8 entity=standard-contracts#10.4 v=no
why: the select cursor and gesture internals are crate-private and lie outside the graph read
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow are paint-side list reads, and the graph paints nothing
@answer n=10 entity=standard-contracts#10.6 v=no
why: the fresh-open guard on a select list concerns a focus restyle that the graph read never runs
@answer n=11 entity=standard-contracts#10.7 v=no
why: the list's row count and placement rules are paint geometry the chunk does not touch
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node::out_of_flow is a layout-pass flag the graph read does not consult
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-block-children flag only changes how a box is painted, and the graph paints nothing
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData::classes is class-atom storage that the graph reads through the vdom, not through classes
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state sets the :open pseudo-class state, which the graph does not read
@answer n=16 entity=standard-contracts#11.1 v=no
why: the NetProvider and handler traits govern fetches, and the chunk makes no fetch and changes no provider
@answer n=17 entity=standard-contracts#12.1 v=no
why: the blitz-html and blitz-net parsing and fetch APIs lie outside the graph read and the stand's wiring
@answer n=18 entity=standard-contracts#13.1 v=no
why: this is paint and shell rendering, including the painted password mask and caret, which the graph does not do, and it does not state MASKED_VALUE
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list is a paint rule for select list boxes that the graph does not paint
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint are text-painting internals the graph does not touch
@answer n=21 entity=standard-contracts#13.4 v=no
why: mix-blend-mode and isolation paint order is a frame rule, and the graph paints nothing
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend's missing frame witness is an upstream-sync carry that the chunk's code and tests do not act on
@answer n=23 entity=standard-contracts#14.1 v=no
why: the windowed shell's redraw request path is shell plumbing that the chunk's read and stand do not drive
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the stand checks build Harness<DioxusDocument> with the pinned 800x600 Light options, and the input helpers settle nothing, so the tests must call settle before reading the graph
@answer n=25 entity=standard-contracts#16.1 v=no
why: the harness clock moves only on tick and advance_time, and the graph read is clock-free
