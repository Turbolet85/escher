# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · yes · stale · mutator.rs and document.rs are reached and gained lines, so the citations at mutator.rs:1769-1778 and document.rs:3194-3202 re-point with no claim moving
mocking-stubbing-discipline#1.2 · no · interaction_state_teardown.rs is not reached and the session does not touch RecordingShell or its set_ime_enabled record
mocking-stubbing-discipline#1.3 · no · link_rel_attribute.rs is not reached and RecordingNetProvider's URL record is untouched
mocking-stubbing-discipline#1.4 · no · render_blocking_stylesheet.rs is not reached and its ManualNetProvider is untouched
mocking-stubbing-discipline#1.5 · no · fixtures/mod.rs is not reached (only its reader count moves) and the task's Untouched list names HeldNet untouched
mocking-stubbing-discipline#1.6 · no · stand_load_wait.rs is not reached and the task's Untouched list names LateNet untouched
mocking-stubbing-discipline#1.7 · no · stand_load_page.rs gained asserts, but the task's Touches section shows its own load wait unchanged and LoadNet's stand.rs is not reached
mocking-stubbing-discipline#1.8 · no · settle.rs is not reached and the task keeps the blitz-test-harness API and semantics unchanged beyond its coverage read
mocking-stubbing-discipline#1.9 · no · the WPT runner and its net provider are untouched, the session reached none of WPT
mocking-stubbing-discipline#1.10 · yes · stale · blitz-vibey-script tests/dom.rs is reached and gained radio_click_fires_change_only_where_it_was_not_checked (dom 30 to 31), so MapFetcher's citations at 784-794 and 819 may have moved
mocking-stubbing-discipline#1.11 · no · the WPT runner's main.rs is untouched, the session reached none of WPT
mocking-stubbing-discipline#1.12 · no · packages/blitz-traits is not reached and its no-op providers are untouched
mocking-stubbing-discipline#1.13 · no · harness.rs is not reached and the task keeps the blitz-test-harness API and semantics unchanged beyond its coverage read
mocking-stubbing-discipline#1.14 · no · scrollbar_drag.rs is not reached and the task names no change to NoopEventHandler or EventDriver
mocking-stubbing-discipline#1.15 · no · custom_widget_layout.rs is not reached and the task names no change to the Probe or SizedProbe stand-ins
mocking-stubbing-discipline#1.16 · no · paint_tree_bench.rs is not reached and packages/blitz-paint shows no diff
mocking-stubbing-discipline#1.17 · no · the WPT runner's virtual time is untouched, the session reached none of WPT
mocking-stubbing-discipline#1.18 · no · none of TimerTicks' cited timer, stand or act files is reached and the task's Untouched list names TimerTicks untouched
mocking-stubbing-discipline#1.19 · no · packages/debug_timer is not reached and the task names no change to it
mocking-stubbing-discipline#1.20 · no · the WPT JS harness is untouched, the session reached none of WPT
mocking-stubbing-discipline#1.21 · no · apps/browser tests and helpers are untouched and url_suggestions.rs is not reached
mocking-stubbing-discipline#1.22 · no · apps/browser tests and helpers are untouched and url_suggestions.rs is not reached
mocking-stubbing-discipline#1.23 · no · wpt_diff_to_pr.py and its test file are not reached and the task keeps their Python test inventory unchanged
mocking-stubbing-discipline#2.1 · yes · stale · document.rs is reached and its line citations past the added text shift, so 3263-3272 and 3313-3321 re-point
mocking-stubbing-discipline#3.1 · yes · stale · ci.yml gained a free-disk step and test_ci_workflows.py five pins, both reached inside the search scope, and the task does not show the mock search holding in them
