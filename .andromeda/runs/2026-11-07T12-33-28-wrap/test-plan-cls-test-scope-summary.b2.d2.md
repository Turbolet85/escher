@answer n=1 entity=test-scope-summary#20.1 v=no
why: blitz-test-harness is in no file the session reached and its helpers keep their signatures, so its counts and readings still hold.
@answer n=2 entity=test-scope-summary#21.1 v=no
why: the failed-load wait in blitz-test-harness settle.rs lies outside every file the session reached.
@answer n=3 entity=test-scope-summary#22.1 v=no
why: escher-telemetry lies outside every file the session reached.
@answer n=4 entity=test-scope-summary#22.2 v=no
why: escher-driver lies outside every file the session reached, and no manifest, lock or leg moved.
@answer n=5 entity=test-scope-summary#23.1 v=no
why: blitz-vibey-script's tests lie outside every file the session reached.
@answer n=6 entity=test-scope-summary#24.1 v=yes mode=stale
why: the dioxus-native-dom unit count moves from 82 to 83 in the same nine files, with the ported task_polled_during_render_can_access_document added to dioxus_document.rs's test module at 846-885, and the citations at 458-494 and 528-845 shift.
@answer n=7 entity=test-scope-summary#24.2 v=yes mode=stale
why: the IME test's citation dioxus_document.rs:755-789 has moved, since lines were added above it at 7, 90-91, 155, 166-168, 171, 302-304 and 307.
@answer n=8 entity=test-scope-summary#24.3 v=yes mode=contradict
why: the upstream test is now ported into dioxus_document.rs at 846-885 and passes, and 286874cc1's buffered render is ported onto the Dioxus 0.7 bridge, so the fork no longer lacks a counterpart.
@answer n=9 entity=test-scope-summary#24.4 v=yes mode=contradict
why: the element_removed test stays out on Dioxus 0.7 for 0.7's scheduling and returns with the Dioxus 0.8 move under res-6ssktw6x, not with 286874cc1, and the guard citation at dioxus_document.rs:242-246 has moved.
@answer n=10 entity=test-scope-summary#25.1 v=no
why: dioxus_document.rs already matched the test search and the new mutation_buffer.rs holds no test, so the matched files and stylo_taffy's none still hold.
@answer n=11 entity=test-scope-summary#26.1 v=yes mode=add-to
why: the gap owned by residual res-llkgqygv now carries its re-carry at 0.3.0 with target next, as the wrap addendum re-says.
@answer n=12 entity=test-scope-summary#27.1 v=no
why: visible_region_transformed is an engine test in blitz-tests and blitz-dom, outside every file the session reached.
@answer n=13 entity=test-scope-summary#28.1 v=no
why: checkable_change_event is an engine test outside every file the session reached.
@answer n=14 entity=test-scope-summary#29.1 v=no
why: text_range_no_change_event is an engine test outside every file the session reached.
@answer n=15 entity=test-scope-summary#30.1 v=no
why: the zoomed-world stand_act_transformed test lies outside every file the session reached.
@answer n=16 entity=test-scope-summary#31.1 v=no
why: render_blocking_stylesheet is an engine test outside every file the session reached.
@answer n=17 entity=test-scope-summary#32.1 v=no
why: stand_act_popup lies outside every file the session reached, and the stand leg is unchanged.
@answer n=18 entity=test-scope-summary#33.1 v=no
why: the stand_lint select and open-list readings lie outside every file the session reached.
@answer n=19 entity=test-scope-summary#34.1 v=no
why: the accessibility_states native-control test lies outside every file the session reached.
@answer n=20 entity=test-scope-summary#35.1 v=no
why: the inert_subtree focus test lies outside every file the session reached.
@answer n=21 entity=test-scope-summary#36.1 v=no
why: select_popup is an engine target outside every file the session reached.
@answer n=22 entity=test-scope-summary#37.1 v=no
why: select_popup_painted is an engine target outside every file the session reached.
@answer n=23 entity=test-scope-summary#38.1 v=no
why: the stand_keyboard Cells and load-page tests lie outside every file the session reached.
@answer n=24 entity=test-scope-summary#39.1 v=no
why: the stand_contrast load-page and Cells tests lie outside every file the session reached.
@answer n=25 entity=test-scope-summary#40.1 v=no
why: the sweep test's helpers live in stand_requirement_sweep.rs, which the session did not reach.
