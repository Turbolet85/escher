//! Every 0.1.0 requirement that holds on every screen is proven through the driver on each lean
//! stand task, in both layout modes, by `Session::run` alone: the screen text fits its budget
//! and gives every node one id, the ids pairwise distinct and each the `author_id` of an
//! accessibility node; a click on the heading returns an empty diff; a Tab press reads the first
//! stop of the task's Tab sequence as the one focused node; one id-addressed acting call returns
//! settled with a diff equal to this file's own; two screenshots are byte-equal 800 × 600 PNGs;
//! and an id no screen read is refused `not-found`, the instance unchanged and the remedy in
//! the refusal's written form. A stated table names, for each requirement, the check that
//! witnesses it on the task or fixture that holds its case, and every named test is checked to
//! exist. A second stated table does the same for each 0.2.0 requirement: the two that are no
//! behaviour of a screen name the crates' unit tests their acceptance places, which the `test`
//! leg runs and `stand-repeat` cannot select, and every other witness is checked to run inside
//! `stand-repeat` — a `stand_` file, one of the targets the leg names one by one, or one of its
//! seven_guis targets — each name read from the leg's own script. A screen's text is content:
//! this file prints none, and a failure message carries the
//! layout mode, the task and a step index, never an id or what a screen reads.

use std::collections::HashSet;

use blitz_dom::Document;
use dioxus_native_dom::SNAPSHOT_TEXT_BUDGET;
use escher_driver::{Call, Cause, Outcome, Session};
use seven_guis::stand::LeanTask;

mod common;
mod session_common;
use common::tab_order;
use session_common::{
    act, advance, click, focused, hold, press, refused_unchanged, screenshot, shot, snapshot,
    type_into,
};

/// What a node line of the screen text says after its role and name: its id, then its state
/// tokens with a quoted value read as one token. `None` for a line that holds no id field, or
/// more than one.
fn read_line(text_line: &str) -> Option<(&str, Vec<&str>)> {
    const FIELD: &str = " id=\"";
    if text_line.matches(FIELD).count() != 1 {
        return None;
    }
    let (_, rest) = text_line.split_once(FIELD)?;
    let (id, rest) = rest.split_once('"')?;
    let mut tokens = Vec::new();
    let mut rest = rest.trim_start();
    while !rest.is_empty() && !rest.starts_with('@') {
        let end = if let Some(quoted) = rest.strip_prefix("value=\"") {
            "value=\"".len() + closing_quote(quoted)? + 1
        } else {
            rest.find(' ').unwrap_or(rest.len())
        };
        tokens.push(&rest[..end]);
        rest = rest[end..].trim_start();
    }
    Some((id, tokens))
}

/// The byte offset of the quote that closes a `Debug`-quoted string whose opening quote has been
/// read.
fn closing_quote(quoted: &str) -> Option<usize> {
    let mut escaped = false;
    for (offset, character) in quoted.char_indices() {
        match character {
            '\\' if !escaped => escaped = true,
            '"' if !escaped => return Some(offset),
            _ => escaped = false,
        }
    }
    None
}

/// The screen text a `snapshot` call answers.
#[track_caller]
fn screen(session: &mut Session, at: &str) -> String {
    match session.run(&snapshot()) {
        Ok(Outcome::Screen { text }) => text,
        _ => panic!("{at}: a snapshot answers the screen"),
    }
}

/// The id-addressed acting call that changes what `task` shows: a click on a control with an
/// effect at boot, the flight type's return flight chosen by `type`, or for the timer an advance
/// of three ticks.
fn acting(task: LeanTask) -> Call {
    match task {
        LeanTask::Counter => click("counter-increment"),
        LeanTask::FlightBooker => type_into("flight-type", "return"),
        LeanTask::Timer => advance(300),
        LeanTask::Crud => click("crud-person-0"),
    }
}

