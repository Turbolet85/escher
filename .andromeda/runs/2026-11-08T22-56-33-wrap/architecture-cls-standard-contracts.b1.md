@answer n=1 entity=standard-contracts#1.1 v=yes mode=stale
why: the session reaches packages/dioxus-native-dom/src/dioxus_document.rs whole, and the cited DioxusDocument Document impl at lines 280-355 can move under its new record_reached and reached_handlers code
@answer n=2 entity=standard-contracts#2.1 v=no
why: the session adds no clock use and touches neither blitz-dom clock.rs nor the Clock re-exports
@answer n=3 entity=standard-contracts#2.2 v=no
why: select_value and its choice writers are blitz-dom document.rs code the session does not reach
@answer n=4 entity=standard-contracts#2.3 v=no
why: the select list state and its readers are blitz-dom document.rs code the session does not reach
@answer n=5 entity=standard-contracts#2.4 v=no
why: live_select_list and reconcile_select_list are blitz-dom document.rs and resolve.rs code the session does not reach
@answer n=6 entity=standard-contracts#2.5 v=no
why: the resolve reconcile call and the removal arm are blitz-dom code the session does not reach
@answer n=7 entity=standard-contracts#2.6 v=yes mode=stale
why: the session reaches packages/dioxus-native-dom/src/lib.rs whole, and its cited crate-root re-export block at lines 48-51 moves when ReachedHandler joins the re-exports, 35 to 36
@answer n=8 entity=standard-contracts#2.7 v=no
why: the multiple-select mutator write is blitz-dom mutator.rs code the session does not reach
@answer n=9 entity=standard-contracts#2.8 v=no
why: NetHandler::error is blitz-dom net.rs code the session does not reach
@answer n=10 entity=standard-contracts#3.1 v=no
why: mutation, query and CSSOM are blitz-dom code, and the test files they cite lie outside the session's reached files
@answer n=11 entity=standard-contracts#3.2 v=no
why: the single-select expanded write is blitz-dom accessibility.rs code the session does not reach
@answer n=12 entity=standard-contracts#3.3 v=no
why: the tree focus write is blitz-dom accessibility.rs code the session does not reach
@answer n=13 entity=standard-contracts#3.4 v=no
why: the native checkable state write is blitz-dom accessibility.rs code the session does not reach
@answer n=14 entity=standard-contracts#3.5 v=no
why: the resolved width and height readings are blitz-dom resolved_style.rs code the session does not reach
@answer n=15 entity=standard-contracts#3.6 v=no
why: restyle on mutation is blitz-dom mutator.rs code the session does not reach
@answer n=16 entity=standard-contracts#4.1 v=no
why: the EventHandler and EventDriver contract is blitz-dom code, and the Dioxus bridge's reached-record write before runtime.handle_event changes no converter, value or handler ordering this entity states
@answer n=17 entity=standard-contracts#4.2 v=no
why: a select list's default actions are blitz-dom events code the session does not reach
@answer n=18 entity=standard-contracts#4.3 v=no
why: the label click arm is blitz-dom pointer.rs code the session does not reach
@answer n=19 entity=standard-contracts#4.4 v=no
why: checkable change dispatch is blitz-dom pointer.rs code the session does not reach
@answer n=20 entity=standard-contracts#5.1 v=no
why: scroll_into_view and visible_region are blitz-dom scrolling.rs code the session does not reach, and the Dioxus and test-harness callers it names keep their readings
@answer n=21 entity=standard-contracts#6.1 v=no
why: the client rect under transforms is blitz-dom document.rs code the session does not reach, and the dioxus-native-dom get_client_rect reader keeps reading get_client_bounding_rect unchanged
@answer n=22 entity=standard-contracts#6.2 v=no
why: visible_region under transforms is blitz-dom scrolling.rs code the session does not reach
@answer n=23 entity=standard-contracts#6.3 v=no
why: text_extent is blitz-dom text_extent.rs code, and the session adds no caller of it
@answer n=24 entity=standard-contracts#6.4 v=no
why: the hit walk's skip of dropped nodes is blitz-dom node.rs code the session does not reach
@answer n=25 entity=standard-contracts#6.5 v=no
why: the non-invertible transform hit rule is blitz-dom node.rs code the session does not reach
