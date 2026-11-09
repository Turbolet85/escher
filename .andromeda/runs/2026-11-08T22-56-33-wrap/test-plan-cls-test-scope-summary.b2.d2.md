@answer n=1 entity=test-scope-summary#20.1 v=no
why: blitz-paint's inline tests and the password-caret check sit in files the session did not reach
@answer n=2 entity=test-scope-summary#21.1 v=no
why: the blending tests live in blitz-paint's layers.rs, which the session did not reach
@answer n=3 entity=test-scope-summary#22.1 v=no
why: blitz-test-harness is not among the files the session reached, so its counts stand
@answer n=4 entity=test-scope-summary#23.1 v=no
why: the failed-load wait lives in blitz-test-harness settle.rs, which the session did not reach
@answer n=5 entity=test-scope-summary#24.1 v=no
why: escher-telemetry is not among the files the session reached
@answer n=6 entity=test-scope-summary#24.2 v=yes mode=stale
why: escher-driver's command-line table moves from 102 to 109 rows, cli_tests.rs from 5 to 6 tests and json.rs from 7 to 8 tests
@answer n=7 entity=test-scope-summary#25.1 v=no
why: blitz-vibey-script is not among the files the session reached
@answer n=8 entity=test-scope-summary#26.1 v=yes mode=stale
why: the crate's unit-test count moves from 104 to 108 and its bridge tests from 8 to 9, the new reached.rs adding four
@answer n=9 entity=test-scope-summary#26.2 v=yes mode=stale
why: its IME test is cited at dioxus_document.rs lines 786-820, a reached file whose lines the change can move, and the report does not show the claim still holds
@answer n=10 entity=test-scope-summary#26.3 v=yes mode=stale
why: its test is cited at dioxus_document.rs lines 877-916, a reached file whose lines the change can move
@answer n=11 entity=test-scope-summary#26.4 v=yes mode=stale
why: its guard is cited at dioxus_document.rs lines 256-260, a reached file the change can move
@answer n=12 entity=test-scope-summary#27.1 v=yes mode=stale
why: the list of dioxus-native-dom files holding tests gains reached.rs with its reached::tests, a file the absence search would now match
@answer n=13 entity=test-scope-summary#28.1 v=yes mode=stale
why: stand_replay's count moves from 3 to 4 with the new kept-actions test, and the shared common module gains tables
@answer n=14 entity=test-scope-summary#29.1 v=no
why: its CLICKABLE table lives in stand_non_lean_tasks.rs, which the session did not reach, and no clickable reading moves
@answer n=15 entity=test-scope-summary#30.1 v=no
why: visible_region_transformed and blitz-dom are not among the files the session reached
@answer n=16 entity=test-scope-summary#31.1 v=no
why: checkable_change_event is an engine check in a file the session did not reach
@answer n=17 entity=test-scope-summary#32.1 v=no
why: text_range_no_change_event is an engine check in a file the session did not reach
@answer n=18 entity=test-scope-summary#33.1 v=no
why: stand_act_transformed.rs is not among the files the session reached and its zoomed case reads unchanged
@answer n=19 entity=test-scope-summary#34.1 v=no
why: render_blocking_stylesheet and its failed-stylesheet cases are in a file the session did not reach
@answer n=20 entity=test-scope-summary#35.1 v=no
why: stand_act_popup.rs is not among the files the session reached, and its leg placement by prefix stays as written
@answer n=21 entity=test-scope-summary#36.1 v=no
why: stand_lint's select cases and the blitz-paint overlap reading are in files the session did not reach
@answer n=22 entity=test-scope-summary#37.1 v=no
why: the native-control check in accessibility_states.rs is in a file the session did not reach
@answer n=23 entity=test-scope-summary#38.1 v=no
why: inert_subtree.rs is not among the files the session reached
@answer n=24 entity=test-scope-summary#39.1 v=no
why: select_popup.rs is not among the files the session reached, and its named-target leg placement stays as written
@answer n=25 entity=test-scope-summary#40.1 v=no
why: select_popup_painted.rs and blitz-paint's select_list.rs are not among the files the session reached