#[test]
fn every_requirement_holds_through_the_driver_on_every_lean_task() {
    for incremental in [false, true] {
        for task in LeanTask::ALL {
            let mode = format!("incremental={incremental}: {task:?}");
            let (mut session, _ticks) = hold(task, incremental);

            // Step 0 (v010-04 · 01 · 03): the screen fits the budget, every node line carries
            // one id, the ids are distinct and each is an accessibility node's `author_id`.
            let at = format!("{mode}: step 0");
            let text = screen(&mut session, &at);
            assert!(
                text.len() <= SNAPSHOT_TEXT_BUDGET,
                "{at}: the screen fits the budget"
            );
            let lines: Vec<Option<(&str, Vec<&str>)>> = text.lines().map(read_line).collect();
            assert!(lines.len() >= 10, "{at}: the screen holds its nodes");
            assert!(
                lines.iter().all(Option::is_some),
                "{at}: every node line carries one id"
            );
            let ids: Vec<&str> = lines.iter().flatten().map(|(id, _)| *id).collect();
            let distinct: HashSet<&str> = ids.iter().copied().collect();
            assert!(distinct.len() == ids.len(), "{at}: the ids are distinct");
            let tree = session.harness().doc.accessibility_tree();
            let author_ids: HashSet<&str> = tree
                .nodes
                .iter()
                .filter_map(|(_, node)| node.author_id())
                .collect();
            assert!(
                ids.iter().all(|id| author_ids.contains(id)),
                "{at}: every id is an accessibility node's author id"
            );

            // Step 1 (v010-06): a click on an element that reacts to nothing returns an empty
            // diff.
            let at = format!("{mode}: step 1");
            let Some(idle) = act(&mut session, &click("task-title")) else {
                panic!("{at}: the click runs");
            };
            assert!(idle.settled, "{at}: the click returns settled");
            assert!(idle.diff.is_empty(), "{at}: the diff is empty");

            // Step 2 (v010-05): a Tab press focuses the first stop of the task's Tab sequence,
            // read back through the driver's screen and the session's focus.
            let at = format!("{mode}: step 2");
            assert!(
                act(&mut session, &press("tab", false)).is_some_and(|acted| acted.settled),
                "{at}: the Tab press runs and returns settled"
            );
            let text = screen(&mut session, &at);
            let focused_in_text: Vec<&str> = text
                .lines()
                .filter_map(read_line)
                .filter(|(_, tokens)| tokens.contains(&"focused"))
                .map(|(id, _)| id)
                .collect();
            let first = tab_order(task)[0];
            assert!(
                focused_in_text == [first],
                "{at}: the screen reads the first Tab stop as the one focused node"
            );
            let (in_snapshot, in_tree) = focused(&session);
            assert!(
                in_snapshot == [first] && in_tree == [first],
                "{at}: the session reads the same focus"
            );

            // Step 3 (v010-09 · 10 · 06): one id-addressed acting call returns settled with a
            // diff that is not empty and equals this file's own.
            let at = format!("{mode}: step 3");
            let before = session.harness().doc.snapshot();
            let Some(acted) = act(&mut session, &acting(task)) else {
                panic!("{at}: the acting call runs");
            };
            let after = session.harness().doc.snapshot();
            assert!(acted.settled, "{at}: the acting call returns settled");
            assert!(!acted.diff.is_empty(), "{at}: the diff is not empty");
            assert!(
                acted.diff == before.diff(&after),
                "{at}: the diff is the diff of the screens either side"
            );

            // Step 4 (v010-07 · 08): two screenshots are byte-equal 800 × 600 PNGs.
            let at = format!("{mode}: step 4");
            let (Some(first_shot), Some(second_shot)) = (
                shot(&mut session, &screenshot()),
                shot(&mut session, &screenshot()),
            ) else {
                panic!("{at}: both screenshots are accepted");
            };
            for one in [&first_shot, &second_shot] {
                assert!(
                    (one.width, one.height) == (800, 600),
                    "{at}: a shot is 800 × 600"
                );
                assert!(
                    one.png.starts_with(b"\x89PNG\r\n\x1a\n"),
                    "{at}: a shot is a PNG"
                );
            }
            assert!(
                first_shot.png == second_shot.png,
                "{at}: the two shots are byte-equal"
            );

            // Step 5 (v010-11 · 14): an id no screen read is refused `not-found`, the instance
            // unchanged, and the refusal's written form carries its remedy.
            let at = format!("{mode}: step 5");
            let unread = click("sweep-no-screen-reads-this");
            let (cause, unchanged) = refused_unchanged(&mut session, &unread);
            assert!(
                cause == Some(Cause::NotFound),
                "{at}: the click is refused not-found"
            );
            assert!(unchanged, "{at}: the refused click changes nothing");
            let Err(refusal) = session.run(&unread) else {
                panic!("{at}: the click is refused again");
            };
            assert!(
                refusal.to_json().contains(Cause::NotFound.remedy()),
                "{at}: the written refusal carries its remedy"
            );
        }
    }
}

