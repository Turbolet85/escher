# Classified — test-plan · mocking-stubbing-discipline.b1

## Answers
mocking-stubbing-discipline#1.1 · no · packages/ is unchanged (its diff is 0) and mutator.rs and document.rs are not reached
mocking-stubbing-discipline#1.2 · no · interaction_state_teardown.rs is not reached and its RecordingShell is unchanged
mocking-stubbing-discipline#1.3 · no · link_rel_attribute.rs is not reached and the session adds no network fake to it
mocking-stubbing-discipline#1.4 · no · render_blocking_stylesheet.rs is not reached and upstream's file-private ManualNetProvider is unchanged
mocking-stubbing-discipline#1.5 · yes · add-to · F2, LoadNet, the load page's third hand-written network stand-in, sits beside HeldNet and LateNet; HeldNet's own description still stands (fixtures/mod.rs is untouched)
mocking-stubbing-discipline#1.6 · yes · add-to · F2, LoadNet, the third hand-written network stand-in, answers from its own thread after 200 ms beside LateNet; LateNet's own description still stands
mocking-stubbing-discipline#1.7 · no · packages/blitz-test-harness/src/settle.rs is unchanged (packages diff 0) and not reached; its Held NetProvider is not in the stand's network group
mocking-stubbing-discipline#1.8 · no · wpt/runner is not reached and the session changed no WPT file
mocking-stubbing-discipline#1.9 · no · packages/blitz-vibey-script is unchanged (packages diff 0) and its dom.rs is not reached
mocking-stubbing-discipline#1.10 · no · wpt/runner/src/main.rs is not reached and the session changed no WPT file
mocking-stubbing-discipline#1.11 · no · packages/blitz-traits is unchanged (packages diff 0)
mocking-stubbing-discipline#1.12 · no · the Harness API and HarnessOptions' fields are untouched and harness.rs is not reached
mocking-stubbing-discipline#1.13 · no · scrollbar_drag.rs is not reached and its NoopEventHandler is unchanged
mocking-stubbing-discipline#1.14 · no · custom_widget_layout.rs is not reached and no widget changed
mocking-stubbing-discipline#1.15 · no · paint_tree_bench.rs is not reached and no paint code changed
mocking-stubbing-discipline#1.16 · no · wpt/runner is not reached and the session changed no WPT file
mocking-stubbing-discipline#1.17 · yes · stale · stand.rs, a reached file, gained lines at 1-9, 12 and 17, so the cited stand.rs lines 92-109 moved
mocking-stubbing-discipline#1.18 · no · packages/debug_timer is unchanged (packages diff 0)
mocking-stubbing-discipline#1.19 · no · wpt/runner's harness_test.rs is not reached and the session changed no WPT file
mocking-stubbing-discipline#1.20 · no · apps/browser is not reached and no app file changed
mocking-stubbing-discipline#1.21 · no · apps/browser is not reached and no app file changed
mocking-stubbing-discipline#1.22 · no · wpt_diff_to_pr.py and its test are not reached and the session changed neither
mocking-stubbing-discipline#2.1 · no · packages/blitz-dom is unchanged (packages diff 0), so document.rs's hover_state_tests and their skip history stand
mocking-stubbing-discipline#3.1 · yes · stale · test_cold_agent.py, a reached file inside the searched .github/scripts/*.py scope, grew from 33 to 37 cases with a new fake server, and the report shows no mock search over it
mocking-stubbing-discipline#3.2 · no · apps/ is not reached and Cargo.toml is unchanged, so the mockall or mock search over the apps still reads the same
