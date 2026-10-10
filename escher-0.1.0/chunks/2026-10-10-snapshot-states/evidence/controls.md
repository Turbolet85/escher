# Controls — 2026-10-10-snapshot-states (plan step 10)

One-shot readings taken by /andromeda-implement on 2026-10-10, before the gates. Each is a reading of a tree that
no longer stands, so no listed entry re-produces it; what they establish is asserted by the two new check files and
the two extended ones. Commands were run from the repository root, stderr merged into stdout, the exit read from the
bare command.

## 1. Red before the code — the checks on the tree as it stood

| When | Command | Exit | Reading |
|---|---|---|---|
| The engine as built: base `f0cb9dd7` plus the new file of step 1 alone | `cargo test -p blitz-tests --locked --no-fail-fast --test accessibility_states` | 101 | 1 passed, 3 failed. `aria_selected_reads_true_and_false_and_nothing_else`, `aria_pressed_reads_its_three_values_and_nothing_else` and `aria_invalid_reads_its_values_and_an_unrecognized_one_as_true` each failed on row 0 of the supporting element: the node read `(None, None, None)` where the table states `(Some(true), None, None)`, `(None, Some(True), None)` and `(None, None, Some(True))`. `aria_checked_sets_none_of_the_three` passed — it holds an absence, and its red is the control `builder-reads-aria-checked-as-toggled` below. |
| The same tree plus the tests of steps 2 and 3 | `cargo test -p blitz-tests --locked --no-fail-fast --test stand_snapshot_state --test stand_act_states` | 101 | Neither target compiles: `error[E0609]: no field` `selected` / `pressed` / `invalid` on type `NodeState` — 6 errors in `stand_snapshot_state`, 3 in `stand_act_states`. The model held four fields. |
| Steps 4 to 8 in place, the stand's two task files still as at the base | `cargo test -p blitz-tests --locked --no-fail-fast --test accessibility_states --test stand_snapshot_state --test stand_act_states --test stand_act_diff` | 101 | `accessibility_states` 4 passed. `stand_snapshot_state` 8 passed, 3 failed — exactly the three new tests (`exactly_the_active_flight_mode_reads_pressed`, `exactly_the_selected_row_reads_selected`, `a_date_that_is_no_date_reads_invalid`). `stand_act_states` 0 passed, 3 failed. `stand_act_diff` 6 passed on its old assertions. The stand stated none of the three. |
| Step 9's attributes in place, `stand_act_diff` not yet restated | the same command | 101 | `accessibility_states` 4 passed, `stand_snapshot_state` 11 passed, `stand_act_states` 3 passed. `stand_act_diff` 4 passed, 2 failed: `a_choice_on_the_flight_booker_returns_the_control_it_enabled` at its line 124 and `create_returns_the_new_row_added_and_delete_returns_it_removed` at its line 197 — the two stated id sets that lack the node whose state the step now changes. |
| The three sets restated | the same command | 0 | 4 + 6 + 3 + 11 passed, 0 failed. |

## 2. Mutation controls — the code in place, one named mutation at a time

Protocol (`.claude/rules/testing.md`): the file's bytes saved, one exact replacement applied (a site that does not
occur exactly once is refused), the named commands run, the saved bytes written back, and the sha256 of the file
compared with the sha256 of the saved bytes before the next control. One control was run alone and its per-target
table read before the batch. **22 controls · 22 restored · 0 that failed to compile.** After the batch the eight
files read the digests below again, and `git status --short` differed from the status saved before the controls by
the implement run dir alone.

Commands: **stand** = `cargo test -p blitz-tests --locked --no-fail-fast --test accessibility_states --test stand_snapshot_state --test stand_act_states --test stand_act_diff` (24 tests) ·
**dnd** = `cargo test -p dioxus-native-dom --locked --no-fail-fast --lib` (60 tests) ·
**drv** = `cargo test -p escher-driver --locked --no-fail-fast --lib` (50 tests).

