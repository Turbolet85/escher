# Red before green — the tests this chunk writes (2026-10-24, /implement P1)

Each test first read green on the written tree (stand_lint 7 passed, text_extent 6, host_log 3, stand_requirement_sweep 3,
stand_act_spans 2 passed 2 ignored, stand_snapshot_parts 3). Then one mutation at a time, applied by a scratchpad script
(`$TMPDIR/claude-1000/…/scratchpad/controls.py`), the named test run alone, the file's bytes written back and compared by
sha256 before the next. Zero `error[` / `could not compile` lines over the whole control log: each red is an assertion.

| Control | Mutation | Test | Reading | Restored |
|---|---|---|---|---|
| lint co-reading | `fx-cut-text`'s line → `"Go"` (stays inside its clip box) | `stand_lint::a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it` | exit 101, FAILED at `stand_lint.rs:256` (the findings list) | sha256 equal |
| rtl twin | `#rtl`'s word → `Words` | `text_extent::right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box` | exit 101, FAILED at `text_extent.rs:143` | sha256 equal |
| vertical twin | `#vertical`'s word → `Words` | `text_extent::vertical_text_reads_as_a_horizontal_line` | exit 101, FAILED at `text_extent.rs:165` | sha256 equal |
| scaled box | `scale(2)` → `scale(3)` | `text_extent::a_transformed_box_reads_its_lines_at_its_client_rect` | exit 101, FAILED at `text_extent.rs:189` | sha256 equal |
| leg drops a named witness | `--test text_extent` removed from `ci-leg.sh` | `stand_requirement_sweep::every_0_2_0_requirement_names_its_witness_inside_stand_repeat` | exit 101, FAILED at `stand_requirement_sweep.rs:1020` (named target 9) | sha256 equal |
| paged read on an unpaged page | the paged hold boots `fixtures::covered` | `stand_act_spans::each_driver_command_leaves_one_line_holding_nothing_it_handled` | exit 101, the child FAILED at `stand_act_spans.rs:687` (part 2 of a paged screen) | sha256 equal |
| client stderr holds the sentinel | the sentinel planted into the joined client stderr | `host_log::a_recording_sessions_clients_log_nothing_of_it_at_trace` | exit 101, FAILED at `host_log.rs:121` (the needle counts) | sha256 equal |