/// What a witness check proves its requirement on.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Subject {
    /// Every lean task; for a verb that reaches no session, whichever task a host boots.
    AllFour,
    /// One lean task, the one that holds the requirement's case.
    Task(LeanTask),
    /// A minimal fixture beside the stand, booted with the stand's options.
    Fixture,
    /// A page the stand serves beside the lean four: Cells or the load page.
    Served,
    /// A crate's unit test, run by the workspace `test` leg: no `--test` target, so outside
    /// `stand-repeat` by construction.
    Unit,
}

use Subject::{AllFour, Fixture, Served, Task, Unit};

/// One check that witnesses a requirement: what it proves it on, its test file relative to the
/// project root, and the name of its `#[test]` function.
type Witness = (Subject, &'static str, &'static str);

const SWEEP: &str = "tests/blitz-tests/tests/stand_requirement_sweep.rs";
const SWEEP_TEST: &str = "every_requirement_holds_through_the_driver_on_every_lean_task";

/// Each 0.1.0 requirement this stand proves, in order, with the checks that witness it. A
/// requirement that holds on every screen names this file's sweep for all four lean tasks; a
/// requirement whose case one task or a fixture holds names the check that proves it there.
const WITNESSES: [(&str, &[Witness]); 16] = [
    (
        "v010-01",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_element_ids.rs",
                "every_element_reads_exactly_one_unique_id",
            ),
        ],
    ),
    (
        "v010-02",
        &[
            (
                AllFour,
                "tests/blitz-tests/tests/stand_session_ids.rs",
                "an_id_names_the_same_element_after_a_command",
            ),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_id_persistence.rs",
                "ids_hold_across_a_remount",
            ),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_id_persistence.rs",
                "ids_hold_across_a_fresh_process",
            ),
        ],
    ),
    (
        "v010-03",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_act_ids.rs",
                "the_snapshot_the_accessibility_tree_and_the_driver_read_one_id",
            ),
        ],
    ),
    (
        "v010-04",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                AllFour,
                "examples/seven_guis/tests/cli_commands.rs",
                "a_snapshot_is_the_in_process_text_as_one_line_inside_its_budget",
            ),
        ],
    ),
    (
        "v010-05",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                Task(LeanTask::FlightBooker),
                "tests/blitz-tests/tests/stand_snapshot_state.rs",
                "a_disabled_control_reads_disabled_until_the_app_enables_it",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_snapshot_state.rs",
                "typed_text_reads_back_as_the_value",
            ),
            (
                Task(LeanTask::Timer),
                "tests/blitz-tests/tests/stand_snapshot_state.rs",
                "the_range_input_reads_the_apps_duration",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_snapshot_state.rs",
                "focus_reads_on_exactly_the_focused_control",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_snapshot_state.rs",
                "a_checkbox_and_a_radio_read_checked_after_a_click",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_snapshot_state.rs",
                "a_typed_password_never_appears_in_the_snapshot",
            ),
        ],
    ),
    (
        "v010-06",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                Task(LeanTask::Counter),
                "tests/blitz-tests/tests/stand_act_diff.rs",
                "a_click_on_the_counter_returns_the_value_it_rewrote",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_act_diff.rs",
                "create_returns_the_new_row_added_and_delete_returns_it_removed",
            ),
        ],
    ),
    (
        "v010-07",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_act_screenshot.rs",
                "the_whole_screen_is_a_png_of_the_harness_raster_and_text_paints",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_act_screenshot.rs",
                "an_element_shot_is_its_bounds_rounded_outward",
            ),
        ],
    ),
    (
        "v010-08",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_screenshot_repeat.rs",
                "a_shot_of_one_state_is_the_same_in_two_runs_and_in_this_one",
            ),
        ],
    ),
    (
        "v010-09",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_act_ids.rs",
                "an_id_addressed_click_has_the_effect_of_a_selector_click_on_a_twin",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_act_keys.rs",
                "tab_and_shift_tab_return_exactly_the_controls_whose_focus_moved",
            ),
        ],
    ),
    (
        "v010-10",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                Task(LeanTask::Timer),
                "tests/blitz-tests/tests/stand_act_timer.rs",
                "a_reset_then_an_advance_returns_settled_with_the_delayed_update_present",
            ),
        ],
    ),
    (
        "v010-11",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_act_refused.rs",
                "one_id_is_not_found_before_a_screen_read_it_and_stale_after",
            ),
            (
                Task(LeanTask::FlightBooker),
                "tests/blitz-tests/tests/stand_act_disabled.rs",
                "a_disabled_date_input_refuses_typing_until_the_app_enables_it",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_act_disabled.rs",
                "a_disabled_delete_refuses_a_click_until_a_row_is_selected",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_act_scroll.rs",
                "a_row_past_the_list_is_refused_scrolled_into_view_and_then_clicked",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_act_obstructed.rs",
                "a_covered_target_is_refused_until_its_cover_is_gone_or_transparent",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_act_obstructed.rs",
                "a_target_beside_a_box_clipped_on_one_axis_is_refused_covered",
            ),
        ],
    ),
    (
        "v010-12",
        &[
            (
                AllFour,
                "examples/seven_guis/tests/cli_commands.rs",
                "each_task_starts_answers_and_stops",
            ),
            (
                Task(LeanTask::Crud),
                "examples/seven_guis/tests/cli_commands.rs",
                "each_verb_is_accepted_and_answers_its_fields",
            ),
            (
                Task(LeanTask::Counter),
                "examples/seven_guis/tests/cli_flow.rs",
                "each_flow_completes_under_plain_sh_and_leaves_nothing",
            ),
            (
                Task(LeanTask::FlightBooker),
                "examples/seven_guis/tests/cli_flow.rs",
                "each_flow_completes_under_plain_sh_and_leaves_nothing",
            ),
        ],
    ),
    (
        "v010-13",
        &[
            (
                Task(LeanTask::Counter),
                "tests/blitz-tests/tests/stand_mcp_tools.rs",
                "a_scripted_mcp_session_completes_the_counter_flow_and_parents_each_command",
            ),
            (
                Task(LeanTask::Counter),
                "examples/seven_guis/tests/mcp_server.rs",
                "the_mcp_role_completes_the_counter_flow_by_tool_calls_alone",
            ),
        ],
    ),
    (
        "v010-14",
        &[
            (AllFour, SWEEP, SWEEP_TEST),
            (
                AllFour,
                "examples/seven_guis/tests/cli_commands.rs",
                "help_answers_the_tools_description_and_reaches_no_session",
            ),
            (
                AllFour,
                "examples/seven_guis/tests/mcp_server.rs",
                "help_reads_the_same_bytes_through_mcp_and_the_command_line",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_mcp_tools.rs",
                "every_cause_a_tool_call_reaches_is_served_with_its_remedy",
            ),
        ],
    ),
    (
        "v010-16",
        &[
            (
                AllFour,
                "tests/blitz-tests/tests/stand_id_edits.rs",
                "a_task_keeps_every_id_when_the_code_around_it_changes",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_id_edits.rs",
                "keyed_elements_keep_their_ids_under_every_edit",
            ),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_actionable_keys.rs",
                "every_actionable_stand_element_reads_an_author_key",
            ),
        ],
    ),
    (
        "v010-17",
        &[
            (
                Task(LeanTask::FlightBooker),
                "tests/blitz-tests/tests/stand_act_states.rs",
                "a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading",
            ),
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_act_states.rs",
                "a_row_click_returns_the_row_selected_and_the_row_it_leaves_not_selected",
            ),
        ],
    ),
];