| Control | File | sha256 (first 12) | Reading |
|---|---|---|---|
| `builder-selected-read-removed` | `packages/blitz-dom/src/accessibility.rs` | `5551373f548b` | stand: exit 101, 20 passed, 4 failed — accessibility_states.rs::aria_selected_reads_true_and_false_and_nothing_else, stand_act_diff.rs::create_returns_the_new_row_added_and_delete_returns_it_removed, stand_act_states.rs::a_row_click_returns_the_row_selected_and_the_row_it_leaves_not_selected, stand_snapshot_state.rs::exactly_the_selected_row_reads_selected<br>dnd: exit 101, 58 passed, 2 failed — snapshot::an_aria_selected_value_reads_selected, snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid |
| `builder-pressed-read-removed` | `packages/blitz-dom/src/accessibility.rs` | `5551373f548b` | stand: exit 101, 20 passed, 4 failed — accessibility_states.rs::aria_pressed_reads_its_three_values_and_nothing_else, stand_act_diff.rs::a_choice_on_the_flight_booker_returns_the_control_it_enabled, stand_act_states.rs::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading, stand_snapshot_state.rs::exactly_the_active_flight_mode_reads_pressed<br>dnd: exit 101, 58 passed, 2 failed — snapshot::an_aria_pressed_value_reads_pressed_and_a_checked_one_does_not, snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid |
| `builder-invalid-read-removed` | `packages/blitz-dom/src/accessibility.rs` | `5551373f548b` | stand: exit 101, 21 passed, 3 failed — accessibility_states.rs::aria_invalid_reads_its_values_and_an_unrecognized_one_as_true, stand_act_states.rs::a_typed_start_date_returns_the_field_invalid_or_not, stand_snapshot_state.rs::a_date_that_is_no_date_reads_invalid<br>dnd: exit 101, 58 passed, 2 failed — snapshot::an_aria_invalid_value_reads_invalid, snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid |
| `builder-invalid-unrecognized-sets-nothing` | `packages/blitz-dom/src/accessibility.rs` | `5551373f548b` | stand: exit 101, 23 passed, 1 failed — accessibility_states.rs::aria_invalid_reads_its_values_and_an_unrecognized_one_as_true<br>dnd: exit 101, 59 passed, 1 failed — snapshot::an_aria_invalid_value_reads_invalid |
| `builder-reads-aria-checked-as-toggled` | `packages/blitz-dom/src/accessibility.rs` | `5551373f548b` | stand: exit 101, 23 passed, 1 failed — accessibility_states.rs::aria_checked_sets_none_of_the_three<br>dnd: exit 101, 59 passed, 1 failed — snapshot::an_aria_pressed_value_reads_pressed_and_a_checked_one_does_not |
| `snapshot-selected-default` | `packages/dioxus-native-dom/src/snapshot.rs` | `6ea17a7e701d` | stand: exit 101, 21 passed, 3 failed — stand_act_diff.rs::create_returns_the_new_row_added_and_delete_returns_it_removed, stand_act_states.rs::a_row_click_returns_the_row_selected_and_the_row_it_leaves_not_selected, stand_snapshot_state.rs::exactly_the_selected_row_reads_selected<br>dnd: exit 101, 58 passed, 2 failed — snapshot::an_aria_selected_value_reads_selected, snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid |
| `snapshot-pressed-default` | `packages/dioxus-native-dom/src/snapshot.rs` | `6ea17a7e701d` | stand: exit 101, 20 passed, 4 failed — stand_act_diff.rs::a_choice_on_the_flight_booker_returns_the_control_it_enabled, stand_act_states.rs::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading, stand_snapshot_state.rs::a_date_that_is_no_date_reads_invalid, stand_snapshot_state.rs::exactly_the_active_flight_mode_reads_pressed<br>dnd: exit 101, 58 passed, 2 failed — snapshot::an_aria_pressed_value_reads_pressed_and_a_checked_one_does_not, snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid |
| `snapshot-invalid-default` | `packages/dioxus-native-dom/src/snapshot.rs` | `6ea17a7e701d` | stand: exit 101, 22 passed, 2 failed — stand_act_states.rs::a_typed_start_date_returns_the_field_invalid_or_not, stand_snapshot_state.rs::a_date_that_is_no_date_reads_invalid<br>dnd: exit 101, 58 passed, 2 failed — snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid, snapshot::an_aria_invalid_value_reads_invalid |
| `snapshot-mixed-reads-pressed` | `packages/dioxus-native-dom/src/snapshot.rs` | `6ea17a7e701d` | stand: exit 0, 24 passed, 0 failed — none<br>dnd: exit 101, 58 passed, 2 failed — snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid, snapshot::an_aria_pressed_value_reads_pressed_and_a_checked_one_does_not |
| `text-selected-token-removed` | `packages/dioxus-native-dom/src/snapshot_text.rs` | `4046f488248f` | dnd: exit 101, 58 passed, 2 failed — snapshot_text::state_tokens_follow_the_model, snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid |
| `text-pressed-token-removed` | `packages/dioxus-native-dom/src/snapshot_text.rs` | `4046f488248f` | dnd: exit 101, 58 passed, 2 failed — snapshot_text::state_tokens_follow_the_model, snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid |
| `text-invalid-token-removed` | `packages/dioxus-native-dom/src/snapshot_text.rs` | `4046f488248f` | dnd: exit 101, 58 passed, 2 failed — snapshot_text::state_tokens_follow_the_model, snapshot_text::a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid |
| `diff-compares-focus-only` | `packages/dioxus-native-dom/src/snapshot_diff.rs` | `97c3992a98a4` | dnd: exit 101, 59 passed, 1 failed — snapshot_diff::a_selected_pressed_or_invalid_reading_alone_names_that_node |
| `json-selected-key-removed` | `packages/escher-driver/src/json.rs` | `dd36fe6aa144` | drv: exit 101, 49 passed, 1 failed — json::a_node_is_written_with_the_schema_fields_it_has |
| `json-pressed-key-removed` | `packages/escher-driver/src/json.rs` | `dd36fe6aa144` | drv: exit 101, 49 passed, 1 failed — json::a_node_is_written_with_the_schema_fields_it_has |
| `json-invalid-key-removed` | `packages/escher-driver/src/json.rs` | `dd36fe6aa144` | drv: exit 101, 49 passed, 1 failed — json::a_node_is_written_with_the_schema_fields_it_has |
| `schema-node-fields-reordered` | `packages/escher-driver/src/schema.rs` | `3d138987805d` | drv: exit 101, 48 passed, 2 failed — json::a_node_is_written_with_the_schema_fields_it_has, schema::each_verb_has_its_stated_shapes_and_the_key_list_its_twelve_names |
| `schema-text-help-row-reworded` | `packages/escher-driver/src/schema.rs` | `3d138987805d` | drv: exit 101, 49 passed, 1 failed — schema::every_verb_states_its_help_its_arguments_and_a_result |
| `schema-changed-help-row-reworded` | `packages/escher-driver/src/schema.rs` | `3d138987805d` | drv: exit 101, 49 passed, 1 failed — schema::every_verb_states_its_help_its_arguments_and_a_result |
| `stand-flight-return-pressed-removed` | `examples/seven_guis/src/tasks/flight_booker.rs` | `5a66dfcfcb60` | stand: exit 101, 21 passed, 3 failed — stand_act_diff.rs::a_choice_on_the_flight_booker_returns_the_control_it_enabled, stand_act_states.rs::a_flight_mode_click_returns_both_mode_buttons_with_their_pressed_reading, stand_snapshot_state.rs::exactly_the_active_flight_mode_reads_pressed |
| `stand-row-selected-removed` | `examples/seven_guis/src/tasks/crud.rs` | `73470c10bd12` | stand: exit 101, 21 passed, 3 failed — stand_act_diff.rs::create_returns_the_new_row_added_and_delete_returns_it_removed, stand_act_states.rs::a_row_click_returns_the_row_selected_and_the_row_it_leaves_not_selected, stand_snapshot_state.rs::exactly_the_selected_row_reads_selected |
| `stand-flight-start-invalid-removed` | `examples/seven_guis/src/tasks/flight_booker.rs` | `5a66dfcfcb60` | stand: exit 101, 22 passed, 2 failed — stand_act_states.rs::a_typed_start_date_returns_the_field_invalid_or_not, stand_snapshot_state.rs::a_date_that_is_no_date_reads_invalid |

