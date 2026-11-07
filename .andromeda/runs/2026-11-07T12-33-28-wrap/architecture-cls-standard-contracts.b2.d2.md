@answer n=1 entity=standard-contracts#6.6 v=no
why: its style-data and resolve-context claim lives in blitz-dom's stylo.rs, which the session does not reach
@answer n=2 entity=standard-contracts#7.1 v=no
why: the layout-pass API lives in blitz-dom's layout modules, which the session does not reach; the bridge only replays mutations
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode stacking-context read lives in blitz-dom's node.rs, which the session does not reach
@answer n=4 entity=standard-contracts#9.1 v=no
why: the node, element and widget types live in blitz-dom, and the session adds only a crate-private buffer in dioxus-native-dom
@answer n=5 entity=standard-contracts#10.1 v=no
why: ElementData::is_disabled lives in blitz-dom's element.rs, which the session does not reach
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled lives in blitz-dom's select_list.rs, which the session does not reach
@answer n=7 entity=standard-contracts#10.3 v=no
why: the crate-private select module lives in blitz-dom's select.rs, which the session does not reach
@answer n=8 entity=standard-contracts#10.4 v=no
why: SelectCursor and the other select readers live in blitz-dom's select.rs and document.rs, which the session does not reach
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow live in blitz-dom's select_list.rs, which the session does not reach
@answer n=10 entity=standard-contracts#10.6 v=no
why: SelectList.fresh and its test live in blitz-dom's select_list and select_popup test, neither reached by the session
@answer n=11 entity=standard-contracts#10.7 v=no
why: the MAX_ROWS and placement rules live in blitz-dom's select_list.rs, which the session does not reach
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node::out_of_flow and its record live in blitz-dom's node.rs and layout module, which the session does not reach
@answer n=13 entity=standard-contracts#10.9 v=no
why: the IS_INLINE_WITH_ONLY_BLOCK_CHILDREN flag lives in blitz-dom's node.rs and construct.rs, which the session does not reach
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData::classes lives in blitz-dom's element.rs and mutator.rs, which the session does not reach
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state lives in blitz-dom's element.rs and mutator.rs, which the session does not reach
@answer n=16 entity=standard-contracts#11.1 v=no
why: the provider traits live in blitz-traits, and the session adds no provider and changes no trait
@answer n=17 entity=standard-contracts#12.1 v=no
why: the blitz-html and blitz-net APIs live in crates the session does not reach
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint_scene and the blitz-shell re-exports live in blitz-paint and blitz-shell, which the session does not reach; the windowed launch calls the bridge as before
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list lives in blitz-paint's select_list render module, which the session does not reach
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint live in blitz-paint's text.rs, which the session does not reach
@answer n=21 entity=standard-contracts#13.4 v=no
why: the blend-layer paint path lives in blitz-paint's blend.rs and render.rs, which the session does not reach
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend has no frame witness and its unit tests live in blitz-paint's layers.rs, which the session does not reach
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider and its redraw path live in blitz-shell, which the session does not reach; the windowed launch calls the bridge as before
@answer n=24 entity=standard-contracts#15.1 v=no
why: the harness API, from_vdom, settle and screenshot live in blitz-test-harness, which the session does not reach; from_vdom and the poll signature are unchanged
@answer n=25 entity=standard-contracts#16.1 v=no
why: the harness clock and its time methods live in blitz-test-harness, which the session does not reach; no clock moved
