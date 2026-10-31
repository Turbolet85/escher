# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · add-to · F13: select_popup's own hand-written CountingShell, a ShellProvider counting request_redraw, joins this shell-fake row, and document.rs gains lines above its cited 3533-3541
mocking-stubbing-discipline#1.2 · no · RecordingShell lives in interaction_state_teardown.rs, which the session did not reach
mocking-stubbing-discipline#1.3 · no · CountedWakes sits in blitz-shell's event.rs, which is not a reached file
mocking-stubbing-discipline#1.4 · no · link_rel_attribute.rs is not a reached file and the session adds no network stand-in
mocking-stubbing-discipline#1.5 · no · render_blocking_stylesheet.rs is not a reached file and its ManualNetProvider is untouched
mocking-stubbing-discipline#1.6 · no · fixtures/mod.rs is unedited and not reached, so HeldNet's claim holds and the session adds no network stand-in
mocking-stubbing-discipline#1.7 · no · stand_load_wait.rs is not a reached file and its in-file LateNet stands
mocking-stubbing-discipline#1.8 · no · LoadNet's stand.rs and stand_load_page.rs are not reached and the session does not touch the load page
mocking-stubbing-discipline#1.9 · no · settle.rs is not a reached file and the blitz-test-harness API is unedited
mocking-stubbing-discipline#1.10 · no · WPT runner files are outside the changed files
mocking-stubbing-discipline#1.11 · no · blitz-vibey-script tests are untouched
mocking-stubbing-discipline#1.12 · no · wpt/runner main.rs is untouched
mocking-stubbing-discipline#1.13 · no · blitz-traits' net, navigation and shell no-op files are not reached
mocking-stubbing-discipline#1.14 · no · harness.rs is not a reached file and dispatch_recorded is unchanged
mocking-stubbing-discipline#1.15 · no · scrollbar_drag.rs is not a reached file
mocking-stubbing-discipline#1.16 · no · custom_widget_layout.rs is not a reached file
mocking-stubbing-discipline#1.17 · no · paint_tree_bench.rs is not a reached file
mocking-stubbing-discipline#1.18 · no · WPT timers sit outside the changed files
mocking-stubbing-discipline#1.19 · no · the seven_guis TimerTicks and timer files are not reached and the session does not move the timer
mocking-stubbing-discipline#1.20 · no · debug_timer is not a reached file
mocking-stubbing-discipline#1.21 · no · WPT JS harness files are untouched
mocking-stubbing-discipline#1.22 · no · apps/browser is untouched
mocking-stubbing-discipline#1.23 · no · apps/browser's no-op notify closure is untouched
mocking-stubbing-discipline#1.24 · no · wpt_diff_to_pr.py is not reached, and the session touched only ci-leg.sh and test_ci_workflows.py under .github/scripts
mocking-stubbing-discipline#2.1 · yes · stale · document.rs is reached whole and gains lines above 3602, so the cited hover_state_tests lines 3602-3611 and 3652-3660 move
