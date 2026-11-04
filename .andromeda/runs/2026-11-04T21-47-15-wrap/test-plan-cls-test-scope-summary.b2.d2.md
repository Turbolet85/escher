@answer n=1 entity=test-scope-summary#20.1 v=no
why: no harness or settle code moved and nothing under packages/ changed, so the failed-load claim still holds on the reached files
@answer n=2 entity=test-scope-summary#21.1 v=no
why: escher-telemetry is untouched and its 15-test count does not depend on Cargo.lock, deny.toml or the new target
@answer n=3 entity=test-scope-summary#21.2 v=no
why: escher-driver is untouched and its unit and count claims stand on the unchanged crate
@answer n=4 entity=test-scope-summary#22.1 v=no
why: blitz-vibey-script is untouched and nothing under packages/ moved
@answer n=5 entity=test-scope-summary#23.1 v=yes mode=add-to
why: task item 6 — the actionable-key check DioxusDocument::unkeyed_actionable over-includes an element whose listener was removed while its data-dioxus-id mark stays, conservative and read from the code, not run
@answer n=6 entity=test-scope-summary#23.2 v=no
why: the IME-on-mount test in dioxus_document.rs is unchanged and nothing under packages/ moved
@answer n=7 entity=test-scope-summary#23.3 v=yes mode=add-to
why: task item 4 — upstream task_polled_during_render_can_access_document, still absent from the fork, is now named the port chunk's witness, the test that inverts dioxus_runtime_probe's hazard pin
@answer n=8 entity=test-scope-summary#23.4 v=yes mode=add-to
why: the planned port of upstream 286874cc1 (F6) keeps flush_queued_mounted_events unchanged with its two callers reshaped, and this left-out test returns with that commit
@answer n=9 entity=test-scope-summary#24.1 v=no
why: stylo_taffy and dioxus-native hold no test and the session adds none under packages/
@answer n=10 entity=test-scope-summary#25.1 v=yes mode=add-to
why: the tests/blitz-tests coverage entry gains the dioxus_runtime_probe target, a Dioxus runtime probe reading the vdom walk, listeners, signal subscribers and the MountedData hazard (F1, task item 5)
@answer n=11 entity=test-scope-summary#26.1 v=no
why: visible_region_transformed is unchanged and nothing under packages/ moved
@answer n=12 entity=test-scope-summary#27.1 v=no
why: checkable_change_event is unchanged and the session touches no checkable-control code
@answer n=13 entity=test-scope-summary#28.1 v=no
why: text_range_no_change_event is unchanged and nothing under packages/ moved
@answer n=14 entity=test-scope-summary#29.1 v=no
why: stand_act_transformed and the zoomed-world check are unchanged and no driver code moved
@answer n=15 entity=test-scope-summary#30.1 v=no
why: render_blocking_stylesheet is unchanged and nothing under packages/ moved
@answer n=16 entity=test-scope-summary#31.1 v=no
why: stand_act_popup is unchanged and no driver or stand check moved
@answer n=17 entity=test-scope-summary#32.1 v=no
why: the stand_lint select cases and the open-list reading are unchanged and no lint code moved
@answer n=18 entity=test-scope-summary#33.1 v=no
why: accessibility_states on a native control is unchanged and nothing under packages/ moved
@answer n=19 entity=test-scope-summary#34.1 v=no
why: inert_subtree's focus checks are unchanged and nothing under packages/ moved
@answer n=20 entity=test-scope-summary#35.1 v=no
why: select_popup is unchanged and no select-list code or leg list moved
@answer n=21 entity=test-scope-summary#36.1 v=no
why: select_popup_painted is unchanged and no painting code or leg list moved
@answer n=22 entity=test-scope-summary#37.1 v=no
why: stand_keyboard on Cells and the load page is unchanged and no keyboard code moved
@answer n=23 entity=test-scope-summary#38.1 v=no
why: stand_contrast on the load page and Cells is unchanged and no styling or contrast code moved
@answer n=24 entity=test-scope-summary#39.1 v=no
why: the 0.2.0 sweep test's helpers are unchanged and stand_requirement_sweep.rs is not in the change
@answer n=25 entity=test-scope-summary#40.1 v=no
why: ci-leg.sh and the stand-repeat leg list are unchanged, the new target is named in neither leg, and the session changes no leg list