/// Each 0.2.0 requirement, in order, with the checks that witness it. The two that are no
/// behaviour of a screen name the unit tests their acceptance places: the code audit's
/// complexity and size readings and the mutation tier, run on the dev host alone, stay their
/// instruments.
const WITNESSES_020: [(&str, &[Witness]); 9] = [
    (
        "v020-01",
        &[
            (
                Unit,
                "packages/blitz-dom/src/document_changed_set_tests.rs",
                "changed_set_fresh_document_reads_no_change",
            ),
            (
                Unit,
                "packages/escher-driver/src/schema_tests.rs",
                "the_twelve_verbs_are_named_in_order_each_at_its_level",
            ),
            (
                Unit,
                "packages/escher-driver/src/mcp_tests.rs",
                "each_line_is_read_by_its_rules",
            ),
        ],
    ),
    (
        "v020-02",
        &[
            (
                Unit,
                "examples/seven_guis/src/tasks/timer.rs",
                "a_delivered_tick_is_taken_once_and_an_empty_wait_parks",
            ),
            (
                Unit,
                "packages/escher-driver/src/mcp_tests.rs",
                "read_request_takes_each_line_with_its_terminator",
            ),
            (
                Unit,
                "packages/dioxus-native-dom/src/dioxus_document_tests.rs",
                "a_created_text_node_is_mapped_and_left_on_the_stack",
            ),
        ],
    ),
    (
        "v020-03",
        &[
            (
                Fixture,
                "tests/blitz-tests/tests/inert_subtree.rs",
                "tab_and_shift_tab_skip_an_inert_subtree",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/inert_subtree.rs",
                "the_tree_holds_no_node_for_an_inert_subtree",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_keyboard.rs",
                "an_inert_subtree_is_skipped_by_tab",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/accessibility_states.rs",
                "aria_checked_reads_its_three_values_on_a_checkable_role_alone",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/accessibility_active_descendant_unread.rs",
                "a_focused_listbox_reads_focus_and_no_active_descendant",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/scrolled_box_absolute_position.rs",
                "a_scrolled_box_reads_its_absolute_position_where_it_stands",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs",
                "a_contain_paint_box_stops_the_hit_at_its_padding_box",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/password_caret_click.rs",
                "a_click_at_one_offset_places_the_caret_by_the_bullets",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_keyboard.rs",
                "a_disabled_attribute_skips_a_form_control_and_not_a_div",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/transformed_client_rect.rs",
                "a_translated_box_reads_its_rect_moved_by_the_translation",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_act_transformed.rs",
                "a_click_on_a_translated_button_is_accepted_and_lands",
            ),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_act_ids.rs",
                "the_snapshot_the_accessibility_tree_and_the_driver_read_one_id",
            ),
        ],
    ),
    (
        "v020-04",
        &[
            (
                Task(LeanTask::Counter),
                "tests/blitz-tests/tests/stand_replay.rs",
                "each_committed_check_holds_on_the_unchanged_stand",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_replay.rs",
                "each_committed_check_turns_red_at_its_first_activation_on_a_changed_counter",
            ),
            (
                Task(LeanTask::Counter),
                "examples/seven_guis/tests/recording.rs",
                "the_committed_checks_are_what_the_binary_records",
            ),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_session_recording.rs",
                "a_recorded_session_keeps_each_answer_but_a_shots_image_and_replays",
            ),
        ],
    ),
    (
        "v020-05",
        &[
            (
                Fixture,
                "tests/blitz-tests/tests/stand_lint.rs",
                "the_planted_defects_are_named_each_by_its_id_and_reason",
            ),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_lint.rs",
                "the_stand_at_boot_reads_its_measured_findings",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_lint.rs",
                "a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it",
            ),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_lint.rs",
                "the_lint_verb_answers_what_the_reader_reads",
            ),
            (
                Task(LeanTask::Counter),
                "examples/seven_guis/tests/cli_commands.rs",
                "lint_answers_one_line_of_findings",
            ),
            (
                AllFour,
                "tests/blitz-tests/tests/stand_mcp_tools.rs",
                "a_lint_tool_call_answers_the_readers_findings_and_parents_its_command",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/text_extent.rs",
                "right_to_left_text_reads_as_a_left_to_right_line_from_the_content_box",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/text_extent.rs",
                "vertical_text_reads_as_a_horizontal_line",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/text_extent.rs",
                "a_transformed_box_reads_its_lines_at_its_client_rect",
            ),
        ],
    ),
    (
        "v020-06",
        &[
            (
                Task(LeanTask::Crud),
                "tests/blitz-tests/tests/stand_virtual_clock.rs",
                "a_shot_after_a_scroll_repeats_until_advance_moves_the_fade",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/virtual_clock_engine.rs",
                "the_scrollbar_fade_moves_only_with_the_clock",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/virtual_clock_engine.rs",
                "a_smooth_scroll_stands_still_until_the_clock_moves",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/virtual_clock_engine.rs",
                "two_clicks_are_a_double_click_by_the_clock_alone",
            ),
        ],
    ),
    (
        "v020-07",
        &[
            (
                Fixture,
                "tests/blitz-tests/tests/stand_load_wait.rs",
                "a_load_answered_inside_the_bound_ends_the_step_settled_with_its_content_on_screen",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_load_wait.rs",
                "a_load_never_answered_ends_the_step_at_the_bound_still_loading",
            ),
            (
                Served,
                "tests/blitz-tests/tests/stand_load_page.rs",
                "the_load_page_click_waits_out_its_load_and_shows_the_message",
            ),
        ],
    ),
    (
        "v020-08",
        &[
            (
                Fixture,
                "tests/blitz-tests/tests/stand_act_select.rs",
                "at_boot_the_snapshot_reads_the_select_its_name_value_and_options",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_act_select.rs",
                "type_with_a_label_chooses_that_option",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_act_select.rs",
                "the_tab_walk_skips_the_disabled_select_before_and_after_the_choices",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/select_interaction.rs",
                "the_arrows_home_and_end_choose_an_enabled_option",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/select_interaction.rs",
                "typed_text_is_kept_for_one_second_of_the_documents_clock",
            ),
            (
                Unit,
                "packages/blitz-dom/src/node/select.rs",
                "the_selectedness_setting_algorithm_keeps_one_chosen_option",
            ),
            (
                Task(LeanTask::FlightBooker),
                "tests/blitz-tests/tests/stand_act_states.rs",
                "a_flight_type_choice_returns_the_select_value_and_both_options_selected_reading",
            ),
        ],
    ),
    (
        "v020-09",
        &[
            (
                Fixture,
                "tests/blitz-tests/tests/stand_snapshot_parts.rs",
                "a_screen_over_the_budget_is_walked_in_parts",
            ),
            (
                Fixture,
                "tests/blitz-tests/tests/stand_snapshot_parts.rs",
                "over_mcp_the_walk_answers_the_json_lines_of_the_same_calls",
            ),
            (
                Task(LeanTask::Counter),
                "examples/seven_guis/tests/cli_commands.rs",
                "snapshot_reads_a_part_by_number_and_a_part_outside_its_bound_is_refused",
            ),
            (
                Served,
                "tests/blitz-tests/tests/stand_cells_served.rs",
                "cells_is_walked_in_parts_through_the_driver",
            ),
        ],
    ),
];

