@answer n=1 entity=integration-test-strategy#1.1 v=no
why: the cited pseudo-element test file and its incremental loop are not among the session's reached files
@answer n=2 entity=integration-test-strategy#2.1 v=no
why: blitz-vibey-script is not reached by the session, which touches none of its dom or selection tests
@answer n=3 entity=integration-test-strategy#3.1 v=no
why: the radio change test lives in blitz-vibey-script, which the session does not reach
@answer n=4 entity=integration-test-strategy#4.1 v=no
why: the URLSearchParams tests live in blitz-vibey-script, which the session does not reach
@answer n=5 entity=integration-test-strategy#5.1 v=no
why: the held-instance stand files it cites are not reached, and the session moves no count or claim of theirs
@answer n=6 entity=integration-test-strategy#6.1 v=no
why: the load-in-flight fetch lives in stand_actions, which the session does not reach
@answer n=7 entity=integration-test-strategy#7.1 v=yes mode=add-to
why: the UI graph's served handlers and owners gain stand_handler_coverage, which holds handler_coverage over the six served tasks (F1)
@answer n=8 entity=integration-test-strategy#8.1 v=no
why: stand_actions and the actions answer it checks are not reached by the session
@answer n=9 entity=integration-test-strategy#9.1 v=no
why: the Cells actions answer size and stand_actions are not reached, the session adding no actions output
@answer n=10 entity=integration-test-strategy#10.1 v=yes mode=contradict
why: stand_replay now holds a_kept_actions_step_replays_and_a_changed_one_stops_naming_nodes, discharging the owed kept-actions replay test (F4)
@answer n=11 entity=integration-test-strategy#11.1 v=no
why: the MCP graph and actions tool tests in stand_mcp_tools are not reached, and the coverage role opens no span
@answer n=12 entity=integration-test-strategy#12.1 v=yes mode=stale
why: cli_commands is reached whole with its usage table and tests moved, and the report does not show the graph refusal still holds there
@answer n=13 entity=integration-test-strategy#13.1 v=yes mode=stale
why: cli_commands is reached whole with its usage table and tests moved, and the report does not show the actions refusal still holds there
@answer n=14 entity=integration-test-strategy#14.1 v=yes mode=stale
why: cli_commands.rs, reached whole with a new coverage test and usage row, can have moved the cited lines 140-222
@answer n=15 entity=integration-test-strategy#14.2 v=yes mode=stale
why: host_log's test count moves 5 to 6 with a coverage run at trace (F6), and host_log.rs is reached whole
@answer n=16 entity=integration-test-strategy#15.1 v=yes mode=stale
why: host_log.rs, reached whole with the coverage test added, can have moved the cited select-list test lines 373-484
@answer n=17 entity=integration-test-strategy#16.1 v=yes mode=stale
why: cli_commands.rs, reached whole with its usage row and coverage test added, can have moved the cited lines 425-470
@answer n=18 entity=integration-test-strategy#17.1 v=no
why: stand_act_select is not reached and the multiple-select claims it holds are untouched
@answer n=19 entity=integration-test-strategy#18.1 v=no
why: stand_act_popup is not reached and its list claims are untouched
@answer n=20 entity=integration-test-strategy#19.1 v=no
why: select_popup is not reached and the engine's list reading is unchanged
@answer n=21 entity=integration-test-strategy#20.1 v=no
why: select_popup_painted is not reached and the paint reading is unchanged
@answer n=22 entity=integration-test-strategy#21.1 v=no
why: render_blocking_stylesheet is not reached by the session
@answer n=23 entity=integration-test-strategy#22.1 v=no
why: the listener rule lives in mutation_writer.rs and dioxus_runtime_probe.rs, neither reached, and the new crate-private read leaves the attribute and mark behaviour unchanged
@answer n=24 entity=integration-test-strategy#23.1 v=no
why: the signal-reader claims live in dioxus_runtime_probe, the graph tests and the stand's tasks, none reached, and no manifest moved
@answer n=25 entity=integration-test-strategy#24.1 v=yes mode=stale
why: host_log.rs and cli_commands.rs, reached whole, gain integration #[test]s outside src, moving the search reading over the apps' test files