What each control changed:

- `builder-*-read-removed` — the value arms of one attribute's `match` in `build_accessibility_node` removed (for
  `aria-invalid`, replaced by an arm that sets nothing).
- `builder-invalid-unrecognized-sets-nothing` — only the exact word `true` sets `Invalid::True`; any other
  unrecognized value sets nothing.
- `builder-reads-aria-checked-as-toggled` — an added read of `aria-checked="true"` into `toggled`. Not in the plan's
  list: it is the red of `aria_checked_sets_none_of_the_three`, which passes on the engine as built.
- `snapshot-*-default` — one field of the `NodeState` literal replaced by `None` / `false`.
- `snapshot-mixed-reads-pressed` — `toggled == Toggled::True` replaced by `toggled != Toggled::False`.
- `text-*-token-removed` — one state's block removed from `write_line`.
- `diff-compares-focus-only` — `was.state != node.state` replaced by a comparison of `focused` alone. Not in the
  plan's list: it is the red of the unit test step 7 adds.
- `json-*-key-removed` — one key's write removed from `node`.
- `schema-node-fields-reordered` — `selected` and `pressed` swapped in `NODE_FIELDS`. `schema-*-help-row-reworded` —
  one word of the table row's help changed (`unpressed` → `pressed`; `false` → `true`), the pin left as written. Not
  in the plan's list: they are the reds of the two restated schema tests.