/// The blitz-tests targets outside the `stand_` prefix that `stand-repeat` names one by one.
const STAND_REPEAT_NAMED: [&str; 10] = [
    "inert_subtree",
    "accessibility_states",
    "scrolled_box_absolute_position",
    "hit_clipped_at_scrolling_box",
    "password_caret_click",
    "transformed_client_rect",
    "accessibility_active_descendant_unread",
    "virtual_clock_engine",
    "select_interaction",
    "text_extent",
];

/// The seven_guis targets `stand-repeat` runs.
const STAND_REPEAT_SEVEN_GUIS: [&str; 8] = [
    "cli_commands",
    "cli_flow",
    "host_binary",
    "host_log",
    "host_timer",
    "mcp_server",
    "mcp_log",
    "recording",
];

const CI_LEG: &str = ".github/scripts/ci-leg.sh";

/// The text of a witness file, read at compile time: `None` for a file this check does not
/// carry.
fn source(file: &str) -> Option<&'static str> {
    Some(match file {
        SWEEP => include_str!("stand_requirement_sweep.rs"),
        "tests/blitz-tests/tests/stand_element_ids.rs" => include_str!("stand_element_ids.rs"),
        "tests/blitz-tests/tests/stand_session_ids.rs" => include_str!("stand_session_ids.rs"),
        "tests/blitz-tests/tests/stand_id_persistence.rs" => {
            include_str!("stand_id_persistence.rs")
        }
        "tests/blitz-tests/tests/stand_act_ids.rs" => include_str!("stand_act_ids.rs"),
        "tests/blitz-tests/tests/stand_snapshot_state.rs" => {
            include_str!("stand_snapshot_state.rs")
        }
        "tests/blitz-tests/tests/stand_act_diff.rs" => include_str!("stand_act_diff.rs"),
        "tests/blitz-tests/tests/stand_act_screenshot.rs" => {
            include_str!("stand_act_screenshot.rs")
        }
        "tests/blitz-tests/tests/stand_screenshot_repeat.rs" => {
            include_str!("stand_screenshot_repeat.rs")
        }
        "tests/blitz-tests/tests/stand_act_keys.rs" => include_str!("stand_act_keys.rs"),
        "tests/blitz-tests/tests/stand_act_timer.rs" => include_str!("stand_act_timer.rs"),
        "tests/blitz-tests/tests/stand_act_refused.rs" => include_str!("stand_act_refused.rs"),
        "tests/blitz-tests/tests/stand_act_disabled.rs" => include_str!("stand_act_disabled.rs"),
        "tests/blitz-tests/tests/stand_act_scroll.rs" => include_str!("stand_act_scroll.rs"),
        "tests/blitz-tests/tests/stand_act_obstructed.rs" => {
            include_str!("stand_act_obstructed.rs")
        }
        "tests/blitz-tests/tests/stand_mcp_tools.rs" => include_str!("stand_mcp_tools.rs"),
        "tests/blitz-tests/tests/stand_id_edits.rs" => include_str!("stand_id_edits.rs"),
        "tests/blitz-tests/tests/stand_actionable_keys.rs" => {
            include_str!("stand_actionable_keys.rs")
        }
        "tests/blitz-tests/tests/stand_act_states.rs" => include_str!("stand_act_states.rs"),
        "examples/seven_guis/tests/cli_commands.rs" => {
            include_str!("../../../examples/seven_guis/tests/cli_commands.rs")
        }
        "examples/seven_guis/tests/cli_flow.rs" => {
            include_str!("../../../examples/seven_guis/tests/cli_flow.rs")
        }
        "examples/seven_guis/tests/mcp_server.rs" => {
            include_str!("../../../examples/seven_guis/tests/mcp_server.rs")
        }
        "tests/blitz-tests/tests/inert_subtree.rs" => include_str!("inert_subtree.rs"),
        "tests/blitz-tests/tests/stand_keyboard.rs" => include_str!("stand_keyboard.rs"),
        "tests/blitz-tests/tests/accessibility_states.rs" => {
            include_str!("accessibility_states.rs")
        }
        "tests/blitz-tests/tests/accessibility_active_descendant_unread.rs" => {
            include_str!("accessibility_active_descendant_unread.rs")
        }
        "tests/blitz-tests/tests/scrolled_box_absolute_position.rs" => {
            include_str!("scrolled_box_absolute_position.rs")
        }
        "tests/blitz-tests/tests/hit_clipped_at_scrolling_box.rs" => {
            include_str!("hit_clipped_at_scrolling_box.rs")
        }
        "tests/blitz-tests/tests/password_caret_click.rs" => {
            include_str!("password_caret_click.rs")
        }
        "tests/blitz-tests/tests/transformed_client_rect.rs" => {
            include_str!("transformed_client_rect.rs")
        }
        "tests/blitz-tests/tests/stand_act_transformed.rs" => {
            include_str!("stand_act_transformed.rs")
        }
        "tests/blitz-tests/tests/stand_replay.rs" => include_str!("stand_replay.rs"),
        "tests/blitz-tests/tests/stand_session_recording.rs" => {
            include_str!("stand_session_recording.rs")
        }
        "tests/blitz-tests/tests/stand_lint.rs" => include_str!("stand_lint.rs"),
        "tests/blitz-tests/tests/text_extent.rs" => include_str!("text_extent.rs"),
        "tests/blitz-tests/tests/stand_virtual_clock.rs" => include_str!("stand_virtual_clock.rs"),
        "tests/blitz-tests/tests/virtual_clock_engine.rs" => {
            include_str!("virtual_clock_engine.rs")
        }
        "tests/blitz-tests/tests/stand_load_wait.rs" => include_str!("stand_load_wait.rs"),
        "tests/blitz-tests/tests/stand_load_page.rs" => include_str!("stand_load_page.rs"),
        "tests/blitz-tests/tests/stand_cells_served.rs" => include_str!("stand_cells_served.rs"),
        "tests/blitz-tests/tests/stand_act_select.rs" => include_str!("stand_act_select.rs"),
        "tests/blitz-tests/tests/select_interaction.rs" => include_str!("select_interaction.rs"),
        "tests/blitz-tests/tests/stand_snapshot_parts.rs" => {
            include_str!("stand_snapshot_parts.rs")
        }
        "examples/seven_guis/tests/recording.rs" => {
            include_str!("../../../examples/seven_guis/tests/recording.rs")
        }
        "examples/seven_guis/src/tasks/timer.rs" => {
            include_str!("../../../examples/seven_guis/src/tasks/timer.rs")
        }
        "packages/blitz-dom/src/document_changed_set_tests.rs" => {
            include_str!("../../../packages/blitz-dom/src/document_changed_set_tests.rs")
        }
        "packages/blitz-dom/src/node/select.rs" => {
            include_str!("../../../packages/blitz-dom/src/node/select.rs")
        }
        "packages/escher-driver/src/schema_tests.rs" => {
            include_str!("../../../packages/escher-driver/src/schema_tests.rs")
        }
        "packages/escher-driver/src/mcp_tests.rs" => {
            include_str!("../../../packages/escher-driver/src/mcp_tests.rs")
        }
        "packages/dioxus-native-dom/src/dioxus_document_tests.rs" => {
            include_str!("../../../packages/dioxus-native-dom/src/dioxus_document_tests.rs")
        }
        CI_LEG => include_str!("../../../.github/scripts/ci-leg.sh"),
        _ => return None,
    })
}

