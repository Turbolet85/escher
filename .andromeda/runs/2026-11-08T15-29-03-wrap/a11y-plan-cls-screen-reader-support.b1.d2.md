@answer n=1 entity=screen-reader-support#1.1 v=no
why: its TextRun rule lives in blitz-dom accessibility.rs, which the session did not reach
@answer n=2 entity=screen-reader-support#1.2 v=no
why: the aria-label naming lives in blitz-dom and its accessibility tests, none of them reached by the session
@answer n=3 entity=screen-reader-support#1.3 v=no
why: the label association lives in blitz-dom and document.rs, neither reached by the session
@answer n=4 entity=screen-reader-support#1.4 v=yes mode=add-to
why: the flight booker's closed-select options read covered with owner flight-type until the list opens (F6), which joins this option reading
@answer n=5 entity=screen-reader-support#1.5 v=no
why: the expanded reading comes from blitz-dom and select_popup.rs, neither reached by the session
@answer n=6 entity=screen-reader-support#1.6 v=yes mode=stale
why: docs/capabilities.md is reached whole and this entity cites its lines 245-247, which may have moved, and no report here shows the limit still holds on the changed file
@answer n=7 entity=screen-reader-support#1.7 v=no
why: the unmeasured open-list reading is recorded as unrunnable-here and the session does not touch it
@answer n=8 entity=screen-reader-support#1.8 v=no
why: the multiple-select row bounds rest on stand_act_select.rs and snapshot.rs, neither reached by the session
@answer n=9 entity=screen-reader-support#1.9 v=no
why: the evt.values reading lives in dioxus_document.rs, which the session did not change
@answer n=10 entity=screen-reader-support#2.1 v=yes mode=stale
why: tests/blitz-tests/tests/common/mod.rs is reached whole and this entity cites its lines 17-185, which may have moved, and no report here shows the claim still holds on it
@answer n=11 entity=screen-reader-support#2.2 v=no
why: stand_graph.rs is not reached and its graph assertions do not cover the new read-only actionable_nodes
@answer n=12 entity=screen-reader-support#3.1 v=yes mode=add-to
why: the actions read now also reads one snapshot per call and its element ids (F4), and the disabled refusal it names now sits in the shared check_target
@answer n=13 entity=screen-reader-support#3.2 v=no
why: the clickable reading from the walk's listeners is unchanged and the entity names no other reader
@answer n=14 entity=screen-reader-support#3.3 v=yes mode=add-to
why: actionable_nodes is one more read-only reader of the UI graph, returning ActionableNode { id, owner, keyed } (F1)
@answer n=15 entity=screen-reader-support#3.4 v=no
why: NODE_FIELDS stays fourteen and no snapshot field moved
@answer n=16 entity=screen-reader-support#3.5 v=no
why: the open-list label leave is unchanged and stand_act_popup.rs is not reached
@answer n=17 entity=screen-reader-support#3.6 v=no
why: the native checked reading lives in blitz-dom and accessibility_states.rs, neither reached by the session
@answer n=18 entity=screen-reader-support#4.1 v=yes mode=contradict
why: the listener leg reads the exact current listeners and no longer the data-dioxus-id mark, so the over-include claim is false, actionable.rs citations moved, and the shared Readings rule, actionable_nodes and the keyed reading join it (F2, F3, F7)
@answer n=19 entity=screen-reader-support#4.2 v=yes mode=add-to
why: beside the per-task CLICKABLE table, ACTIONABLE_AT_BOOT counts counter 2, flight-booker 7, timer 3, crud 11, cells 678, load 2, and the four lean tasks hold 3 disabled controls at boot (F5)
@answer n=20 entity=screen-reader-support#4.3 v=no
why: dioxus_runtime_probe.rs is not reached and its counted links are unchanged
@answer n=21 entity=screen-reader-support#4.4 v=no
why: the clickable values on counter and Cells come from the snapshot, which the session did not change
@answer n=22 entity=screen-reader-support#4.5 v=yes mode=add-to
why: the readers of the walk's exact listener reading now include unkeyed_actionable and actionable_nodes beside clickable, and a listener removed on a re-render is read by neither (F8)
@answer n=23 entity=screen-reader-support#4.6 v=yes mode=add-to
why: on Cells 461 of 678 cells read off-screen for click, type and screenshot, all 678 take scroll, and the unpaged actions answer is 106,088 bytes (F9)
@answer n=24 entity=screen-reader-support#4.7 v=yes mode=add-to
why: the load page's empty reading of the check is now measured on the exact listener leg, so the claim that the check reads empty on all six tasks must carry that measurement
@answer n=25 entity=screen-reader-support#5.1 v=yes mode=add-to
why: the statement that the actionable-key check still returns nothing is now measured on the exact listener leg
