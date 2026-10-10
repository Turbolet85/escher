# Counts — the workspace test leg before and after the merge

## Pre-merge reading (step 1)

- Tree: `09f479b8b2e61d997a84f115ec8c8802c47769fb` (the chunk start commit), no merge staged.
- Read: 2026-10-10T01:19Z, `bash .github/scripts/ci-leg.sh test` → exit 0.
- Tally, by the count entry's `awk` over `target/ci-logs/test.log`: `lines 154 passed 657 failed 0 ignored 10` — the figures test-plan §9 last recorded, re-taken and not copied.
- One row per `Running …` / `Doc-tests …` line with the `test result:` line that follows it; the build hash of each binary is dropped.

## Pre-merge table

| # | target (binary · source) | passed | failed | ignored |
|---|---|---|---|---|
| 1 | accesskit_xplat · unittests src/lib.rs | 0 | 0 | 0 |
| 2 | blitz · unittests src/lib.rs | 0 | 0 | 0 |
| 3 | blitz_dom · unittests src/lib.rs | 64 | 0 | 0 |
| 4 | stylo_usage · tests/stylo_usage.rs | 0 | 0 | 0 |
| 5 | blitz_html · unittests src/lib.rs | 1 | 0 | 0 |
| 6 | blitz_net · unittests src/lib.rs | 0 | 0 | 0 |
| 7 | blitz_paint · unittests src/lib.rs | 8 | 0 | 0 |
| 8 | blitz_shell · unittests src/lib.rs | 0 | 0 | 0 |
| 9 | blitz_test_harness · unittests src/lib.rs | 11 | 0 | 0 |
| 10 | blitz_tests · unittests lib.rs | 0 | 0 | 0 |
| 11 | accessibility_hidden · tests/accessibility_hidden.rs | 6 | 0 | 0 |
| 12 | accessibility_names · tests/accessibility_names.rs | 5 | 0 | 0 |
| 13 | accessibility_roles · tests/accessibility_roles.rs | 6 | 0 | 0 |
| 14 | animations · tests/animations.rs | 1 | 0 | 0 |
| 15 | anonymous_block_cache_invalidation · tests/anonymous_block_cache_invalidation.rs | 3 | 0 | 0 |
| 16 | anonymous_block_leak · tests/anonymous_block_leak.rs | 2 | 0 | 0 |
| 17 | background_size · tests/background_size.rs | 2 | 0 | 0 |
| 18 | br_trailing_line · tests/br_trailing_line.rs | 3 | 0 | 0 |
| 19 | comment_layout · tests/comment_layout.rs | 4 | 0 | 0 |
| 20 | custom_widget_layout · tests/custom_widget_layout.rs | 6 | 0 | 0 |
| 21 | detached_attribute · tests/detached_attribute.rs | 1 | 0 | 0 |
| 22 | details_element · tests/details_element.rs | 5 | 0 | 0 |
| 23 | device_coalescing · tests/device_coalescing.rs | 4 | 0 | 0 |
| 24 | dioxus_falsy_boolean_attrs · tests/dioxus_falsy_boolean_attrs.rs | 3 | 0 | 0 |
| 25 | dioxus_falsy_disabled · tests/dioxus_falsy_disabled.rs | 1 | 0 | 0 |
| 26 | dir_attribute · tests/dir_attribute.rs | 5 | 0 | 0 |
| 27 | display_contents · tests/display_contents.rs | 4 | 0 | 0 |
| 28 | flex_grid_order · tests/flex_grid_order.rs | 6 | 0 | 0 |
| 29 | focusability_updates · tests/focusability_updates.rs | 3 | 0 | 0 |
| 30 | fragment_navigation · tests/fragment_navigation.rs | 19 | 0 | 0 |
| 31 | harness_smoke · tests/harness_smoke.rs | 5 | 0 | 0 |
| 32 | hover_dom_ancestors · tests/hover_dom_ancestors.rs | 2 | 0 | 0 |
| 33 | incremental_oracle · tests/incremental_oracle.rs | 7 | 0 | 0 |
| 34 | inline_bfc_padding · tests/inline_bfc_padding.rs | 1 | 0 | 0 |
| 35 | inline_box_baseline · tests/inline_box_baseline.rs | 4 | 0 | 0 |
| 36 | inline_box_scrollable_overflow · tests/inline_box_scrollable_overflow.rs | 1 | 0 | 0 |
| 37 | inline_fragment_rects · tests/inline_fragment_rects.rs | 4 | 0 | 0 |
| 38 | inline_svg_restyle · tests/inline_svg_restyle.rs | 2 | 0 | 0 |
| 39 | inline_svg_serialize · tests/inline_svg_serialize.rs | 4 | 0 | 0 |
| 40 | inner_html_leak · tests/inner_html_leak.rs | 1 | 0 | 0 |
| 41 | interaction_state_canonicalization · tests/interaction_state_canonicalization.rs | 5 | 0 | 0 |
| 42 | interaction_state_teardown · tests/interaction_state_teardown.rs | 4 | 0 | 0 |
| 43 | lang_attribute · tests/lang_attribute.rs | 7 | 0 | 0 |
| 44 | line_break · tests/line_break.rs | 3 | 0 | 0 |
| 45 | link_rel_attribute · tests/link_rel_attribute.rs | 2 | 0 | 0 |
| 46 | oof_dynamic_cb · tests/oof_dynamic_cb.rs | 11 | 0 | 0 |
| 47 | outset_box_shadow_shape · tests/outset_box_shadow_shape.rs | 3 | 0 | 0 |
| 48 | paint_order · tests/paint_order.rs | 4 | 0 | 0 |
| 49 | paint_tree_bench · tests/paint_tree_bench.rs | 0 | 0 | 3 |
| 50 | paint_tree_incremental · tests/paint_tree_incremental.rs | 12 | 0 | 0 |
| 51 | pointer_events · tests/pointer_events.rs | 5 | 0 | 0 |
| 52 | pre_overflow_scroll · tests/pre_overflow_scroll.rs | 1 | 0 | 0 |
| 53 | pseudo_element_update · tests/pseudo_element_update.rs | 1 | 0 | 0 |
| 54 | rem_after_viewport_change · tests/rem_after_viewport_change.rs | 2 | 0 | 0 |
| 55 | render_blocking_stylesheet · tests/render_blocking_stylesheet.rs | 1 | 0 | 0 |
| 56 | resize_restyle · tests/resize_restyle.rs | 3 | 0 | 0 |
| 57 | rotate_z_axis · tests/rotate_z_axis.rs | 3 | 0 | 0 |
| 58 | scoped_query_selector · tests/scoped_query_selector.rs | 5 | 0 | 0 |
| 59 | scroll_into_view_nested · tests/scroll_into_view_nested.rs | 7 | 0 | 0 |
| 60 | scrollbar_drag · tests/scrollbar_drag.rs | 7 | 0 | 0 |
| 61 | scrollbars · tests/scrollbars.rs | 12 | 0 | 0 |
| 62 | stale_dirty_descendants · tests/stale_dirty_descendants.rs | 1 | 0 | 0 |
| 63 | stale_interaction_state · tests/stale_interaction_state.rs | 5 | 0 | 0 |
| 64 | stale_node_mapping · tests/stale_node_mapping.rs | 4 | 0 | 0 |
| 65 | stand_accessibility_ids · tests/stand_accessibility_ids.rs | 5 | 0 | 0 |
| 66 | stand_act_diff · tests/stand_act_diff.rs | 6 | 0 | 0 |
| 67 | stand_act_disabled · tests/stand_act_disabled.rs | 2 | 0 | 0 |
| 68 | stand_act_ids · tests/stand_act_ids.rs | 2 | 0 | 0 |
| 69 | stand_act_keys · tests/stand_act_keys.rs | 2 | 0 | 0 |
| 70 | stand_act_obstructed · tests/stand_act_obstructed.rs | 3 | 0 | 0 |
| 71 | stand_act_range · tests/stand_act_range.rs | 1 | 0 | 0 |
| 72 | stand_act_refused · tests/stand_act_refused.rs | 3 | 0 | 0 |
| 73 | stand_act_scroll · tests/stand_act_scroll.rs | 4 | 0 | 0 |
| 74 | stand_act_spans · tests/stand_act_spans.rs | 2 | 0 | 2 |
| 75 | stand_act_timer · tests/stand_act_timer.rs | 4 | 0 | 0 |
| 76 | stand_actionable_keys · tests/stand_actionable_keys.rs | 3 | 0 | 0 |
| 77 | stand_boot · tests/stand_boot.rs | 6 | 0 | 0 |
| 78 | stand_counter · tests/stand_counter.rs | 1 | 0 | 0 |
| 79 | stand_crud · tests/stand_crud.rs | 2 | 0 | 0 |
| 80 | stand_diff · tests/stand_diff.rs | 9 | 0 | 0 |
| 81 | stand_element_ids · tests/stand_element_ids.rs | 4 | 0 | 0 |
| 82 | stand_flight_booker · tests/stand_flight_booker.rs | 1 | 0 | 0 |
| 83 | stand_id_edits · tests/stand_id_edits.rs | 5 | 0 | 0 |
| 84 | stand_id_persistence · tests/stand_id_persistence.rs | 3 | 0 | 1 |
| 85 | stand_session_fresh · tests/stand_session_fresh.rs | 2 | 0 | 0 |
| 86 | stand_session_ids · tests/stand_session_ids.rs | 2 | 0 | 0 |
| 87 | stand_session_lifecycle · tests/stand_session_lifecycle.rs | 1 | 0 | 1 |
| 88 | stand_session_quiet · tests/stand_session_quiet.rs | 1 | 0 | 1 |
| 89 | stand_session_state · tests/stand_session_state.rs | 3 | 0 | 0 |
| 90 | stand_settle · tests/stand_settle.rs | 8 | 0 | 0 |
| 91 | stand_snapshot · tests/stand_snapshot.rs | 8 | 0 | 0 |
| 92 | stand_snapshot_state · tests/stand_snapshot_state.rs | 8 | 0 | 0 |
| 93 | stand_snapshot_text · tests/stand_snapshot_text.rs | 6 | 0 | 0 |
| 94 | stand_timer · tests/stand_timer.rs | 3 | 0 | 0 |
| 95 | style_property_invalidation · tests/style_property_invalidation.rs | 3 | 0 | 0 |
| 96 | svg_attr_sizing · tests/svg_attr_sizing.rs | 5 | 0 | 0 |
| 97 | svg_background_size · tests/svg_background_size.rs | 3 | 0 | 0 |
| 98 | telemetry_drop · tests/telemetry_drop.rs | 1 | 0 | 1 |
| 99 | telemetry_init_idempotent · tests/telemetry_init_idempotent.rs | 1 | 0 | 0 |
| 100 | telemetry_panic_hook · tests/telemetry_panic_hook.rs | 1 | 0 | 0 |
| 101 | telemetry_scrub · tests/telemetry_scrub.rs | 1 | 0 | 0 |
| 102 | telemetry_stdout_silent · tests/telemetry_stdout_silent.rs | 1 | 0 | 1 |
| 103 | text_selection_anonymous_block · tests/text_selection_anonymous_block.rs | 2 | 0 | 0 |
| 104 | touch_action · tests/touch_action.rs | 6 | 0 | 0 |
| 105 | touch_events · tests/touch_events.rs | 6 | 0 | 0 |
| 106 | transform_2d_subset · tests/transform_2d_subset.rs | 2 | 0 | 0 |
| 107 | transform_viewport_scale · tests/transform_viewport_scale.rs | 8 | 0 | 0 |
| 108 | whitespace_modes · tests/whitespace_modes.rs | 8 | 0 | 0 |
| 109 | blitz_traits · unittests src/lib.rs | 0 | 0 | 0 |
| 110 | blitz_vibey_script · unittests src/lib.rs | 0 | 0 | 0 |
| 111 | dom · tests/dom.rs | 26 | 0 | 0 |
| 112 | preact · tests/preact.rs | 2 | 0 | 0 |
| 113 | blitz · unittests src/main.rs | 28 | 0 | 0 |
| 114 | browser_persistence · unittests src/lib.rs | 10 | 0 | 0 |
| 115 | bump · unittests src/main.rs | 0 | 0 | 0 |
| 116 | counter · unittests src/lib.rs | 0 | 0 | 0 |
| 117 | counter · unittests src/main.rs | 0 | 0 | 0 |
| 118 | debug_timer · unittests src/lib.rs | 0 | 0 | 0 |
| 119 | dioxus_native · unittests src/lib.rs | 0 | 0 | 0 |
| 120 | dioxus_native_dom · unittests src/lib.rs | 55 | 0 | 0 |
| 121 | escher_driver · unittests src/lib.rs | 29 | 0 | 0 |
| 122 | escher_telemetry · unittests src/lib.rs | 15 | 0 | 0 |
| 123 | rdme · unittests src/main.rs | 0 | 0 | 0 |
| 124 | seven_guis · unittests src/lib.rs | 1 | 0 | 0 |
| 125 | escher_session · unittests src/session_host.rs | 0 | 0 | 0 |
| 126 | seven_guis_native · unittests src/main.rs | 0 | 0 | 0 |
| 127 | host_binary · tests/host_binary.rs | 2 | 0 | 0 |
| 128 | host_log · tests/host_log.rs | 1 | 0 | 0 |
| 129 | stylo_taffy · unittests src/lib.rs | 0 | 0 | 0 |
| 130 | todomvc · unittests src/lib.rs | 0 | 0 | 0 |
| 131 | todomvc_native · unittests src/main.rs | 0 | 0 | 0 |
| 132 | transparent · unittests src/main.rs | 0 | 0 | 0 |
| 133 | wasm_hello · unittests src/lib.rs | 0 | 0 | 0 |
| 134 | wgpu_texture · unittests src/main.rs | 0 | 0 | 0 |
| 135 | wpt · unittests src/main.rs | 15 | 0 | 0 |
| 136 | accesskit_xplat · doc-tests | 1 | 0 | 0 |
| 137 | blitz · doc-tests | 0 | 0 | 0 |
| 138 | blitz_dom · doc-tests | 0 | 0 | 0 |
| 139 | blitz_html · doc-tests | 0 | 0 | 0 |
| 140 | blitz_net · doc-tests | 0 | 0 | 0 |
| 141 | blitz_paint · doc-tests | 0 | 0 | 0 |
| 142 | blitz_shell · doc-tests | 0 | 0 | 0 |
| 143 | blitz_test_harness · doc-tests | 0 | 0 | 0 |
| 144 | blitz_tests · doc-tests | 0 | 0 | 0 |
| 145 | blitz_traits · doc-tests | 0 | 0 | 0 |
| 146 | blitz_vibey_script · doc-tests | 1 | 0 | 0 |
| 147 | browser_persistence · doc-tests | 0 | 0 | 0 |
| 148 | debug_timer · doc-tests | 0 | 0 | 0 |
| 149 | dioxus_native · doc-tests | 0 | 0 | 0 |
| 150 | dioxus_native_dom · doc-tests | 1 | 0 | 0 |
| 151 | escher_driver · doc-tests | 0 | 0 | 0 |
| 152 | escher_telemetry · doc-tests | 0 | 0 | 0 |
| 153 | seven_guis · doc-tests | 0 | 0 | 0 |
| 154 | stylo_taffy · doc-tests | 0 | 0 | 0 |