#[test]
fn every_requirement_names_its_witness() {
    const EVERY_SCREEN: [&str; 11] = [
        "v010-01", "v010-03", "v010-04", "v010-05", "v010-06", "v010-07", "v010-08", "v010-09",
        "v010-10", "v010-11", "v010-14",
    ];
    let requirements: Vec<String> = (1..=17)
        .filter(|n| *n != 15)
        .map(|n| format!("v010-{n:02}"))
        .collect();
    assert_eq!(WITNESSES.len(), 16);
    assert!(
        WITNESSES
            .iter()
            .map(|(requirement, _)| *requirement)
            .eq(requirements.iter().map(String::as_str)),
        "the rows name the requirements in order"
    );
    for (row, (requirement, witnesses)) in WITNESSES.iter().enumerate() {
        assert!(!witnesses.is_empty(), "row {row}: names a witness");
        assert_eq!(
            witnesses.contains(&(AllFour, SWEEP, SWEEP_TEST)),
            EVERY_SCREEN.contains(requirement),
            "row {row}: names the sweep for all four exactly when it holds on every screen"
        );
        for (index, (_, file, test)) in witnesses.iter().enumerate() {
            let Some(text) = source(file) else {
                panic!("row {row}, witness {index}: the file is carried");
            };
            assert!(
                text.contains(&format!("fn {test}(")),
                "row {row}, witness {index}: the test exists in its file"
            );
        }
    }
}

