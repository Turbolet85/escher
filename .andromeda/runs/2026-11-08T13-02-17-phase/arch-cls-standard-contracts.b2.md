@answer n=1 entity=standard-contracts#6.6 v=no
why: the style-data filter in TElement::borrow_data is blitz-dom style resolve, which the read neither names nor calls, so it is a re-check every change owes
@answer n=2 entity=standard-contracts#7.1 v=no
why: the layout-pass traits and paint-tree API are blitz-dom layout internals the read does not name or reach
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode stacking arm is paint and style behaviour the read does not touch
@answer n=4 entity=standard-contracts#9.1 v=no
why: the node, special-element and widget types lie outside the read's actionable set and verb table, and the scope names none of them
@answer n=5 entity=standard-contracts#10.1 v=yes
why: ElementData::is_disabled is the disabled reading the scope names for the shared refusal path
@answer n=6 entity=standard-contracts#10.2 v=yes
why: row_disabled is the open list's disabled-row reading beside is_disabled and option_disabled, which the disabled refusal answers for list options
@answer n=7 entity=standard-contracts#10.3 v=yes
why: the select module's options and is_select_option readers define the owner-served select's options the read lists as children
@answer n=8 entity=standard-contracts#10.4 v=yes
why: option_disabled and owner_select are the disabled reading and the select-serves-its-options owner the scope names
@answer n=9 entity=standard-contracts#10.5 v=yes
why: SelectListView and SelectListRow are the public reading of the open list that the refusal path answers first
@answer n=10 entity=standard-contracts#10.6 v=no
why: the fresh flag guards a click's own focus restyle, and the read runs no step and restyles nothing
@answer n=11 entity=standard-contracts#10.7 v=yes
why: MAX_ROWS and the list's placement decide which options of an open list are shown and covered, the refusals the read answers
@answer n=12 entity=standard-contracts#10.8 v=no
why: the out-of-flow flag is a layout-pass record the read neither names nor reads through visible_region or hit
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-block-children paint skip is paint behaviour the read does not reach
@answer n=14 entity=standard-contracts#10.10 v=no
why: the classes atom list is styling data, and the read keys on author keys and paths, not classes
@answer n=15 entity=standard-contracts#10.11 v=no
why: the open-state flush for details and dialog is outside the scope's actionable set and refusals
@answer n=16 entity=standard-contracts#11.1 v=no
why: the provider traits and fetch are networking, and the read makes no fetch and waits on no load
@answer n=17 entity=standard-contracts#12.1 v=no
why: the blitz-html and blitz-net crate API is not used by the read
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint, shell and the password-mask caret path govern painting and pointer placement, which the read neither does nor dispatches
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list paints the open list, and the read paints nothing
@answer n=20 entity=standard-contracts#13.3 v=no
why: the stroke_text paint helper is unused by the read
@answer n=21 entity=standard-contracts#13.4 v=no
why: blend-layer paint order is paint behaviour, and the read's covered answer comes from hit, not paint
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend's missing frame witness is a paint concern outside the read
@answer n=23 entity=standard-contracts#14.1 v=no
why: the BlitzShellProvider redraw path is shell behaviour, and the read dispatches nothing and requests no redraw
@answer n=24 entity=standard-contracts#15.1 v=yes
why: the harness constructors, inspection, HarnessOptions and settle are the tools the minimal fixture and the stand proof use, and the read must not settle
@answer n=25 entity=standard-contracts#16.1 v=yes
why: the harness's time rule names the virtual clock the read must not move, and settle and screenshot keep it still
