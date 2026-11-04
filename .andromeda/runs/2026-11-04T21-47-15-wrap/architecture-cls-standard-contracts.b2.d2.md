@answer n=1 entity=standard-contracts#6.6 v=no
why: the session changed no packages/blitz-dom style code, only the new blitz-tests probe target, the Cargo.lock pulldown-cmark entry and deny.toml
@answer n=2 entity=standard-contracts#7.1 v=no
why: packages/blitz-dom layout and stylo_taffy are unmoved, the session changed no engine crate under packages/
@answer n=3 entity=standard-contracts#8.1 v=no
why: the node.rs stacking-context arm at packages/blitz-dom is unmoved, the session changed no engine crate under packages/
@answer n=4 entity=standard-contracts#9.1 v=no
why: NodeData, SpecialElementData and the widget and range types under packages/blitz-dom are unmoved, the session changed no engine code
@answer n=5 entity=standard-contracts#10.1 v=no
why: ElementData::is_disabled under packages/blitz-dom/src/node/element.rs is unmoved, the session changed no engine code
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled in packages/blitz-dom/src/node/select_list.rs is unmoved, the session changed no engine code
@answer n=7 entity=standard-contracts#10.3 v=no
why: the crate-private node::select module under packages/blitz-dom is unmoved, the session changed no engine code
@answer n=8 entity=standard-contracts#10.4 v=no
why: the select cursor and write-path items in packages/blitz-dom/src/node/select.rs are unmoved, the session changed no engine code
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow and their colour constants are unmoved, the session changed no engine code
@answer n=10 entity=standard-contracts#10.6 v=no
why: the SelectList fresh flag and the select_popup test it cites are unmoved, the new probe is a separate target that touches neither
@answer n=11 entity=standard-contracts#10.7 v=no
why: the select list layout rules in packages/blitz-dom/src/node/select_list.rs are unmoved, the session changed no engine code
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node::out_of_flow and its record and reader under packages/blitz-dom are unmoved, the session changed no engine code
@answer n=13 entity=standard-contracts#10.9 v=no
why: NodeFlags::IS_INLINE_WITH_ONLY_BLOCK_CHILDREN and its layout site under packages/blitz-dom are unmoved, the session changed no engine code
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData::classes and its sync sites under packages/blitz-dom are unmoved, the session changed no engine code
@answer n=15 entity=standard-contracts#10.11 v=no
why: ElementData::flush_open_state and its callers under packages/blitz-dom are unmoved, the session changed no engine code
@answer n=16 entity=standard-contracts#11.1 v=no
why: the provider traits in packages/blitz-traits are unmoved, the session changed no net or shell code
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-html and blitz-net under packages/ are unmoved, the session changed no HTML or networking code
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint_scene and the blitz-shell re-exports under packages/blitz-paint and packages/blitz-shell are unmoved, the session changed no paint or shell code
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list under packages/blitz-paint is unmoved, the session changed no paint code
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint under packages/blitz-paint/src/text.rs are unmoved, the session changed no paint code
@answer n=21 entity=standard-contracts#13.4 v=no
why: the blend layer in packages/blitz-paint is unmoved, the session changed no paint code
@answer n=22 entity=standard-contracts#13.5 v=no
why: the new probe reads the Dioxus DOM through public API and paints no frame, so no fork test paints a blended element or names convert_blend_mode and the claim holds
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider and its redraw path under packages/blitz-shell are unmoved, the session changed no shell code
@answer n=24 entity=standard-contracts#15.1 v=no
why: blitz-test-harness exports, constructors, settle and screenshot are unmoved, the probe only consumes the harness and no harness source or Cargo.toml moved
@answer n=25 entity=standard-contracts#16.1 v=no
why: the Cargo.lock change bumps only the pulldown-cmark entry, so the harness entry's color line and its one-line claim still hold
