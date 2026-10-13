# Counts — 2026-10-13-upstream-sync-polish-and-ship

Both readings are `bash .github/scripts/ci-leg.sh test` (`cargo test --workspace --locked`), one merged stream (`target/ci-logs/test.log`). Each `Running …` / `Doc-tests …` line is paired with the next `test result:` line.

- **Pre-merge** — chunk start `a703136878e47c0cf583448bbac02f06f8365be5`, read 2026-10-13 (step 1), leg exit 0: **174 result lines · 849 passed · 0 failed · 12 ignored**. This equals the last recorded figures (test-plan §9 at 2026-10-12-headless-screenshot).
- **Post-merge** — the merged, uncommitted tree (`MERGE_HEAD` `434e06615d5f47a3df92c0f94e916df088fdcf4d`), read 2026-10-13 inside `bash .github/scripts/ci-leg.sh fast` (gate entry 25, exit 0; tally entry 26): **179 result lines · 893 passed · 0 failed · 12 ignored**.

## Every difference, attributed

The rows are joined by target name, and every row not listed here reads the same before and after.

| target | before | after | cause |
|---|---|---|---|
| `tests/anonymous_block_restyle.rs` | — | 10 passed | upstream tests added — new file (#1174; edited by #1175) |
| `tests/button_alignment.rs` | — | 16 passed | upstream tests added — new file (#1175) |
| `tests/checked_attribute.rs` | — | 2 passed | upstream tests added — new file (#1187) |
| `tests/pointer_leave.rs` | — | 7 passed | upstream tests added — new file (#1176; edited by #1177, #1189) |
| `tests/ua_stylesheet_order.rs` | — | 3 passed | upstream tests added — new file (#1189) |
| `unittests src/lib.rs [dioxus_native_dom]` | 61 passed | 67 passed | upstream tests added — `mutation_writer.rs`: `falsy_boolean_attributes_are_removed`, `boolean_attributes_follow_their_value_after_mount`; `dioxus_document.rs`: `removed_nodes_are_dropped`, `suspended_nodes_are_dropped_and_recreated`, `todo_like_stress`, `mounted_data_of_removed_element_reports_error` |

- Result lines: +5, the five new blitz-tests targets (predicted 174 → 179).
- Passed: +44, which is 38 in the five new files and 6 in dioxus-native-dom's unit tests (predicted 849 → 893; basis `git diff 7832c177 434e0661 | grep -c '^+\s*#\[test\]'` = 44).
- Ignored: unchanged at 12. Failed: 0 on both sides.
- `a_removed_element_reads_no_id` (step 6) is in both counts: ok before on the detached lifetime, and ok after on the dropped one.

## Other tallies read on the merged tree
- The CI-scripts leg reads `Ran 79 tests` · `OK`, against 78 at the chunk start. The +1 is upstream's added test in `.github/scripts/test_wpt_area_changes.py` (#1183).
- The a11y leg reads 3 result lines · 15 passed · 0 failed, unchanged.
- `bash scripts/agent-run.sh run stand` reads `run.end` passed 139 · failed 0 · ignored 7, unchanged.
- `bash scripts/agent-run.sh run all` reads `run.end` passed 487 · failed 0 · ignored 12. That is 449 → 487 (+38), upstream's five new files.
- `cargo test --workspace --locked --no-run` lists 128 executables (123 at the chunk start, +5) and `all-target 0`.
- `tests/blitz-tests/tests/*.rs` holds 118 files (113 at the chunk start). The by-name walk reads 0 absent, and each of the five new files ran exactly once as a target of its own.

## Per-target table (pre · post)

| target | pre passed | pre failed | pre ignored | post passed | post failed | post ignored |
|---|---|---|---|---|---|---|
| `unittests src/lib.rs [accesskit_xplat]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [blitz]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [blitz_dom]` | 89 | 0 | 0 | 89 | 0 | 0 |
| `tests/stylo_usage.rs` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [blitz_html]` | 1 | 0 | 0 | 1 | 0 | 0 |
| `unittests src/lib.rs [blitz_net]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [blitz_paint]` | 8 | 0 | 0 | 8 | 0 | 0 |
| `unittests src/lib.rs [blitz_shell]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [blitz_test_harness]` | 17 | 0 | 0 | 17 | 0 | 0 |
| `unittests lib.rs [blitz_tests]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `tests/accessibility_hidden.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/accessibility_names.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/accessibility_roles.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/accessibility_states.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/animations.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/anonymous_block_cache_invalidation.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/anonymous_block_leak.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/anonymous_block_percentage_height.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/autofocus_attribute.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/background_size.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/br_trailing_line.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/comment_layout.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/custom_widget_layout.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/detached_attribute.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/details_element.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/device_coalescing.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/dioxus_falsy_boolean_attrs.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/dioxus_falsy_disabled.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/dir_attribute.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/display_contents.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/flex_grid_order.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/focusability_updates.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/fragment_navigation.rs` | 19 | 0 | 0 | 19 | 0 | 0 |
| `tests/harness_smoke.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/hit_clipped_at_scrolling_box.rs` | 9 | 0 | 0 | 9 | 0 | 0 |
| `tests/hover_dom_ancestors.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/incremental_oracle.rs` | 7 | 0 | 0 | 7 | 0 | 0 |
| `tests/inline_bfc_padding.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/inline_box_baseline.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/inline_box_scrollable_overflow.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/inline_fragment_rects.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/inline_svg_restyle.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/inline_svg_serialize.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/inner_html_leak.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/interaction_state_canonicalization.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/interaction_state_teardown.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/lang_attribute.rs` | 7 | 0 | 0 | 7 | 0 | 0 |
| `tests/line_break.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/link_rel_attribute.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/oof_dynamic_cb.rs` | 11 | 0 | 0 | 11 | 0 | 0 |
| `tests/outset_box_shadow_shape.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/paint_order.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/paint_tree_bench.rs` | 0 | 0 | 3 | 0 | 0 | 3 |
| `tests/paint_tree_incremental.rs` | 12 | 0 | 0 | 12 | 0 | 0 |
| `tests/password_painted_masked.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/pointer_events.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/pre_overflow_scroll.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/pseudo_element_update.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/rem_after_viewport_change.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/render_blocking_stylesheet.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/resize_restyle.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/rotate_z_axis.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/scoped_query_selector.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/scroll_into_view_nested.rs` | 7 | 0 | 0 | 7 | 0 | 0 |
| `tests/scrollbar_drag.rs` | 7 | 0 | 0 | 7 | 0 | 0 |
| `tests/scrollbars.rs` | 12 | 0 | 0 | 12 | 0 | 0 |
| `tests/scrolled_box_absolute_position.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/scrolled_box_client_rect.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/stale_dirty_descendants.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/stale_interaction_state.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/stale_node_mapping.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/stand_accessibility_ids.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/stand_act_diff.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/stand_act_disabled.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/stand_act_filled.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/stand_act_ids.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/stand_act_keys.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/stand_act_obstructed.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/stand_act_range.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/stand_act_refused.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/stand_act_screenshot.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/stand_act_scroll.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/stand_act_spans.rs` | 2 | 0 | 2 | 2 | 0 | 2 |
| `tests/stand_act_states.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/stand_act_timer.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/stand_actionable_keys.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/stand_boot.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/stand_counter.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/stand_crud.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/stand_diff.rs` | 10 | 0 | 0 | 10 | 0 | 0 |
| `tests/stand_element_ids.rs` | 4 | 0 | 0 | 4 | 0 | 0 |
| `tests/stand_flight_booker.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/stand_id_edits.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/stand_id_persistence.rs` | 3 | 0 | 1 | 3 | 0 | 1 |
| `tests/stand_mcp_masked.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/stand_mcp_tools.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/stand_screenshot_repeat.rs` | 1 | 0 | 1 | 1 | 0 | 1 |
| `tests/stand_session_fresh.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/stand_session_ids.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/stand_session_lifecycle.rs` | 2 | 0 | 2 | 2 | 0 | 2 |
| `tests/stand_session_quiet.rs` | 1 | 0 | 1 | 1 | 0 | 1 |
| `tests/stand_session_state.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/stand_settle.rs` | 8 | 0 | 0 | 8 | 0 | 0 |
| `tests/stand_snapshot.rs` | 8 | 0 | 0 | 8 | 0 | 0 |
| `tests/stand_snapshot_state.rs` | 11 | 0 | 0 | 11 | 0 | 0 |
| `tests/stand_snapshot_text.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/stand_timer.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/style_property_invalidation.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/svg_attr_sizing.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `tests/svg_background_size.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/telemetry_drop.rs` | 1 | 0 | 1 | 1 | 0 | 1 |
| `tests/telemetry_init_idempotent.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/telemetry_panic_hook.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/telemetry_scrub.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/telemetry_stdout_silent.rs` | 1 | 0 | 1 | 1 | 0 | 1 |
| `tests/text_selection_anonymous_block.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/text_transform.rs` | 3 | 0 | 0 | 3 | 0 | 0 |
| `tests/touch_action.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/touch_events.rs` | 6 | 0 | 0 | 6 | 0 | 0 |
| `tests/transform_2d_subset.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/transform_viewport_scale.rs` | 8 | 0 | 0 | 8 | 0 | 0 |
| `tests/whitespace_modes.rs` | 8 | 0 | 0 | 8 | 0 | 0 |
| `unittests src/lib.rs [blitz_traits]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [blitz_vibey_script]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `tests/dom.rs` | 27 | 0 | 0 | 27 | 0 | 0 |
| `tests/preact.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/selection.rs` | 15 | 0 | 0 | 15 | 0 | 0 |
| `unittests src/main.rs [blitz]` | 28 | 0 | 0 | 28 | 0 | 0 |
| `unittests src/lib.rs [browser_persistence]` | 10 | 0 | 0 | 10 | 0 | 0 |
| `unittests src/main.rs [bump]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [counter]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/main.rs [counter]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [debug_timer]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [dioxus_native]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [dioxus_native_dom]` | 61 | 0 | 0 | 67 | 0 | 0 |
| `unittests src/lib.rs [escher_driver]` | 72 | 0 | 0 | 72 | 0 | 0 |
| `unittests src/lib.rs [escher_telemetry]` | 15 | 0 | 0 | 15 | 0 | 0 |
| `unittests src/main.rs [rdme]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [seven_guis]` | 1 | 0 | 0 | 1 | 0 | 0 |
| `unittests src/session_host.rs [escher_session]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/main.rs [seven_guis_native]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `tests/cli_commands.rs` | 11 | 0 | 0 | 11 | 0 | 0 |
| `tests/cli_flow.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/host_binary.rs` | 2 | 0 | 0 | 2 | 0 | 0 |
| `tests/host_log.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/host_timer.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/mcp_log.rs` | 1 | 0 | 0 | 1 | 0 | 0 |
| `tests/mcp_server.rs` | 5 | 0 | 0 | 5 | 0 | 0 |
| `unittests src/lib.rs [stylo_taffy]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [todomvc]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/main.rs [todomvc_native]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/main.rs [transparent]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/lib.rs [wasm_hello]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/main.rs [wgpu_texture]` | 0 | 0 | 0 | 0 | 0 | 0 |
| `unittests src/main.rs [wpt]` | 28 | 0 | 0 | 28 | 0 | 0 |
| `doc accesskit_xplat` | 1 | 0 | 0 | 1 | 0 | 0 |
| `doc blitz` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_dom` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_html` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_net` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_paint` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_shell` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_test_harness` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_tests` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_traits` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc blitz_vibey_script` | 1 | 0 | 0 | 1 | 0 | 0 |
| `doc browser_persistence` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc debug_timer` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc dioxus_native` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc dioxus_native_dom` | 1 | 0 | 0 | 1 | 0 | 0 |
| `doc escher_driver` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc escher_telemetry` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc seven_guis` | 0 | 0 | 0 | 0 | 0 | 0 |
| `doc stylo_taffy` | 0 | 0 | 0 | 0 | 0 | 0 |
| `tests/anonymous_block_restyle.rs` | — | — | — | 10 | 0 | 0 |
| `tests/button_alignment.rs` | — | — | — | 16 | 0 | 0 |
| `tests/checked_attribute.rs` | — | — | — | 2 | 0 | 0 |
| `tests/pointer_leave.rs` | — | — | — | 7 | 0 | 0 |
| `tests/ua_stylesheet_order.rs` | — | — | — | 3 | 0 | 0 |