#[test]
fn every_0_2_0_requirement_names_its_witness_inside_stand_repeat() {
    const BLITZ_TESTS: &str = "tests/blitz-tests/tests/";
    const SEVEN_GUIS: &str = "examples/seven_guis/tests/";
    let requirements: Vec<String> = (1..=9).map(|n| format!("v020-{n:02}")).collect();
    assert_eq!(WITNESSES_020.len(), 9);
    assert!(
        WITNESSES_020
            .iter()
            .map(|(requirement, _)| *requirement)
            .eq(requirements.iter().map(String::as_str)),
        "the rows name the requirements in order"
    );

    let Some(leg) = source(CI_LEG) else {
        panic!("the leg script is carried");
    };
    assert!(
        leg.contains("tests/blitz-tests/tests/stand_*.rs"),
        "stand-repeat selects every stand_ file"
    );
    for (index, name) in STAND_REPEAT_NAMED
        .iter()
        .chain(&STAND_REPEAT_SEVEN_GUIS)
        .enumerate()
    {
        assert!(
            leg.contains(&format!("--test {name}")),
            "named target {index}: the leg runs it"
        );
    }

    for (row, (requirement, witnesses)) in WITNESSES_020.iter().enumerate() {
        assert!(!witnesses.is_empty(), "row {row}: names a witness");
        let units = witnesses
            .iter()
            .filter(|(subject, _, _)| *subject == Unit)
            .count();
        if ["v020-01", "v020-02"].contains(requirement) {
            assert!(
                units == witnesses.len(),
                "row {row}: names unit tests alone"
            );
        } else {
            assert!(
                units < witnesses.len(),
                "row {row}: names a witness stand-repeat runs"
            );
        }
        for (index, (subject, file, test)) in witnesses.iter().enumerate() {
            let Some(text) = source(file) else {
                panic!("row {row}, witness {index}: the file is carried");
            };
            assert!(
                text.contains(&format!("fn {test}(")),
                "row {row}, witness {index}: the test exists in its file"
            );
            if *subject == Unit {
                continue;
            }
            let inside = if let Some(name) = file.strip_prefix(BLITZ_TESTS) {
                let target = name.strip_suffix(".rs").unwrap_or(name);
                target.starts_with("stand_") || STAND_REPEAT_NAMED.contains(&target)
            } else if let Some(name) = file.strip_prefix(SEVEN_GUIS) {
                let target = name.strip_suffix(".rs").unwrap_or(name);
                STAND_REPEAT_SEVEN_GUIS.contains(&target)
            } else {
                false
            };
            assert!(
                inside,
                "row {row}, witness {index}: stand-repeat runs its target"
            );
        }
    }
}