Totals: lines 154 · passed 657 · failed 0 · ignored 10 · unattributed 0

## Post-merge reading (the gate block's `fast` entry)

- Tree: the chunk start with `7832c177ff272128154bac58afe56c2b9164b417` merged and resolved, uncommitted (`MERGE_HEAD` = the pin).
- Read: 2026-10-10T01:25Z, `bash .github/scripts/ci-leg.sh fast` → exit 0; the table is its `test` leg's log.
- Tally, by the same `awk`: `lines 158 passed 719 failed 0 ignored 10` — the forecast (158 lines, 719 passed, 10 ignored), measured.

## Every difference, attributed

Result lines 154 → 158 (+4), passed 657 → 719 (+62), failed 0 → 0, ignored 10 → 10. Eight targets differ; the other 150 rows read the same counts before and after. Each added test is a `#[test]` line upstream's delta adds (`git diff 23354585 7832c177`, 63 added, 0 removed), by file:

| target | pre | post | Δ passed | upstream file · commit |
|---|---|---|---|---|
| `anonymous_block_percentage_height` (new target) | absent | 1 | +1 | `tests/blitz-tests/tests/anonymous_block_percentage_height.rs` · `7a26ccc4` (#1149) |
| `autofocus_attribute` (new target) | absent | 5 | +5 | `tests/blitz-tests/tests/autofocus_attribute.rs` · `6369e8bd` (#1085) |
| `text_transform` (new target) | absent | 3 | +3 | `tests/blitz-tests/tests/text_transform.rs` · `cb7b08dc` (#1093) |
| `selection` (new target, blitz-vibey-script) | absent | 15 | +15 | `packages/blitz-vibey-script/tests/selection.rs` · `5399689d` (#1094) |
| `blitz_dom` unit tests | 64 | 87 | +23 | `packages/blitz-dom/src/layout/text_transform.rs` 22 · `1e444d2d` (#947), `cb7b08dc` (#1093), `031372cc` (#1095); `packages/blitz-dom/src/debug.rs` 1 · #1099 / #1101 |
| `dom` (blitz-vibey-script) | 26 | 27 | +1 | `packages/blitz-vibey-script/tests/dom.rs` · `2408516b` (#1147) |
| `inline_fragment_rects` | 4 | 5 | +1 | `tests/blitz-tests/tests/inline_fragment_rects.rs` · `ec8bae96` (#1150) |
| `wpt` unit tests | 15 | 28 | +13 | `wpt/runner/src/test_variants.rs` 10, `test_runners/ref_test.rs` 2, `main.rs` 1 · `70528177` (#1117) |

Sum: 1 + 5 + 3 + 15 + 23 + 1 + 1 + 13 = 62. The 63rd added `#[test]` is `all_test_files_are_included` in `tests/blitz-tests/tests/all.rs` (#1123), which no `cargo test` builds here (`test = false`): the log holds no `Running tests/all.rs` line.

No test of ours changed its count, and no target of ours is absent: every row of the pre-merge table but the eight above appears in the post-merge table with the same passed / failed / ignored. Class of every moved number: `upstream tests added` (see `moved-readings.md`).

## Post-merge table

| # | target (binary · source) | passed | failed | ignored |
|---|---|---|---|---|
| 1 | accesskit_xplat · unittests src/lib.rs | 0 | 0 | 0 |
| 2 | blitz · unittests src/lib.rs | 0 | 0 | 0 |
| 3 | blitz_dom · unittests src/lib.rs | 87 | 0 | 0 |
| 4 | stylo_usage · tests/stylo_usage.rs | 0 | 0 | 0 |
| 5 | blitz_html · unittests src/lib.rs | 1 | 0 | 0 |
| 6 | blitz_net · unittests src/lib.rs | 0 | 0 | 0 |
| 7 | blitz_paint · unittests src/lib.rs | 8 | 0 | 0 |
| 8 | blitz_shell · unittests src/lib.rs | 0 | 0 | 0 |
| 9 | blitz_test_harness · unittests src/lib.rs | 11 | 0 | 0 |
| 10 | blitz_tests · unittests lib.rs | 0 | 0 | 0 |
| 11 | accessibility_hidden · tests/accessibility_hidden.rs | 6 | 0 | 0 |
| 12 | accessibility_names · tests/accessibility_names.rs | 5 | 0 | 0 |
| 13 | accessibility_roles · tests/accessibility_roles.rs | 6 | 0 | 0 |
| 14 | animations · tests/animations.rs | 1 | 0 | 0 |
| 15 | anonymous_block_cache_invalidation · tests/anonymous_block_cache_invalidation.rs | 3 | 0 | 0 |
| 16 | anonymous_block_leak · tests/anonymous_block_leak.rs | 2 | 0 | 0 |
| 17 | anonymous_block_percentage_height · tests/anonymous_block_percentage_height.rs | 1 | 0 | 0 |
| 18 | autofocus_attribute · tests/autofocus_attribute.rs | 5 | 0 | 0 |
| 19 | background_size · tests/background_size.rs | 2 | 0 | 0 |
| 20 | br_trailing_line · tests/br_trailing_line.rs | 3 | 0 | 0 |
| 21 | comment_layout · tests/comment_layout.rs | 4 | 0 | 0 |
| 22 | custom_widget_layout · tests/custom_widget_layout.rs | 6 | 0 | 0 |
| 23 | detached_attribute · tests/detached_attribute.rs | 1 | 0 | 0 |
| 24 | details_element · tests/details_element.rs | 5 | 0 | 0 |
| 25 | device_coalescing · tests/device_coalescing.rs | 4 | 0 | 0 |
| 26 | dioxus_falsy_boolean_attrs · tests/dioxus_falsy_boolean_attrs.rs | 3 | 0 | 0 |
| 27 | dioxus_falsy_disabled · tests/dioxus_falsy_disabled.rs | 1 | 0 | 0 |
| 28 | dir_attribute · tests/dir_attribute.rs | 5 | 0 | 0 |
| 29 | display_contents · tests/display_contents.rs | 4 | 0 | 0 |
| 30 | flex_grid_order · tests/flex_grid_order.rs | 6 | 0 | 0 |
| 31 | focusability_updates · tests/focusability_updates.rs | 3 | 0 | 0 |
| 32 | fragment_navigation · tests/fragment_navigation.rs | 19 | 0 | 0 |
| 33 | harness_smoke · tests/harness_smoke.rs | 5 | 0 | 0 |
| 34 | hover_dom_ancestors · tests/hover_dom_ancestors.rs | 2 | 0 | 0 |
| 35 | incremental_oracle · tests/incremental_oracle.rs | 7 | 0 | 0 |
| 36 | inline_bfc_padding · tests/inline_bfc_padding.rs | 1 | 0 | 0 |
| 37 | inline_box_baseline · tests/inline_box_baseline.rs | 4 | 0 | 0 |
| 38 | inline_box_scrollable_overflow · tests/inline_box_scrollable_overflow.rs | 1 | 0 | 0 |
| 39 | inline_fragment_rects · tests/inline_fragment_rects.rs | 5 | 0 | 0 |
| 40 | inline_svg_restyle · tests/inline_svg_restyle.rs | 2 | 0 | 0 |
| 41 | inline_svg_serialize · tests/inline_svg_serialize.rs | 4 | 0 | 0 |
| 42 | inner_html_leak · tests/inner_html_leak.rs | 1 | 0 | 0 |
| 43 | interaction_state_canonicalization · tests/interaction_state_canonicalization.rs | 5 | 0 | 0 |
| 44 | interaction_state_teardown · tests/interaction_state_teardown.rs | 4 | 0 | 0 |
| 45 | lang_attribute · tests/lang_attribute.rs | 7 | 0 | 0 |
| 46 | line_break · tests/line_break.rs | 3 | 0 | 0 |
| 47 | link_rel_attribute · tests/link_rel_attribute.rs | 2 | 0 | 0 |
| 48 | oof_dynamic_cb · tests/oof_dynamic_cb.rs | 11 | 0 | 0 |
| 49 | outset_box_shadow_shape · tests/outset_box_shadow_shape.rs | 3 | 0 | 0 |
| 50 | paint_order · tests/paint_order.rs | 4 | 0 | 0 |
| 51 | paint_tree_bench · tests/paint_tree_bench.rs | 0 | 0 | 3 |
| 52 | paint_tree_incremental · tests/paint_tree_incremental.rs | 12 | 0 | 0 |
| 53 | pointer_events · tests/pointer_events.rs | 5 | 0 | 0 |
| 54 | pre_overflow_scroll · tests/pre_overflow_scroll.rs | 1 | 0 | 0 |
| 55 | pseudo_element_update · tests/pseudo_element_update.rs | 1 | 0 | 0 |
| 56 | rem_after_viewport_change · tests/rem_after_viewport_change.rs | 2 | 0 | 0 |
| 57 | render_blocking_stylesheet · tests/render_blocking_stylesheet.rs | 1 | 0 | 0 |
| 58 | resize_restyle · tests/resize_restyle.rs | 3 | 0 | 0 |
| 59 | rotate_z_axis · tests/rotate_z_axis.rs | 3 | 0 | 0 |
| 60 | scoped_query_selector · tests/scoped_query_selector.rs | 5 | 0 | 0 |
| 61 | scroll_into_view_nested · tests/scroll_into_view_nested.rs | 7 | 0 | 0 |
| 62 | scrollbar_drag · tests/scrollbar_drag.rs | 7 | 0 | 0 |
| 63 | scrollbars · tests/scrollbars.rs | 12 | 0 | 0 |
| 64 | stale_dirty_descendants · tests/stale_dirty_descendants.rs | 1 | 0 | 0 |
| 65 | stale_interaction_state · tests/stale_interaction_state.rs | 5 | 0 | 0 |
| 66 | stale_node_mapping · tests/stale_node_mapping.rs | 4 | 0 | 0 |
| 67 | stand_accessibility_ids · tests/stand_accessibility_ids.rs | 5 | 0 | 0 |
| 68 | stand_act_diff · tests/stand_act_diff.rs | 6 | 0 | 0 |
| 69 | stand_act_disabled · tests/stand_act_disabled.rs | 2 | 0 | 0 |
| 70 | stand_act_ids · tests/stand_act_ids.rs | 2 | 0 | 0 |
| 71 | stand_act_keys · tests/stand_act_keys.rs | 2 | 0 | 0 |
| 72 | stand_act_obstructed · tests/stand_act_obstructed.rs | 3 | 0 | 0 |
| 73 | stand_act_range · tests/stand_act_range.rs | 1 | 0 | 0 |
| 74 | stand_act_refused · tests/stand_act_refused.rs | 3 | 0 | 0 |
| 75 | stand_act_scroll · tests/stand_act_scroll.rs | 4 | 0 | 0 |
| 76 | stand_act_spans · tests/stand_act_spans.rs | 2 | 0 | 2 |
| 77 | stand_act_timer · tests/stand_act_timer.rs | 4 | 0 | 0 |
| 78 | stand_actionable_keys · tests/stand_actionable_keys.rs | 3 | 0 | 0 |
| 79 | stand_boot · tests/stand_boot.rs | 6 | 0 | 0 |
| 80 | stand_counter · tests/stand_counter.rs | 1 | 0 | 0 |
| 81 | stand_crud · tests/stand_crud.rs | 2 | 0 | 0 |
| 82 | stand_diff · tests/stand_diff.rs | 9 | 0 | 0 |
| 83 | stand_element_ids · tests/stand_element_ids.rs | 4 | 0 | 0 |
| 84 | stand_flight_booker · tests/stand_flight_booker.rs | 1 | 0 | 0 |
| 85 | stand_id_edits · tests/stand_id_edits.rs | 5 | 0 | 0 |
| 86 | stand_id_persistence · tests/stand_id_persistence.rs | 3 | 0 | 1 |
| 87 | stand_session_fresh · tests/stand_session_fresh.rs | 2 | 0 | 0 |
| 88 | stand_session_ids · tests/stand_session_ids.rs | 2 | 0 | 0 |
| 89 | stand_session_lifecycle · tests/stand_session_lifecycle.rs | 1 | 0 | 1 |
| 90 | stand_session_quiet · tests/stand_session_quiet.rs | 1 | 0 | 1 |
| 91 | stand_session_state · tests/stand_session_state.rs | 3 | 0 | 0 |
| 92 | stand_settle · tests/stand_settle.rs | 8 | 0 | 0 |
| 93 | stand_snapshot · tests/stand_snapshot.rs | 8 | 0 | 0 |
| 94 | stand_snapshot_state · tests/stand_snapshot_state.rs | 8 | 0 | 0 |
| 95 | stand_snapshot_text · tests/stand_snapshot_text.rs | 6 | 0 | 0 |
| 96 | stand_timer · tests/stand_timer.rs | 3 | 0 | 0 |
| 97 | style_property_invalidation · tests/style_property_invalidation.rs | 3 | 0 | 0 |
| 98 | svg_attr_sizing · tests/svg_attr_sizing.rs | 5 | 0 | 0 |
| 99 | svg_background_size · tests/svg_background_size.rs | 3 | 0 | 0 |
| 100 | telemetry_drop · tests/telemetry_drop.rs | 1 | 0 | 1 |
| 101 | telemetry_init_idempotent · tests/telemetry_init_idempotent.rs | 1 | 0 | 0 |
| 102 | telemetry_panic_hook · tests/telemetry_panic_hook.rs | 1 | 0 | 0 |
| 103 | telemetry_scrub · tests/telemetry_scrub.rs | 1 | 0 | 0 |
| 104 | telemetry_stdout_silent · tests/telemetry_stdout_silent.rs | 1 | 0 | 1 |
| 105 | text_selection_anonymous_block · tests/text_selection_anonymous_block.rs | 2 | 0 | 0 |
| 106 | text_transform · tests/text_transform.rs | 3 | 0 | 0 |
| 107 | touch_action · tests/touch_action.rs | 6 | 0 | 0 |
| 108 | touch_events · tests/touch_events.rs | 6 | 0 | 0 |
| 109 | transform_2d_subset · tests/transform_2d_subset.rs | 2 | 0 | 0 |
| 110 | transform_viewport_scale · tests/transform_viewport_scale.rs | 8 | 0 | 0 |
| 111 | whitespace_modes · tests/whitespace_modes.rs | 8 | 0 | 0 |
| 112 | blitz_traits · unittests src/lib.rs | 0 | 0 | 0 |
| 113 | blitz_vibey_script · unittests src/lib.rs | 0 | 0 | 0 |
| 114 | dom · tests/dom.rs | 27 | 0 | 0 |
| 115 | preact · tests/preact.rs | 2 | 0 | 0 |
| 116 | selection · tests/selection.rs | 15 | 0 | 0 |
| 117 | blitz · unittests src/main.rs | 28 | 0 | 0 |
| 118 | browser_persistence · unittests src/lib.rs | 10 | 0 | 0 |
| 119 | bump · unittests src/main.rs | 0 | 0 | 0 |
| 120 | counter · unittests src/lib.rs | 0 | 0 | 0 |
| 121 | counter · unittests src/main.rs | 0 | 0 | 0 |
| 122 | debug_timer · unittests src/lib.rs | 0 | 0 | 0 |
| 123 | dioxus_native · unittests src/lib.rs | 0 | 0 | 0 |
| 124 | dioxus_native_dom · unittests src/lib.rs | 55 | 0 | 0 |
| 125 | escher_driver · unittests src/lib.rs | 29 | 0 | 0 |
| 126 | escher_telemetry · unittests src/lib.rs | 15 | 0 | 0 |
| 127 | rdme · unittests src/main.rs | 0 | 0 | 0 |
| 128 | seven_guis · unittests src/lib.rs | 1 | 0 | 0 |
| 129 | escher_session · unittests src/session_host.rs | 0 | 0 | 0 |
| 130 | seven_guis_native · unittests src/main.rs | 0 | 0 | 0 |
| 131 | host_binary · tests/host_binary.rs | 2 | 0 | 0 |
| 132 | host_log · tests/host_log.rs | 1 | 0 | 0 |
| 133 | stylo_taffy · unittests src/lib.rs | 0 | 0 | 0 |
| 134 | todomvc · unittests src/lib.rs | 0 | 0 | 0 |
| 135 | todomvc_native · unittests src/main.rs | 0 | 0 | 0 |
| 136 | transparent · unittests src/main.rs | 0 | 0 | 0 |
| 137 | wasm_hello · unittests src/lib.rs | 0 | 0 | 0 |
| 138 | wgpu_texture · unittests src/main.rs | 0 | 0 | 0 |
| 139 | wpt · unittests src/main.rs | 28 | 0 | 0 |
| 140 | accesskit_xplat · doc-tests | 1 | 0 | 0 |
| 141 | blitz · doc-tests | 0 | 0 | 0 |
| 142 | blitz_dom · doc-tests | 0 | 0 | 0 |
| 143 | blitz_html · doc-tests | 0 | 0 | 0 |
| 144 | blitz_net · doc-tests | 0 | 0 | 0 |
| 145 | blitz_paint · doc-tests | 0 | 0 | 0 |
| 146 | blitz_shell · doc-tests | 0 | 0 | 0 |
| 147 | blitz_test_harness · doc-tests | 0 | 0 | 0 |
| 148 | blitz_tests · doc-tests | 0 | 0 | 0 |
| 149 | blitz_traits · doc-tests | 0 | 0 | 0 |
| 150 | blitz_vibey_script · doc-tests | 1 | 0 | 0 |
| 151 | browser_persistence · doc-tests | 0 | 0 | 0 |
| 152 | debug_timer · doc-tests | 0 | 0 | 0 |
| 153 | dioxus_native · doc-tests | 0 | 0 | 0 |
| 154 | dioxus_native_dom · doc-tests | 1 | 0 | 0 |
| 155 | escher_driver · doc-tests | 0 | 0 | 0 |
| 156 | escher_telemetry · doc-tests | 0 | 0 | 0 |
| 157 | seven_guis · doc-tests | 0 | 0 | 0 |
| 158 | stylo_taffy · doc-tests | 0 | 0 | 0 |

Totals: lines 158 · passed 719 · failed 0 · ignored 10 · unattributed 0
