# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · add-to · F13: select_popup's own hand-written CountingShell (a ShellProvider counting request_redraw) joins this hand-written shell-fake row; the cited document.rs:3533-3541 also shifts with the lines added above it in the reached document.rs
mocking-stubbing-discipline#1.2 · no · interaction_state_teardown.rs is not a reached file and the RecordingShell set_ime_enabled record still holds
mocking-stubbing-discipline#1.3 · no · blitz-shell event.rs and its CountedWakes test are not reached; the session adds no wake counter
mocking-stubbing-discipline#1.4 · no · link_rel_attribute.rs is not reached and RecordingNetProvider's URL record is unchanged
mocking-stubbing-discipline#1.5 · no · render_blocking_stylesheet.rs is not reached and ManualNetProvider is unchanged
mocking-stubbing-discipline#1.6 · no · fixtures/mod.rs is not reached and stays unedited; HeldNet keeps its three stand-in roles and the session adds no network stand-in
mocking-stubbing-discipline#1.7 · no · stand_load_wait.rs is not reached; LateNet's self-answering claim stands
mocking-stubbing-discipline#1.8 · no · LoadNet sits in examples/seven_guis/src/stand.rs and stand_load_page.rs, neither reached; the session adds no network stand-in
mocking-stubbing-discipline#1.9 · no · blitz-test-harness settle.rs is not reached; the Scripted Document and Held NetProvider stand
mocking-stubbing-discipline#1.10 · no · the WPT runner's net_provider.rs is outside the changed files
mocking-stubbing-discipline#1.11 · no · blitz-vibey-script tests are untouched and not reached
mocking-stubbing-discipline#1.12 · no · wpt/runner/src/main.rs is outside the changed files
mocking-stubbing-discipline#1.13 · no · the blitz-traits net, navigation and shell files are not reached
mocking-stubbing-discipline#1.14 · no · harness.rs is not reached and the harness API is unedited, so RecordingHandler stands
mocking-stubbing-discipline#1.15 · no · scrollbar_drag.rs is not reached
mocking-stubbing-discipline#1.16 · no · custom_widget_layout.rs is not reached
mocking-stubbing-discipline#1.17 · no · paint_tree_bench.rs is not reached
mocking-stubbing-discipline#1.18 · no · WPT virtual time is outside the changed files
mocking-stubbing-discipline#1.19 · no · the TimerTicks sources (timer.rs, stand.rs, stand_timer.rs, stand_settle.rs, stand_act_timer.rs) are not reached and the timer step is unchanged
mocking-stubbing-discipline#1.20 · no · debug_timer is not reached
mocking-stubbing-discipline#1.21 · no · the WPT JS harness is outside the changed files
mocking-stubbing-discipline#1.22 · no · apps/browser is untouched and not reached
mocking-stubbing-discipline#1.23 · no · apps/browser url_suggestions.rs is untouched and not reached
mocking-stubbing-discipline#1.24 · no · wpt_diff_to_pr.py and its test are not reached; the reached ci-leg.sh and test_ci_workflows.py hold no GitHub API fake
mocking-stubbing-discipline#2.1 · yes · stale · the hover_state_tests citations at document.rs:3602-3611 and :3652-3660 shift with the lines added above them in the reached document.rs (761-1089, 1408-1410, 2404-2406); the skip claim itself does not move
