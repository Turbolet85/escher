# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · stale · mutator.rs and document.rs are merge surfaces that gained tests and inserted lines, so the cited lines 1917-1926 and 3565-3573 can move; the fakes still count redraws
mocking-stubbing-discipline#1.2 · no · interaction_state_teardown.rs is not in the reached files and RecordingShell's claim is unaffected
mocking-stubbing-discipline#1.3 · yes · stale · select_popup.rs is reached and lost the row-3 skip near line 513, so the cited lines 1030-1086 can shift; the redraw claim holds
mocking-stubbing-discipline#1.4 · no · blitz-shell event.rs is not reached and CountedWakes is unaffected
mocking-stubbing-discipline#1.5 · no · link_rel_attribute.rs is not reached and RecordingNetProvider still records URLs
mocking-stubbing-discipline#1.6 · yes · stale · render_blocking_stylesheet.rs is a merge surface that gained 3 tests, so cited lines 16-30 and 131-135 can move
mocking-stubbing-discipline#1.7 · no · HeldNet's files are not reached and it still answers through bytes, so its claim holds
mocking-stubbing-discipline#1.8 · no · stand_load_wait.rs is not reached and LateNet answers successfully, so its claim holds
mocking-stubbing-discipline#1.9 · no · examples/seven_guis has no diff and LoadNet's file is not reached
mocking-stubbing-discipline#1.10 · yes · stale · settle.rs is reached and gained the error-forwarding counter change and a new test, so cited lines 213-248 and 323-380 can shift
mocking-stubbing-discipline#1.11 · no · WPT runner sources are not reached
mocking-stubbing-discipline#1.12 · no · dom.rs is not reached and MapFetcher's NotFound behaviour is unchanged
mocking-stubbing-discipline#1.13 · no · WPT runner main.rs is not reached
mocking-stubbing-discipline#1.14 · yes · stale · blitz-traits net.rs is a merge surface, so the no-op provider lines 168-176 can move
mocking-stubbing-discipline#1.15 · no · harness.rs is not reached and RecordingHandler is unchanged
mocking-stubbing-discipline#1.16 · no · scrollbar_drag.rs is not reached
mocking-stubbing-discipline#1.17 · no · custom_widget_layout.rs is not reached and the Probe stubs still implement Widget
mocking-stubbing-discipline#1.18 · no · paint_tree_bench.rs is not reached
mocking-stubbing-discipline#1.19 · no · WPT runner sources are not reached
mocking-stubbing-discipline#1.20 · no · examples/seven_guis has no diff and stand_timer.rs and stand_settle.rs are not reached
mocking-stubbing-discipline#1.21 · no · packages/debug_timer is not reached
mocking-stubbing-discipline#1.22 · no · WPT runner sources are not reached
mocking-stubbing-discipline#1.23 · no · no reached file lies under apps/
mocking-stubbing-discipline#1.24 · no · no reached file lies under apps/
mocking-stubbing-discipline#1.25 · no · wpt_diff_to_pr.py is not reached and its dry-run is unchanged
