@answer n=1 entity=standard-contracts#6.6 v=no
why: the session reaches no blitz-dom style file, and its changes sit in dioxus-native-dom, escher-driver and the stand tests
@answer n=2 entity=standard-contracts#7.1 v=no
why: the session changes no blitz-dom layout file, and the layout API it names is untouched by the reached crates
@answer n=3 entity=standard-contracts#8.1 v=no
why: the session changes no blitz-dom node file, so the mix-blend-mode arm in node.rs stands as written
@answer n=4 entity=standard-contracts#9.1 v=no
why: the session changes no blitz-dom node or widget file, and the Widget trait and SpecialElementData stand
@answer n=5 entity=standard-contracts#10.1 v=no
why: the session touches no blitz-dom element file, so the is_disabled reading stands
@answer n=6 entity=standard-contracts#10.2 v=no
why: no select_list.rs change is reached by the session, so row_disabled stands
@answer n=7 entity=standard-contracts#10.3 v=no
why: the session reaches no blitz-dom select module, so the node::select rules and readers stand
@answer n=8 entity=standard-contracts#10.4 v=no
why: the session reaches no blitz-dom select module, so SelectCursor, apply_select_choice and the select_cursor field stand
@answer n=9 entity=standard-contracts#10.5 v=no
why: no select_list.rs change is reached, so SelectListView, SelectListRow and their colour constants stand
@answer n=10 entity=standard-contracts#10.6 v=no
why: no blitz-dom select or focus change is reached, and the cited select_popup.rs is not a changed file
@answer n=11 entity=standard-contracts#10.7 v=no
why: no select_list.rs change is reached, so the MAX_ROWS cap and the list placement stand
@answer n=12 entity=standard-contracts#10.8 v=no
why: no blitz-dom node or layout change is reached, so the recorded out-of-flow flag stands
@answer n=13 entity=standard-contracts#10.9 v=no
why: no blitz-dom construct or node change is reached, so the inline-with-block-children flag stands
@answer n=14 entity=standard-contracts#10.10 v=no
why: the session changes no blitz-dom element or mutator file, so the classes atoms stand
@answer n=15 entity=standard-contracts#10.11 v=no
why: no blitz-dom element, mutator or stylo change is reached, so flush_open_state and the :open reading stand
@answer n=16 entity=standard-contracts#11.1 v=no
why: blitz-traits is not a changed file and the session adds no provider or net trait change
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-html and blitz-net are not reached by any changed file, so their public API stands
@answer n=18 entity=standard-contracts#13.1 v=no
why: blitz-paint and blitz-shell are not reached, so paint_scene, the password mask and the shell re-exports stand
@answer n=19 entity=standard-contracts#13.2 v=no
why: no blitz-paint select-list change is reached, so paint_select_list stands
@answer n=20 entity=standard-contracts#13.3 v=no
why: no blitz-paint text change is reached, so stroke_text and RunPaint stand
@answer n=21 entity=standard-contracts#13.4 v=no
why: no blitz-paint change is reached, so the blend layer path stands
@answer n=22 entity=standard-contracts#13.5 v=no
why: no blitz-paint test or code change is reached, so the blend frame-witness gap is unchanged
@answer n=23 entity=standard-contracts#14.1 v=no
why: blitz-shell is not a changed file, so BlitzShellProvider and its redraw path stand
@answer n=24 entity=standard-contracts#15.1 v=no
why: blitz-test-harness is not a changed file, and its Harness<DioxusDocument> generic example still holds on the changed dioxus_document.rs
@answer n=25 entity=standard-contracts#16.1 v=no
why: the harness clock and its helpers live in blitz-test-harness, which the session does not reach