- `stand-*-removed` — one attribute line removed from a task file.

Readings worth their own line:

- **`snapshot-mixed-reads-pressed` leaves the four stand files green** (24 passed): no stand screen shows a mixed
  toggle, so the limit is held by the two unit tests alone at the snapshot and in the text, and by
  `accessibility_states` at the engine (`Toggled::Mixed` on the node).
- **`snapshot-pressed-default` also fails `a_date_that_is_no_date_reads_invalid`**: that test reads the flight booker
  through the file's `stated` helper, which asserts all three readings of every node against the accessibility tree,
  so a pressed reading that disagrees with the tree stops it. The same helper is why no control of one state can pass
  a stand test of another on a screen that states both.
- **`a_mixed_toggle_is_written_unpressed_and_an_error_kind_invalid` fails under ten controls**: its one fixture holds
  a line for each of the three states.
- Every new test and every extended one read red at least once: the 4 of `accessibility_states`, the 3 added to
  `stand_snapshot_state`, the 3 of `stand_act_states`, the 2 restated in `stand_act_diff`, the 3 added to
  `snapshot.rs`, `state_tokens_follow_the_model` and the fixture test in `snapshot_text.rs`, the one in
  `snapshot_diff.rs`, `a_node_is_written_with_the_schema_fields_it_has`, and the two schema tests.

sha256 of each file, whole, before every control on it and after the batch:

| File | sha256 |
|---|---|
| `packages/blitz-dom/src/accessibility.rs` | `5551373f548be5eb4a802cf8c0c11a79dc5936ced05df32282e5380269d8d9ec` |
| `packages/dioxus-native-dom/src/snapshot.rs` | `6ea17a7e701db3595f583a403a9f02b1f77f8305fbfd3972b82cc1c5e447223a` |
| `packages/dioxus-native-dom/src/snapshot_text.rs` | `4046f488248f0c557e5e7ca6646be4480377256d68034dcd34b28add8f8d3b5f` |
| `packages/dioxus-native-dom/src/snapshot_diff.rs` | `97c3992a98a4aeff8e90c26161bbf91b1e5abbc0ba59fa7bbd504ecac2b6a383` |
| `packages/escher-driver/src/json.rs` | `dd36fe6aa14438f96ffb4f96f78cc5cce2729e5316c726379185e7a924878981` |
| `packages/escher-driver/src/schema.rs` | `3d138987805d352eb3443328b6cc9a866349d8df23cf4d25665f505f73320d4e` |
| `examples/seven_guis/src/tasks/flight_booker.rs` | `5a66dfcfcb601c202de994f1779f189e0cda21e23b371c907ec401c229366ac4` |
| `examples/seven_guis/src/tasks/crud.rs` | `73470c10bd12816121cbe850e8e4fef6547cf4f6fcf374cea975433a7e97e6f5` |

## 3. The two help texts against the plan

Both help texts were read out of `packages/escher-driver/src/schema.rs` with each string continuation folded as the
compiler folds it and compared with the wording the plan's step 8 states: the `text` help, 332 characters, is held by
exactly 2 literals of the file (the table row and its pin), and the `changed` help, 290 characters, by exactly 2.
