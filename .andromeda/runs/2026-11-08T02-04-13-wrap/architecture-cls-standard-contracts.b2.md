@answer n=1 entity=standard-contracts#6.6 v=no
why: the session edits no blitz-dom file, so the stylo.rs style-data claim and its cited lines hold.
@answer n=2 entity=standard-contracts#7.1 v=no
why: the session edits no blitz-dom layout file, so the LayoutPassState and Taffy trait claims hold.
@answer n=3 entity=standard-contracts#8.1 v=no
why: the session edits no blitz-dom node.rs, so the mix-blend-mode stacking-context reading holds.
@answer n=4 entity=standard-contracts#9.1 v=no
why: blitz-dom's node kinds and the Widget trait are unedited, and the session adds no node kind or widget.
@answer n=5 entity=standard-contracts#10.1 v=no
why: the is_disabled reading and its three engine consumers are unchanged, and the snapshot's read of it is an outside reader that adds no stated engine claim.
@answer n=6 entity=standard-contracts#10.2 v=no
why: row_disabled in select_list.rs is unedited and the session adds no select reading.
@answer n=7 entity=standard-contracts#10.3 v=no
why: the crate-private select module in blitz-dom is unedited.
@answer n=8 entity=standard-contracts#10.4 v=no
why: the select cursor and its helpers in select.rs are unedited.
@answer n=9 entity=standard-contracts#10.5 v=no
why: SelectListView and SelectListRow in select_list.rs and the blitz-dom root re-exports are unedited.
@answer n=10 entity=standard-contracts#10.6 v=no
why: the SelectList.fresh focus-restyle path is unedited and the cited select_popup test is not among the reached files.
@answer n=11 entity=standard-contracts#10.7 v=no
why: the list placement rules in select_list.rs are unedited.
@answer n=12 entity=standard-contracts#10.8 v=no
why: Node::out_of_flow and style_is_out_of_flow in blitz-dom are unedited.
@answer n=13 entity=standard-contracts#10.9 v=no
why: the inline-with-block-children flag in node.rs and construct.rs is unedited.
@answer n=14 entity=standard-contracts#10.10 v=no
why: ElementData classes in element.rs and the mutator's class arm are unedited.
@answer n=15 entity=standard-contracts#10.11 v=no
why: flush_open_state in element.rs and the mutator's open arm are unedited.
@answer n=16 entity=standard-contracts#11.1 v=no
why: blitz-traits net.rs and shell.rs are unedited and no manifest or dependency moved.
@answer n=17 entity=standard-contracts#12.1 v=no
why: blitz-html and blitz-net are unedited and no networking dependency moved.
@answer n=18 entity=standard-contracts#13.1 v=no
why: blitz-paint and blitz-shell are unedited, and the session edits no paint or shell file.
@answer n=19 entity=standard-contracts#13.2 v=no
why: blitz-paint's select_list render module is unedited.
@answer n=20 entity=standard-contracts#13.3 v=no
why: stroke_text and RunPaint in blitz-paint's text.rs are unedited.
@answer n=21 entity=standard-contracts#13.4 v=no
why: the blitz-paint blend and render paths are unedited.
@answer n=22 entity=standard-contracts#13.5 v=no
why: layers.rs and its three unit tests are unedited, and the session adds no blend test.
@answer n=23 entity=standard-contracts#14.1 v=no
why: BlitzShellProvider in blitz-shell is unedited, and the session's event dispatch change sits in dioxus-native-dom, not in the shell provider.
@answer n=24 entity=standard-contracts#15.1 v=no
why: blitz-test-harness is not among the reached files, and its exports, settle and screenshot readings hold unchanged.
@answer n=25 entity=standard-contracts#16.1 v=no
why: harness.rs is not among the reached files, and the session moves no harness clock.
