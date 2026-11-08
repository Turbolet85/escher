@answer n=1 entity=standard-contracts#6.6 v=no
why: the session reaches only escher-driver, dioxus-native-dom and stand test files, and this blitz-dom style-data rule lives in packages/blitz-dom/src/stylo.rs, which it does not reach
@answer n=2 entity=standard-contracts#7.1 v=no
why: the layout traits and stylo_taffy exports live in blitz-dom and stylo_taffy files the session does not reach, and blitz-dom is untouched
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode reading in packages/blitz-dom/src/node/node.rs is an engine file the session does not reach
@answer n=4 entity=standard-contracts#9.1 v=no
why: the NodeData, SpecialElementData and Widget surfaces are blitz-dom node files the session does not reach, and ActionableNode is a dioxus-native-dom type that joins no node list here
@answer n=5 entity=standard-contracts#10.1 v=no
why: ElementData::is_disabled lives in packages/blitz-dom/src/node/element.rs, which the session does not reach, and the dioxus actionable rule named in fact 6 does not read it
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled in node::select_list is a blitz-dom reader the session does not reach
@answer n=7 entity=standard-contracts#10.3 v=no
why: the node::select module holds blitz-dom select rules the session does not reach
@answer n=8 entity=standard-contracts#10.4 v=no
why: SelectCursor, RowGesture and the other node::select items are blitz-dom internals the session does not reach
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow are blitz-dom read types whose files the session does not reach
@answer n=10 entity=standard-contracts#10.6 v=no
why: SelectList.fresh and its select_popup test are blitz-dom and tests/blitz-tests files the session does not reach
@answer n=11 entity=standard-contracts#10.7 v=no
why: the MAX_ROWS and placement rules live in packages/blitz-dom/src/node/select_list.rs, which the session does not reach
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node::out_of_flow and its recorded flag are blitz-dom node and layout code the session does not reach
@answer n=13 entity=standard-contracts#10.9 v=no
why: NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN and paints_own_box are blitz-dom code the session does not reach
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData::classes and its sync in mutator.rs and stylo.rs are blitz-dom code the session does not reach
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state and the open-state arms are blitz-dom code the session does not reach
@answer n=16 entity=standard-contracts#11.1 v=no
why: the provider traits live in packages/blitz-traits, which the session does not reach, and it adds no provider
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-html and blitz-net are crates the session does not reach
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint_scene and the blitz-shell surface are blitz-paint and blitz-shell code the session does not reach
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list lives in blitz-paint's render::select_list, which the session does not reach
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and stroke_text_as are blitz-paint text code the session does not reach
@answer n=21 entity=standard-contracts#13.4 v=no
why: the blend layer paint path is blitz-paint code the session does not reach
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend has no frame witness and the session adds no blitz-paint test, so the claim holds
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider is blitz-shell code the session does not reach
@answer n=24 entity=standard-contracts#15.1 v=no
why: the harness source is not in the session's file list, its scroll_into_view caller the driver scroll arm still calls it, and the screenshot and settle claims hold unchanged
@answer n=25 entity=standard-contracts#16.1 v=no
why: the harness clock and its constructors are blitz-test-harness code the session does not reach, and it moves no clock
