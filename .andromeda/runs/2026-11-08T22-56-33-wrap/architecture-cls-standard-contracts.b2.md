@answer n=1 entity=standard-contracts#6.6 v=no
why: the session's reached files are dioxus-native-dom, escher-driver and blitz-tests stand files, none of them blitz-dom's style-resolve code.
@answer n=2 entity=standard-contracts#7.1 v=no
why: no blitz-dom layout or stylo_taffy file is among the session's reached files, so the layout API it lists is not reached.
@answer n=3 entity=standard-contracts#8.1 v=no
why: the node stacking-context arm in blitz-dom node.rs is not in any file the session reaches.
@answer n=4 entity=standard-contracts#9.1 v=no
why: the node and widget types are blitz-dom's, and the session reaches no blitz-dom node or widget file.
@answer n=5 entity=standard-contracts#10.1 v=no
why: ElementData::is_disabled in blitz-dom element.rs is not in any file the session reaches.
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled in blitz-dom select_list.rs is not in any file the session reaches.
@answer n=7 entity=standard-contracts#10.3 v=no
why: the blitz-dom node::select module is not in any file the session reaches.
@answer n=8 entity=standard-contracts#10.4 v=no
why: the blitz-dom node::select internals and document.rs select_cursor are not in any file the session reaches.
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow in blitz-dom select_list.rs are not in any file the session reaches.
@answer n=10 entity=standard-contracts#10.6 v=no
why: the SelectList fresh-open rule and its cited select_popup test are not in any file the session reaches.
@answer n=11 entity=standard-contracts#10.7 v=no
why: the select list's open placement in blitz-dom select_list.rs is not in any file the session reaches.
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node::out_of_flow in blitz-dom node.rs and layout/mod.rs is not in any file the session reaches.
@answer n=13 entity=standard-contracts#10.9 v=no
why: IS_INLINE_WITH_ONLY_BLOCK_CHILDREN in blitz-dom node.rs and construct.rs is not in any file the session reaches.
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData::classes in blitz-dom element.rs, mutator.rs and stylo.rs is not in any file the session reaches.
@answer n=15 entity=standard-contracts#10.11 v=no
why: ElementData::flush_open_state in blitz-dom element.rs and mutator.rs is not in any file the session reaches.
@answer n=16 entity=standard-contracts#11.1 v=no
why: the blitz-traits provider traits are not in any file the session reaches.
@answer n=17 entity=standard-contracts#12.1 v=no
why: the blitz-html and blitz-net files it names are not in any file the session reaches.
@answer n=18 entity=standard-contracts#13.1 v=no
why: the blitz-paint and blitz-shell paint and shell files it names are not in any file the session reaches.
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list in blitz-paint render/select_list.rs is not in any file the session reaches.
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and stroke_text_as in blitz-paint text.rs are not in any file the session reaches.
@answer n=21 entity=standard-contracts#13.4 v=no
why: the blend code in blitz-paint blend.rs, lib.rs and render.rs is not in any file the session reaches.
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend's missing frame witness concerns blitz-paint layers.rs and upstream tests, none of which the session reaches.
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider and its redraw path in blitz-shell are not in any file the session reaches.
@answer n=24 entity=standard-contracts#15.1 v=no
why: blitz-test-harness is not in any file the session reaches, and the session adds to DioxusDocument without changing the harness's exports or its generic example.
@answer n=25 entity=standard-contracts#16.1 v=no
why: the harness's VirtualClock, time and tick are blitz-test-harness code the session does not reach.
