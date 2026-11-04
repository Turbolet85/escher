@answer n=1 entity=test-scope-summary#20.1 v=no
why: the session touches only dioxus_runtime_probe.rs, Cargo.lock and deny.toml, and this blitz-test-harness settle claim rests on packages/blitz-test-harness/src/settle.rs which the session does not reach
@answer n=2 entity=test-scope-summary#21.1 v=no
why: the session changes no escher-telemetry file and the lockfile bump names no telemetry dependency, so the 15 telemetry unit tests stand
@answer n=3 entity=test-scope-summary#21.2 v=no
why: the session changes no escher-driver file and the pulldown-cmark bump reaches only rdme, so the driver's test counts and files stand
@answer n=4 entity=test-scope-summary#22.1 v=no
why: the session changes no blitz-vibey-script test and the lockfile bump does not reach the script's DOM, Selection, Preact or URLSearchParams tests
@answer n=5 entity=test-scope-summary#23.1 v=no
why: nothing under packages/ changed, so the dioxus-native-dom unit test counts and file list stand
@answer n=6 entity=test-scope-summary#23.2 v=no
why: nothing under packages/ changed, so the IME-on-mount test in dioxus_document.rs stands as stated
@answer n=7 entity=test-scope-summary#23.3 v=yes mode=add-to
why: the test task_polled_during_render_can_access_document still has no fork counterpart, and the session names it as the port chunk's witness that inverts dioxus_runtime_probe's hazard pin
@answer n=8 entity=test-scope-summary#23.4 v=yes mode=add-to
why: the left-out 286874cc1 guard in flush_queued_mounted_events still reads true, and the planned port keeps flush_queued_mounted_events unchanged with its two callers changing shape (F6)
@answer n=9 entity=test-scope-summary#24.1 v=no
why: the search over the slice's 21 files is unchanged, since the probe is a blitz-tests file and adds no test under dioxus-native or stylo_taffy
@answer n=10 entity=test-scope-summary#25.1 v=yes mode=add-to
why: the tests/blitz-tests coverage entry gains the dioxus_runtime_probe target reading the vdom walk, listeners, signal subscribers and the MountedData hazard (F1)
@answer n=11 entity=test-scope-summary#26.1 v=no
why: the visible_region_transformed target and packages/blitz-dom/src/scrolling.rs are not reached by the session
@answer n=12 entity=test-scope-summary#27.1 v=no
why: the checkable_change_event target is not reached by the session
@answer n=13 entity=test-scope-summary#28.1 v=no
why: the text_range_no_change_event target is not reached by the session
@answer n=14 entity=test-scope-summary#29.1 v=no
why: the stand_act_transformed zoomed-world check is not reached by the session, and the stand's test counts stay at 237 over 51 files
@answer n=15 entity=test-scope-summary#30.1 v=no
why: the render_blocking_stylesheet target is not reached by the session
@answer n=16 entity=test-scope-summary#31.1 v=no
why: the stand_act_popup file and its run stand and stand-repeat selection are unchanged, the new target being named in neither leg
@answer n=17 entity=test-scope-summary#32.1 v=no
why: the stand_lint select cases and the stand_act_popup lint read are unchanged by the session
@answer n=18 entity=test-scope-summary#33.1 v=no
why: the accessibility_states native-control check is unchanged and the lockfile bump does not reach the accesskit crates it names
@answer n=19 entity=test-scope-summary#34.1 v=no
why: the inert_subtree focus checks are not reached by the session
@answer n=20 entity=test-scope-summary#35.1 v=no
why: the select_popup target and its run selection (workspace test, run all, the stand-repeat named test, not run stand or a11y) are unchanged, the new target joining only run all and the workspace leg
@answer n=21 entity=test-scope-summary#36.1 v=no
why: the select_popup_painted target and the ci-leg.sh lines it names are unchanged by the session
@answer n=22 entity=test-scope-summary#37.1 v=no
why: the stand_keyboard Cells and load-page checks are not reached by the session
@answer n=23 entity=test-scope-summary#38.1 v=no
why: the stand_contrast load-page and Cells checks are not reached by the session
@answer n=24 entity=test-scope-summary#39.1 v=no
why: stand_requirement_sweep.rs and its 0.2.0 helpers are unchanged, and the new target is not a witness of any 0.2.0 requirement
@answer n=25 entity=test-scope-summary#40.1 v=no
why: .github/scripts/ci-leg.sh and the stand-repeat and a11y leg lists are unchanged, and the new target is named in neither leg
