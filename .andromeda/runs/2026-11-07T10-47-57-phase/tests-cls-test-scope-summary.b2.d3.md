@answer n=1 entity=test-scope-summary#20.1 v=no
why: inventory of blitz-test-harness inline unit tests over a scripted document; the chunk changes no harness code and the poll answer it keeps is witnessed by the stand checks
@answer n=2 entity=test-scope-summary#21.1 v=no
why: a failed load ending the harness wait on the counted handler; no Dioxus render or poll path is reached
@answer n=3 entity=test-scope-summary#22.1 v=no
why: escher-telemetry is neither edited nor reached by the Dioxus render path
@answer n=4 entity=test-scope-summary#22.2 v=no
why: escher-driver is neither edited nor reached; its unit tests build no virtual DOM and its stand checks bind separately
@answer n=5 entity=test-scope-summary#23.1 v=no
why: blitz-vibey-script tests cover ScriptDocument, which is not a Dioxus document
@answer n=6 entity=test-scope-summary#24.1 v=yes
why: the dioxus_document.rs test module, the bridge_tests child module and mutation_writer::tests read the render path the chunk edits and must stay green
@answer n=7 entity=test-scope-summary#24.2 v=yes
why: input_focused_on_mount_enables_ime sits in dioxus_document.rs's test module and pins the IME call at editor construction on the initial_build path the chunk changes
@answer n=8 entity=test-scope-summary#24.3 v=yes
why: this is the absent upstream test task_polled_during_render_can_access_document that the chunk ports into dioxus_document.rs mod tests as its witness
@answer n=9 entity=test-scope-summary#24.4 v=no
why: records a gap the chunk leaves open by design; the guard sits in flush_queued_mounted_events, a unit the change keeps as before and does not edit
@answer n=10 entity=test-scope-summary#25.1 v=no
why: records absent tests in dioxus-native and stylo_taffy, neither edited nor asserted by the chunk
@answer n=11 entity=test-scope-summary#26.1 v=yes
why: its stand_* and blitz-tests checks (ids across re-render, snapshots, diffs, stale_node_mapping, incremental_oracle, falsy boolean attributes, stand_session_*) are the chunk's re-asserted witnesses of unchanged behaviour
@answer n=12 entity=test-scope-summary#27.1 v=no
why: visible_region_transformed is an engine check of BaseDocument geometry the change neither edits nor names in its scope
@answer n=13 entity=test-scope-summary#28.1 v=no
why: checkable_change_event is an engine check on parsed pages, not a Dioxus document path the chunk reaches
@answer n=14 entity=test-scope-summary#29.1 v=no
why: text_range_no_change_event is an engine check on parsed pages, not a Dioxus document path the chunk reaches
@answer n=15 entity=test-scope-summary#30.1 v=yes
why: stand_act_transformed is a stand_* check, which the chunk re-asserts as witness of unchanged stand behaviour
@answer n=16 entity=test-scope-summary#31.1 v=no
why: upstream render_blocking_stylesheet tests on parsed documents exercise engine loading, which the chunk does not reach
@answer n=17 entity=test-scope-summary#32.1 v=yes
why: stand_act_popup is a stand_* check driving the select list through the held seven_guis session that the buffered poll serves
@answer n=18 entity=test-scope-summary#33.1 v=yes
why: stand_lint is a stand_* check whose lint readings at boot and on the open list are witnesses of unchanged stand behaviour
@answer n=19 entity=test-scope-summary#34.1 v=no
why: accessibility_states on a native control is an engine check on parsed pages, not a stand check and not on the Dioxus path
@answer n=20 entity=test-scope-summary#35.1 v=no
why: inert_subtree focus over engine pages is not a stand check and the chunk does not reach it
@answer n=21 entity=test-scope-summary#36.1 v=no
why: select_popup is an engine check on parsed pages, not a stand check and not reached through the Dioxus bridge
@answer n=22 entity=test-scope-summary#37.1 v=no
why: select_popup_painted reads frames on the paint path, which the chunk does not change
@answer n=23 entity=test-scope-summary#38.1 v=yes
why: stand_keyboard is a stand_* check, re-asserted by the chunk as witness of unchanged stand behaviour
@answer n=24 entity=test-scope-summary#39.1 v=yes
why: stand_contrast is a stand_* check whose computed-style readings on the load page and Cells the chunk re-asserts
@answer n=25 entity=test-scope-summary#40.1 v=no
why: states the sweep test's own helper structure and messages, which the chunk neither changes nor is required to satisfy
