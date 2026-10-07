# Mutation controls — 2026-10-07-refusal-detection (plan step 11)

Run 2026-10-07 by a scratch script: each mutation applied alone to the one file it names, then
`cargo test -p blitz-tests --locked --no-fail-fast` over seven targets — `stand_act_disabled`,
`stand_act_obstructed`, `stand_act_scroll`, `stand_act_refused`, `stand_act_ids`,
`scroll_into_view_nested`, `fragment_navigation` — then the file restored from the bytes read
before the mutation. Every mutated tree compiled. After the ninth, the SHA-256 of both mutated
files (`packages/escher-driver/src/execute.rs`, `packages/blitz-dom/src/scrolling.rs`) equals
the one taken before the first, and the same seven targets read green: 19 · 7 · 2 · 3 · 4 · 3 ·
2 passed, 0 failed (`fragment_navigation` · `scroll_into_view_nested` · `stand_act_disabled` ·
`stand_act_obstructed` · `stand_act_scroll` · `stand_act_refused` · `stand_act_ids`).

A first run of the same script is not recorded: its reader took the test lines and the target
lines from two separate streams and attributed no failure to a target. The run recorded here
reads one merged stream.

| # | mutation (file) | named check | red — failed tests of the named check | other targets red | `fragment_navigation` |
|---|---|---|---|---|---|
| 1 | the `disabled` reading removed (`execute.rs`) | `stand_act_disabled` | 2 of 2: `a_disabled_date_input_refuses_typing_until_the_app_enables_it`, `a_disabled_delete_refuses_a_click_until_a_row_is_selected` | `stand_act_ids` 2 of 2 · `stand_act_obstructed` 1 of 3 (`a_target_holding_two_causes_reads_the_earlier_one`) | 19 passed |
| 2 | the `covered` reading removed (`execute.rs`) | `stand_act_obstructed` | 1 of 3: `a_covered_target_is_refused_until_its_cover_is_gone_or_transparent` | `stand_act_ids` 1 of 2 (`the_snapshot_the_accessibility_tree_and_the_driver_read_one_id` — the header's spacer) | 19 passed |
| 3 | the `off-screen` reading removed (`execute.rs`) | `stand_act_obstructed` | 2 of 3: `a_target_out_of_view_is_refused_off_screen`, `a_target_holding_two_causes_reads_the_earlier_one` | `stand_act_scroll` 3 of 4 | 19 passed |
| 4 | the record never consulted — an unread id and a read one both answer `not-found` (`execute.rs`) | `stand_act_refused` | 2 of 3: `a_refused_call_names_its_cause_and_leaves_the_instance_unchanged`, `one_id_is_not_found_before_a_screen_read_it_and_stale_after` | none | 19 passed |
| 5 | the page point taken without the viewport's scroll (`execute.rs`) | `stand_act_scroll` | 1 of 4: `a_far_button_is_scrolled_into_a_viewport_that_scrolls_and_then_clicked` | none | 19 passed |
| 6 | `disabled` moved after `covered` in the order (`execute.rs`) | `stand_act_obstructed` | 1 of 3: `a_target_holding_two_causes_reads_the_earlier_one` | none | 19 passed |
| 7 | the engine's walk of the nested boxes removed, so only the viewport scrolls (`scrolling.rs`) | `scroll_into_view_nested` and the CRUD scenario of `stand_act_scroll` | `scroll_into_view_nested` 5 of 7: `a_box_in_view_is_scrolled_and_the_viewport_is_not`, `a_box_below_the_viewport_and_the_viewport_both_move`, `two_nested_boxes_both_move`, `a_hidden_box_is_scrolled_and_a_visible_one_is_not`, `a_smooth_scroll_writes_the_nested_box_at_once_and_animates_the_viewport` · `stand_act_scroll` 1 of 4: `a_row_past_the_list_is_refused_scrolled_into_view_and_then_clicked` | none | 19 passed |
| 8 | the `off-screen` reading made against the viewport alone (`execute.rs`) | the CRUD scenario of `stand_act_scroll` and the nested row of `stand_act_obstructed` | `stand_act_scroll` 1 of 4: `a_row_past_the_list_is_refused_scrolled_into_view_and_then_clicked` · `stand_act_obstructed` 1 of 3: `a_target_out_of_view_is_refused_off_screen` | none | 19 passed |
| 9 | `in_view` answered true always (`execute.rs`) | the unreachable fixture of `stand_act_scroll` | 1 of 4: `a_target_no_scroll_can_reach_is_told_in_the_result` | none | 19 passed |

Read off the table:

- Each of the nine mutations turns its named check red, and each is green again on the
  restored tree.
- `fragment_navigation` — the standing witness of a viewport-only scroll, unedited — holds 19
  passed under every mutation, the walk-removed one included: removing the nested walk returns
  the engine to its behaviour before this chunk, which that file's pages do not tell apart.
- Under mutation 7 the two tests of `scroll_into_view_nested` that stay green are the two
  whose page holds no nested box to scroll: a target already in view, and a target above the
  document's origin.
- Mutation 1 also reds `stand_act_ids` whole and mutation 2 one of its tests: its restated
  table pins the three disabled controls and the header's spacer by cause.
