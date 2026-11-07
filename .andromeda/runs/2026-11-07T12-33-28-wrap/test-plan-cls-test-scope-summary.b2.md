@answer n=1 entity=test-scope-summary#20.1 v=no
why: blitz-test-harness's settle, screenshot and contrast unit tests are outside the four files the session changed
@answer n=2 entity=test-scope-summary#21.1 v=no
why: the harness's CountedHandler and settle_waiting are outside the four files the session changed
@answer n=3 entity=test-scope-summary#22.1 v=no
why: escher-telemetry is outside the four files the session changed
@answer n=4 entity=test-scope-summary#22.2 v=no
why: escher-driver is outside the four files the session changed
@answer n=5 entity=test-scope-summary#23.1 v=no
why: blitz-vibey-script is outside the four files the session changed
@answer n=6 entity=test-scope-summary#24.1 v=yes mode=stale
why: the session adds task_polled_during_render_can_access_document to dioxus_document.rs's test module (lines 846-885), so the dioxus-native-dom unit count moves from 82 to 83 and the test-module citation 528-845 no longer covers the whole list
@answer n=7 entity=test-scope-summary#24.2 v=yes mode=stale
why: the session adds lines to dioxus_document.rs above 755 (at 7, 90-91, 155, 166-168, 171, 302-304 and 307), so the IME test's citation 755-789 has shifted
@answer n=8 entity=test-scope-summary#24.3 v=yes mode=contradict
why: the session ports task_polled_during_render_can_access_document into dioxus_document.rs's test module (846-885), so the claim that it has no counterpart in that module no longer holds
@answer n=9 entity=test-scope-summary#24.4 v=yes mode=stale
why: the session adds lines above 242 in dioxus_document.rs (at 7, 90-91, 155, 166-168 and 171), so the flush_queued_mounted_events guard citation 242-246 has moved; the left-out reading itself still holds on 0.7
@answer n=10 entity=test-scope-summary#25.1 v=no
why: the new test lands in dioxus_document.rs, a file already listed as a match, and the new mutation_buffer module holds no test, so the no-test claim for dioxus-native and stylo_taffy still holds
@answer n=11 entity=test-scope-summary#26.1 v=yes mode=add-to
why: F1 joins the Dioxus integration coverage: the probe's spawned pins act on nodes mounted by an earlier render, and the limit is recorded as an open residual row (target next)
@answer n=12 entity=test-scope-summary#27.1 v=no
why: visible_region_transformed and the blitz-dom scrolling code it covers are outside the session's change set
@answer n=13 entity=test-scope-summary#28.1 v=no
why: checkable_change_event is outside the session's change set
@answer n=14 entity=test-scope-summary#29.1 v=no
why: text_range_no_change_event is outside the session's change set
@answer n=15 entity=test-scope-summary#30.1 v=no
why: stand_act_transformed's zoomed-world check is outside the session's change set
@answer n=16 entity=test-scope-summary#31.1 v=no
why: render_blocking_stylesheet is outside the session's change set
@answer n=17 entity=test-scope-summary#32.1 v=no
why: stand_act_popup is outside the session's change set
@answer n=18 entity=test-scope-summary#33.1 v=no
why: stand_lint's select cases are outside the session's change set
@answer n=19 entity=test-scope-summary#34.1 v=no
why: accessibility_states' native-control check is outside the session's change set
@answer n=20 entity=test-scope-summary#35.1 v=no
why: inert_subtree's focus checks are outside the session's change set
@answer n=21 entity=test-scope-summary#36.1 v=no
why: select_popup is outside the session's change set
@answer n=22 entity=test-scope-summary#37.1 v=no
why: select_popup_painted is outside the session's change set
@answer n=23 entity=test-scope-summary#38.1 v=no
why: stand_keyboard's Cells and load-page checks are outside the session's change set
@answer n=24 entity=test-scope-summary#39.1 v=no
why: stand_contrast's load-page and Cells checks are outside the session's change set
@answer n=25 entity=test-scope-summary#40.1 v=no
why: the 0.2.0 sweep test's helpers in stand_requirement_sweep.rs are outside the session's change set
