@answer n=1 entity=standard-contracts#6.6 v=no
why: the session's reached files are dioxus-native-dom and the probe test, and blitz-dom's style data and resolve context are neither among them nor moved by the change
@answer n=2 entity=standard-contracts#7.1 v=no
why: blitz-dom's layout pass state and its Taffy trait implementations are in no reached file and the session changes no layout code
@answer n=3 entity=standard-contracts#8.1 v=no
why: the mix-blend-mode reading in Node's stacking-context arm is blitz-dom code in no reached file
@answer n=4 entity=standard-contracts#9.1 v=no
why: the node, element and widget types are blitz-dom and blitz-paint items in no reached file and the session changes none of them
@answer n=5 entity=standard-contracts#10.1 v=no
why: ElementData::is_disabled is blitz-dom code in no reached file and the session leaves its reading unchanged
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled in node::select_list is blitz-dom code in no reached file and the session does not change it
@answer n=7 entity=standard-contracts#10.3 v=no
why: the crate-private node::select module is blitz-dom code in no reached file and the session does not change it
@answer n=8 entity=standard-contracts#10.4 v=no
why: the select cursor and gesture items in node::select are blitz-dom code in no reached file and the session does not change them
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow are blitz-dom read types in no reached file and the session changes neither
@answer n=10 entity=standard-contracts#10.6 v=no
why: the SelectList fresh flag and its test are blitz-dom select behaviour in no reached file and the session leaves it unchanged
@answer n=11 entity=standard-contracts#10.7 v=no
why: the list's MAX_ROWS and placement rules in select_list.rs are blitz-dom code in no reached file and the session does not change them
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node out_of_flow recording is blitz-dom code in no reached file and the session changes no style traversal
@answer n=13 entity=standard-contracts#10.9 v=no
why: IS_INLINE_WITH_ONLY_BLOCK_CHILDREN and paints_own_box are blitz-dom layout and paint code in no reached file and the session does not change them
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData classes and parse_classes are blitz-dom items in no reached file and the session does not change them
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state and the open-state reads are blitz-dom code in no reached file and the session does not change them
@answer n=16 entity=standard-contracts#11.1 v=no
why: the blitz-traits provider traits are in no reached file and the session changes no trait
@answer n=17 entity=standard-contracts#12.1 v=no
why: the blitz-html and blitz-net public API is in no reached file and the session changes neither crate
@answer n=18 entity=standard-contracts#13.1 v=no
why: paint_scene, the blitz-shell re-exports and the password mask paint are blitz-paint and blitz-shell code in no reached file and the session changes none of them
@answer n=19 entity=standard-contracts#13.2 v=no
why: paint_select_list in blitz-paint is in no reached file and the session leaves it unchanged
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint in blitz-paint are in no reached file and the session changes no text paint
@answer n=21 entity=standard-contracts#13.4 v=no
why: blitz-paint's blend module and layer push are in no reached file and the session changes no paint code
@answer n=22 entity=standard-contracts#13.5 v=no
why: the blend's missing frame witness and its three layers.rs unit tests are blitz-paint facts in no reached file that the session does not move
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider's redraw path is blitz-shell code in no reached file and the session changes no shell code
@answer n=24 entity=standard-contracts#15.1 v=no
why: the harness's constructors, settle and screenshot surfaces are unchanged, blitz-test-harness is in no reached file, and the session moves no lock line
@answer n=25 entity=standard-contracts#16.1 v=no
why: the harness's clock and time methods are unchanged and blitz-test-harness is in no reached file
